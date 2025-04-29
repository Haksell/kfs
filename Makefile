DEBUG ?= false
ifeq ($(DEBUG), true)
BUILD_MODE := debug
else ifeq ($(DEBUG),false)
BUILD_MODE := release
CARGO_FLAGS := --release
GRUB_FLAGS := --compress xz
else
$(error DEBUG must be either true or false)
endif

BUILD := build/$(BUILD_MODE)
ISOFILES := $(BUILD)/isofiles
KERNEL := $(BUILD)/kfs.bin
ISO := $(BUILD)/kfs.iso
TARGET := kfs
RUST_OS := target/$(TARGET)/$(BUILD_MODE)/libkfs.a
LINKER_SCRIPT := linker.ld
GRUB_CFG := grub.cfg

ASM_SRCS := $(wildcard asm/*.asm)
ASM_OBJS := $(patsubst asm/%.asm, $(BUILD)/asm/%.o, $(ASM_SRCS))

RESET := \033[0m
GREEN := \033[1m\033[32m

all: $(ISO)

re: clean all

run: all
	@qemu-system-i386 -cdrom $(ISO) -device isa-debug-exit,iobase=0xf4,iosize=0x04; \
    ret=$$?; \
    if [ $$ret -ne 0 ] && [ $$ret -ne 33 ]; then \
        echo "Failed with status $$ret."; \
        exit $$ret; \
    fi

rerun: clean run

clean:
	rm -rf build || true
	cargo clean || true
	vagrant destroy -f || true
	rm -rf .vagrant || true
	rm -rf *VBox*.log || true

vm:
	@vagrant up

$(ISO): $(KERNEL) $(GRUB_CFG) $(TARGET).json vm
	@mkdir -p $(ISOFILES)/boot/grub
	@cp $(KERNEL) $(ISOFILES)/boot
	@cp $(GRUB_CFG) $(ISOFILES)/boot/grub
	@vagrant ssh -c "cd /vagrant && grub-mkrescue -o $(ISO) $(GRUB_FLAGS) $(ISOFILES)"
	@rm -rf $(ISOFILES)

$(KERNEL): $(RUST_OS) $(ASM_OBJS) $(LINKER_SCRIPT)
	@mkdir -p $(dir $@)
	@ld -m elf_i386 -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS) $(RUST_OS)

$(RUST_OS):
	@export RUST_TARGET_PATH=$(shell pwd) ; cargo build --target $(TARGET) $(CARGO_FLAGS)

$(ASM_OBJS): $(BUILD)/asm/%.o: asm/%.asm
	@mkdir -p $(dir $@)
	@nasm -f elf32 $< -o $@
	@echo "$(GREEN)+++ $@$(RESET)"

loc:
	@find src -name '*.rs' | sort | xargs wc -l

.PHONY: all re run rerun clean $(RUST_OS) vm loc
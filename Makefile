ARCH ?= i386
DEBUG ?= false

ifeq ($(ARCH), i386)
LD_FLAGS := -m elf_i386
ELF_FORMAT := elf32
else ifeq ($(ARCH), x86_64)
ELF_FORMAT := elf64
else
$(error ARCH must be either i386 or x86_64)
endif

ifeq ($(DEBUG),true)
BUILD_MODE := debug
else ifeq ($(DEBUG),false)
BUILD_MODE := release
CARGO_FLAGS := --release
GRUB_FLAGS := --compress xz
else
$(error DEBUG must be either true or false)
endif

BUILD := build/$(ARCH)/$(BUILD_MODE)
ISOFILES := $(BUILD)/isofiles
KERNEL := $(BUILD)/kernel-$(ARCH).bin
ISO := $(BUILD)/os-$(ARCH).iso
TARGET := arch/$(ARCH)/kfs
RUST_OS := target/$(TARGET)/$(BUILD_MODE)/libkfs.a
LINKER_SCRIPT := arch/linker.ld
GRUB_CFG := arch/grub.cfg
QEMU := qemu-system-$(ARCH)

ASM_SHARED_SRCS := $(wildcard arch/*.asm)
ASM_SHARED_OBJS := $(patsubst arch/%.asm, $(BUILD)/asm/%.o, $(ASM_SHARED_SRCS))
ASM_ARCH_SRCS := $(wildcard arch/$(ARCH)/*.asm)
ASM_ARCH_OBJS := $(patsubst arch/$(ARCH)/%.asm, $(BUILD)/asm/%.o, $(ASM_ARCH_SRCS))

RESET := \033[0m
GREEN := \033[1m\033[32m

all: $(ISO)

re: clean all

run: all
	@$(QEMU) -cdrom $(ISO) -device isa-debug-exit,iobase=0xf4,iosize=0x04; \
    ret=$$?; \
    if [ $$ret -ne 0 ] && [ $$ret -ne 33 ]; then \
        echo "Failed with status $$ret."; \
        exit $$ret; \
    fi

rerun: re run

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
	@cp $(KERNEL) $(ISOFILES)/boot/kernel.bin
	@cp $(GRUB_CFG) $(ISOFILES)/boot/grub
	@vagrant ssh -c "cd /vagrant && grub-mkrescue -o $(ISO) $(GRUB_FLAGS) $(ISOFILES)"
	@rm -rf $(ISOFILES)

$(KERNEL): $(RUST_OS) $(ASM_ARCH_OBJS) $(ASM_SHARED_OBJS) $(LINKER_SCRIPT)
	@mkdir -p $(shell dirname $@)
	@ld $(LD_FLAGS) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_ARCH_OBJS) $(ASM_SHARED_OBJS) $(RUST_OS)

$(RUST_OS):
	@export RUST_TARGET_PATH=$(shell pwd) ; cargo build --target $(TARGET) $(CARGO_FLAGS)

$(ASM_SHARED_OBJS): $(BUILD)/asm/%.o: arch/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f $(ELF_FORMAT) $< -o $@
	@echo "$(GREEN)+++ $@$(RESET)"

$(ASM_ARCH_OBJS): $(BUILD)/asm/%.o: arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f $(ELF_FORMAT) $< -o $@
	@echo "$(GREEN)+++ $@$(RESET)"

loc:
	@find src -name '*.rs' | sort | xargs wc -l

.PHONY: all re run rerun clean $(RUST_OS) vm loc
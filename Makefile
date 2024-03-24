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
ASM_SRCS := $(wildcard arch/$(ARCH)/*.asm)
ASM_OBJS := $(patsubst arch/$(ARCH)/%.asm, $(BUILD)/arch/$(ARCH)/%.o, $(ASM_SRCS))
QEMU := qemu-system-$(ARCH)

all: $(ISO)

re: clean all

run: all
	@$(QEMU) -cdrom $(ISO)

rerun: re run

clean:
	rm -rf $(BUILD) || true
	cargo clean || true
	vagrant destroy -f || true
	rm -rf .vagrant || true
	rm -rf *VBox*.log || true

vm:
	@vagrant up

$(ISO): $(KERNEL) $(GRUB_CFG) $(ASM_SRCS) $(TARGET).json vm
	@mkdir -p $(ISOFILES)/boot/grub
	@cp $(KERNEL) $(ISOFILES)/boot/kernel.bin
	@cp $(GRUB_CFG) $(ISOFILES)/boot/grub
	@vagrant ssh -c "cd /vagrant && grub-mkrescue -o $(ISO) $(ISOFILES)"
	@rm -rf $(ISOFILES)

$(KERNEL): $(RUST_OS) $(ASM_OBJS) $(LINKER_SCRIPT)
	@ld $(LD_FLAGS) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS) $(RUST_OS)

$(RUST_OS):
	@export RUST_TARGET_PATH=$(shell pwd) ; cargo build --target $(TARGET) $(CARGO_FLAGS)

$(ASM_OBJS): $(BUILD)/arch/$(ARCH)/%.o: arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f $(ELF_FORMAT) $< -o $@

loc:
	@find src -name '*.rs' | sort | xargs wc -l

.PHONY: all re run rerun clean $(RUST_OS) vm loc
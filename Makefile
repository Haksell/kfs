ARCH ?= i386
BUILD := build
ISOFILES := $(BUILD)/isofiles
KERNEL := $(BUILD)/kernel-$(ARCH).bin
ISO := $(BUILD)/os-$(ARCH).iso
TARGET := src/arch/$(ARCH)/kfs
RUST_OS := target/$(TARGET)/debug/libkfs.a
# RUST_OS := target/$(TARGET)/release/libkfs.a
LINKER_SCRIPT := src/arch/linker.ld
GRUB_CFG := src/arch/grub.cfg
RUST_SRCS := $(wildcard src/*.rs) # TODO: handle subfolders
ASM_SRCS := $(wildcard src/arch/$(ARCH)/*.asm)
ASM_OBJS := $(patsubst src/arch/$(ARCH)/%.asm, $(BUILD)/arch/$(ARCH)/%.o, $(ASM_SRCS))
QEMU := qemu-system-$(ARCH)

ifeq ($(ARCH), i386)
	LD_FLAGS := -m elf_i386
	ELF_FORMAT := elf32
else
	ELF_FORMAT := elf64
endif

all:
	docker build -t kfs .
	docker run --rm -e ARCH=$(ARCH) -v $(shell pwd):/workspace kfs

re: clean all

run: all
	@$(QEMU) -cdrom $(ISO)

rerun: re run

clean:
	rm -rf $(BUILD)
	cargo clean

# Rules below should only be executed inside Docker

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUB_CFG) $(ASM_SRCS) $(TARGET).json
	@mkdir -p $(ISOFILES)/boot/grub
	@cp $(KERNEL) $(ISOFILES)/boot/kernel.bin
	@cp $(GRUB_CFG) $(ISOFILES)/boot/grub
	@grub-mkrescue -o $(ISO) $(ISOFILES) 2> /dev/null
	@rm -r $(ISOFILES)

# TODO: use cargo instead of xargo
$(RUST_OS): $(RUST_SRCS)
	@export RUST_TARGET_PATH=$(shell pwd) ; xargo build --target $(TARGET)

$(KERNEL): $(RUST_OS) $(ASM_OBJS) $(LINKER_SCRIPT)
	@ld $(LD_FLAGS) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS) $(RUST_OS)

$(ASM_OBJS): $(BUILD)/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f $(ELF_FORMAT) $< -o $@

.PHONY: all re run rerun iso clean
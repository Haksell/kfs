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

run_vm: $(ISO)
	@$(QEMU) -cdrom $(ISO)

all: $(ISO)

re: clean all

run: all
	@$(QEMU) -cdrom $(ISO)

rerun: re run

clean:
	rm -rf $(BUILD)
	cargo clean

$(ISO): $(KERNEL) $(GRUB_CFG) $(ASM_SRCS) $(TARGET).json
	@mkdir -p $(ISOFILES)/boot/grub
	@cp $(KERNEL) $(ISOFILES)/boot/kernel.bin
	@cp $(GRUB_CFG) $(ISOFILES)/boot/grub
	@vagrant ssh -c "cd /vagrant && grub-mkrescue -o $(ISO) $(ISOFILES)"
	@rm -r $(ISOFILES)

$(RUST_OS): $(RUST_SRCS)
	@export RUST_TARGET_PATH=$(shell pwd) ; cargo build --target $(TARGET)

$(KERNEL): $(RUST_OS) $(ASM_OBJS) $(LINKER_SCRIPT)
	@ld $(LD_FLAGS) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS) $(RUST_OS)

rust_os: $(RUST_OS)

kernel: $(KERNEL)

asm: $(ASM_OBJS)

$(ASM_OBJS): $(BUILD)/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f $(ELF_FORMAT) $< -o $@

.PHONY: all re run rerun iso clean
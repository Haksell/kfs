ARCH ?= i386
BUILD := build
ISOFILES := $(BUILD)/isofiles
KERNEL := $(BUILD)/kernel-$(ARCH).bin
ISO := $(BUILD)/os-$(ARCH).iso
TARGET := arch/$(ARCH)/kfs
RUST_OS := target/$(TARGET)/debug/libkfs.a
# RUST_OS := target/$(TARGET)/release/libkfs.a
LINKER_SCRIPT := arch/linker.ld
GRUB_CFG := arch/grub.cfg
ASM_SRCS := $(wildcard arch/$(ARCH)/*.asm)
ASM_OBJS := $(patsubst arch/$(ARCH)/%.asm, $(BUILD)/arch/$(ARCH)/%.o, $(ASM_SRCS))
QEMU := qemu-system-$(ARCH)

ifeq ($(ARCH), i386)
	LD_FLAGS := -m elf_i386
	ELF_FORMAT := elf32
else
	ELF_FORMAT := elf64
endif

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

$(RUST_OS):
	@export RUST_TARGET_PATH=$(shell pwd) ; cargo build --target $(TARGET)

$(KERNEL): $(RUST_OS) $(ASM_OBJS) $(LINKER_SCRIPT)
	@ld $(LD_FLAGS) -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS) $(RUST_OS)

$(ASM_OBJS): $(BUILD)/arch/$(ARCH)/%.o: arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f $(ELF_FORMAT) $< -o $@

.PHONY: all re run rerun clean $(RUST_OS) vm
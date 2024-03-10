ARCH ?= x86_64
KERNEL := build/kernel-$(ARCH).bin
ISO := build/os-$(ARCH).iso
TARGET ?= $(ARCH)-kfs
RUST_OS := target/$(TARGET)/debug/libkfs.a
LINKER_SCRIPT := src/arch/$(ARCH)/linker.ld
GRUB_CFG := src/arch/$(ARCH)/grub.cfg
ASM_SRCS := $(wildcard src/arch/$(ARCH)/*.asm)
ASM_OBJS := $(patsubst src/arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(ASM_SRCS))
QEMU := qemu-system-x86_64 # TODO: depends on ARCH

all:
	docker build -t kfs .
	docker run --rm -v $(shell pwd):/workspace kfs

re: clean all

run: all
	@$(QEMU) -cdrom $(ISO)

rerun: re run

clean:
	rm -rf build
	cargo clean

# Rules below should only be executed inside Docker

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUB_CFG)
	@mkdir -p build/isofiles/boot/grub
	@cp $(KERNEL) build/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) build/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(RUST_OS):
	@export RUST_TARGET_PATH=$(shell pwd) ; xargo build --target $(TARGET)

$(KERNEL): $(RUST_OS) $(ASM_OBJS) $(LINKER_SCRIPT)
	@ld -n --gc-sections -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS) $(RUST_OS)

build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@


.PHONY: all re run rerun iso clean
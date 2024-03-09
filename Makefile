ARCH ?= x86_64
KERNEL := build/kernel-$(ARCH).bin
ISO := build/os-$(ARCH).iso
LINKER_SCRIPT := src/arch/$(ARCH)/linker.ld
GRUB_CFG := src/arch/$(ARCH)/grub.cfg
ASM_SRCS := $(wildcard src/arch/$(ARCH)/*.asm)
ASM_OBJS := $(patsubst src/arch/$(ARCH)/%.asm, build/arch/$(ARCH)/%.o, $(ASM_SRCS))

all:
	docker build -t osdev .
	docker run --rm -v .:/workspace osdev

re: clean all

run: all
	@qemu-system-x86_64 -cdrom $(ISO)

rerun: clean run

clean:
	@rm -rf build

iso: $(ISO)

$(ISO): $(KERNEL) $(GRUB_CFG)
	@mkdir -p build/isofiles/boot/grub
	@cp $(KERNEL) build/isofiles/boot/kernel.bin
	@cp $(GRUB_CFG) build/isofiles/boot/grub
	@grub-mkrescue -o $(ISO) build/isofiles
	@rm -r build/isofiles

$(KERNEL): $(ASM_OBJS) $(LINKER_SCRIPT)
	@ld -n -T $(LINKER_SCRIPT) -o $(KERNEL) $(ASM_OBJS)

build/arch/$(ARCH)/%.o: src/arch/$(ARCH)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@

.PHONY: all clean run iso
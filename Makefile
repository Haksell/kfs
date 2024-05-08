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

$(ISO): $(KERNEL) $(GRUB_CFG) $(TARGET).json
	@mkdir -p $(ISOFILES)/boot/grub
	@cp $(KERNEL) $(ISOFILES)/boot
	@cp $(GRUB_CFG) $(ISOFILES)/boot/grub
	@grub-mkrescue -o $(ISO) $(GRUB_FLAGS) $(ISOFILES)
	@rm -rf $(ISOFILES)

# xorriso 1.5.4 : RockRidge filesystem manipulator, libburnia project.
# Drive current: -outdev 'stdio:build/release/kfs.iso'
# Media current: stdio file, overwriteable
# Media status : is blank
# Media summary: 0 sessions, 0 data blocks, 0 data, 4096g free
# Added to ISO image: directory '/'='/tmp/grub.UMv145'
# xorriso : UPDATE :     294 files added in 1 seconds
# Added to ISO image: directory '/'='/mnt/nfs/homes/axbrisse/Desktop/cursus/kfs/build/release/isofiles'
# xorriso : UPDATE :     298 files added in 1 seconds
# ISO image produced: 2864 sectors
# Written to medium : 2864 sectors at LBA 0
# Writing to 'stdio:build/release/kfs.iso' completed successfully.

# xorriso 1.5.2 : RockRidge filesystem manipulator, libburnia project.
# Drive current: -outdev 'stdio:build/release/kfs.iso'
# Media current: stdio file, overwriteable
# Media status : is blank
# Media summary: 0 sessions, 0 data blocks, 0 data, 4096g free
# Added to ISO image: directory '/'='/tmp/grub.yH8ewI'
# xorriso : UPDATE :     335 files added in 1 seconds
# Added to ISO image: directory '/'='/vagrant/build/release/isofiles'
# xorriso : UPDATE :     339 files added in 1 seconds
# xorriso : NOTE : Copying to System Area: 512 bytes from file '/usr/lib/grub/i386-pc/boot_hybrid.img'
# ISO image produced: 1832 sectors
# Written to medium : 1832 sectors at LBA 0
# Writing to 'stdio:build/release/kfs.iso' completed successfully.

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

.PHONY: all re run rerun clean $(RUST_OS) loc
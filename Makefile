ISO := kfs.iso
ISOFILES := isofiles
KERNEL := $(ISOFILES)/boot/kernel.bin
ASM_FILES := $(wildcard *.asm)
OBJ_FILES := $(ASM_FILES:.asm=.o)
RM := rm -f

all:
	docker build -t osdev .
	docker run --rm -v .:/workspace osdev

re: fclean all

run: all
	qemu-system-x86_64 -cdrom $(ISO)

rerun: fclean run

iso: $(ISO)

$(ISO): $(KERNEL)
	grub-mkrescue -o $(ISO) $(ISOFILES)

reiso: fclean iso

kernel: $(KERNEL)

$(KERNEL): $(OBJ_FILES)
	ld -n -o $(KERNEL) -T linker.ld $(OBJ_FILES)

rekernel: fclean kernel

%.o: %.asm
	nasm -f elf64 $< -o $@

clean:
	$(RM) $(OBJ_FILES)

fclean: clean
	$(RM) $(KERNEL) $(ISO)

.PHONY: all re iso reiso run rerun kernel rekernel clean fclean

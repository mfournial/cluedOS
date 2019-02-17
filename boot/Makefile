ASS=nasm
LINK=ld
KERNEL=isofiles/boot/kernel.bin

.PHONY: all multiboot boot linking clean kernel

all: $(KERNEL)

multiboot.o:
	$(ASS) -f elf64 multiboot_header.asm -o $@

boot.o:
	$(ASS) -f elf64 boot.asm -o $@

linking: multiboot.o boot.o
	$(LD) -n -o $(KERNEL) -T linker.ld $^

$(KERNEL): linking
	 grub-mkrescue -o os.iso isofiles

run: $(KERNEL)
	 qemu-system-x86_64 -cdrom os.iso

clean:
	rm -f $(KERNEL) *.o os.iso

# KFS

## subject

### kfs1

-   [x] Install GRUB on a virtual image
-   [x] Write an ASM boot code that handles multiboot header, and use GRUB to init and call main function of the kernel itself.
-   [x] Write basic kernel code of the choosen language.
-   [x] Compile it with correct flags, and link it to make it bootable.
-   [ ] Once all of those steps above are done, you can write some helpers like kernel types or basic functions (strlen, strcmp, ...)
-   [x] Code the interface between your kernel and the screen.
-   [x] Display "42" on the screen.
-   [x] For the link part, you must create a linker file with the GNU linker (ld).
-   [x] Your Makefile must compile all your source files with the right flags and the right compiler.
-   [x] After compilation, all the objects must be linked together in order to create the final Kernel binary.
-   [x] Add scroll and cursor support to your I/O interface.
-   [x] Add colors support to your I/O interface.
-   [x] Add helpers like printf / printk in order to print information / debug easily.
-   [x] Handle keyboard entries and print them.
-   [ ] Handle different screens, and keyboard shortcuts to switch easily between then.

### kfs2

-   [ ] You must create a Global Descriptor Table.
-   [ ] Your GDT must contain: Kernel Code
-   [ ] Your GDT must contain: Kernel Data
-   [ ] Your GDT must contain: Kernel stack
-   [ ] Your GDT must contain: User code
-   [ ] Your GDT must contain: User data
-   [ ] Your GDT must contain: User stack
-   [ ] You must declare your GDT to the BIOS.
-   [ ] The GDT must be set at address 0x00000800.
-   [ ] Shell: `pks` (print the kernel stack, in a human-friendly way)
-   [ ] Shell: `reboot` command
-   [ ] Shell: `halt` command
-   [ ] Shell: other commands for debugging purposes

## todo

### misc

-   [ ] implement `x86_64`
-   [ ] implement `pc-keyboard`
-   [ ] make everything work with `i386`
-   [ ] `exit_qemu` from https://github.com/Haksell/os_blog_v2 without `x86_64` crate
-   [ ] call `exit_qemu` on `Esc`, `Ctrl+D` or `exit` command
-   [ ] bring back testing and more useful stuff from from https://github.com/Haksell/os_blog_v2
-   [ ] colorful tests (with color module based on `colored`)
-   [ ] fix compiler warnings
-   [ ] install `grub-mkrescue` and all its dependencies locally
-   [ ] set timeout=10 for correction
-   [ ] check your work should not exceed 10 MB before push

### shell

-   [ ] screens with F1-F12
-   [ ] each screen has a different color
-   [ ] handle up/down
-   [ ] implement shell history
-   [ ] basic shell commands

### later

-   [ ] finish 1st edition
-   [ ] finish 2nd edition
-   [ ] `qemu` in terminal like lsimanic (`-display curses` with a black screen)
-   [ ] find project name and rebrand
-   [ ] nice help menu with `Code page 437` border characters (kfs-2)
-   [ ] use KVM on top of QEMU?
-   [ ] specify exact nightly version

## resources

-   https://os.phil-opp.com/edition-1/
-   https://os.phil-opp.com/edition-2/
-   https://osdev.org/Main_Page
-   https://github.com/rust-osdev
-   https://pages.cs.wisc.edu/~remzi/OSTEP
-   https://singlelogin.re/book/25182527/e03396/modern-operating-systems.html
-   http://www.brokenthorn.com/Resources/OSDevIndex.html
-   https://www.gnu.org/software/grub/manual/multiboot2/multiboot.pdf
-   `#os-dev`: https://discord.com/channels/273534239310479360/375706574133526529

## artistic direction

![artistic direction](https://upload.wikimedia.org/wikipedia/commons/a/a0/VirtualBox_TempleOS_x64_27_02_2021_20_43_48.png)

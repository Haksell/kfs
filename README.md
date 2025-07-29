# kfs

## subject

### kfs3

- [ ] You must implement a complete, stable and functionnal memory system in your kernel.
- [ ] You must enable memory paging in your kernel
- [ ] You must code a memory structure that handle paging and memory rights
- [ ] You must define kernel and user space
- [ ] You must implement a function to create / get memory pages
- [ ] You must implement kmalloc, kfree, ksize, kbrk for physical memory
- [ ] You must implement vmalloc, vfree, vsize, vbrk for virtual memory
- [ ] You must handle "kernel panics" (print, stop the kernel)

### kfs4

- [ ] Create an Interrupts Descriptor Table, fill it and register it
- [ ] Hardware Interrupts
- [ ] Software Interrupts
- [ ] A signal-callback system on your Kernel API
- [ ] An interface to schedule signals
- [ ] Global Panic Fault handling
- [ ] An interface to clean registers before a panic / halt
- [ ] An interface to save the stack before a panic
- [ ] When you’re done with all of that, you’ll have to implement a IDT keyboard handling system.
- [ ] It has not been said, but syscalls are also handled by the IDT. You can’t implement them now (No processus / Execution), but a good start could be coding the base functions for it, it could save you some work.
- [ ] Also, you can add some features to the keyboard handler, for example multi layouts (qwerty, azerty), base functions like get_line (just like read: waits for characters and return them when \n is pressed).

## todo

### present

- [ ] bring back `enable_paging` from old x86_64 code
- [ ] bring back `setup_page_tables` from old x86_64 code and adapt it to 32-bit

### future

- [ ] find project name and rebrand
- [ ] optimize vga history with a ring buffer
- [ ] nice help menu with `Code page 437` border characters (kfs-2)
- [ ] specify exact nightly version
- [ ] bring back testing and more useful stuff from [v2](https://github.com/Haksell/writing_an_os_in_rust/tree/master/v2)
- [ ] colorful tests (with color module based on `colored`)
- [ ] full exploration of possible deadlocks
- [ ] `print_screen` creates a file using serial port
- [ ] `insert`
- [ ] use https://doc.rust-lang.org/nightly/core/cell/ instead of `lazy_static` crate
- [ ] [Interrupt handlers should only perform the minimal amount of work necessary](https://os.phil-opp.com/async-await/#scancode-queue)
- [ ] warning screen (F11)
- [ ] debug screen (F12)
- [ ] optimize `x86_64` target
- [ ] finish 1st edition
- [ ] remove stack segments
- [ ] separate user and kernel stacks
- [ ] mouse (scroll history)

## check before each push

- update nightly version
- set timeout=10 (and maybe several entries) for correction
- check executable does not exceed 10 MB before push (try with release mode)
- fix compiler warnings

## artistic direction

![artistic direction](https://upload.wikimedia.org/wikipedia/commons/a/a0/VirtualBox_TempleOS_x64_27_02_2021_20_43_48.png)

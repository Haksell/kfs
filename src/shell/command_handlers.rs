use {
    super::Shell,
    crate::{
        port::Port,
        print, println,
        vga_buffer::{VGA_WIDTH, WRITER},
    },
    core::{arch::asm, ptr::addr_of},
    lazy_static::lazy_static,
};

const HEXDUMP_LINE_SIZE: usize = 16;

unsafe extern "C" {
    static gdt_start: usize;
    static gdt_pointer: usize;
    static stack_top: usize;
    static stack_bottom: usize;
}

lazy_static! {
    static ref GDT_START: usize = addr_of!(gdt_start) as usize;
    static ref GDT_POINTER: usize = addr_of!(gdt_pointer) as usize;
    static ref STACK_TOP: usize = addr_of!(stack_top) as usize;
    static ref STACK_BOTTOM: usize = addr_of!(stack_bottom) as usize;
}

#[expect(dead_code)] // TODO: remove because it doesn't make sense to never use success or failed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// TODO: more generic exit which doesn't only work on QEMU
pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe { Port::new(0xf4).write(exit_code as u32) }
}

fn hexdump(start: usize, end: usize) {
    let mut last_line = [0; HEXDUMP_LINE_SIZE];
    let mut line = [0; HEXDUMP_LINE_SIZE];
    let mut last_was_same = false;
    for (i, current) in (start..end).step_by(HEXDUMP_LINE_SIZE).enumerate() {
        for (j, cell) in line.iter_mut().enumerate() {
            *cell = unsafe { *((current + j) as *const u8) };
        }
        if i == 0 || line != last_line {
            print!("{:08x}   ", current);
            for (j, byte) in line.iter().enumerate() {
                print!("{:02x} ", byte);
                if j & 7 == 7 {
                    print!(" ");
                }
            }
            print!(" |");
            for byte in line {
                WRITER
                    .lock()
                    .write_byte(if byte == 0x0a { 0x20 } else { byte });
            }
            println!("|");
            last_line = line;
            last_was_same = false;
        } else if !last_was_same {
            println!("*");
            last_was_same = true;
        }
    }
}

#[derive(Clone, Copy)]
pub struct CommandHandler {
    pub name: &'static [u8],
    pub description: &'static [u8],
    pub handler: fn(&Shell), // Does it really make sense to take a shell as argument?
}

pub const COMMAND_HANDLERS: &[CommandHandler] = &[
    CommandHandler {
        name: b"clear",
        description: b"Clear the screen.",
        handler: |_: &Shell| WRITER.lock().clear_screen(),
    },
    CommandHandler {
        name: b"exit",
        description: b"Exit the system.",
        handler: |_: &Shell| exit_qemu(QemuExitCode::Success),
    },
    CommandHandler {
        name: b"halt",
        description: b"Halt the system.",
        handler: |_: &Shell| {
            unsafe { asm!("cli") }
            print!("System halted.");
            WRITER.lock().set_cursor(VGA_WIDTH);
            unsafe { asm!("hlt") }
        },
    },
    CommandHandler {
        name: b"help",
        description: b"Show this help message.",
        handler: |_: &Shell| {
            println!("Available commands:");
            let max_length = COMMAND_HANDLERS
                .iter()
                .map(|handler| handler.name.len())
                .max()
                .unwrap();
            for handler in COMMAND_HANDLERS {
                println!(
                    "- {:max_length$}   {}",
                    core::str::from_utf8(handler.name).unwrap_or("invalid utf-8"),
                    core::str::from_utf8(handler.description).unwrap_or("invalid utf-8"),
                );
            }
        },
    },
    CommandHandler {
        name: b"pgdt",
        description: b"Print the GDT.",
        handler: |_: &Shell| {
            for address in (*GDT_START..*GDT_POINTER).step_by(8) {
                print!("{:#07x}:", address);
                for i in 0..8 {
                    let value = unsafe { *((address + i) as *const u8) };
                    print!(" {value:08b}");
                }
                println!();
            }
        },
    },
    CommandHandler {
        name: b"pks",
        description: b"Print the kernel stack.",
        handler: |_: &Shell| hexdump(*STACK_BOTTOM, *STACK_TOP),
    },
    CommandHandler {
        name: b"reboot",
        description: b"Reboot the system.",
        handler: |_: &Shell| unsafe { Port::new(0x64).write(0xfe_u8) },
    },
    CommandHandler {
        name: b"tty",
        description: b"Show the current screen number.",
        handler: |shell: &Shell| println!("F{}", shell.screen_idx + 1),
    },
];

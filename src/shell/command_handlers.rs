use super::Shell;
use crate::{
    port::Port,
    print, println,
    vga_buffer::{VGA_WIDTH, WRITER},
};
use core::arch::asm;

const HEXDUMP_LINE_SIZE: usize = 16;

extern "C" {
    static stack_top: usize;
    static stack_bottom: usize;
}

#[allow(dead_code)] // TODO: remove because it doesn't make sense to never use success or failed
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

#[derive(Clone, Copy)]
pub struct CommandHandler {
    pub name: &'static [u8],
    description: &'static [u8],
    pub handler: fn(&Shell), // Does it really make sense to take a shell as argument?
}

pub const COMMAND_HANDLERS: [CommandHandler; 7] = [
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
        handler: |_: &Shell| unsafe {
            asm!("cli");
            print!("System halted.");
            WRITER.lock().set_cursor(VGA_WIDTH);
            asm!("hlt");
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
            for handler in COMMAND_HANDLERS.iter() {
                println!(
                    "- {:max_length$}   {}",
                    core::str::from_utf8(handler.name).unwrap_or("invalid utf-8"),
                    core::str::from_utf8(handler.description).unwrap_or("invalid utf-8"),
                );
            }
        },
    },
    CommandHandler {
        name: b"pks",
        description: b"Print the kernel stack.",
        handler: |_: &Shell| {
            let top: usize;
            let bottom: usize;
            unsafe {
                top = &stack_top as *const usize as usize;
                bottom = &stack_bottom as *const usize as usize;
            }

            let mut _saxophones: [u8; 2000] = [0xfb; 2000];

            let mut last_line: [u8; HEXDUMP_LINE_SIZE] = [0; HEXDUMP_LINE_SIZE];
            let mut line: [u8; HEXDUMP_LINE_SIZE] = [0; HEXDUMP_LINE_SIZE];
            let mut last_was_same: bool = false;

            for (i, current) in (bottom..top).step_by(HEXDUMP_LINE_SIZE).enumerate() {
                for j in 0..HEXDUMP_LINE_SIZE {
                    line[j] = unsafe { *((current + j) as *const u8) };
                }
                if i == 0 || line != last_line {
                    print!("{:08x}  ", current);
                    for (i, byte) in line.iter().enumerate() {
                        print!("{:02x} ", byte);
                        if i & 7 == 7 {
                            print!(" ");
                        }
                    }
                    print!("|");
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

            println!("Stack range: {:#X} to {:#X}", bottom, top);
            println!("Saxophones address: {:p}", &_saxophones);
        },
    },
    CommandHandler {
        name: b"reboot",
        description: b"Reboot the system.",
        handler: |_: &Shell| unsafe { Port::new(0x64).write(0xFEu8) },
    },
    CommandHandler {
        name: b"tty",
        description: b"Show the current screen number.",
        handler: |shell: &Shell| println!("F{}", shell.screen_idx + 1),
    },
];

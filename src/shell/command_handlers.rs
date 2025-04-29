use super::Shell;
use crate::{
    port::Port,
    print, println,
    vga_buffer::{VGA_WIDTH, WRITER},
};
use core::arch::asm;
use lazy_static::lazy_static;

const HEXDUMP_LINE_SIZE: usize = 16;

extern "C" {
    static gdt_start: usize;
    static gdt_pointer: usize;
    static stack_top: usize;
    static stack_bottom: usize;
}

lazy_static! {
    static ref GDT_START: usize = unsafe { &gdt_start as *const usize as usize };
    static ref GDT_POINTER: usize = unsafe { &gdt_pointer as *const usize as usize };
    static ref STACK_TOP: usize = unsafe { &stack_top as *const usize as usize };
    static ref STACK_BOTTOM: usize = unsafe { &stack_bottom as *const usize as usize };
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

fn hexdump(start: usize, end: usize) {
    let mut last_line: [u8; HEXDUMP_LINE_SIZE] = [0; HEXDUMP_LINE_SIZE];
    let mut line: [u8; HEXDUMP_LINE_SIZE] = [0; HEXDUMP_LINE_SIZE];
    let mut last_was_same: bool = false;
    for (i, current) in (start..end).step_by(HEXDUMP_LINE_SIZE).enumerate() {
        for j in 0..HEXDUMP_LINE_SIZE {
            line[j] = unsafe { *((current + j) as *const u8) }; // TODO: one-liner
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
    println!("Hexdump from {:#X} to {:#X}", start, end);
}

#[derive(Clone, Copy)]
pub struct CommandHandler {
    pub name: &'static [u8],
    description: &'static [u8],
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
        name: b"pgdt",
        description: b"Print the GDT.",
        handler: |_: &Shell| {
            for address in (*GDT_START..*GDT_POINTER).step_by(8) {
                print!("{:#x}:", address);
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
        handler: |_: &Shell| {
            let mut _saxophones: [u8; 2000] = [0xfb; 2000];
            hexdump(*STACK_BOTTOM, *STACK_TOP);
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

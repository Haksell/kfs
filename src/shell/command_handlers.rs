use super::Shell;
use crate::{
    port::Port,
    print, println,
    vga_buffer::{VGA_WIDTH, WRITER},
};

#[derive(Clone, Copy)]
pub struct CommandHandler {
    pub name: &'static [u8],
    description: &'static [u8],
    pub handler: fn(&Shell), // Does it really make sense to take a shell as argument?
}

pub const COMMAND_HANDLERS: [CommandHandler; 6] = [
    CommandHandler {
        name: b"clear",
        description: b"Clear the screen.",
        handler: |_: &Shell| WRITER.lock().clear_screen(),
    },
    CommandHandler {
        name: b"halt",
        description: b"Halt the system.",
        handler: |_: &Shell| unsafe {
            core::arch::asm!("cli");
            print!("System halted.");
            WRITER.lock().set_cursor(VGA_WIDTH);
            core::arch::asm!("hlt");
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
        handler: |_: &Shell| println!("<<< TODO >>>"),
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

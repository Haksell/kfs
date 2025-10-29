#![no_std]
#![feature(abi_x86_interrupt)]

mod interrupts;
mod keyboard;
mod port;
mod shell;
mod vga_buffer;

use {
    crate::{shell::SHELL, vga_buffer::WRITER},
    core::{arch::asm, panic::PanicInfo},
};

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    WRITER.lock().clear_vga_buffer();
    SHELL.lock().init();
    interrupts::init();
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: Yellow on Black
    println!("{}", info);
    hlt_loop()
}

fn hlt_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

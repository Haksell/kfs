#![no_std]
#![feature(abi_x86_interrupt)]

use core::{arch::asm, panic::PanicInfo};

mod entry;
mod idt;
mod interrupts;
mod pic;
mod port;
mod shell;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn kernel_main() {
    interrupts::init();
    vga_buffer::WRITER.lock().clear_vga_buffer();
    shell::SHELL.lock().init();
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: Yellow on Black
    println!("{}", info);
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}

#[inline]
pub fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}

#![no_std]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod idt;
mod interrupts;
mod pic;
mod shell;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize() }; // TODO: init instead of initialize
    x86_64::instructions::interrupts::enable();
    vga_buffer::clear_vga_buffer();
    shell::SHELL.lock().init();
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

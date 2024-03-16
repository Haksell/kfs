#![no_std]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod gdt;
mod idt;
mod interrupts;
mod pic;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    init();
    vga_buffer::clear_screen();
    println!("K{}S {}", 'F', 6 * 7);
    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

fn init() {
    gdt::init();
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

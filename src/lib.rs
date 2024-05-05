#![no_std]
#![feature(abi_x86_interrupt, exclusive_range_pattern)]

mod interrupts;
mod keyboard;
mod port;
mod shell;
mod vga_buffer;

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main() {
    vga_buffer::WRITER.lock().clear_vga_buffer();
    shell::SHELL.lock().init();
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

#![no_std]

use core::panic::PanicInfo;

extern crate rlibc;
extern crate volatile;
extern crate spin;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn rust_main() {
    vga_buffer::clear_screen();
    println!("KFS {}", 6 * 7);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

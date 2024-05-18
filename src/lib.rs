#![no_std]
#![feature(abi_x86_interrupt, exclusive_range_pattern)]

mod interrupts;
mod keyboard;
mod memory;
mod port;
mod shell;
mod vga_buffer;

use core::arch::asm;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_header_address: usize) {
    let boot_info = unsafe { memory::multiboot::load(multiboot_header_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");
    let elf_sections_tag = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    vga_buffer::WRITER.lock().clear_vga_buffer();
    shell::SHELL.lock().init();

    println!("Memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!(
            "     start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length
        );
    }

    println!("kernel sections:");
    for section in elf_sections_tag.sections() {
        println!(
            "    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.start_address(),
            section.size(),
            section.flags()
        );
    }

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

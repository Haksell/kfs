#![no_std]
#![feature(abi_x86_interrupt, exclusive_range_pattern)]

mod interrupts;
mod keyboard;
mod memory;
mod port;
mod shell;
mod vga_buffer;

extern crate multiboot2;

use crate::memory::frame::FrameAllocator;
use core::arch::asm;
use core::panic::PanicInfo;

use multiboot2::{BootInformationHeader, ElfSectionFlags};

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_header_address: usize) {
    let boot_info = unsafe {
        multiboot2::BootInformation::load(multiboot_header_address as *const BootInformationHeader)
            .unwrap()
    };

    let our_boot_info = unsafe { memory::multiboot::load(multiboot_header_address) };

    let memory_map_tag = our_boot_info
        .memory_map_tag()
        .expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections().expect("Elf-sections tag required");

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
    for section in elf_sections_tag {
        if section.flags() != ElfSectionFlags::empty() {
            println!(
                "    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
                section.start_address(),
                section.size(),
                section.flags()
            );
        }
    }

    let kernel_start = boot_info
        .elf_sections()
        .unwrap()
        .map(|s| s.start_address())
        .min()
        .unwrap();

    let kernel_end = boot_info
        .elf_sections()
        .unwrap()
        .map(|s| s.start_address() + s.size())
        .max()
        .unwrap();

    let multiboot_end = multiboot_header_address + boot_info.total_size();

    println!(
        "kernel_start: {:#x}, kernel_end: {:#x}",
        kernel_start, kernel_end
    );
    println!(
        "multiboot_start: {:#x}, multiboot_end: {:#x}",
        multiboot_header_address, multiboot_end
    );

    let mut frame_allocator = memory::frame::AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        multiboot_header_address,
        multiboot_end,
        memory_map_tag.memory_areas(),
    );

    for i in 0..10 {
        match frame_allocator.allocate_frame() {
            None => {
                println!("allocated {} frames", i);
                break;
            }
            Some(frame) => println!("{:?}", frame),
        }
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

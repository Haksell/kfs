mod idt;
mod pic;

use {
    self::{idt::InterruptDescriptorTable, pic::ChainedPics},
    crate::{
        keyboard::{Keyboard, layouts::us104::Us104Key, scancodes::set1::ScancodeSet1},
        port::Port,
        shell::SHELL,
    },
    core::arch::asm,
    lazy_static::lazy_static,
    spin::Mutex,
};

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[InterruptIndex::Timer as usize].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    _red_zone: [u8; 128], // TODO: fill
}

pub fn init() {
    IDT.load();
    unsafe { PICS.lock().init() };
    enable();
}

const INTERRUPT_FLAG: usize = 1 << 9;

#[inline]
fn are_enabled() -> bool {
    let r: usize;

    unsafe {
        asm!("pushfd; pop {}", out(reg) r, options(nomem, preserves_flags));
    }

    r & INTERRUPT_FLAG != 0
}

#[inline]
fn enable() {
    unsafe {
        asm!("sti", options(preserves_flags, nostack));
    }
}

#[inline]
fn disable() {
    unsafe {
        asm!("cli", options(preserves_flags, nostack));
    }
}

#[inline]
pub fn without_interrupts<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let saved_intpt_flag = are_enabled();
    if saved_intpt_flag {
        disable();
    }
    let ret = f();
    if saved_intpt_flag {
        enable();
    }
    ret
}

extern "x86-interrupt" fn timer_interrupt_handler(_: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_: InterruptStackFrame) {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(Us104Key, ScancodeSet1::new(),));
    }

    let mut keyboard = KEYBOARD.lock();
    let scancode: u8 = unsafe { Port::new(0x60).read() };

    if let Some(key) = keyboard.add_byte(scancode) {
        SHELL.lock().send_key(key);
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}

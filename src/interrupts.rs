use crate::idt::InterruptDescriptorTable;
use crate::keyboard::{layouts, scancodes, Keyboard};
use crate::pic::ChainedPics;
use crate::port::Port;
use crate::shell::SHELL;
use core::arch::asm;
use lazy_static::lazy_static;
use spin::Mutex;

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

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init() {
    IDT.load();
    unsafe { PICS.lock().init() };
}

const INTERRUPT_FLAG: usize = 1 << 9;

#[inline]
fn are_enabled() -> bool {
    let r: usize;

    unsafe {
        asm!("pushfq; pop {}", out(reg) r, options(nomem, preserves_flags));
    }

    r & INTERRUPT_FLAG != 0
}

#[inline]
pub fn enable() {
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

extern "x86-interrupt" fn timer_interrupt_handler() {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler() {
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, scancodes::ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                layouts::Us104Key,
                scancodes::ScancodeSet1::new(),
            ));
    }

    let mut keyboard = KEYBOARD.lock();
    let scancode: u8 = unsafe { Port::new(0x60).read() };

    if let Some(key) = keyboard.add_byte(scancode) {
        SHELL.lock().send_key(key);
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

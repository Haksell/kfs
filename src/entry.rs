use bit_field::BitField;
use core::{arch::asm, marker::PhantomData};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    gdt_selector: u16,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
    phantom: PhantomData<F>,
}

impl<F> Entry<F> {
    #[inline]
    pub const fn missing() -> Self {
        Self {
            pointer_low: 0,
            gdt_selector: 0,
            options: EntryOptions::minimal(),
            pointer_middle: 0,
            pointer_high: 0,
            reserved: 0,
            phantom: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn set_handler_addr(&mut self, addr: u64) -> &mut EntryOptions {
        self.pointer_low = addr as u16;
        self.pointer_middle = (addr >> 16) as u16;
        self.pointer_high = (addr >> 32) as u32;

        self.gdt_selector = CS::get_reg().0;

        self.options.set_present(true);

        &mut self.options
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn();

pub trait HandlerFuncType {
    fn to_virt_addr(self) -> u64;
}

impl HandlerFuncType for HandlerFunc {
    fn to_virt_addr(self) -> u64 {
        self as u64
    }
}

impl<F: HandlerFuncType> Entry<F> {
    #[inline]
    pub fn set_handler_fn(&mut self, handler: F) -> &mut EntryOptions {
        unsafe { self.set_handler_addr(handler.to_virt_addr()) }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct EntryOptions(u16);

impl EntryOptions {
    #[inline]
    const fn minimal() -> Self {
        EntryOptions(0b1110_0000_0000)
    }

    #[inline]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
struct SegmentSelector(pub u16);

trait Segment {
    fn get_reg() -> SegmentSelector;
}

struct CS;

impl Segment for CS {
    #[inline]
    fn get_reg() -> SegmentSelector {
        let segment: u16;
        unsafe {
            asm!(concat!("mov {0:x}, cs"), out(reg) segment, options(nomem, nostack, preserves_flags));
        }
        SegmentSelector(segment)
    }
}

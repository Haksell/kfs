use core::{arch::asm, marker::PhantomData};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Entry<F> {
    pointer_low: u16,
    gdt_selector: u16,
    options: EntryOptions,
    pointer_middle: u16,
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
            phantom: PhantomData,
        }
    }

    #[inline]
    pub unsafe fn set_handler_addr(&mut self, addr: usize) -> &mut EntryOptions {
        self.pointer_low = addr as u16;
        self.pointer_middle = (addr >> 16) as u16;
        self.gdt_selector = CS::get_reg();
        self.options.set_present();
        &mut self.options
    }
}

pub type HandlerFunc = extern "x86-interrupt" fn();

pub trait HandlerFuncType {
    fn to_virt_addr(self) -> usize;
}

impl HandlerFuncType for HandlerFunc {
    fn to_virt_addr(self) -> usize {
        self as usize
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
        Self(0b1110_0000_0000)
    }

    #[inline]
    pub fn set_present(&mut self) {
        self.0 |= 1 << 15;
    }
}

struct CS;

impl CS {
    #[inline]
    fn get_reg() -> u16 {
        let segment: u16;
        unsafe {
            asm!(concat!("mov {0:x}, cs"), out(reg) segment, options(nomem, nostack, preserves_flags));
        }
        segment
    }
}

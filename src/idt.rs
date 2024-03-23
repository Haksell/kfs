use crate::entry::{Entry, HandlerFunc};
use core::{
    arch::asm,
    ops::{Index, IndexMut},
};

const IDT_SIZE: usize = 256;
const NB_BUILTINS: usize = 32;
const NB_INTERRUPTS: usize = IDT_SIZE - NB_BUILTINS;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base: usize,
}

#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    builtins: [Entry<HandlerFunc>; NB_BUILTINS],
    interrupts: [Entry<HandlerFunc>; NB_INTERRUPTS],
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            builtins: [Entry::missing(); NB_BUILTINS],
            interrupts: [Entry::missing(); NB_INTERRUPTS],
        }
    }

    pub fn load(&'static self) {
        unsafe {
            asm!("lidt [{}]", in(reg) &self.pointer(), options(readonly, nostack, preserves_flags));
        }
    }

    fn pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            base: self as *const _ as usize,
            limit: (core::mem::size_of::<Self>() - 1) as u16,
        }
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = Entry<HandlerFunc>;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            i @ 0..NB_BUILTINS => &self.builtins[i],
            _ => &self.interrupts[i - NB_BUILTINS],
        }
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            i @ 0..NB_BUILTINS => &mut self.builtins[i],
            _ => &mut self.interrupts[i - NB_BUILTINS],
        }
    }
}

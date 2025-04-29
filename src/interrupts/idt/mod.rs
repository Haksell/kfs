mod entry;

use self::entry::{Entry, HandlerFunc};
use core::arch::asm;
use core::ops::{Index, IndexMut};

const IDT_SIZE: usize = 256;
const NB_BUILTINS: usize = 32;
const NB_INTERRUPTS: usize = IDT_SIZE - NB_BUILTINS;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
struct DescriptorTablePointer {
    limit: u16,
    base: u32,
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
            base: self as *const _ as u32,
            limit: (core::mem::size_of::<Self>() - 1) as u16,
        }
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = Entry<HandlerFunc>;

    fn index(&self, i: usize) -> &Self::Output {
        if i < NB_BUILTINS {
            &self.builtins[i]
        } else {
            &self.interrupts[i - NB_BUILTINS]
        }
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        if i < NB_BUILTINS {
            &mut self.builtins[i]
        } else {
            &mut self.interrupts[i - NB_BUILTINS]
        }
    }
}

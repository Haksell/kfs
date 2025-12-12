mod entry;

use {
    self::entry::{Entry, HandlerFunc},
    core::{
        arch::asm,
        ops::{Index, IndexMut},
    },
};

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
            base: core::ptr::from_ref(self) as u32,
            limit: (core::mem::size_of::<Self>() - 1) as u16,
        }
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = Entry<HandlerFunc>;

    fn index(&self, index: usize) -> &Self::Output {
        if index < NB_BUILTINS {
            &self.builtins[index]
        } else {
            &self.interrupts[index - NB_BUILTINS]
        }
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index < NB_BUILTINS {
            &mut self.builtins[index]
        } else {
            &mut self.interrupts[index - NB_BUILTINS]
        }
    }
}

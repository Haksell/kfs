use core::ops::{Index, IndexMut};

use x86_64::{
    structures::idt::{Entry, HandlerFunc, HandlerFuncType},
    VirtAddr,
};

#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    builtins: [Entry<HandlerFunc>; 32],
    interrupts: [Entry<HandlerFunc>; 256 - 32],
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            builtins: [Entry::missing(); 32],
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    pub fn load(&'static self) {
        unsafe {
            x86_64::instructions::tables::lidt(&self.pointer());
        }
    }

    fn pointer(&self) -> x86_64::structures::DescriptorTablePointer {
        x86_64::structures::DescriptorTablePointer {
            base: VirtAddr::new(self as *const _ as u64),
            limit: (core::mem::size_of::<Self>() - 1) as u16,
        }
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = Entry<HandlerFunc>;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            i @ 0..=31 => &self.builtins[i],
            _ => &self.interrupts[i - 32],
        }
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match i {
            i @ 0..=31 => &mut self.builtins[i],
            _ => &mut self.interrupts[i - 32],
        }
    }
}

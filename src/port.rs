use core::{arch::asm, marker::PhantomData};

pub trait PortRead {
    unsafe fn read_from_port(port: u16) -> Self;
}

pub trait PortWrite {
    unsafe fn write_to_port(port: u16, value: Self);
}

impl PortRead for u8 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u8 {
        let value: u8;
        unsafe {
            asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        value
    }
}

impl PortRead for u16 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u16 {
        let value: u16;
        unsafe {
            asm!("in ax, dx", out("ax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        value
    }
}

impl PortRead for u32 {
    #[inline]
    unsafe fn read_from_port(port: u16) -> u32 {
        let value: u32;
        unsafe {
            asm!("in eax, dx", out("eax") value, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        value
    }
}

impl PortWrite for u8 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u8) {
        unsafe {
            asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
        }
    }
}

impl PortWrite for u16 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u16) {
        unsafe {
            asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags));
        }
    }
}

impl PortWrite for u32 {
    #[inline]
    unsafe fn write_to_port(port: u16, value: u32) {
        unsafe {
            asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags));
        }
    }
}

pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    #[inline]
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            phantom: PhantomData,
        }
    }
}

impl<T: PortRead> Port<T> {
    #[inline]
    pub unsafe fn read(&mut self) -> T {
        unsafe { T::read_from_port(self.port) }
    }
}

impl<T: PortWrite> Port<T> {
    #[inline]
    pub unsafe fn write(&mut self, value: T) {
        unsafe { T::write_to_port(self.port, value) }
    }
}

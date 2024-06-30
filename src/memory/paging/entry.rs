use crate::memory::frame::Frame;

pub struct Entry(usize);

bitflags! {
    pub struct EntryFlags: usize {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const HUGE_PAGE =       1 << 7;
    }
}

const ADDRESS_MASK: usize = 0xffff_f000;

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.flags().contains(EntryFlags::PRESENT) {
            Some(Frame::containing_address(self.0 & ADDRESS_MASK))
        } else {
            None
        }
    }

    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
        assert!(frame.start_address() & !ADDRESS_MASK == 0);
        self.0 = frame.start_address() | flags.bits();
    }
}

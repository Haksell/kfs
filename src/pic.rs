use crate::port::Port;

const CMD_INIT: u8 = 0x11;
const CMD_END_OF_INTERRUPT: u8 = 0x20;

const MODE_8086: u8 = 0x01;

struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
    cascade: u8,
}

impl Pic {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }

    unsafe fn read_mask(&mut self) -> u8 {
        self.data.read()
    }

    unsafe fn write_mask(&mut self, mask: u8) {
        self.data.write(mask)
    }

    // Initialization Command Words

    unsafe fn icw1(&mut self) {
        self.command.write(CMD_INIT);
    }

    unsafe fn icw2(&mut self) {
        self.data.write(self.offset);
    }

    unsafe fn icw3(&mut self) {
        self.data.write(self.cascade);
    }

    unsafe fn icw4(&mut self) {
        self.data.write(MODE_8086);
    }
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: Port::new(0x20),
                    data: Port::new(0x21),
                    cascade: 0b100,
                },
                Pic {
                    offset: offset2,
                    command: Port::new(0xA0),
                    data: Port::new(0xA1),
                    cascade: 2,
                },
            ],
        }
    }

    pub unsafe fn init(&mut self) {
        let mut wait_port: Port<u8> = Port::new(0x80);
        let mut wait = || wait_port.write(0);
        let saved_masks = self.read_masks();
        for icw in &[Pic::icw1, Pic::icw2, Pic::icw3, Pic::icw4] {
            for pic in &mut self.pics {
                icw(pic);
                wait();
            }
        }
        self.write_masks(saved_masks[0], saved_masks[1])
    }

    unsafe fn read_masks(&mut self) -> [u8; 2] {
        [self.pics[0].read_mask(), self.pics[1].read_mask()]
    }

    unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) {
        self.pics[0].write_mask(mask1);
        self.pics[1].write_mask(mask2);
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.pics[1].handles_interrupt(interrupt_id) {
            self.pics[1].end_of_interrupt();
            self.pics[0].end_of_interrupt();
        } else if self.pics[0].handles_interrupt(interrupt_id) {
            self.pics[0].end_of_interrupt();
        }
    }
}

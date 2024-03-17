use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

use crate::port::Port;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    fn empty() -> Self {
        Self {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black),
        }
    }
}

// TODO: put in some utility module
fn write_port(port: u16, value: u8) {
    unsafe {
        let mut port = Port::new(port);
        port.write(value);
    }
}

fn update_cursor(row: usize, col: usize) {
    let pos = row * VGA_WIDTH + col;
    write_port(0x3D4, 0x0E);
    write_port(0x3D5, (pos >> 8) as u8);
    write_port(0x3D4, 0x0F);
    write_port(0x3D5, (pos & 0xFF) as u8);
}

pub const VGA_HEIGHT: usize = 25;
pub const VGA_WIDTH: usize = 80;
pub const VGA_HISTORY: usize = 100; // has to be ≥ VGA_HEIGHT
pub const VGA_SCREENS: usize = 4;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
    screen_idx: usize,
    screens: [[[ScreenChar; VGA_WIDTH]; VGA_HEIGHT]; VGA_SCREENS],
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= VGA_WIDTH {
                    self.new_line();
                }
                let screen_char = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.buffer.chars[VGA_HEIGHT - 1][self.column_position].write(screen_char);
                self.screens[self.screen_idx][VGA_HEIGHT - 1][self.column_position] = screen_char;
                self.column_position += 1;
            }
        }
        update_cursor(VGA_HEIGHT - 1, self.column_position);
    }

    // TODO: write_bytes that accepts a &[u8] and only moves the cursor once

    pub fn write_bytes(&mut self, byte: u8, repeat: usize) {
        for _ in 0..repeat {
            self.write_byte(byte);
        }
    }

    pub fn set_foreground_color(&mut self, foreground_code: Color) {
        // TODO: keep old background color
        self.color_code = ColorCode::new(foreground_code, Color::Black);
    }

    pub fn reset_foreground_color(&mut self) {
        self.set_foreground_color(Color::White);
    }

    pub fn set_cursor(&mut self, col: usize) {
        self.column_position = col;
        update_cursor(VGA_HEIGHT - 1, self.column_position);
    }

    pub fn switch_screen(&mut self, screen_idx: usize, cursor: usize) {
        if screen_idx != self.screen_idx && screen_idx < VGA_SCREENS && cursor < VGA_WIDTH {
            self.screen_idx = screen_idx;
            for y in 0..VGA_HEIGHT {
                for x in 0..VGA_WIDTH {
                    self.buffer.chars[y][x].write(self.screens[screen_idx][y][x]);
                }
            }
            self.set_cursor(cursor)
        }
    }

    pub fn clear_vga_buffer(&mut self) {
        let blank = ScreenChar::empty();
        for y in 0..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.buffer.chars[y][x].write(blank);
            }
        }
    }

    fn new_line(&mut self) {
        for y in 1..VGA_HEIGHT {
            for x in 0..VGA_WIDTH {
                self.buffer.chars[y - 1][x].write(self.screens[self.screen_idx][y][x]);
                self.screens[self.screen_idx][y - 1][x] = self.screens[self.screen_idx][y][x];
            }
        }
        self.clear_row(VGA_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, y: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for x in 0..VGA_WIDTH {
            self.buffer.chars[y][x].write(blank);
            self.screens[self.screen_idx][y][x] = blank;
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        screen_idx: 0,
        screens: [[[ScreenChar::empty(); VGA_WIDTH]; VGA_HEIGHT]; VGA_SCREENS],
    });
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

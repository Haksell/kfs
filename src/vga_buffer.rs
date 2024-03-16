use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::port::Port;

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
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

fn write_port(port: u16, value: u8) {
    unsafe {
        let mut port = Port::new(port);
        port.write(value);
    }
}

fn update_cursor(row: usize, col: usize) {
    let pos = row * BUFFER_WIDTH + col;
    write_port(0x3D4, 0x0E);
    write_port(0x3D5, (pos >> 8) as u8);
    write_port(0x3D4, 0x0F);
    write_port(0x3D5, (pos & 0xFF) as u8);
}

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            0x08 => {
                if self.column_position >= 1 {
                    self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position - 1].write(
                        ScreenChar {
                            ascii_character: b' ',
                            color_code: self.color_code,
                        },
                    );
                    self.column_position -= 1;
                }
            }
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.buffer.chars[BUFFER_HEIGHT - 1][self.column_position].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column_position += 1;
            }
        }
        update_cursor(BUFFER_HEIGHT - 1, self.column_position);
    }

    pub fn set_foreground_color(&mut self, foreground_code: Color) {
        // TODO: keep old background color
        self.color_code = ColorCode::new(foreground_code, Color::Black);
    }

    fn new_line(&mut self) {
        for y in 1..BUFFER_HEIGHT {
            for x in 0..BUFFER_WIDTH {
                self.buffer.chars[y - 1][x].write(self.buffer.chars[y][x].read());
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | 0x08 | b'\n' => self.write_byte(byte),
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

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}

pub const MENU_MARGIN: usize = 10;

fn print_welcome_line(left: u8, middle: u8, right: u8) {
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
    WRITER.lock().write_byte(left);
    for _ in 0..BUFFER_WIDTH - 2 - 2 * (MENU_MARGIN) {
        WRITER.lock().write_byte(middle);
    }
    WRITER.lock().write_byte(right);
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
}

fn print_welcome_title() {
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
    WRITER.lock().write_byte(b'\xba');
    for _ in 0..26 {
        WRITER.lock().write_byte(b' ');
    }
    WRITER.lock().write_byte(b'K');
    WRITER.lock().write_byte(b'F');
    WRITER.lock().write_byte(b'S');
    WRITER.lock().write_byte(b' ');
    WRITER.lock().write_byte(b'4');
    WRITER.lock().write_byte(b'2');
    for _ in 0..26 {
        WRITER.lock().write_byte(b' ');
    }
    WRITER.lock().write_byte(b'\xba');
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
}

// TODO: macro that accepts format args
pub fn print_welcome() {
    // TODO: write directly at correct position instead of using write_byte
    WRITER.lock().set_foreground_color(Color::LightRed);
    print_welcome_line(b'\xc9', b'\xcd', b'\xbb');
    print_welcome_line(b'\xba', b' ', b'\xba');
    print_welcome_title();
    print_welcome_line(b'\xba', b' ', b'\xba');
    print_welcome_line(b'\xc8', b'\xcd', b'\xbc');
    WRITER.lock().set_foreground_color(Color::White);
    println!();
    println!();
}

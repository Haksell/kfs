use crate::{
    print, println,
    vga_buffer::{Color, WRITER},
};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::Mutex;

// TODO: make all of that a struct?

const PROMPT: &'static str = "> ";
const MAX_COMMAND_LEN: usize = crate::vga_buffer::BUFFER_WIDTH - PROMPT.len() - 1;

struct CommandBuffer {
    buffer: [char; MAX_COMMAND_LEN],
    length: usize,
}

lazy_static! {
    static ref COMMAND: Mutex<CommandBuffer> = Mutex::new(CommandBuffer {
        buffer: ['\0'; MAX_COMMAND_LEN],
        length: 0,
    });
}

fn print_prompt() {
    crate::vga_buffer::WRITER
        .lock()
        .set_foreground_color(Color::LightGreen);
    print!("{}", PROMPT);
    crate::vga_buffer::WRITER.lock().reset_foreground_color();
}

const MENU_MARGIN: usize = 10;

fn print_welcome_line(left: u8, middle: u8, right: u8) {
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
    WRITER.lock().write_byte(left);
    for _ in 0..crate::vga_buffer::BUFFER_WIDTH - 2 - 2 * (MENU_MARGIN) {
        WRITER.lock().write_byte(middle);
    }
    WRITER.lock().write_byte(right);
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
}

fn print_welcome_title(s: &str) {
    let remaining_width = crate::vga_buffer::BUFFER_WIDTH - 2 - 2 * MENU_MARGIN - s.len();
    let left_padding = remaining_width >> 1;
    let right_padding = (remaining_width + 1) >> 1;
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
    WRITER.lock().write_byte(b'\xba');
    for _ in 0..left_padding {
        WRITER.lock().write_byte(b' ');
    }
    for b in s.bytes() {
        WRITER.lock().write_byte(b);
    }
    for _ in 0..right_padding {
        WRITER.lock().write_byte(b' ');
    }
    WRITER.lock().write_byte(b'\xba');
    for _ in 0..MENU_MARGIN {
        WRITER.lock().write_byte(b' ');
    }
}

fn print_welcome() {
    WRITER.lock().set_foreground_color(Color::LightGreen);
    print_welcome_line(b'\xc9', b'\xcd', b'\xbb');
    print_welcome_line(b'\xba', b' ', b'\xba');
    print_welcome_title("KFS 42");
    print_welcome_line(b'\xba', b' ', b'\xba');
    print_welcome_line(b'\xc8', b'\xcd', b'\xbc');
    WRITER.lock().reset_foreground_color();
    println!();
}

pub fn init() {
    print_welcome();
    print_prompt();
}

pub fn send_key(key: DecodedKey) {
    match key {
        DecodedKey::Unicode(character) => match character {
            '\x08' => {
                let mut command = COMMAND.lock();
                if command.length > 0 {
                    command.length -= 1;
                    let len = command.length;
                    command.buffer[len] = '\0';
                    print!("{}", character);
                }
            }
            '\n' => {
                println!();
                let len = COMMAND.lock().length;
                if len > 0 {
                    for i in (0..len).rev() {
                        print!("{}", COMMAND.lock().buffer[i]);
                    }
                    println!();
                }
                COMMAND.lock().length = 0;
                print_prompt();
            }
            _ => {
                let mut command = COMMAND.lock();
                if command.length < MAX_COMMAND_LEN {
                    let len = command.length;
                    command.buffer[len] = character;
                    command.length += 1;
                    print!("{}", character);
                }
            }
        },
        DecodedKey::RawKey(key) => print!("{:?}", key),
    }
}

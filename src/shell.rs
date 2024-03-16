use crate::{
    print, println,
    vga_buffer::{Color, BUFFER_WIDTH, WRITER},
};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::Mutex;

// TODO: make all of that a struct?

const PROMPT: &'static str = "> "; // TODO: &[u8]
const MAX_COMMAND_LEN: usize = BUFFER_WIDTH - PROMPT.len() - 1;

struct CommandBuffer {
    buffer: [char; MAX_COMMAND_LEN],
    len: usize,
    pos: usize,
}

lazy_static! {
    static ref COMMAND: Mutex<CommandBuffer> = Mutex::new(CommandBuffer {
        buffer: ['\0'; MAX_COMMAND_LEN],
        len: 0,
        pos: 0,
    });
}

fn print_prompt() {
    WRITER.lock().set_foreground_color(Color::LightGreen);
    print!("{}", PROMPT);
    WRITER.lock().reset_foreground_color();
}

const MENU_MARGIN: usize = 10;

fn print_welcome_line(left: u8, middle: u8, right: u8) {
    WRITER.lock().write_bytes(b' ', MENU_MARGIN);
    WRITER.lock().write_byte(left);
    WRITER
        .lock()
        .write_bytes(middle, BUFFER_WIDTH - 2 - 2 * MENU_MARGIN);
    WRITER.lock().write_byte(right);
    WRITER.lock().write_bytes(b' ', MENU_MARGIN);
}

fn print_welcome_title(s: &str) {
    let remaining_width = BUFFER_WIDTH - 2 - 2 * MENU_MARGIN - s.len();
    WRITER.lock().write_bytes(b' ', MENU_MARGIN);
    WRITER.lock().write_byte(b'\xba');
    WRITER.lock().write_bytes(b' ', remaining_width >> 1);
    for b in s.bytes() {
        WRITER.lock().write_byte(b);
    }
    WRITER.lock().write_bytes(b' ', (remaining_width + 1) >> 1);
    WRITER.lock().write_byte(b'\xba');
    WRITER.lock().write_bytes(b' ', MENU_MARGIN);
}

fn print_welcome() {
    WRITER.lock().set_foreground_color(Color::LightGreen);
    print_welcome_line(b'\xc9', b'\xcd', b'\xbb');
    print_welcome_line(b'\xba', b' ', b'\xba');
    print_welcome_title("KFS 42");
    print_welcome_line(b'\xba', b' ', b'\xba');
    print_welcome_line(b'\xc8', b'\xcd', b'\xbc');
    WRITER.lock().reset_foreground_color();
}

pub fn init() {
    print_welcome();
    println!();
    println!();
    print_prompt();
}

pub fn send_key(key: DecodedKey) {
    use pc_keyboard::KeyCode;
    let mut command = COMMAND.lock();
    match key {
        DecodedKey::Unicode(character) => match character {
            '\x08' => {
                if command.len > 0 {
                    command.len -= 1;
                    command.pos -= 1;
                    // TODO: move evrything left if len != pos
                    print!("{}", character);
                }
            }
            '\n' => {
                println!();
                if command.len > 0 {
                    for i in (0..command.len).rev() {
                        print!("{}", command.buffer[i]);
                    }
                    println!();
                }
                command.len = 0;
                command.pos = 0;
                print_prompt();
            }
            _ => {
                if command.len < MAX_COMMAND_LEN {
                    for i in (command.pos..command.len).rev() {
                        command.buffer[i + 1] = command.buffer[i];
                    }
                    let pos = command.pos;
                    command.buffer[pos] = character;
                    command.len += 1;
                    WRITER.lock().set_cursor(PROMPT.len() + command.pos);
                    for i in command.pos..command.len {
                        print!("{}", command.buffer[i]);
                    }
                    command.pos += 1;
                    WRITER.lock().set_cursor(PROMPT.len() + command.pos);
                }
            }
        },
        DecodedKey::RawKey(key) => match key {
            KeyCode::ArrowLeft => {
                if command.pos > 0 {
                    WRITER.lock().move_left();
                    command.pos -= 1;
                }
            }
            KeyCode::ArrowRight => {
                let len = command.len;
                if command.pos < len {
                    WRITER.lock().move_right();
                    command.pos += 1;
                }
            }
            _ => print!("{:?}", key),
        },
    }
}

use crate::{
    print, println,
    vga_buffer::{Color, VGA_WIDTH, WRITER},
};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::{Mutex, MutexGuard};

// TODO: test profusely, especially special characters

// Maybe an enum or a transparent struct would be better?
mod special_char {
    pub const BACKSPACE: char = '\x08';
    pub const NEWLINE: char = '\x0a';
    pub const DELETE: char = '\x7f';
}

const PRIMARY_COLOR: Color = Color::LightGreen;
const PROMPT: &'static str = "> "; // TODO: &[u8]
const MAX_COMMAND_LEN: usize = VGA_WIDTH - PROMPT.len() - 1;

struct CommandBuffer {
    buffer: [char; MAX_COMMAND_LEN], // TODO: [u8; MAX_COMMAND_LEN]
    len: usize,
    pos: usize,
}

impl CommandBuffer {
    pub fn set_pos(&mut self, pos: usize) {
        // Breaks if pos > MAX_COMMAND_LEN. Use assert!() ?
        self.pos = pos;
        WRITER.lock().set_cursor(PROMPT.len() + pos);
    }

    pub fn move_left(&mut self) {
        if self.pos > 0 {
            self.set_pos(self.pos - 1);
        }
    }

    pub fn move_right(&mut self) {
        if self.pos < self.len {
            self.set_pos(self.pos + 1);
        }
    }
}

lazy_static! {
    static ref COMMAND: Mutex<CommandBuffer> = Mutex::new(CommandBuffer {
        buffer: ['\0'; MAX_COMMAND_LEN],
        len: 0,
        pos: 0,
    });
}

fn print_prompt() {
    WRITER.lock().set_foreground_color(PRIMARY_COLOR);
    print!("{}", PROMPT);
    WRITER.lock().reset_foreground_color();
}

const MENU_MARGIN: usize = 10;

fn print_welcome_line(left: u8, middle: u8, right: u8) {
    WRITER.lock().write_bytes(b' ', MENU_MARGIN);
    WRITER.lock().write_byte(left);
    WRITER
        .lock()
        .write_bytes(middle, VGA_WIDTH - 2 - 2 * MENU_MARGIN);
    WRITER.lock().write_byte(right);
    WRITER.lock().write_bytes(b' ', MENU_MARGIN);
}

fn print_welcome_title(s: &str) {
    let remaining_width = VGA_WIDTH - 2 - 2 * MENU_MARGIN - s.len();
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
    WRITER.lock().set_foreground_color(PRIMARY_COLOR);
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

fn delete_char(command: &mut MutexGuard<CommandBuffer>, decrement_pos: bool) {
    command.len -= 1;
    if decrement_pos {
        command.pos -= 1;
    }
    for i in command.pos..command.len {
        command.buffer[i] = command.buffer[i + 1];
    }
    WRITER.lock().set_cursor(PROMPT.len() + command.pos);
    for i in command.pos..command.len {
        print!("{}", command.buffer[i]);
    }
    print!(" ");
    WRITER.lock().set_cursor(PROMPT.len() + command.pos);
}

fn execute_command(command: &MutexGuard<CommandBuffer>) {
    // TODO: basic commands:
    // - change screen
    // - get basic info
    // - exit
    for i in (0..command.len).rev() {
        print!("{}", command.buffer[i]);
    }
    println!();
}

pub fn send_key(key: DecodedKey) {
    use pc_keyboard::KeyCode;
    let mut command = COMMAND.lock();
    let start_len = command.len;
    let start_pos = command.pos;
    match key {
        DecodedKey::Unicode(character) => match character {
            special_char::NEWLINE => {
                println!();
                if command.len > 0 {
                    execute_command(&command);
                }
                print_prompt();
                command.len = 0;
                command.set_pos(0);
            }
            special_char::BACKSPACE => {
                if command.pos > 0 {
                    delete_char(&mut command, true);
                }
            }
            special_char::DELETE => {
                if command.pos < command.len {
                    delete_char(&mut command, false);
                }
            }
            '\x20'..='\x7e' => {
                if command.len < MAX_COMMAND_LEN {
                    for i in (command.pos..command.len).rev() {
                        command.buffer[i + 1] = command.buffer[i];
                    }
                    command.buffer[start_pos] = character;
                    WRITER.lock().set_cursor(PROMPT.len() + command.pos);
                    command.len += 1;
                    for i in command.pos..command.len {
                        print!("{}", command.buffer[i]);
                    }
                    command.pos += 1;
                    WRITER.lock().set_cursor(PROMPT.len() + command.pos);
                }
            }
            _ => {}
        },
        DecodedKey::RawKey(key) => match key {
            KeyCode::ArrowLeft => command.move_left(),
            KeyCode::ArrowRight => command.move_right(),
            KeyCode::Home => command.set_pos(0),
            KeyCode::End => command.set_pos(start_len),
            _ => print!("{:?}", key), // TODO: remove
        },
    }
}

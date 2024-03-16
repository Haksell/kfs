use crate::{print, println, vga_buffer::Color};
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
    crate::vga_buffer::WRITER
        .lock()
        .set_foreground_color(Color::White);
}

pub fn init() {
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
                for i in (0..len).rev() {
                    print!("{}", COMMAND.lock().buffer[i]);
                }
                COMMAND.lock().length = 0;
                println!();
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

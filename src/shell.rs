use crate::{print, println};
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

pub fn new_line() {
    COMMAND.lock().length = 0;
    println!();
    print!("{}", PROMPT);
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
            '\n' => new_line(),
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

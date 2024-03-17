use crate::{
    print, println,
    vga_buffer::{Color, VGA_SCREENS, VGA_WIDTH, WRITER},
};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use spin::Mutex;

// TODO: test profusely, especially special characters
// There are too many locks, I'm scared of deadlocks in case of interrupts

// Maybe an enum or a transparent struct would be better?
mod special_char {
    pub const BACKSPACE: char = '\x08';
    pub const NEWLINE: char = '\x0a';
    pub const DELETE: char = '\x7f';
}

const SCREEN_COLORS: [Color; VGA_SCREENS] = [
    Color::LightGreen,
    Color::LightCyan,
    Color::LightRed,
    Color::Pink,
];
const PROMPT: &'static str = "> "; // TODO: [ScreenChar; PROMPT_LEN]
const MAX_COMMAND_LEN: usize = VGA_WIDTH - PROMPT.len() - 1;
const MENU_MARGIN: usize = 10;

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

struct Shell {
    screen_idx: usize,
    commands: [CommandBuffer; VGA_SCREENS],
}

lazy_static! {
    static ref SHELL: Mutex<Shell> = Mutex::new(Shell {
        screen_idx: 0,
        commands: core::array::from_fn(|_| CommandBuffer {
            buffer: ['\0'; MAX_COMMAND_LEN],
            len: 0,
            pos: 0,
        }),
    });
}

impl Shell {
    pub fn switch_screen(&mut self, screen_idx: usize) {
        if screen_idx != self.screen_idx {
            self.screen_idx = screen_idx;
            let mut writer = WRITER.lock();
            writer.switch_screen(screen_idx);
            writer.set_cursor(PROMPT.len() + self.commands[screen_idx].pos);
        }
    }

    fn print_prompt(&self) {
        WRITER
            .lock()
            .set_foreground_color(SCREEN_COLORS[self.screen_idx]);
        print!("{}", PROMPT);
        WRITER.lock().reset_foreground_color();
    }

    fn print_welcome_line(&self, left: u8, middle: u8, right: u8) {
        WRITER.lock().write_bytes(b' ', MENU_MARGIN);
        WRITER.lock().write_byte(left);
        WRITER
            .lock()
            .write_bytes(middle, VGA_WIDTH - 2 - 2 * MENU_MARGIN);
        WRITER.lock().write_byte(right);
        WRITER.lock().write_bytes(b' ', MENU_MARGIN);
    }

    fn print_welcome_title(&self, s: &str) {
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

    fn print_welcome(&self) {
        WRITER
            .lock()
            .set_foreground_color(SCREEN_COLORS[self.screen_idx]);
        self.print_welcome_line(b'\xc9', b'\xcd', b'\xbb');
        self.print_welcome_line(b'\xba', b' ', b'\xba');
        self.print_welcome_title("KFS 42"); // TODO: print screen_idx instead of 42
        self.print_welcome_line(b'\xba', b' ', b'\xba');
        self.print_welcome_line(b'\xc8', b'\xcd', b'\xbc');
        WRITER.lock().reset_foreground_color();
    }

    pub fn init(&mut self) {
        for i in (0..VGA_SCREENS).rev() {
            // TODO: don't write to vga_buffer for screens 1..VGA_SCREENS
            self.switch_screen(i);
            self.print_welcome();
            println!();
            self.print_prompt();
        }
    }
}

fn delete_char(command: &mut CommandBuffer, decrement_pos: bool) {
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

fn execute_command(command: &CommandBuffer) {
    // TODO: basic commands:
    // - clear screen
    // - get basic info
    // - exit
    // - print shell number
    for i in (0..command.len).rev() {
        print!("{}", command.buffer[i]);
    }
    println!();
}

// TODO: method of SHELL?
pub fn send_key(key: DecodedKey) {
    use pc_keyboard::KeyCode;
    let mut shell = SHELL.lock();
    let screen_idx = shell.screen_idx;
    // TODO: find a way to do let command = shell.commands[screen_idx])
    let start_len = shell.commands[screen_idx].len;
    let start_pos = shell.commands[screen_idx].pos;
    match key {
        DecodedKey::Unicode(character) => match character {
            special_char::NEWLINE => {
                println!();
                if shell.commands[screen_idx].len > 0 {
                    execute_command(&shell.commands[screen_idx]);
                }
                shell.print_prompt();
                shell.commands[screen_idx].len = 0;
                shell.commands[screen_idx].set_pos(0);
            }
            special_char::BACKSPACE => {
                if shell.commands[screen_idx].pos > 0 {
                    delete_char(&mut shell.commands[screen_idx], true);
                }
            }
            special_char::DELETE => {
                if start_pos < start_len {
                    delete_char(&mut shell.commands[screen_idx], false);
                }
            }
            '\x20'..='\x7e' => {
                if start_len < MAX_COMMAND_LEN {
                    let command = &mut shell.commands[screen_idx];
                    for i in (start_pos..start_len).rev() {
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
            KeyCode::ArrowLeft => shell.commands[screen_idx].move_left(),
            KeyCode::ArrowRight => shell.commands[screen_idx].move_right(),
            KeyCode::Home => shell.commands[screen_idx].set_pos(0),
            KeyCode::End => shell.commands[screen_idx].set_pos(start_len),
            // TODO: use range F1..F12 once we implement the keyboard crate
            KeyCode::F1 => shell.switch_screen(0),
            KeyCode::F2 => shell.switch_screen(1),
            KeyCode::F3 => shell.switch_screen(2),
            KeyCode::F4 => shell.switch_screen(3),
            KeyCode::F5 => shell.switch_screen(4),
            KeyCode::F6 => shell.switch_screen(5),
            KeyCode::F7 => shell.switch_screen(6),
            KeyCode::F8 => shell.switch_screen(7),
            KeyCode::F9 => shell.switch_screen(8),
            KeyCode::F10 => shell.switch_screen(9),
            KeyCode::F11 => shell.switch_screen(10),
            KeyCode::F12 => shell.switch_screen(11),
            _ => print!("{:?}", key), // TODO: remove
        },
    }
}

pub fn init() {
    SHELL.lock().init();
}

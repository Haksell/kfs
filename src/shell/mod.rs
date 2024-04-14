mod command_handlers;

use crate::keyboard::{DecodedKey, KeyCode};
use crate::println;
use crate::vga_buffer::{Color, VGA_SCREENS, VGA_WIDTH, WRITER};
use command_handlers::{exit_qemu, QemuExitCode, COMMAND_HANDLERS};
use lazy_static::lazy_static;
use spin::Mutex;

// TODO: test profusely, especially special characters
// There are too many locks, I'm scared of deadlocks in case of interrupts

// Maybe an enum or a transparent struct would be better?
mod special_char {
    pub const BACKSPACE: char = '\x08';
    pub const NEWLINE: char = '\x0a';
    pub const ESCAPE: char = '\x1b';
    pub const DELETE: char = '\x7f';
}

const PROMPT: &[u8] = b"> ";
const MAX_COMMAND_LEN: usize = VGA_WIDTH - PROMPT.len() - 1;
const WELCOME_MARGIN: usize = 2;
const CORNER_REPEAT: usize = 3; // 1 for something not too weird

struct CommandBuffer {
    buffer: [u8; MAX_COMMAND_LEN],
    len: usize,
    pos: usize,
    color: Color,
}

impl CommandBuffer {
    pub fn new(color: Color) -> Self {
        Self {
            buffer: [0; MAX_COMMAND_LEN],
            len: 0,
            pos: 0,
            color,
        }
    }

    fn set_pos(&mut self, pos: usize) {
        // Breaks if pos > MAX_COMMAND_LEN. Use assert!() ?
        self.pos = pos;
        WRITER.lock().set_cursor(PROMPT.len() + pos);
    }

    fn move_left(&mut self) {
        if self.pos > 0 {
            self.set_pos(self.pos - 1);
        }
    }

    fn move_right(&mut self) {
        if self.pos < self.len {
            self.set_pos(self.pos + 1);
        }
    }

    fn trimmed(&self) -> &[u8] {
        // b'\0' and b'\xff' are also whitespace, but they can't be typed for now
        let mut start = 0;
        while start < self.len && self.buffer[start] == b' ' {
            start += 1;
        }
        let mut end = self.len;
        while end > start && self.buffer[end - 1] == b' ' {
            end -= 1;
        }
        &self.buffer[start..end]
    }
}

pub struct Shell {
    screen_idx: usize,
    commands: [CommandBuffer; VGA_SCREENS],
}

impl Shell {
    pub fn init(&mut self) {
        for i in (0..VGA_SCREENS).rev() {
            // TODO: don't write to vga_buffer for screens 1..VGA_SCREENS
            self.screen_idx = i;
            WRITER.lock().switch_screen(i, 0);
            self.print_welcome();
            WRITER.lock().write_bytes(b'\n', 2);
            self.print_prompt();
            WRITER.lock().reset_history();
        }
    }

    pub fn send_key(&mut self, key: DecodedKey) {
        let screen_idx = self.screen_idx;
        let start_len = self.commands[screen_idx].len;
        let start_pos = self.commands[screen_idx].pos;
        match key {
            DecodedKey::Unicode(character) => match character {
                special_char::NEWLINE => {
                    WRITER.lock().write_byte(b'\n');
                    self.execute_command();
                    self.print_prompt();
                    self.commands[screen_idx].len = 0;
                    self.commands[screen_idx].set_pos(0);
                }
                special_char::BACKSPACE => {
                    if self.commands[screen_idx].pos > 0 {
                        self.delete_char(screen_idx, true);
                    }
                }
                special_char::ESCAPE => exit_qemu(QemuExitCode::Success),
                special_char::DELETE => {
                    if start_pos < start_len {
                        self.delete_char(screen_idx, false);
                    }
                }
                '\x20'..='\x7e' => {
                    if start_len < MAX_COMMAND_LEN {
                        let command = &mut self.commands[screen_idx];
                        for i in (start_pos..start_len).rev() {
                            command.buffer[i + 1] = command.buffer[i];
                        }
                        command.buffer[start_pos] = character as u8;
                        WRITER.lock().set_cursor(PROMPT.len() + command.pos);
                        command.len += 1;
                        for i in command.pos..command.len {
                            WRITER.lock().write_byte(command.buffer[i]);
                        }
                        command.set_pos(command.pos + 1);
                    }
                }
                _ => {}
            },
            DecodedKey::RawKey(k) => match k {
                KeyCode::ArrowLeft => self.commands[screen_idx].move_left(),
                KeyCode::ArrowRight => self.commands[screen_idx].move_right(),
                KeyCode::Home => self.commands[screen_idx].set_pos(0),
                KeyCode::End => self.commands[screen_idx].set_pos(start_len),
                KeyCode::ArrowUp => WRITER.lock().move_up(),
                KeyCode::ArrowDown => WRITER.lock().move_down(),
                KeyCode::PageUp => WRITER.lock().move_all_the_way_up(),
                KeyCode::PageDown => WRITER.lock().move_all_the_way_down(),
                k if (KeyCode::F1..=KeyCode::F4).contains(&k) => {
                    self.switch_screen((k as u8 - KeyCode::F1 as u8) as usize)
                }
                _ => {}
            },
        }
    }

    fn switch_screen(&mut self, screen_idx: usize) {
        if screen_idx != self.screen_idx && screen_idx < VGA_SCREENS {
            self.screen_idx = screen_idx;
            WRITER
                .lock()
                .switch_screen(screen_idx, self.commands[screen_idx].pos + PROMPT.len());
        }
    }

    fn print_prompt(&self) {
        WRITER
            .lock()
            .set_foreground_color(self.commands[self.screen_idx].color);
        for &byte in PROMPT {
            WRITER.lock().write_byte(byte);
        }
        WRITER.lock().reset_foreground_color();
    }

    fn print_welcome_line(left: u8, left2: u8, middle: u8, right2: u8, right: u8) {
        WRITER.lock().write_bytes(b' ', WELCOME_MARGIN);
        WRITER.lock().write_byte(left);
        WRITER.lock().write_bytes(left2, CORNER_REPEAT);
        WRITER
            .lock()
            .write_bytes(middle, VGA_WIDTH - 2 * (WELCOME_MARGIN + CORNER_REPEAT + 1));
        WRITER.lock().write_bytes(right2, CORNER_REPEAT);
        WRITER.lock().write_byte(right);
        WRITER.lock().write_bytes(b' ', WELCOME_MARGIN);
    }

    fn print_welcome_title(s: &'static [u8]) {
        let remaining_width = VGA_WIDTH - 2 - 2 * WELCOME_MARGIN - s.len();
        WRITER.lock().write_bytes(b' ', WELCOME_MARGIN);
        WRITER.lock().write_byte(b'\xba');
        WRITER.lock().write_bytes(b' ', remaining_width >> 1);
        for &b in s {
            WRITER.lock().write_byte(b);
        }
        WRITER.lock().write_bytes(b' ', (remaining_width + 1) >> 1);
        WRITER.lock().write_byte(b'\xba');
        WRITER.lock().write_bytes(b' ', WELCOME_MARGIN);
    }

    fn print_welcome(&self) {
        WRITER
            .lock()
            .set_foreground_color(self.commands[self.screen_idx].color);
        Self::print_welcome_line(b' ', b'\xc9', b'\xcd', b'\xbb', b' ');
        Self::print_welcome_line(b'\xc9', b'\xbc', b' ', b'\xc8', b'\xbb');
        Self::print_welcome_title(b"\x20\x20\x20\x20\x20\x20\x3a\x3a\x3a\x20\x20\x20\x20\x3a\x3a\x3a\x20\x3a\x3a\x3a\x3a\x3a\x3a\x3a\x3a\x3a\x3a\x20\x3a\x3a\x3a\x3a\x3a\x3a\x3a\x3a\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x3a\x3a\x3a\x20\x20\x20\x20\x20\x3a\x3a\x3a\x3a\x3a\x3a\x3a\x3a");
        Self::print_welcome_title(b"\x20\x20\x20\x20\x20\x3a\x2b\x3a\x20\x20\x20\x3a\x2b\x3a\x20\x20\x3a\x2b\x3a\x20\x20\x20\x20\x20\x20\x20\x3a\x2b\x3a\x20\x20\x20\x20\x3a\x2b\x3a\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x3a\x2b\x3a\x20\x20\x20\x20\x20\x3a\x2b\x3a\x20\x20\x20\x20\x3a\x2b\x3a");
        Self::print_welcome_title(b"\x20\x20\x20\x20\x2b\x3a\x2b\x20\x20\x2b\x3a\x2b\x20\x20\x20\x2b\x3a\x2b\x20\x20\x20\x20\x20\x20\x20\x2b\x3a\x2b\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x2b\x3a\x2b\x20\x2b\x3a\x2b\x20\x20\x20\x20\x20\x20\x20\x20\x2b\x3a\x2b\x20\x20");
        Self::print_welcome_title(b"\x20\x20\x20\x2b\x23\x2b\x2b\x3a\x2b\x2b\x20\x20\x20\x20\x3a\x23\x3a\x3a\x2b\x3a\x3a\x23\x20\x20\x2b\x23\x2b\x2b\x3a\x2b\x2b\x23\x2b\x2b\x20\x20\x20\x20\x20\x20\x20\x20\x2b\x23\x2b\x20\x20\x2b\x3a\x2b\x20\x20\x20\x20\x20\x20\x2b\x23\x2b\x20\x20\x20\x20\x20");
        Self::print_welcome_title(b"\x20\x20\x2b\x23\x2b\x20\x20\x2b\x23\x2b\x20\x20\x20\x2b\x23\x2b\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x2b\x23\x2b\x20\x20\x20\x20\x20\x20\x20\x2b\x23\x2b\x23\x2b\x23\x2b\x23\x2b\x23\x2b\x20\x20\x2b\x23\x2b\x20\x20\x20\x20\x20\x20\x20\x20");
        Self::print_welcome_title(b"\x20\x23\x2b\x23\x20\x20\x20\x23\x2b\x23\x20\x20\x23\x2b\x23\x20\x20\x20\x20\x20\x20\x20\x23\x2b\x23\x20\x20\x20\x20\x23\x2b\x23\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x23\x2b\x23\x20\x20\x20\x23\x2b\x23\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20");
        Self::print_welcome_title(b"\x23\x23\x23\x20\x20\x20\x20\x23\x23\x23\x20\x23\x23\x23\x20\x20\x20\x20\x20\x20\x20\x20\x23\x23\x23\x23\x23\x23\x23\x23\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x23\x23\x23\x20\x20\x23\x23\x23\x23\x23\x23\x23\x23\x23\x23\x20\x20\x20\x20\x20");
        Self::print_welcome_line(b'\xc8', b'\xbb', b' ', b'\xc9', b'\xbc');
        Self::print_welcome_line(b' ', b'\xc8', b'\xcd', b'\xbc', b' ');
        WRITER.lock().reset_foreground_color();
    }

    fn delete_char(&mut self, screen_idx: usize, decrement_pos: bool) {
        let command = &mut self.commands[screen_idx];
        command.len -= 1;
        if decrement_pos {
            command.pos -= 1;
        }
        for i in command.pos..command.len {
            command.buffer[i] = command.buffer[i + 1];
        }
        WRITER.lock().set_cursor(PROMPT.len() + command.pos);
        for i in command.pos..command.len {
            WRITER.lock().write_byte(command.buffer[i]);
        }
        WRITER.lock().write_byte(b' ');
        WRITER.lock().set_cursor(PROMPT.len() + command.pos);
    }

    fn execute_command(&self) {
        // TODO: split args (right now it's just a single command)
        let command_buffer = self.commands[self.screen_idx].trimmed();
        if command_buffer.is_empty() {
            return;
        }
        for handler in COMMAND_HANDLERS.iter() {
            if handler.name == command_buffer {
                (handler.handler)(&self);
                return;
            }
        }
        println!(
            "kfs: command not found: \"{}\"",
            core::str::from_utf8(command_buffer).unwrap_or("invalid utf-8")
        );
    }
}

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell {
        screen_idx: 0,
        commands: [
            CommandBuffer::new(Color::Pink),
            CommandBuffer::new(Color::LightCyan),
            CommandBuffer::new(Color::LightRed),
            CommandBuffer::new(Color::LightGreen),
        ],
    });
}

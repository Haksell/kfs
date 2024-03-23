use super::super::{DecodedKey, HandleControl, KeyCode, KeyboardLayout, Modifiers};

pub struct Us104Key;

impl KeyboardLayout for Us104Key {
    fn map_keycode(
        &self,
        keycode: KeyCode,
        modifiers: &Modifiers,
        handle_ctrl: HandleControl,
    ) -> DecodedKey {
        match keycode {
            KeyCode::Oem8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('~')
                } else {
                    DecodedKey::Unicode('`')
                }
            }
            KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
            KeyCode::Key1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('!')
                } else {
                    DecodedKey::Unicode('1')
                }
            }
            KeyCode::Key2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('@')
                } else {
                    DecodedKey::Unicode('2')
                }
            }
            KeyCode::Key3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('#')
                } else {
                    DecodedKey::Unicode('3')
                }
            }
            KeyCode::Key4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('$')
                } else {
                    DecodedKey::Unicode('4')
                }
            }
            KeyCode::Key5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('%')
                } else {
                    DecodedKey::Unicode('5')
                }
            }
            KeyCode::Key6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('^')
                } else {
                    DecodedKey::Unicode('6')
                }
            }
            KeyCode::Key7 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('&')
                } else {
                    DecodedKey::Unicode('7')
                }
            }
            KeyCode::Key8 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('*')
                } else {
                    DecodedKey::Unicode('8')
                }
            }
            KeyCode::Key9 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('(')
                } else {
                    DecodedKey::Unicode('9')
                }
            }
            KeyCode::Key0 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(')')
                } else {
                    DecodedKey::Unicode('0')
                }
            }
            KeyCode::OemMinus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('_')
                } else {
                    DecodedKey::Unicode('-')
                }
            }
            KeyCode::OemPlus => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('+')
                } else {
                    DecodedKey::Unicode('=')
                }
            }
            KeyCode::Backspace => DecodedKey::Unicode(0x08.into()),
            KeyCode::Tab => DecodedKey::Unicode(0x09.into()),
            KeyCode::Q => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('Q')
                } else {
                    DecodedKey::Unicode('q')
                }
            }
            KeyCode::W => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('W')
                } else {
                    DecodedKey::Unicode('w')
                }
            }
            KeyCode::E => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('E')
                } else {
                    DecodedKey::Unicode('e')
                }
            }
            KeyCode::R => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('R')
                } else {
                    DecodedKey::Unicode('r')
                }
            }
            KeyCode::T => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('T')
                } else {
                    DecodedKey::Unicode('t')
                }
            }
            KeyCode::Y => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('Y')
                } else {
                    DecodedKey::Unicode('y')
                }
            }
            KeyCode::U => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('U')
                } else {
                    DecodedKey::Unicode('u')
                }
            }
            KeyCode::I => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('I')
                } else {
                    DecodedKey::Unicode('i')
                }
            }
            KeyCode::O => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('O')
                } else {
                    DecodedKey::Unicode('o')
                }
            }
            KeyCode::P => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('P')
                } else {
                    DecodedKey::Unicode('p')
                }
            }
            KeyCode::Oem4 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('{')
                } else {
                    DecodedKey::Unicode('[')
                }
            }
            KeyCode::Oem6 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('}')
                } else {
                    DecodedKey::Unicode(']')
                }
            }
            KeyCode::Oem5 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('|')
                } else {
                    DecodedKey::Unicode('\\')
                }
            }
            KeyCode::A => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('A')
                } else {
                    DecodedKey::Unicode('a')
                }
            }
            KeyCode::S => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('S')
                } else {
                    DecodedKey::Unicode('s')
                }
            }
            KeyCode::D => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('D')
                } else {
                    DecodedKey::Unicode('d')
                }
            }
            KeyCode::F => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('F')
                } else {
                    DecodedKey::Unicode('f')
                }
            }
            KeyCode::G => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('G')
                } else {
                    DecodedKey::Unicode('g')
                }
            }
            KeyCode::H => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('H')
                } else {
                    DecodedKey::Unicode('h')
                }
            }
            KeyCode::J => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('J')
                } else {
                    DecodedKey::Unicode('j')
                }
            }
            KeyCode::K => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('K')
                } else {
                    DecodedKey::Unicode('k')
                }
            }
            KeyCode::L => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('L')
                } else {
                    DecodedKey::Unicode('l')
                }
            }
            KeyCode::Oem1 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode(':')
                } else {
                    DecodedKey::Unicode(';')
                }
            }
            KeyCode::Oem3 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('"')
                } else {
                    DecodedKey::Unicode('\'')
                }
            }
            // Enter gives LF, not CRLF or CR
            KeyCode::Return => DecodedKey::Unicode(10.into()),
            KeyCode::Z => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('Z')
                } else {
                    DecodedKey::Unicode('z')
                }
            }
            KeyCode::X => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('X')
                } else {
                    DecodedKey::Unicode('x')
                }
            }
            KeyCode::C => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('C')
                } else {
                    DecodedKey::Unicode('c')
                }
            }
            KeyCode::V => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('V')
                } else {
                    DecodedKey::Unicode('v')
                }
            }
            KeyCode::B => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('B')
                } else {
                    DecodedKey::Unicode('b')
                }
            }
            KeyCode::N => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('N')
                } else {
                    DecodedKey::Unicode('n')
                }
            }
            KeyCode::M => {
                if modifiers.is_caps() {
                    DecodedKey::Unicode('M')
                } else {
                    DecodedKey::Unicode('m')
                }
            }
            KeyCode::OemComma => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('<')
                } else {
                    DecodedKey::Unicode(',')
                }
            }
            KeyCode::OemPeriod => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('>')
                } else {
                    DecodedKey::Unicode('.')
                }
            }
            KeyCode::Oem2 => {
                if modifiers.is_shifted() {
                    DecodedKey::Unicode('?')
                } else {
                    DecodedKey::Unicode('/')
                }
            }
            KeyCode::Spacebar => DecodedKey::Unicode(' '),
            KeyCode::Delete => DecodedKey::Unicode(127.into()),
            KeyCode::NumpadDivide => DecodedKey::Unicode('/'),
            KeyCode::NumpadMultiply => DecodedKey::Unicode('*'),
            KeyCode::NumpadSubtract => DecodedKey::Unicode('-'),
            KeyCode::Numpad7 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('7')
                } else {
                    DecodedKey::RawKey(KeyCode::Home)
                }
            }
            KeyCode::Numpad8 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('8')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowUp)
                }
            }
            KeyCode::Numpad9 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('9')
                } else {
                    DecodedKey::RawKey(KeyCode::PageUp)
                }
            }
            KeyCode::NumpadAdd => DecodedKey::Unicode('+'),
            KeyCode::Numpad4 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('4')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowLeft)
                }
            }
            KeyCode::Numpad5 => DecodedKey::Unicode('5'),
            KeyCode::Numpad6 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('6')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowRight)
                }
            }
            KeyCode::Numpad1 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('1')
                } else {
                    DecodedKey::RawKey(KeyCode::End)
                }
            }
            KeyCode::Numpad2 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('2')
                } else {
                    DecodedKey::RawKey(KeyCode::ArrowDown)
                }
            }
            KeyCode::Numpad3 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('3')
                } else {
                    DecodedKey::RawKey(KeyCode::PageDown)
                }
            }
            KeyCode::Numpad0 => {
                if modifiers.numlock {
                    DecodedKey::Unicode('0')
                } else {
                    DecodedKey::RawKey(KeyCode::Insert)
                }
            }
            KeyCode::NumpadPeriod => {
                if modifiers.numlock {
                    DecodedKey::Unicode('.')
                } else {
                    DecodedKey::Unicode(127.into())
                }
            }
            KeyCode::NumpadEnter => DecodedKey::Unicode(10.into()),
            k => DecodedKey::RawKey(k),
        }
    }
}

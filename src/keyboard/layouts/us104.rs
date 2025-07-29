use super::{
    super::{DecodedKey, KeyCode, Modifiers},
    KeyboardLayout,
};

pub struct Us104Key;

const KEY_SHIFTS: [char; 10] = [')', '!', '@', '#', '$', '%', '^', '&', '*', '('];
const NUMPAD_SHIFTS: [KeyCode; 10] = [
    KeyCode::Insert,
    KeyCode::End,
    KeyCode::ArrowDown,
    KeyCode::PageDown,
    KeyCode::ArrowLeft,
    KeyCode::Numpad5,
    KeyCode::ArrowRight,
    KeyCode::Home,
    KeyCode::ArrowUp,
    KeyCode::PageUp,
];

impl KeyboardLayout for Us104Key {
    fn map_keycode(&self, keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey {
        match keycode {
            KeyCode::Escape => DecodedKey::Unicode(0x1B.into()),
            k if (KeyCode::A..=KeyCode::Z).contains(&k) => {
                DecodedKey::Unicode((k as u8 | if modifiers.is_caps() { 64 } else { 96 }).into())
            }
            k if (KeyCode::Key0..=KeyCode::Key9).contains(&k) => {
                let num = k as u8 - KeyCode::Key0 as u8;
                DecodedKey::Unicode(if modifiers.is_shifted() {
                    KEY_SHIFTS[num as usize]
                } else {
                    (num | 48).into()
                })
            }
            k if (KeyCode::Numpad0..=KeyCode::Numpad9).contains(&k) => {
                let num = k as u8 - KeyCode::Numpad0 as u8;
                if modifiers.numlock {
                    DecodedKey::Unicode((num | 48).into())
                } else {
                    DecodedKey::RawKey(NUMPAD_SHIFTS[num as usize])
                }
            }
            KeyCode::OemOpen => DecodedKey::Unicode(if modifiers.is_shifted() { '{' } else { '[' }),
            KeyCode::OemClose => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '}' } else { ']' })
            }
            KeyCode::OemPipe => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '|' } else { '\\' })
            }
            KeyCode::OemColon => {
                DecodedKey::Unicode(if modifiers.is_shifted() { ':' } else { ';' })
            }
            KeyCode::OemQuote => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '"' } else { '\'' })
            }
            KeyCode::OemComma => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '<' } else { ',' })
            }
            KeyCode::OemPeriod => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '>' } else { '.' })
            }
            KeyCode::OemQuestion => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '?' } else { '/' })
            }
            KeyCode::OemTilde => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '~' } else { '`' })
            }
            KeyCode::OemMinus => {
                DecodedKey::Unicode(if modifiers.is_shifted() { '_' } else { '-' })
            }
            KeyCode::OemPlus => DecodedKey::Unicode(if modifiers.is_shifted() { '+' } else { '=' }),
            KeyCode::NumpadPeriod => {
                DecodedKey::Unicode(if modifiers.numlock { '.' } else { 127.into() })
            }
            KeyCode::NumpadDivide => DecodedKey::Unicode('/'),
            KeyCode::NumpadMultiply => DecodedKey::Unicode('*'),
            KeyCode::NumpadSubtract => DecodedKey::Unicode('-'),
            KeyCode::NumpadAdd => DecodedKey::Unicode('+'),
            KeyCode::Backspace => DecodedKey::Unicode(8.into()),
            KeyCode::Tab => DecodedKey::Unicode('\t'),
            KeyCode::Spacebar => DecodedKey::Unicode(' '),
            KeyCode::Delete => DecodedKey::Unicode(127.into()),
            KeyCode::Enter | KeyCode::NumpadEnter => DecodedKey::Unicode('\n'),
            k => DecodedKey::RawKey(k),
        }
    }
}

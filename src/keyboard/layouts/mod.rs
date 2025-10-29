pub mod us104;

use super::{DecodedKey, KeyCode, Modifiers};

pub trait KeyboardLayout {
    fn map_keycode(&self, keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey;
}

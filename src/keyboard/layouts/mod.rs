mod us104;

pub use self::us104::Us104Key;

use super::{DecodedKey, KeyCode, Modifiers};

pub trait KeyboardLayout {
    fn map_keycode(&self, keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey;
}

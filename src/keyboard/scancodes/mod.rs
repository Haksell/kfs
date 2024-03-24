mod set1;

pub use self::set1::ScancodeSet1;

use super::{Error, KeyEvent};

const EXTENDED_KEY_CODE: u8 = 0xE0;
const EXTENDED2_KEY_CODE: u8 = 0xE1;

pub trait ScancodeSet {
    fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error>;
}

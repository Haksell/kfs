mod set1;

pub use self::set1::ScancodeSet1;

use super::{Error, KeyEvent};

pub trait ScancodeSet {
    fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error>;
}

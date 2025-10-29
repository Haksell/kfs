pub mod set1;

use super::{Error, KeyEvent};

pub trait ScancodeSet {
    fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error>;
}

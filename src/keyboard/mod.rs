pub mod layouts;
pub mod scancodes;

use {layouts::KeyboardLayout, scancodes::ScancodeSet};

#[derive(Debug)]
pub struct Keyboard<L, S>
where
    L: KeyboardLayout,
    S: ScancodeSet,
{
    layout: L,
    scancode_set: S,
    modifiers: Modifiers,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    UnknownKeyCode,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
#[repr(u8)]
pub enum KeyCode {
    Escape,
    // ======= LETTERS =======
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    // ======= TOP NUMBERS =======
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    // ======= NUMPAD NUMBERS =======
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    // ======= NUMPAD OTHERS =======
    NumpadLock,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadAdd,
    NumpadEnter,
    NumpadPeriod,
    // ======= NAVIGATION =======
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    // ======= ARROWS =======
    ArrowUp,
    ArrowRight,
    ArrowDown,
    ArrowLeft,
    // ======= CONTROL =======
    Tab,
    CapsLock,
    LeftShift,
    Spacebar,
    Backspace,
    Enter,
    RightShift,
    // ======= FUNCTIONS KEYS =======
    F1,
    F2,
    F3,
    F4,
    // ======= DOUBLE ASCII (names subject to change) =======
    OemTilde,
    OemMinus,
    OemPlus,
    OemOpen,
    OemClose,
    OemPipe,
    OemColon,
    OemQuote,
    OemComma,
    OemPeriod,
    OemQuestion,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum KeyState {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
    code: KeyCode,
    state: KeyState,
}

impl KeyEvent {
    const fn new(code: KeyCode, state: KeyState) -> Self {
        Self { code, state }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Modifiers {
    lshift: bool,
    rshift: bool,
    numlock: bool,
    capslock: bool,
}

impl Modifiers {
    const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    const fn is_caps(&self) -> bool {
        self.is_shifted() ^ self.capslock
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DecodedKey {
    RawKey(KeyCode),
    Unicode(char),
}

impl<L, S> Keyboard<L, S>
where
    L: KeyboardLayout,
    S: ScancodeSet,
{
    pub const fn new(layout: L, scancode_set: S) -> Self {
        Self {
            layout,
            scancode_set,
            // TODO: get numlock and capslock value from PS/2 (0xED)
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                numlock: true,
                capslock: false,
            },
        }
    }

    pub fn add_byte(&mut self, byte: u8) -> Option<DecodedKey> {
        match self.scancode_set.add_byte(byte) {
            Ok(Some(key_event)) => self.process_keyevent(key_event),
            _ => None,
        }
    }

    fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        match ev.code {
            KeyCode::LeftShift => self.modifiers.lshift = ev.state == KeyState::Down,
            KeyCode::RightShift => self.modifiers.rshift = ev.state == KeyState::Down,
            KeyCode::CapsLock => {
                if ev.state == KeyState::Down {
                    self.modifiers.capslock = !self.modifiers.capslock
                }
            }
            KeyCode::NumpadLock => {
                if ev.state == KeyState::Down {
                    self.modifiers.numlock = !self.modifiers.numlock
                }
            }
            _ => {
                if ev.state == KeyState::Down {
                    return Some(self.layout.map_keycode(ev.code, &self.modifiers));
                }
            }
        }
        None
    }
}

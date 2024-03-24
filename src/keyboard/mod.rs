pub mod layouts;
pub mod scancodes;

use layouts::KeyboardLayout;
use scancodes::ScancodeSet;

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
    // Letters
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
    // ========= Row 1 (the F-keys) =========
    F1,
    F2,
    F3,
    F4,
    PrintScreen,
    SysRq,
    ScrollLock,
    // ========= Row 2 (the numbers) =========
    Oem8,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    OemMinus,
    OemPlus,
    Backspace,
    Insert,
    Home,
    PageUp,
    NumpadLock,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    // ========= Row 3 (QWERTY) =========
    Tab,
    Oem4,
    Oem6,
    Oem5,
    Oem7,
    Delete,
    End,
    PageDown,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    // ========= Row 4 (ASDF) =========
    CapsLock,
    Oem1,
    Oem3,
    Return,
    Numpad4,
    Numpad5,
    Numpad6,
    // ========= Row 5 (ZXCV) =========
    LShift,
    OemComma,
    OemPeriod,
    Oem2,
    RShift,
    ArrowUp,
    Numpad1,
    Numpad2,
    Numpad3,
    NumpadEnter,
    // ========= Row 6 (modifers and space bar) =========
    LWin,
    Spacebar,
    RWin,
    Apps,
    ArrowLeft,
    ArrowDown,
    ArrowRight,
    Numpad0,
    NumpadPeriod,
    // ========= JIS 109-key extra keys =========
    Oem9,
    Oem10,
    Oem11,
    Oem12,
    Oem13,
    // ========= Extra Keys ========= (TODO: remove for now)
    PrevTrack,
    NextTrack,
    Mute,
    Calculator,
    Play,
    Stop,
    VolumeDown,
    VolumeUp,
    WWWHome,
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
    const fn new(code: KeyCode, state: KeyState) -> KeyEvent {
        KeyEvent { code, state }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Modifiers {
    pub lshift: bool,
    pub rshift: bool,
    pub numlock: bool,
    pub capslock: bool,
}

impl Modifiers {
    pub const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    pub const fn is_caps(&self) -> bool {
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
    pub const fn new(layout: L, scancode_set: S) -> Keyboard<L, S> {
        Keyboard {
            layout,
            scancode_set,
            // TODO: check if there is a way to get accurate modifiers state at the start
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
            KeyCode::LShift => self.modifiers.lshift = ev.state == KeyState::Down,
            KeyCode::RShift => self.modifiers.rshift = ev.state == KeyState::Down,
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

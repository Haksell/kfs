pub mod layouts;
pub mod scancodes;

#[derive(Debug)]
pub struct Keyboard<L, S>
where
    S: ScancodeSet,
    L: KeyboardLayout,
{
    scancode_set: S,
    layout: L,
    modifiers: Modifiers,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    UnknownKeyCode,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[repr(u8)]
pub enum KeyCode {
    // ========= Row 1 (the F-keys) =========
    Escape,
    F1,
    F2,
    F3,
    F4,
    PrintScreen,
    SysRq,
    ScrollLock,
    PauseBreak,
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
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
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
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Oem1,
    Oem3,
    Return,
    Numpad4,
    Numpad5,
    Numpad6,
    // ========= Row 5 (ZXCV) =========
    LShift,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
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
    LControl,
    LWin,
    LAlt,
    Spacebar,
    RAltGr,
    RWin,
    Apps,
    RControl,
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
    RControl2,
    RAlt2,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub state: KeyState,
}

// TODO: put in subfolder without pub
pub trait KeyboardLayout {
    fn map_keycode(&self, keycode: KeyCode, modifiers: &Modifiers) -> DecodedKey;
}

// TODO: put in subfolder without pub
pub trait ScancodeSet {
    fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error>;
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Modifiers {
    pub lshift: bool,
    pub rshift: bool,
    pub lctrl: bool,
    pub rctrl: bool,
    pub numlock: bool,
    pub capslock: bool,
    pub lalt: bool,
    pub ralt: bool,
    pub rctrl2: bool,
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
    pub const fn new(scancode_set: S, layout: L) -> Keyboard<L, S> {
        Keyboard {
            scancode_set,
            layout,
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                lctrl: false,
                rctrl: false,
                numlock: true,
                capslock: false,
                lalt: false,
                ralt: false,
                rctrl2: false,
            },
        }
    }

    pub fn add_byte(&mut self, byte: u8) -> Result<Option<KeyEvent>, Error> {
        self.scancode_set.add_byte(byte)
    }

    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        match ev {
            KeyEvent {
                code: KeyCode::LShift,
                state: KeyState::Down,
            } => {
                self.modifiers.lshift = true;
                Some(DecodedKey::RawKey(KeyCode::LShift))
            }
            KeyEvent {
                code: KeyCode::RShift,
                state: KeyState::Down,
            } => {
                self.modifiers.rshift = true;
                Some(DecodedKey::RawKey(KeyCode::RShift))
            }
            KeyEvent {
                code: KeyCode::LShift,
                state: KeyState::Up,
            } => {
                self.modifiers.lshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::RShift,
                state: KeyState::Up,
            } => {
                self.modifiers.rshift = false;
                None
            }
            KeyEvent {
                code: KeyCode::CapsLock,
                state: KeyState::Down,
            } => {
                self.modifiers.capslock = !self.modifiers.capslock;
                Some(DecodedKey::RawKey(KeyCode::CapsLock))
            }
            KeyEvent {
                code: KeyCode::NumpadLock,
                state: KeyState::Down,
            } => {
                if self.modifiers.rctrl2 {
                    Some(DecodedKey::RawKey(KeyCode::PauseBreak))
                } else {
                    self.modifiers.numlock = !self.modifiers.numlock;
                    Some(DecodedKey::RawKey(KeyCode::NumpadLock))
                }
            }
            KeyEvent {
                code: KeyCode::LControl,
                state: KeyState::Down,
            } => {
                self.modifiers.lctrl = true;
                Some(DecodedKey::RawKey(KeyCode::LControl))
            }
            KeyEvent {
                code: KeyCode::LControl,
                state: KeyState::Up,
            } => {
                self.modifiers.lctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::RControl,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl = true;
                Some(DecodedKey::RawKey(KeyCode::RControl))
            }
            KeyEvent {
                code: KeyCode::RControl,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl = false;
                None
            }
            KeyEvent {
                code: KeyCode::LAlt,
                state: KeyState::Down,
            } => {
                self.modifiers.lalt = true;
                Some(DecodedKey::RawKey(KeyCode::LAlt))
            }
            KeyEvent {
                code: KeyCode::LAlt,
                state: KeyState::Up,
            } => {
                self.modifiers.lalt = false;
                None
            }
            KeyEvent {
                code: KeyCode::RAltGr,
                state: KeyState::Down,
            } => {
                self.modifiers.ralt = true;
                Some(DecodedKey::RawKey(KeyCode::RAltGr))
            }
            KeyEvent {
                code: KeyCode::RAltGr,
                state: KeyState::Up,
            } => {
                self.modifiers.ralt = false;
                None
            }
            KeyEvent {
                code: KeyCode::RControl2,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl2 = true;
                Some(DecodedKey::RawKey(KeyCode::RControl2))
            }
            KeyEvent {
                code: KeyCode::RControl2,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl2 = false;
                None
            }
            KeyEvent {
                code: c,
                state: KeyState::Down,
            } => Some(self.layout.map_keycode(c, &self.modifiers)),
            _ => None,
        }
    }
}

impl KeyEvent {
    pub const fn new(code: KeyCode, state: KeyState) -> KeyEvent {
        KeyEvent { code, state }
    }
}

impl Modifiers {
    pub const fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    pub const fn is_caps(&self) -> bool {
        self.is_shifted() ^ self.capslock
    }
}

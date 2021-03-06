use darksiders1_sys::target;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub trait InputEventExt {
    fn data_ptr(&self) -> *const ();
}

impl InputEventExt for target::keen__InputEvent {
    fn data_ptr(&self) -> *const () {
        // Work around pdbindgen unsupported union
        unsafe { (self as *const Self).cast::<u8>().offset(4).cast::<()>() }
    }
}

#[repr(u8)]
#[derive(Eq, PartialEq, TryFromPrimitive)]
pub enum ControllerClass {
    None = 0x0,
    Keyboard = 0x1,
    Mouse = 0x2,
    Controller = 0x3,
    Touch = 0x4,
    Count = 0x5,
}

#[repr(u8)]
#[derive(Eq, PartialEq, TryFromPrimitive)]
pub enum InputEventType {
    GameButtonDown = 0x0,
    GameButtonUp = 0x1,
    GameAxis = 0x2,
    GameAxis2d = 0x3,
    ControllerConnected = 0x4,
    ControllerDisconnected = 0x5,
    ControllerTypeChanged = 0x6,
    RawButtonDown = 0x7,
    RawButtonUp = 0x8,
    RawAxis = 0x9,
    Key = 0xA,
    MouseMove = 0xB,
    MouseRelativeMovement = 0xC,
    MouseWheel = 0xD,
    FocusLost = 0xE,
    FocusRestored = 0xF,
    TouchBegin = 0x10,
    TouchEnd = 0x11,
    TouchCancel = 0x12,
    TouchMove = 0x13,
    VirtualKeyboardOpened = 0x14,
    VirtualKeyboardText = 0x15,
    VirtualKeyboardClosed = 0x16,
    Flick = 0x17,
    Drag = 0x18,
    DragBegin = 0x19,
    DragEnd = 0x1A,
    Count = 0x1B,
}

#[repr(u32)]
#[derive(Eq, PartialEq, IntoPrimitive)]
pub enum KeyboardButtonId {
    InputId_Unknown = 0xFFFF,
    NumLock = 0x0,
    CapsLock = 0x1,
    ScrollLock = 0x2,
    Compose = 0x3,
    Kana = 0x4,
    LeftShift = 0x5,
    LeftCtrl = 0x6,
    LeftAlt = 0x7,
    LeftWindows = 0x8,
    RightShift = 0x9,
    RightCtrl = 0xA,
    RightAlt = 0xB,
    RightWindows = 0xC,
    Break = 0xD,
    Backspace = 0xE,
    Tab = 0xF,
    Return = 0x10,
    Pause = 0x11,
    Escape = 0x12,
    Space = 0x13,
    PageUp = 0x14,
    PageDown = 0x15,
    End = 0x16,
    Home = 0x17,
    ArrowLeft = 0x18,
    ArrowUp = 0x19,
    ArrowRight = 0x1A,
    ArrowDown = 0x1B,
    Print = 0x1C,
    Insert = 0x1D,
    Delete = 0x1E,
    _0 = 0x1F,
    _1 = 0x20,
    _2 = 0x21,
    _3 = 0x22,
    _4 = 0x23,
    _5 = 0x24,
    _6 = 0x25,
    _7 = 0x26,
    _8 = 0x27,
    _9 = 0x28,
    A = 0x29,
    B = 0x2A,
    C = 0x2B,
    D = 0x2C,
    E = 0x2D,
    F = 0x2E,
    G = 0x2F,
    H = 0x30,
    I = 0x31,
    J = 0x32,
    K = 0x33,
    L = 0x34,
    M = 0x35,
    N = 0x36,
    O = 0x37,
    P = 0x38,
    Q = 0x39,
    R = 0x3A,
    S = 0x3B,
    T = 0x3C,
    U = 0x3D,
    V = 0x3E,
    W = 0x3F,
    X = 0x40,
    Y = 0x41,
    Z = 0x42,
    Numpad0 = 0x43,
    Numpad1 = 0x44,
    Numpad2 = 0x45,
    Numpad3 = 0x46,
    Numpad4 = 0x47,
    Numpad5 = 0x48,
    Numpad6 = 0x49,
    Numpad7 = 0x4A,
    Numpad8 = 0x4B,
    Numpad9 = 0x4C,
    Multiply = 0x4D,
    Add = 0x4E,
    Subtract = 0x4F,
    Divide = 0x50,
    F1 = 0x51,
    F2 = 0x52,
    F3 = 0x53,
    F4 = 0x54,
    F5 = 0x55,
    F6 = 0x56,
    F7 = 0x57,
    F8 = 0x58,
    F9 = 0x59,
    F10 = 0x5A,
    F11 = 0x5B,
    F12 = 0x5C,
    F13 = 0x5D,
    F14 = 0x5E,
    F15 = 0x5F,
    F16 = 0x60,
    F17 = 0x61,
    F18 = 0x62,
    F19 = 0x63,
    F20 = 0x64,
    F21 = 0x65,
    F22 = 0x66,
    F23 = 0x67,
    F24 = 0x68,
    ArrowLeftRight = 0x69,
    ArrowUpDown = 0x6A,
    AltGr = 0x6B,
    Minus = 0x6C,
    EqualPlus = 0x6D,
    LeftSquareCurlyBracket = 0x6E,
    RightSquareCurlyBracket = 0x6F,
    ColonSemicolon = 0x70,
    ApostropheDoubleQuote = 0x71,
    AccentTilde = 0x72,
    BackslashVerticalLine = 0x73,
    CommaLessThan = 0x74,
    PeriodGreaterThan = 0x75,
    SlashQuestionMark = 0x76,
    BackslashVerticalLine2 = 0x77,
    ControllerKeyboard_ButtonCount = 0x78,
}

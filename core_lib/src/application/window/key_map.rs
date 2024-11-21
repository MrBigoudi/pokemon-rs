use log::warn;

use winit::{event::ElementState, keyboard::KeyCode};

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum KeyState {
    Pressed,
    #[default]
    Released,
}

impl KeyState {
    pub fn from_winit(state: ElementState) -> Self {
        match state {
            ElementState::Pressed => Self::Pressed,
            ElementState::Released => Self::Released,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Key {
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
    Left,
    Right,
    Up,
    Down,
    Enter,
    Delete,
    Escape,
    Shift,
    Super,
}

impl Key {
    pub fn from_winit(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::KeyA => Some(Key::A),
            KeyCode::KeyB => Some(Key::B),
            KeyCode::KeyC => Some(Key::C),
            KeyCode::KeyD => Some(Key::D),
            KeyCode::KeyE => Some(Key::E),
            KeyCode::KeyF => Some(Key::F),
            KeyCode::KeyG => Some(Key::G),
            KeyCode::KeyH => Some(Key::H),
            KeyCode::KeyI => Some(Key::I),
            KeyCode::KeyJ => Some(Key::J),
            KeyCode::KeyK => Some(Key::K),
            KeyCode::KeyL => Some(Key::L),
            KeyCode::KeyM => Some(Key::M),
            KeyCode::KeyN => Some(Key::N),
            KeyCode::KeyO => Some(Key::O),
            KeyCode::KeyP => Some(Key::P),
            KeyCode::KeyQ => Some(Key::Q),
            KeyCode::KeyR => Some(Key::R),
            KeyCode::KeyS => Some(Key::S),
            KeyCode::KeyT => Some(Key::T),
            KeyCode::KeyU => Some(Key::U),
            KeyCode::KeyV => Some(Key::V),
            KeyCode::KeyW => Some(Key::W),
            KeyCode::KeyX => Some(Key::X),
            KeyCode::KeyY => Some(Key::Y),
            KeyCode::KeyZ => Some(Key::Z),
            KeyCode::ArrowLeft => Some(Key::Left),
            KeyCode::ArrowRight => Some(Key::Right),
            KeyCode::ArrowUp => Some(Key::Up),
            KeyCode::ArrowDown => Some(Key::Down),
            KeyCode::Enter => Some(Key::Enter),
            KeyCode::Delete => Some(Key::Delete),
            KeyCode::Escape => Some(Key::Escape),
            KeyCode::ShiftLeft => Some(Key::Shift),
            KeyCode::SuperLeft => Some(Key::Super),
            _ => {
                warn! {"Winit Key: {:?} is not yet supported...", key_code};
                None
            }
        }
    }
}

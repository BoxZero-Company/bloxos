
#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    KeyDown { key: u32 },
    KeyUp { key: u32 },
    MouseMove { x: i32, y: i32 },
    MouseButtonDown { button: u8 },
    MouseButtonUp { button: u8 },
}

pub struct InputManager {
    initialized: bool,
}

impl InputManager {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub const fn is_initialized(&self) -> bool {
        self.initialized
    }
}

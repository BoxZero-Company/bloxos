pub struct ServiceManager {
    initialized: bool,
}

impl ServiceManager {
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
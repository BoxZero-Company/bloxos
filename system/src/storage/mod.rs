pub struct StorageManager {
    initialized: bool,
}

impl StorageManager {
    pub const fn new() -> Self {
        Self { initialized: false }
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }
}
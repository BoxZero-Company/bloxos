pub struct NetworkManager {
    enabled: bool,
}

impl NetworkManager {
    pub const fn new() -> Self {
        Self { enabled: false }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }
}
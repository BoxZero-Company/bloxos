
pub struct ProgressBar {
    pub value: u8,
}

impl ProgressBar {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn set(&mut self, value: u8) {
        self.value = value.min(100);
    }

    pub fn filled_width(&self, width: usize) -> usize {
        width * self.value as usize / 100
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Metadata {
    pub size: usize,
    pub created_at: u64,
    pub modified_at: u64,
    pub is_directory: bool,
}

impl Metadata {
    pub const fn new(is_directory: bool) -> Self {
        Self {
            size: 0,
            created_at: 0,
            modified_at: 0,
            is_directory,
        }
    }

    pub fn update_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn touch(&mut self, timestamp: u64) {
        if self.created_at == 0 {
            self.created_at = timestamp;
        }

        self.modified_at = timestamp;
    }
}

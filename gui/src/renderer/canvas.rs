
use super::framebuffer::Framebuffer;

pub struct Canvas<'a> {
    pub fb: Framebuffer<'a>,
}

impl<'a> Canvas<'a> {
    pub fn new(fb: Framebuffer<'a>) -> Self {
        Self { fb }
    }

    pub fn background(&mut self) {
        self.fb.clear(0xFF08090D);
    }

    pub fn panel(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        self.fb.rectangle(
            x, y, width, height, 0xFF17191F,
        );
    }

    pub fn accent(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) {
        self.fb.rectangle(
            x, y, width, height, 0xFFB7FF36,
        );
    }

    pub fn taskbar(&mut self) {
        let height = 48;

        self.fb.rectangle(
            0,
            self.fb.height.saturating_sub(height),
            self.fb.width,
            height,
            0xFF17191F,
        );
    }
}

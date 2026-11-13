
use crate::gui_rect::Rect;

pub struct Button {
    pub bounds: Rect,
    pub text: &'static str,
    pub active: bool,
}

impl Button {
    pub fn new(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        text: &'static str,
    ) -> Self {
        Self {
            bounds: Rect {
                x,
                y,
                width,
                height,
            },
            text,
            active: false,
        }
    }

    pub fn click(&mut self, x: usize, y: usize) -> bool {
        if self.bounds.contains(x, y) {
            self.active = true;
            true
        } else {
            false
        }
    }
}


#![no_std]

pub mod renderer;
pub mod widgets;
pub mod themes;
pub mod input;
pub mod gui_rect;
pub mod desktop;
pub mod apps;

pub struct GuiEngine {
    pub width: usize,
    pub height: usize,
    pub background: u32,
}

impl GuiEngine {
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            background: 0xFF08090D,
        }
    }

    pub fn render_background(&self, framebuffer: &mut [u32]) {
        framebuffer.fill(self.background);
    }
}

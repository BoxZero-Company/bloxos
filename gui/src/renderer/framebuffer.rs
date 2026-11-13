
#[derive(Clone, Copy)]
pub struct PixelFormat {
    pub red_position: u8,
    pub red_size: u8,
    pub green_position: u8,
    pub green_size: u8,
    pub blue_position: u8,
    pub blue_size: u8,
}

impl PixelFormat {
    pub const RGB888: Self = Self {
        red_position: 16,
        red_size: 8,
        green_position: 8,
        green_size: 8,
        blue_position: 0,
        blue_size: 8,
    };

    fn valid(self) -> bool {
        let channels = [
            (self.red_position, self.red_size),
            (self.green_position, self.green_size),
            (self.blue_position, self.blue_size),
        ];

        for (position, size) in channels {
            if size == 0 || size > 8 {
                return false;
            }

            if position as u16 + size as u16 > 32 {
                return false;
            }
        }

        true
    }
}

pub struct Framebuffer<'a> {
    pub buffer: &'a mut [u32],
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub format: PixelFormat,
}

impl<'a> Framebuffer<'a> {
    pub fn new(
        buffer: &'a mut [u32],
        width: usize,
        height: usize,
        pitch: usize,
    ) -> Self {
        Self::new_with_format(
            buffer,
            width,
            height,
            pitch,
            PixelFormat::RGB888,
        )
    }

    pub fn new_with_format(
        buffer: &'a mut [u32],
        width: usize,
        height: usize,
        pitch: usize,
        format: PixelFormat,
    ) -> Self {
        Self {
            buffer,
            width,
            height,
            pitch,
            format,
        }
    }

    fn scale_channel(value: u8, size: u8) -> u32 {
        if size == 0 || size > 8 {
            return 0;
        }

        let max = (1u32 << size) - 1;
        (value as u32 * max + 127) / 255
    }

    fn encode_color(&self, color: u32) -> u32 {
        if !self.format.valid() {
            return color;
        }

        let red = ((color >> 16) & 0xFF) as u8;
        let green = ((color >> 8) & 0xFF) as u8;
        let blue = (color & 0xFF) as u8;

        let f = self.format;

        (Self::scale_channel(red, f.red_size) << f.red_position)
            | (Self::scale_channel(green, f.green_size) << f.green_position)
            | (Self::scale_channel(blue, f.blue_size) << f.blue_position)
    }

    pub fn clear(&mut self, color: u32) {
        let pixel = self.encode_color(color);
        let required = self.pitch.saturating_mul(self.height);
        let length = required.min(self.buffer.len());

        self.buffer[..length].fill(pixel);
    }

    pub fn pixel(
        &mut self,
        x: usize,
        y: usize,
        color: u32,
    ) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = y
            .saturating_mul(self.pitch)
            .saturating_add(x);

        let pixel = self.encode_color(color);

        if let Some(target) = self.buffer.get_mut(index) {
            *target = pixel;
        }
    }

    pub fn rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        let start_x = x.min(self.width);
        let start_y = y.min(self.height);

        let end_x = x.saturating_add(width).min(self.width);
        let end_y = y.saturating_add(height).min(self.height);

        let pixel = self.encode_color(color);

        for py in start_y..end_y {
            let row = py.saturating_mul(self.pitch);

            for px in start_x..end_x {
                let index = row.saturating_add(px);

                if let Some(target) = self.buffer.get_mut(index) {
                    *target = pixel;
                }
            }
        }
    }

    pub fn horizontal_line(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        color: u32,
    ) {
        self.rectangle(x, y, width, 1, color);
    }

    pub fn vertical_line(
        &mut self,
        x: usize,
        y: usize,
        height: usize,
        color: u32,
    ) {
        self.rectangle(x, y, 1, height, color);
    }
}

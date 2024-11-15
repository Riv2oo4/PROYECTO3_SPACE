use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0x000000); // Negro
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, radius: i32, color: Color) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 1 - radius;

        while x <= y {
            for dx in [-x, x].iter() {
                for dy in [-y, y].iter() {
                    self.set_pixel(cx + dx, cy + dy, color.to_hex());
                    self.set_pixel(cx + dy, cy + dx, color.to_hex());
                }
            }

            x += 1;
            if d < 0 {
                d += 2 * x + 1;
            } else {
                y -= 1;
                d += 2 * (x - y) + 1;
            }
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = y as usize * self.width + x as usize;
            self.buffer[index] = color;
        }
    }
}

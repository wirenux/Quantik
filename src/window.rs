use minifb::{Key, Window, WindowOptions};

pub struct AppWindow {
    inner: Window,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}


impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::BLACK; width * height],
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.pixels.fill(color);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }

    pub fn get_pixels(&self, x: usize, y: usize) -> Color {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x]
        } else {
            Color::BLACK
        }
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.pixels.iter().map(|c| c.to_u32()).collect()
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if self.width != new_width || self.height != new_height {
            self.width = new_width;
            self.height = new_height;
            self.pixels = vec![Color::BLACK; new_width * new_height];
        }
    }
}

impl AppWindow {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let mut window = Window::new(
            title,
            width,
            height,
            WindowOptions {
                resize: true,
                ..WindowOptions::default()
            },
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(120);
        Self { inner: window }
    }

    pub fn active(&self) -> bool {
        self.inner.is_open() && !self.inner.is_key_down(Key::Escape)
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.inner.get_size()
    }

    pub fn update_with_buffer(&mut self, buffer: &[u32], width: usize, height: usize) {
        self.inner.update_with_buffer(buffer, width, height).unwrap();
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
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
}

fn main() {
    let mut framebuffer = Framebuffer::new(10, 10);
    Framebuffer::set_pixel(&mut framebuffer, 1, 1, Color::WHITE);
    println!("{:?}", framebuffer.pixels);
    println!("{:?}", Framebuffer::get_pixels(&framebuffer, 1, 1));
}

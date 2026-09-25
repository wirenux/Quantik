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

    pub fn draw_line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, color: Color) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < self.width as i32 && y0 >= 0 && y0 < self.height as i32 {
                self.set_pixel(x0 as usize, y0 as usize, color);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
        (c.0 - a.0) * (b.1 - a.1) - (c.1 - a.1) * (b.0 - a.0)
    }

    pub fn draw_triangle(&mut self, p0: (usize, usize), p1: (usize, usize), p2: (usize, usize), color: Color) {
        let min_x = p0.0.min(p1.0).min(p2.0);
        let max_x = p0.0.max(p1.0).max(p2.0).min(self.width - 1);

        let min_y = p0.1.min(p1.1).min(p2.1);
        let max_y = p0.1.max(p1.1).max(p2.1).min(self.height - 1);

        let v0 = (p0.0 as f32, p0.1 as f32);
        let v1 = (p1.0 as f32, p1.1 as f32);
        let v2 = (p2.0 as f32, p2.1 as f32);

        // total area of the triangle
        let area = Self::edge_function(v0, v1, v2);
    
        // if triangle is falt (null area), useless to draw it
        if area.abs() < 1e-6 {
            return;
        }

        // going trought every pixel in the "binding box"
        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = (x as f32 + 0.5, y as f32 + 0.5);

                let w0 = Self::edge_function(v1, v2, p);
                let w1 = Self::edge_function(v2, v0, p);
                let w2 = Self::edge_function(v0, v1, p);

                let is_inside = if area > 0.0 {
                    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                } else {
                    w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                };

                if is_inside {
                    self.set_pixel(x, y, color);
                }
            }
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

    pub fn is_key_down(&self, key: Key) -> bool {
        self.inner.is_key_down(key)
    }
}

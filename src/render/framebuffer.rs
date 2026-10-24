use rayon::prelude::*;

use super::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
    pub depth: Vec<f32>,
    pub atom_ids: Vec<usize>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::BLACK; width * height],
            depth: vec![f32::INFINITY; width * height],
            atom_ids: vec![usize::MAX; width * height],
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.pixels.fill(color);
        self.depth.fill(f32::INFINITY);
        self.atom_ids.fill(usize::MAX);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.pixels[index] = color;
        }
    }

    pub fn set_pixel_depth(&mut self, x: usize, y: usize, color: Color, depth: f32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if depth < self.depth[index] {
                self.pixels[index] = color;
                self.depth[index] = depth;
            }
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
        self.pixels.par_iter().map(|c| c.to_u32()).collect()
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if self.width != new_width || self.height != new_height {
            self.width = new_width;
            self.height = new_height;
            self.pixels = vec![Color::BLACK; new_width * new_height];
            self.depth = vec![f32::INFINITY; new_width * new_height];
            self.atom_ids = vec![usize::MAX; new_width * new_height];
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: Color) {
        let x_end = (x + w).min(self.width);
        let y_end = (y + h).min(self.height);
        for yy in y..y_end {
            for xx in x..x_end {
                self.set_pixel(xx, yy, color);
            }
        }
    }

    pub fn resolve_into(&self, dst: &mut Framebuffer, ss: usize) {
        let dw = dst.width;
        let src_w = self.width;
        let n = (ss * ss) as u32;

        dst.pixels
            .par_chunks_mut(dw)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..dw {
                    let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
                    for sy in 0..ss {
                        let base = (y * ss + sy) * src_w + x * ss;
                        for sx in 0..ss {
                            let c = self.pixels[base + sx];
                            r += c.r as u32;
                            g += c.g as u32;
                            b += c.b as u32;
                        }
                    }
                    row[x] = Color::new((r / n) as u8, (g / n) as u8, (b / n) as u8);
                }
            });
    }
}

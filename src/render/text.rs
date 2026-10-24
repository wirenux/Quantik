use font8x8::{UnicodeFonts, BASIC_FONTS};

use super::color::Color;
use super::framebuffer::Framebuffer;

impl Framebuffer {
    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, scale: usize, color: Color) {
        let scale = scale.max(1);
        let mut cursor_x = x;

        for ch in text.chars() {
            if let Some(glyph) = BASIC_FONTS.get(ch) {
                for (row, byte) in glyph.iter().enumerate() {
                    for col in 0..8 {
                        if (byte >> col) & 1 == 1 {
                            let px = cursor_x + col * scale;
                            let py = y + row * scale;
                            for dy in 0..scale {
                                for dx in 0..scale {
                                    self.set_pixel(px + dx, py + dy, color);
                                }
                            }
                        }
                    }
                }
            }
            cursor_x += 8 * scale;
        }
    }

    pub fn text_width(text: &str) -> usize {
        text.chars().count() * 8
    }

    pub fn draw_formula(&mut self, x: usize, y: usize, text: &str, scale: usize, color: Color) {
        let scale = scale.max(1);
        let small = (scale / 2).max(1);
        let sub_offset = 8 * scale - 8 * small;
        let mut cursor_x = x;

        for ch in text.chars() {
            let is_digit = ch.is_ascii_digit();
            let s = if is_digit { small } else { scale };
            let oy = if is_digit { sub_offset } else { 0 };

            if let Some(glyph) = BASIC_FONTS.get(ch) {
                for (row, byte) in glyph.iter().enumerate() {
                    for col in 0..8 {
                        if (byte >> col) & 1 == 1 {
                            let px = cursor_x + col * s;
                            let py = y + oy + row * s;
                            for dy in 0..s {
                                for dx in 0..s {
                                    self.set_pixel(px + dx, py + dy, color);
                                }
                            }
                        }
                    }
                }
            }
            cursor_x += 8 * s;
        }
    }

    pub fn formula_width(text: &str, scale: usize) -> usize {
        let scale = scale.max(1);
        let small = (scale / 2).max(1);
        text.chars()
            .map(|ch| {
                if ch.is_ascii_digit() {
                    8 * small
                } else {
                    8 * scale
                }
            })
            .sum()
    }

    pub fn draw_text_centered(&mut self, y: usize, text: &str, scale: usize, color: Color) {
        let scale = scale.max(1);

        let w = Self::text_width(text) * scale;
        let x = self.width.saturating_sub(w) / 2;

        self.draw_text(x, y, text, scale, color);
    }

    pub fn text_height(scale: usize) -> usize {
        8 * scale.max(1)
    }
}

use font8x8::{UnicodeFonts, BASIC_FONTS};
use minifb::{
    Key, Menu, MouseButton, MouseMode, Window, WindowOptions, MENU_KEY_COMMAND, MENU_KEY_CTRL,
};
use rayon::prelude::*;

pub struct AppWindow {
    inner: Window,
    menu_handle: Option<minifb::MenuHandle>,
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
    pub depth: Vec<f32>,
    pub atom_ids: Vec<usize>,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0 };
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
    };

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

    pub fn draw_line_depth(
        &mut self,
        mut x0: i32,
        mut y0: i32,
        x1: i32,
        y1: i32,
        d0: f32,
        d1: f32,
        color: Color,
    ) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let total_steps = dx.max(-dy) as f32;
        let mut step: f32 = 0.0;

        loop {
            if x0 >= 0 && x0 < self.width as i32 && y0 >= 0 && y0 < self.height as i32 {
                let t = if total_steps > 0.0 {
                    step / total_steps
                } else {
                    0.0
                };
                let inv_w = (1.0 - t) / d0 + t / d1;
                let depth = 1.0 / inv_w;
                self.set_pixel_depth(x0 as usize, y0 as usize, color, depth);
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
            step += 1.0;
        }
    }

    fn edge_function(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
        (c.0 - a.0) * (b.1 - a.1) - (c.1 - a.1) * (b.0 - a.0)
    }

    pub fn draw_triangle(
        &mut self,
        p0: (usize, usize),
        p1: (usize, usize),
        p2: (usize, usize),
        d0: f32,
        d1: f32,
        d2: f32,
        int0: f32,
        int1: f32,
        int2: f32,
        base_color: Color,
        atom_id: usize,
    ) {
        let min_x = p0.0.min(p1.0).min(p2.0);
        let max_x = p0.0.max(p1.0).max(p2.0).min(self.width - 1);

        let min_y = p0.1.min(p1.1).min(p2.1);
        let max_y = p0.1.max(p1.1).max(p2.1).min(self.height - 1);

        // triangle inside the margin ring, so nothing to draw
        if min_x > max_x || min_y > max_y {
            return;
        }

        let v0 = (p0.0 as f32, p0.1 as f32);
        let v1 = (p1.0 as f32, p1.1 as f32);
        let v2 = (p2.0 as f32, p2.1 as f32);

        // total area of the triangle
        let area = Self::edge_function(v0, v1, v2);

        // if triangle is falt (null area), useless to draw it
        if area.abs() < 1e-6 {
            return;
        }

        let inv_area = 1.0 / area;
        let inv_d0 = 1.0 / d0;
        let inv_d1 = 1.0 / d1;
        let inv_d2 = 1.0 / d2;

        let i0d = int0 * inv_d0;
        let i1d = int1 * inv_d1;
        let i2d = int2 * inv_d2;

        let dw0_dx = v2.1 - v1.1;
        let dw0_dy = v1.0 - v2.0;

        let dw1_dx = v0.1 - v2.1;
        let dw1_dy = v2.0 - v0.0;

        let dw2_dx = v1.1 - v0.1;
        let dw2_dy = v0.0 - v1.0;

        // initial w value at pixel center
        let start = (min_x as f32 + 0.5, min_y as f32 + 0.5);
        let mut row_w0 = Self::edge_function(v1, v2, start);
        let mut row_w1 = Self::edge_function(v2, v0, start);
        let mut row_w2 = Self::edge_function(v0, v1, start);

        let dinv_w_dx = (dw0_dx * inv_d0 + dw1_dx * inv_d1 + dw2_dx * inv_d2) * inv_area;
        let di_sum_dx = (dw0_dx * i0d + dw1_dx * i1d + dw2_dx * i2d) * inv_area;

        let jump_w0 = dw0_dy;
        let jump_w1 = dw1_dy;
        let jump_w2 = dw2_dy;

        let area_positive = area > 0.0;
        let width = self.width;

        let base_r = base_color.r as f32;
        let base_g = base_color.g as f32;
        let base_b = base_color.b as f32;

        // going trought every pixel in the "binding box"
        for y in min_y..=max_y {
            let mut w0 = row_w0;
            let mut w1 = row_w1;
            let mut w2 = row_w2;

            // recompute inv_w from the row starting w values each row.
            let mut inv_w = (w0 * inv_d0 + w1 * inv_d1 + w2 * inv_d2) * inv_area;
            let mut i_sum = (w0 * i0d + w1 * i1d + w2 * i2d) * inv_area;

            let row_offset = y * width;

            for x in min_x..=max_x {
                let is_inside = if area_positive {
                    w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0
                } else {
                    w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0
                };

                if is_inside {
                    let inv_inv_w = 1.0 / inv_w;
                    let depth = inv_inv_w;

                    let idx = row_offset + x;
                    if depth < self.depth[idx] {
                        let intensity = i_sum * inv_inv_w;
                        let r = (base_r * intensity).clamp(0.0, 255.0) as u8;
                        let g = (base_g * intensity).clamp(0.0, 255.0) as u8;
                        let b = (base_b * intensity).clamp(0.0, 255.0) as u8;

                        self.pixels[idx] = Color::new(r, g, b);
                        self.depth[idx] = depth;
                        self.atom_ids[idx] = atom_id;
                    }
                }

                w0 += dw0_dx;
                w1 += dw1_dx;
                w2 += dw2_dx;
                inv_w += dinv_w_dx;
                i_sum += di_sum_dx;
            }

            row_w0 += jump_w0;
            row_w1 += jump_w1;
            row_w2 += jump_w2;
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
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.set_target_fps(120);

        let window_menu = Menu::new("Window").unwrap();
        let _window_handle = window.add_menu(&window_menu);

        let mut menu = Menu::new("File").unwrap();
        menu.add_item("Open...", 1) // ID 1 for Open
            .shortcut(Key::O, MENU_KEY_CTRL) // ctrl even for macOS 'cause minifb make the change
            .build();

        let menu_handle = Some(window.add_menu(&menu));

        Self {
            inner: window,
            menu_handle,
        }
    }

    pub fn active(&self) -> bool {
        self.inner.is_open() && !self.inner.is_key_down(Key::Escape)
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.inner.get_size()
    }

    pub fn update_with_buffer(&mut self, buffer: &[u32], width: usize, height: usize) {
        self.inner
            .update_with_buffer(buffer, width, height)
            .unwrap();
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.inner.is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.inner.is_key_pressed(key, minifb::KeyRepeat::No)
    }

    pub fn get_mouse_pos(&self) -> Option<(f32, f32)> {
        self.inner.get_mouse_pos(MouseMode::Discard)
    }

    pub fn is_mouse_down(&self, left: bool) -> bool {
        self.inner.get_mouse_down(if left {
            MouseButton::Left
        } else {
            MouseButton::Right
        })
    }

    pub fn get_scroll_wheel(&self) -> Option<(f32, f32)> {
        self.inner.get_scroll_wheel()
    }

    pub fn set_title(&mut self, title: &str) {
        self.inner.set_title(title);
    }

    pub fn is_menu_pressed(&mut self) -> Option<usize> {
        self.inner.is_menu_pressed()
    }
}

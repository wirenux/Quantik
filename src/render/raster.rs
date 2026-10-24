use super::color::Color;
use super::framebuffer::Framebuffer;
use crate::math::edge_function;

impl Framebuffer {
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

    pub fn draw_triangle(
        &mut self,
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        d0: f32,
        d1: f32,
        d2: f32,
        int0: f32,
        int1: f32,
        int2: f32,
        base_color: Color,
        atom_id: usize,
    ) {
        let min_x_f = p0.0.min(p1.0).min(p2.0);
        let max_x_f = p0.0.max(p1.0).max(p2.0);
        let min_y_f = p0.1.min(p1.1).min(p2.1);
        let max_y_f = p0.1.max(p1.1).max(p2.1);

        // if triangle outside of the screen don't need to draw it
        if max_x_f < 0.0
            || min_x_f > (self.width as f32 - 1.0)
            || max_y_f < 0.0
            || min_y_f > (self.height as f32 - 1.0)
        {
            return;
        }

        let min_x = min_x_f.max(0.0).floor() as usize;
        let max_x = max_x_f.min(self.width as f32 - 1.0).floor() as usize;
        let min_y = min_y_f.max(0.0).floor() as usize;
        let max_y = max_y_f.min(self.height as f32 - 1.0).floor() as usize;

        let v0 = p0;
        let v1 = p1;
        let v2 = p2;

        // total area of the triangle
        let area = edge_function(v0, v1, v2);

        // if triangle is falt, useless to draw it
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
        let mut row_w0 = edge_function(v1, v2, start);
        let mut row_w1 = edge_function(v2, v0, start);
        let mut row_w2 = edge_function(v0, v1, start);

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

            // recompute inv_w from the row starting w values each row
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

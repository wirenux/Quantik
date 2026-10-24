use glam::Vec3;
use minifb::Key;

use super::Camera;
use crate::window::AppWindow;

impl Camera {
    pub fn handle_input(&mut self, window: &AppWindow) -> bool {
        let mut changed = false;

        let mouse = window.get_mouse_pos();
        let left_down = window.is_mouse_down(true);
        let right_down = window.is_mouse_down(false);

        if left_down {
            if let (Some((mx, my)), Some((px, py))) = (mouse, self.prev_mouse) {
                let dx = mx - px;
                let dy = my - py;
                let sensitivity = 0.005_f32;

                self.yaw -= dx * sensitivity;
                self.pitch += dy * sensitivity;

                let max_pitch = 89.0_f32.to_radians();
                self.pitch = self.pitch.clamp(-max_pitch, max_pitch);

                changed = true;
            }
            self.prev_mouse = mouse;
        } else {
            self.prev_mouse = None;
        }

        if right_down {
            if let (Some((mx, my)), Some((px, py))) = (mouse, self.prev_right_mouse) {
                let dx = mx - px;
                let dy = my - py;
                let sensitivity = 0.002_f32 * self.distance;

                let cp = self.pitch.cos();
                let sp = self.pitch.sin();
                let cy = self.yaw.cos();
                let sy = self.yaw.sin();

                let forward = Vec3::new(cp * sy, sp, cp * cy).normalize();

                let right = Vec3::Y.cross(forward).normalize();

                let up = forward.cross(right);

                self.target -= right * (dx * sensitivity);
                self.target += up * (dy * sensitivity);

                self.recompute_position();

                changed = true;
            }
            self.prev_right_mouse = mouse;
        } else {
            self.prev_right_mouse = None;
        }

        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            self.distance *= 1.0 - scroll_y * 0.1;
            self.distance = self.distance.clamp(self.min_distance, self.max_distance);
            changed = true;
        }

        let speed = 0.02_f32;
        if window.is_key_down(Key::Left) {
            self.yaw -= speed;
            changed = true;
        }
        if window.is_key_down(Key::Right) {
            self.yaw += speed;
            changed = true;
        }
        if window.is_key_down(Key::Up) {
            self.pitch += speed;
            changed = true;
        }
        if window.is_key_down(Key::Down) {
            self.pitch -= speed;
            changed = true;
        }
        if window.is_key_down(Key::W) {
            self.distance -= 0.1;
            changed = true;
        }
        if window.is_key_down(Key::S) {
            self.distance += 0.1;
            changed = true;
        }
        if window.is_key_down(Key::Minus) {
            self.auto_rotate_speed /= 1.05;
            changed = true;
        }
        if window.is_key_down(Key::Equal) {
            self.auto_rotate_speed *= 1.05;
            changed = true;
        }
        if window.is_key_pressed(Key::Space) {
            self.auto_rotate = !self.auto_rotate;
            changed = true;
        }
        if window.is_key_down(Key::R) {
            self.yaw = 0.0;
            self.pitch = 0.0;
            self.distance = 10.0;
            self.target = Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            self.recompute_position();
            changed = true;
        }

        if self.auto_rotate {
            self.yaw += self.auto_rotate_speed;
            changed = true;
        }

        if changed {
            let max_pitch = 89.0_f32.to_radians();
            self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
            self.distance = self.distance.clamp(self.min_distance, self.max_distance);
            self.recompute_position();
        }

        changed
    }
}

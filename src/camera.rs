use crate::window::AppWindow;
use glam::{Mat4, Vec3};
use minifb::Key;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_degrees: f32,
    pub near: f32,
    pub far: f32,
    pub prev_mouse: Option<(f32, f32)>,
    pub prev_right_mouse: Option<(f32, f32)>,
    pub min_distance: f32,
    pub max_distance: f32,
    pub auto_rotate: bool,
}

impl Camera {
    pub fn new(target: Vec3, distance: f32, fov_degrees: f32, near: f32, far: f32) -> Self {
        let mut cam = Self {
            position: Vec3::ZERO,
            target,
            distance,
            yaw: 0.0,
            pitch: 0.0,
            fov_degrees,
            near,
            far,
            prev_mouse: None,
            prev_right_mouse: None,
            min_distance: 0.1,
            max_distance: 100.0,
            auto_rotate: false,
        };
        cam.recompute_position();
        cam
    }

    fn recompute_position(&mut self) {
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();

        self.position = self.target
            + Vec3::new(
                self.distance * cp * sy,
                self.distance * sp,
                self.distance * cp * cy,
            );
    }

    pub fn set_distance_bounds(&mut self, min: f32, max: f32) {
        self.min_distance = min;
        self.max_distance = max;
        self.distance = self.distance.clamp(min, max);
        self.recompute_position();
    }

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
        if window.is_key_down(Key::W) || window.is_key_down(Key::Equal) {
            self.distance -= 0.1;
            changed = true;
        }
        if window.is_key_down(Key::S) || window.is_key_down(Key::Minus) {
            self.distance += 0.1;
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
            self.yaw += 0.01;
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

    pub fn view_matrix(&self) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(self.position, self.target, Vec3::Y)
    }

    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        glam::camera::rh::proj::directx::perspective(
            self.fov_degrees.to_radians(), // fov
            aspect_ratio,
            self.near, // render distance (nearest)
            self.far,  // render distance (farest)
        )
    }
}

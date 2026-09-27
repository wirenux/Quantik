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
        };
        cam.recompute_position();
        cam
    }

    fn recompute_position(&mut self) {
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();

        self.position = self.target + Vec3::new(
            self.distance * cp * sy,
            self.distance * sp,
            self.distance * cp * cy,
        );
    }

    pub fn handle_input(&mut self, window: &AppWindow) -> bool {
        let mut changed = false;

        let mouse = window.get_mouse_pos();
        let left_down = window.is_mouse_down(true);

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

        if let Some((_, scroll_y)) = window.get_scroll_wheel() {
            self.distance *= 1.0 - scroll_y * 0.1;
            self.distance = self.distance.clamp(4.5, 50.0);
            changed = true;
        }

        let speed = 0.02_f32;
        if window.is_key_down(Key::Left)  { self.yaw   -= speed; changed = true; }
        if window.is_key_down(Key::Right) { self.yaw   += speed; changed = true; }
        if window.is_key_down(Key::Up)    { self.pitch += speed; changed = true; }
        if window.is_key_down(Key::Down)  { self.pitch -= speed; changed = true; }
        if window.is_key_down(Key::W)     { self.distance -= 0.1; changed = true; }
        if window.is_key_down(Key::S)     { self.distance += 0.1; changed = true; }

        
        if changed {
            let max_pitch = 89.0_f32.to_radians();
            self.pitch = self.pitch.clamp(-max_pitch, max_pitch);
            self.distance = self.distance.clamp(4.5, 50.0);
            self.recompute_position();
        }

        changed
    }

    pub fn view_matrix(&self) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(self.position, self.target, Vec3::Y)
    }
    
    pub fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        glam::camera::rh::proj::directx::perspective(
            self.fov_degrees.to_radians(),  // fov
            aspect_ratio,
            self.near,                      // render distance (nearest)
            self.far,                       // render distance (farest)
        )
    }
}

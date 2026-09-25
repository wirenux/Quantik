use crate::window::AppWindow;
use glam::{Mat4, Vec3};
use minifb::Key;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_degrees: f32,
    pub near: f32,
    pub far: f32,
    pub speed: f32,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3, fov_degrees: f32, near: f32, far: f32) -> Self {
        Self {
            position,
            target,
            up: Vec3::Y,
            fov_degrees,
            near,
            far,
            speed: 0.05,
        }
    }

    pub fn handle_input(&mut self, window: &AppWindow) {
        if window.is_key_down(Key::Left) {
            self.position.x -= self.speed;
        }
        if window.is_key_down(Key::Right) {
            self.position.x += self.speed;
        }

        if window.is_key_down(Key::Up) {
            self.position.y += self.speed;
        }
        if window.is_key_down(Key::Down) {
            self.position.y -= self.speed;
        }

        if window.is_key_down(Key::W) {
            self.position.z -= self.speed; // zoom in
        }
        if window.is_key_down(Key::S) {
            self.position.z += self.speed; // zoom out
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        glam::camera::rh::view::look_at_mat4(self.position, self.target, self.up)
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

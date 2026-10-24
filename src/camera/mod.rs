mod input;

use glam::{Mat4, Vec3};

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
    pub auto_rotate_speed: f32,
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
            auto_rotate_speed: 0.01,
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

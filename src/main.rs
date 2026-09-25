mod window;
mod mesh;
mod camera;

use camera::Camera;
use glam::{Mat4, Vec3};
use mesh::Mesh;
use window::{AppWindow, Color, Framebuffer};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = AppWindow::new("Quantik - MOLECULE_NAME: CID", WIDTH, HEIGHT);

    // let cube = Mesh::generate_cube(1.0);
    let sphere = Mesh::generate_uv_sphere(1.0, 32, 16);
    let angle: f32 = 0.0;

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::ZERO,
        60.0,
        0.1,
        100.0,
    );

    while window.active() {
        let (current_width, current_height) = window.get_size();

        if framebuffer.width != current_width || framebuffer.height != current_height {
            framebuffer.resize(current_width, current_height); // resize content of fb when releasing the resize handle
        }

        framebuffer.clear(Color::BLACK);

        camera.handle_input(&window);

        let model = Mat4::from_rotation_x(angle) * Mat4::from_rotation_y(angle);

        sphere.draw(&mut framebuffer, model, &camera);
        
        let buffer = framebuffer.to_u32_buffer();
        window.update_with_buffer(&buffer, framebuffer.width, framebuffer.height)
    }
}

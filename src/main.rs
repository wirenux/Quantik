mod window;
mod mesh;
mod camera;

use camera::Camera;
use glam::{Mat4, Quat, Vec3};
use mesh::Mesh;
use window::{AppWindow, Color, Framebuffer};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut window = AppWindow::new("Quantik - MOLECULE_NAME: CID", WIDTH, HEIGHT);

    let sphere = Mesh::generate_uv_sphere(1.0, 24, 12);
    let cylinder = Mesh::generate_cylinder(1.0, 1.0, 8);

    let a = Vec3::new(-1.5, 0.0, 0.0);
    let b = Vec3::new( 1.5, 0.5, 0.0);

    let model_a = Mat4::from_translation(a) * Mat4::from_scale(Vec3::splat(0.4));
    let model_b = Mat4::from_translation(b) * Mat4::from_scale(Vec3::splat(0.4));

    let dir = (b - a).normalize();
    let mid = (a + b) * 0.5;
    let len = (b - a).length();
    let rot = Quat::from_rotation_arc(Vec3::Y, dir);
    let model_bond = Mat4::from_translation(mid)
        * Mat4::from_quat(rot)
        * Mat4::from_scale(Vec3::new(0.1, len, 0.1));

    let mut camera = Camera::new(
        Vec3::ZERO,
        5.0,
        60.0,
        0.1,
        100.0,
    );

    let mut frames = 0u32;
    let mut last_print = std::time::Instant::now();

    let mut needs_redraw = true;

    while window.active() {
        let (current_width, current_height) = window.get_size();

        if framebuffer.width != current_width || framebuffer.height != current_height {
            framebuffer.resize(current_width, current_height); // resize content of fb when releasing the resize handle
            needs_redraw = true;
        }


        camera.handle_input(&window);
        if camera.handle_input(&window) {
            needs_redraw = true;
        }

        if needs_redraw {
            framebuffer.clear(Color::BLACK);

            sphere.draw(&mut framebuffer, model_a, &camera);
            sphere.draw(&mut framebuffer, model_b, &camera);
            cylinder.draw(&mut framebuffer, model_bond, &camera);

            needs_redraw = false;
            frames += 1;
        }

        let buffer = framebuffer.to_u32_buffer();
        window.update_with_buffer(&buffer, framebuffer.width, framebuffer.height);

        let now = std::time::Instant::now();
        if now.duration_since(last_print).as_secs_f32() >= 1.0 {
            println!("FPS: {}", frames);
            frames = 0;
            last_print = now;
        }
    }
}

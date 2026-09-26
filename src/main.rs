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

    
    let sphere_a = Mesh::generate_uv_sphere(0.6, 32, 16);
    let cube = Mesh::generate_cube(1.0);

    let model_a = Mat4::from_translation(Vec3::new(-0.4, 0.0,  0.5));
    let model_b = Mat4::from_translation(Vec3::new( 0.4, 0.0, -0.5));

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::ZERO,
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


        let prev_pos = camera.position;
        camera.handle_input(&window);
        if camera.position != prev_pos {
            needs_redraw = true;
        }

        if needs_redraw {
            framebuffer.clear(Color::BLACK);
            cube.draw(&mut framebuffer, model_b, &camera);
            sphere_a.draw(&mut framebuffer, model_a, &camera);
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

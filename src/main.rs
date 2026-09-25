mod window;
use window::{AppWindow, Color, Framebuffer};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    Framebuffer::set_pixel(&mut framebuffer, 1, 1, Color::WHITE);
    Framebuffer::set_pixel(&mut framebuffer, 2, 1, Color { r: 255, g: 0, b: 0});
    Framebuffer::set_pixel(&mut framebuffer, 3, 1, Color::WHITE);

    let mut window = AppWindow::new("Quantik - MOLECULE_NAME: CID", WIDTH, HEIGHT);

    while window.active() {
        let (current_width, current_height) = window.get_size();

        if framebuffer.width != current_width || framebuffer.height != current_height {
            framebuffer.resize(current_width, current_height); // resize content of fb when releasing the resize handle
            
            // future update loop
            framebuffer.set_pixel(1, 1, Color::WHITE);
            framebuffer.set_pixel(2, 1, Color::RED);
            framebuffer.set_pixel(3, 1, Color::WHITE);
        }

        let buffer = framebuffer.to_u32_buffer();

        window.update_with_buffer(&buffer, framebuffer.width, framebuffer.height)
    }
}

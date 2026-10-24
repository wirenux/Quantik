use crate::render::{Color, Framebuffer};

pub fn draw_welcome(framebuffer: &mut Framebuffer) {
    #[cfg(target_os = "macos")]
    let shortcut_hint = "File > Open... (Cmd+O)";
    #[cfg(not(target_os = "macos"))]
    let shortcut_hint = "File > Open... (Ctrl+O)";

    #[cfg(target_os = "macos")]
    let shortcut_example = "Or use Cmd+E to open a built-in example";
    #[cfg(not(target_os = "macos"))]
    let shortcut_example = "Or use Ctrl+E to open a built-in example";

    let scale1 = 4;
    let scale2 = 2;

    let h1 = Framebuffer::text_height(scale1);
    let h2 = Framebuffer::text_height(scale2);

    let gap = 32;
    let total = h1 + gap + h2;
    let y0 = framebuffer.height.saturating_sub(total) / 2;

    framebuffer.draw_text_centered(y0, "Please open a .sdf file", scale1, Color::WHITE);
    framebuffer.draw_text_centered(y0 + h1 + 8, shortcut_example, scale2, Color::WHITE);
    framebuffer.draw_text_centered(y0 + h1 + gap, shortcut_hint, scale2, Color::GREY);
    framebuffer.draw_text(0, framebuffer.height - 20, "By @wirenux", 2, Color::WHITE);
}

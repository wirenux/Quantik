use crate::molecule::{molecule_formula, Molecule};
use crate::render::{Color, Framebuffer};

pub fn draw_examples_menu(framebuffer: &mut Framebuffer, examples: &[(&'static str, Molecule)]) {
    let row_h = 36;
    let pad = 20;
    let title_scale = 3;
    let list_scale = 2;
    let gap_title_list = 16;

    let title_h = Framebuffer::text_height(title_scale) + pad;
    let list_h = row_h * examples.len();
    let footer_h = Framebuffer::text_height(list_scale) + pad * 2;

    let w = 425;
    let h = title_h + gap_title_list + list_h + footer_h;
    let x = (framebuffer.width.saturating_sub(w)) / 2;
    let y = (framebuffer.height.saturating_sub(h)) / 2;

    framebuffer.draw_rect(x - 2, y - 2, w + 4, h + 4, Color::WHITE);
    framebuffer.draw_rect(x, y, w, h, Color::DARK_GREY);

    framebuffer.draw_text(x + pad, y + pad, "Examples", title_scale, Color::AMBER);

    for (i, (name, m)) in examples.iter().enumerate() {
        // draw the mol name + formula
        let y_line = y + title_h + gap_title_list + i * row_h;
        let mut cx = x + pad + 10;

        let idx_text = format!("{}. ", i + 1);
        framebuffer.draw_text(cx, y_line, &idx_text, list_scale, Color::AMBER);
        cx += Framebuffer::text_width(&idx_text) * list_scale;

        let name_text = format!("{}  ", name);
        framebuffer.draw_text(cx, y_line, &name_text, list_scale, Color::WHITE);
        cx += Framebuffer::text_width(&name_text) * list_scale;

        framebuffer.draw_text(cx, y_line, "(", list_scale, Color::WHITE);
        cx += 8 * list_scale;

        let formula = molecule_formula(m);
        framebuffer.draw_formula(cx, y_line, &formula, list_scale, Color::LIGHT_PURPLE);
        cx += Framebuffer::formula_width(&formula, list_scale);

        framebuffer.draw_text(cx, y_line, ")", list_scale, Color::WHITE);
    }

    let footer = "Press a number to load";
    framebuffer.draw_text(
        x + pad,
        y + title_h + gap_title_list + list_h + pad,
        footer,
        list_scale,
        Color::GREY,
    );
}

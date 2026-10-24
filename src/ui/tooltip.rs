use crate::molecule::{atom_name, molar_mass, vdw_radius, Atom};
use crate::render::{Color, Framebuffer};

pub fn draw_atom_tooltip(framebuffer: &mut Framebuffer, atom: &Atom, idx: usize, mx: f32, my: f32) {
    let main_text = format!("{} ({}) #{}", atom_name(&atom.element), atom.element, idx);

    // space is used for indentation in the tooltip
    let atom_radius_text = format!(" Radius: {} A", vdw_radius(&atom.element.to_string()));
    let molar_mass_text = format!(" MM: {:.2} g/mol", molar_mass(&atom.element.to_string()));

    let pad = 4;
    let scale = 2;

    let main_text_w = Framebuffer::text_width(&main_text) * scale;
    let atom_radius_text_w = Framebuffer::text_width(&atom_radius_text) * scale;
    let molar_mass_text_w = Framebuffer::text_width(&molar_mass_text) * scale;
    let text_w = main_text_w.max(atom_radius_text_w).max(molar_mass_text_w); // get biggest number b/w main_text_w & atom_radius_text_w

    let text_h = 16;
    let box_w = text_w + pad * 2;
    let box_h = text_h + pad * 11;

    let ox = (mx as usize).saturating_add(12);
    let oy = (my as usize).saturating_add(12);

    let bx = ox.min(framebuffer.width.saturating_sub(box_w));
    let by = oy.min(framebuffer.height.saturating_sub(box_h));

    let line_h = Framebuffer::text_height(scale) + 2;

    framebuffer.draw_rect(bx - 2, by - 2, box_w + 4, box_h + 4, Color::WHITE);
    framebuffer.draw_rect(bx, by, box_w, box_h, Color::DARK_GREY);

    framebuffer.draw_text(bx + pad, by + pad, &main_text, scale, Color::AMBER);
    framebuffer.draw_text(
        bx + pad,
        by + pad + line_h,
        &atom_radius_text,
        scale,
        Color::WHITE,
    );
    framebuffer.draw_text(
        bx + pad,
        by + pad + 2 * line_h,
        &molar_mass_text,
        scale,
        Color::WHITE,
    );
}

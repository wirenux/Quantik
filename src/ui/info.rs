use crate::molecule::{molecule_formula, molecule_name, total_molar_mass, Molecule};
use crate::render::{Color, Framebuffer};

pub fn draw_molecule_info(framebuffer: &mut Framebuffer, m: &Molecule) {
    let title_text = format!("{} (CID: {})", molecule_name(m.cid), m.cid);
    let formula_text = format!(" Formula: {}", molecule_formula(m));
    let nb_atoms_text = format!(" Atoms: {}", m.atoms.len());
    let nb_bonds_text = format!(" Bonds: {}", m.bonds.len());
    let total_mm_text = format!(" MM: {:.2} g/mol", total_molar_mass(m));

    let pad = 4;
    let scale = 2;
    let line_h = Framebuffer::text_height(scale) + 2;

    let line_widths = [
        Framebuffer::text_width(&title_text) * scale,
        Framebuffer::formula_width(&formula_text, scale),
        Framebuffer::text_width(&nb_atoms_text) * scale,
        Framebuffer::text_width(&nb_bonds_text) * scale,
        Framebuffer::text_width(&total_mm_text) * scale,
    ];
    let text_w = *line_widths.iter().max().unwrap_or(&0);

    let box_w = text_w + pad * 2;
    let box_h = line_h * 5 + pad * 2;

    let bx: usize = 8;
    let by: usize = 8;

    framebuffer.draw_rect(bx - 2, by - 2, box_w + 4, box_h + 4, Color::WHITE);
    framebuffer.draw_rect(bx, by, box_w, box_h, Color::DARK_GREY);

    framebuffer.draw_text(bx + pad, by + pad, &title_text, scale, Color::AMBER);
    framebuffer.draw_formula(
        bx + pad,
        by + pad + line_h,
        &formula_text,
        scale,
        Color::WHITE,
    );
    framebuffer.draw_text(
        bx + pad,
        by + pad + 2 * line_h,
        &nb_atoms_text,
        scale,
        Color::WHITE,
    );
    framebuffer.draw_text(
        bx + pad,
        by + pad + 3 * line_h,
        &nb_bonds_text,
        scale,
        Color::WHITE,
    );
    framebuffer.draw_text(
        bx + pad,
        by + pad + 4 * line_h,
        &total_mm_text,
        scale,
        Color::WHITE,
    );
}

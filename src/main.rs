mod camera;
mod mesh;
mod molecule;
mod window;

use camera::Camera;
use glam::{Mat4, Quat, Vec3};
use mesh::Mesh;
use minifb::Key;
use molecule::{cpk_color, molecule_name, Molecule};
use rfd::FileDialog;
use window::{AppWindow, Color, Framebuffer};

use crate::molecule::{atom_name, make_examples, molar_mass, molecule_formula, total_molar_mass, vdw_radius};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const SS_BQ: usize = 1;
const SS_HQ: usize = 3;

fn reframe(m: &Molecule) -> (Vec3, Camera) {
    // reframe the cam around the new mol
    let centroid: Vec3 = m.atoms.iter().map(|a| a.position).sum::<Vec3>() / m.atoms.len() as f32;

    let max_r = m
        .atoms
        .iter()
        .map(|a| (a.position - centroid).length())
        .fold(0.0_f32, f32::max);

    let mut cam = Camera::new(Vec3::ZERO, max_r * 4.0, 60.0, 0.1, 100.0);
    cam.set_distance_bounds(max_r * 1.2, max_r * 8.0);
    (-centroid, cam)
}

fn main() {
    let mut ss = SS_BQ;

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mut render_fb = Framebuffer::new(WIDTH * ss, HEIGHT * ss);

    let mut window = AppWindow::new("Quantik", WIDTH, HEIGHT);

    let sphere = Mesh::generate_uv_sphere(1.0, 16, 16);
    let cylinder = Mesh::generate_cylinder(1.0, 1.0, 16);

    let examples = make_examples();

    let mut mol: Option<Molecule> = None;
    let mut shift = Vec3::ZERO;
    let mut camera = Camera::new(Vec3::ZERO, 10.0, 60.0, 0.1, 100.0);

    let mut show_examples = false;

    let mut frames = 0u32;
    let mut last_print = std::time::Instant::now();
    let mut needs_redraw = true;
    let mut last_hover: Option<usize> = None;

    while window.active() {
        if window.is_key_pressed(Key::Escape) {
            if show_examples {
                show_examples = false;
                needs_redraw = true;
            } else {
                break;
            }
        }

        if let Some(menu_id) = window.is_menu_pressed() {
            match menu_id {
                1 => {
                    if let Some(file) = FileDialog::new()
                        .add_filter("SDF File", &["sdf", "txt"])
                        .pick_file()
                    && let Ok(new_mol) = Molecule::from_sdf_file(file.to_str().unwrap_or("")) {
                        let (s, c) = reframe(&new_mol);

                        shift = s; // move molecule centroid, so the camera can look at (0, 0, 0)
                        camera = c;

                        let new_title = format!(
                            "Quantik - {} ({})",
                            molecule_name(new_mol.cid),
                            new_mol.cid
                        );

                        window.set_title(&new_title);

                        mol = Some(new_mol);
                        last_hover = None;
                        show_examples = false;
                        needs_redraw = true;
                    }
                }
                2 => {
                    show_examples = !show_examples;
                    needs_redraw = true;
                }
                _ => {}
            }
        }

        if show_examples {
            const KEYS: [Key; 9] = [
                Key::Key1, Key::Key2, Key::Key3,
                Key::Key4, Key::Key5, Key::Key6,
                Key::Key7, Key::Key8, Key::Key9,
            ];
            for (i, key) in KEYS.iter().enumerate() {
                if window.is_key_pressed(*key) && let Some((_name, template)) = examples.get(i) {
                    let new_mol = template.clone();
                    let (s, c) = reframe(&new_mol);

                    shift = s;
                    camera = c;

                    let title = format!(
                        "Quantik - {} ({})",
                        molecule_name(new_mol.cid),
                        new_mol.cid
                    );

                    window.set_title(&title);
                    mol = Some(new_mol);
                    last_hover = None;
                    show_examples = false;
                    needs_redraw = true;
                }
            }
        }

        let (current_width, current_height) = window.get_size();

        if framebuffer.width != current_width || framebuffer.height != current_height {
            framebuffer.resize(current_width, current_height); // resize content of fb (on macos it's when releasing the resize handle)
            render_fb.resize(current_width * ss, current_height * ss);
            needs_redraw = true;
        }

        let moved = mol.is_some() && camera.handle_input(&window);
        if moved {
            needs_redraw = true;
        }

        let want = if moved {
            SS_BQ
        } else {
            SS_HQ
        };

        if want != ss {
            ss = want;
            render_fb.resize(current_width * ss, current_height * ss);
            needs_redraw = true;
        }

        let mouse_pos = window.get_mouse_pos();
        let hovered: Option<usize> = mouse_pos.and_then(|(mx, my)| {
            let x = (mx * ss as f32) as usize;
            let y = (my * ss as f32) as usize;
            if x < render_fb.width && y < render_fb.height {
                let idx = y * render_fb.width + x;
                let atom_idx = render_fb.atom_ids[idx];
                if atom_idx != usize::MAX {
                    Some(atom_idx)
                } else {
                    None
                }
            } else {
                None
            }
        });

        if needs_redraw {
            framebuffer.clear(Color::BLACK);
            render_fb.clear(Color::BLACK);

            match &mol {
                Some(m) => {
                    for (index, atom) in m.atoms.iter().enumerate() {
                        let (r, g, b) = cpk_color(&atom.element);
                        let color = Color::new(r, g, b);
                        let radius = vdw_radius(&atom.element) * 0.25;

                        let model = Mat4::from_translation(atom.position + shift)
                            * Mat4::from_scale(Vec3::splat(radius));

                        sphere.draw(&mut render_fb, model, &camera, color, index);
                    }

                    let bond_radius = 0.08;
                    for bond in &m.bonds {
                        // get the 2 atoms world pos (shifted to be centered)
                        let pa = m.atoms[bond.a].position + shift;
                        let pb = m.atoms[bond.b].position + shift;

                        let dir = (pb - pa).normalize(); // dir from atom A to atom B
                        let mid = (pa + pb) * 0.5; // center point of the bond (where the cylinder is)
                        let len = (pb - pa).length(); // length of the bond
                        let rot = Quat::from_rotation_arc(Vec3::Y, dir); // rotation to make the cylinder align with A and B
                        let prep = dir.any_orthogonal_vector();

                        let n = bond.order as f32; // bond.order in f32 (so the offset can be calculated)
                        let spacing = 0.20; // spacing b/w multiple cylinder

                        for i in 0..bond.order {
                            // n = 1 : 0 offset
                            // n = 2 : -0.075, +0.075 offset
                            // n = 3 : -0.15, +0.15 offset
                            let offset = (i as f32 - (n - 1.0) * 0.5) * spacing;

                            let model = Mat4::from_translation(mid + prep * offset)
                                * Mat4::from_quat(rot)
                                * Mat4::from_scale(Vec3::new(bond_radius, len, bond_radius));
                            cylinder.draw(
                                &mut render_fb,
                                model,
                                &camera,
                                Color::WHITE,
                                usize::MAX,
                            );
                        }
                    }

                    render_fb.resolve_into(&mut framebuffer, ss);

                    if let (Some(idx), Some((mx, my))) = (last_hover, mouse_pos) {
                        let atom = &m.atoms[idx];
                        let main_text =
                            format!("{} ({}) #{}", atom_name(&atom.element), atom.element, idx);

                        // space is used for indentation in the tooltip
                        let atom_radius_text =
                            format!(" Radius: {} A", vdw_radius(&atom.element.to_string()));
                        let molar_mass_text =
                            format!(" MM: {:.2} g/mol", molar_mass(&atom.element.to_string()));

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

                        framebuffer.draw_text(
                            bx + pad,
                            by + pad,
                            &main_text,
                            scale,
                            Color::AMBER,
                        );
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

                    framebuffer.draw_text(
                        bx + pad,
                        by + pad,
                        &title_text,
                        scale,
                        Color::AMBER,
                    );
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

                None => {
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

                    framebuffer.draw_text_centered(
                        y0,
                        "Please open a .sdf file",
                        scale1,
                        Color::WHITE,
                    );
                    framebuffer.draw_text_centered(
                        y0 + h1 + 8,
                        shortcut_example,
                        scale2,
                        Color::WHITE,
                    );
                    framebuffer.draw_text_centered(
                        y0 + h1 + gap,
                        shortcut_hint,
                        scale2,
                        Color::GREY,
                    );
                    framebuffer.draw_text(
                        0,
                        framebuffer.height - 20,
                        "By @wirenux",
                        2,
                        Color::WHITE
                    );
                }
            }

            if show_examples {
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

                for (i, (name, m)) in examples.iter().enumerate() { // draw the mol name + formula
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
                framebuffer.draw_text(x + pad, y + title_h + gap_title_list + list_h + pad, footer, list_scale, Color::GREY);
            }

            needs_redraw = false;
            frames += 1;
        }

        if hovered != last_hover {
            needs_redraw = true;
            last_hover = hovered;
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

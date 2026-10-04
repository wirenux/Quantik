mod camera;
mod mesh;
mod molecule;
mod window;

use camera::Camera;
use glam::{Mat4, Quat, Vec3};
use mesh::Mesh;
use molecule::{cpk_color, molecule_name, Molecule};
use rfd::FileDialog;
use window::{AppWindow, Color, Framebuffer};

use crate::molecule::{atom_name, molar_mass, vdw_radius};

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let file = FileDialog::new()
        .add_filter("sdf", &["sdf", "txt"])
        .pick_file()
        .expect("No file was selected");

    let mol = Molecule::from_sdf_file(file.to_str().expect("Load failed")).expect("load failed");

    let title = format!("Quantik - {} ({})", molecule_name(mol.cid), mol.cid);
    let mut window = AppWindow::new(&title, WIDTH, HEIGHT);

    // DEV
    println!("CID: {}", mol.cid);
    for (i, a) in mol.atoms.iter().enumerate() {
        println!("atom {}: {} @ {:?}", i, a.element, a.position);
    }
    for b in &mol.bonds {
        println!("bond {} - {} (order {})", b.a, b.b, b.order);
    }
    if let Some((mouse_x, mouse_y)) = window.get_mouse_pos() {
        println!("{} - {}", mouse_x, mouse_y);
    }
    // END OF DEV

    let centroid: Vec3 =
        mol.atoms.iter().map(|a| a.position).sum::<Vec3>() / mol.atoms.len() as f32; // average position of all the atoms

    let max_r = mol
        .atoms
        .iter()
        .map(|a| (a.position - centroid).length())
        .fold(0.0_f32, f32::max); // distance from the centroid to the farest atom

    let shift = -centroid; // move molecule centroid, so the camera can look at (0, 0, 0)

    let sphere = Mesh::generate_uv_sphere(1.0, 16, 16);
    let cylinder = Mesh::generate_cylinder(1.0, 1.0, 16);

    let mut camera = Camera::new(Vec3::ZERO, max_r * 4.0, 60.0, 0.1, 100.0);

    camera.set_distance_bounds(max_r * 1.2, max_r * 8.0);

    let mut frames = 0u32;
    let mut last_print = std::time::Instant::now();

    let mut needs_redraw = true;

    let mut last_hover: Option<usize> = None;

    while window.active() {
        let (current_width, current_height) = window.get_size();

        if framebuffer.width != current_width || framebuffer.height != current_height {
            framebuffer.resize(current_width, current_height); // resize content of fb when releasing the resize handle
            needs_redraw = true;
        }

        if camera.handle_input(&window) {
            needs_redraw = true;
        }

        let mouse_pos = window.get_mouse_pos();
        let hovered: Option<usize> = mouse_pos.and_then(|(mx, my)| {
            let mx = mx as usize;
            let my = my as usize;
            if mx < framebuffer.width && my < framebuffer.height {
                let idx = my * framebuffer.width + mx;
                let atom_idx = framebuffer.atom_ids[idx];
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

            let bond_radius = 0.08;

            for (index, atom) in mol.atoms.iter().enumerate() {
                let (r, g, b) = cpk_color(&atom.element);
                let color = Color::new(r, g, b);
                let radius = vdw_radius(&atom.element) * 0.25;

                let model = Mat4::from_translation(atom.position + shift)
                    * Mat4::from_scale(Vec3::splat(radius));

                sphere.draw(&mut framebuffer, model, &camera, color, index);
            }

            for bond in &mol.bonds {
                // get the 2 atoms world pos (shifted to be centered)
                let pa = mol.atoms[bond.a].position + shift;
                let pb = mol.atoms[bond.b].position + shift;

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
                        &mut framebuffer,
                        model,
                        &camera,
                        Color {
                            r: 255,
                            g: 255,
                            b: 255,
                        },
                        usize::MAX,
                    );
                }
            }

            if let (Some(idx), Some((mx, my))) = (last_hover, mouse_pos) {
                let atom = &mol.atoms[idx];
                let main_text = format!("{} ({}) #{}", atom_name(&atom.element), atom.element, idx);
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

                let line_h = 8 * scale + 2;

                framebuffer.draw_rect(bx - 2, by - 2, box_w + 4, box_h + 4, Color::WHITE);
                framebuffer.draw_rect(bx, by, box_w, box_h, Color::new(30, 30, 30));

                framebuffer.draw_text(
                    bx + pad,
                    by + pad,
                    &main_text,
                    scale,
                    Color::new(255, 200, 0),
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

            //if let Some((mouse_x, mouse_y)) = window.get_mouse_pos() {
            //    let mx = mouse_x as usize;
            //    let my = mouse_y as usize;
            //
            //    if mx < framebuffer.width && my < framebuffer.height {
            //        let pixel_index = my * framebuffer.width + mx;
            //        let atom_idx = framebuffer.atom_ids[pixel_index];
            //
            //        if atom_idx != usize::MAX {
            //            let hovered_atom = &mol.atoms[atom_idx];
            //            println!("Atom:{} ID:{}", hovered_atom.element, atom_idx);
            //        }
            //    }
            //}
        }
    }
}

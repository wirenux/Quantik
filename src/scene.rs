use glam::{Mat4, Quat, Vec3};

use crate::camera::Camera;
use crate::mesh::Mesh;
use crate::molecule::{cpk_color, lone_pair_dirs, lone_pairs, vdw_radius, Molecule};
use crate::render::{Color, Framebuffer};

pub fn draw_molecule(
    render_fb: &mut Framebuffer,
    m: &Molecule,
    shift: Vec3,
    camera: &Camera,
    sphere: &Mesh,
    cylinder: &Mesh,
    show_lone_pairs: bool,
) {
    for (index, atom) in m.atoms.iter().enumerate() {
        let (r, g, b) = cpk_color(&atom.element);
        let color = Color::new(r, g, b);
        let radius = vdw_radius(&atom.element) * 0.25;

        let model =
            Mat4::from_translation(atom.position + shift) * Mat4::from_scale(Vec3::splat(radius));

        sphere.draw(render_fb, model, camera, color, index);
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
        let spacing = 0.30; // spacing b/w multiple cylinder

        for i in 0..bond.order {
            // n = 1 : 0 offset
            // n = 2 : -0.075, +0.075 offset
            // n = 3 : -0.15, +0.15 offset
            let offset = (i as f32 - (n - 1.0) * 0.5) * spacing;

            let model = Mat4::from_translation(mid + prep * offset)
                * Mat4::from_quat(rot)
                * Mat4::from_scale(Vec3::new(bond_radius, len, bond_radius));
            cylinder.draw(render_fb, model, camera, Color::WHITE, usize::MAX);
        }
    }

    if show_lone_pairs {
        for (i, &count) in lone_pairs(m).iter().enumerate() {
            let atom = &m.atoms[i];
            let origin = atom.position + shift;
            let dist = vdw_radius(&atom.element) * 0.25 + 0.20;

            // unit vectors towards the bonded neighbours
            let neighbors: Vec<Vec3> = m
                .bonds
                .iter()
                .filter_map(|b| {
                    let other = if b.a == i {
                        b.b
                    } else if b.b == i {
                        b.a
                    } else {
                        return None;
                    };
                    Some((m.atoms[other].position - atom.position).normalize_or_zero())
                })
                .collect();

            // in-plane direction between two bonds (the H-H direction in water)
            let side_ref = if neighbors.len() >= 2 {
                neighbors[0] - neighbors[1]
            } else {
                Vec3::ZERO
            };

            for dir in lone_pair_dirs(m, i, count) {
                // keep only the part perpendicular to the lobe
                let e = (side_ref - dir * dir.dot(side_ref))
                    .try_normalize()
                    .unwrap_or_else(|| dir.any_orthogonal_vector().normalize());

                for sign in [-1.0_f32, 1.0] {
                    let d = (dir + e * (0.25 * sign)).normalize(); // 0.25 = spacing within a pair
                    let model = Mat4::from_translation(origin + d * dist)
                        * Mat4::from_scale(Vec3::splat(0.06));
                    sphere.draw(render_fb, model, camera, Color::LIGHT_PURPLE, usize::MAX);
                }
            }
        }
    }
}

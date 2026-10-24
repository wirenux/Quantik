use glam::{Mat4, Quat, Vec3};

use crate::camera::Camera;
use crate::mesh::Mesh;
use crate::molecule::{cpk_color, vdw_radius, Molecule};
use crate::render::{Color, Framebuffer};

pub fn draw_molecule(
    render_fb: &mut Framebuffer,
    m: &Molecule,
    shift: Vec3,
    camera: &Camera,
    sphere: &Mesh,
    cylinder: &Mesh,
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
        let spacing = 0.20; // spacing b/w multiple cylinder

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
}

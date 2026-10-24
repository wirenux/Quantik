use glam::Vec3;

use super::model::Molecule;

pub fn make_examples() -> Vec<(&'static str, Molecule)> {
    use std::f32::consts::PI;

    let mut v = Vec::new();

    v.push((
        "Water",
        Molecule::from_parts(
            962,
            vec![
                ("O", Vec3::new(0.000, 0.000, 0.0)),
                ("H", Vec3::new(0.757, 0.587, 0.0)),
                ("H", Vec3::new(-0.757, 0.587, 0.0)),
            ],
            vec![(0, 1, 1), (0, 2, 1)],
        ),
    ));

    let t = 1.09 / 3.0_f32.sqrt();
    v.push((
        "Methane",
        Molecule::from_parts(
            297,
            vec![
                ("C", Vec3::new(0.0, 0.0, 0.0)),
                ("H", Vec3::new(t, t, t)),
                ("H", Vec3::new(t, -t, -t)),
                ("H", Vec3::new(-t, t, -t)),
                ("H", Vec3::new(-t, -t, t)),
            ],
            vec![(0, 1, 1), (0, 2, 1), (0, 3, 1), (0, 4, 1)],
        ),
    ));

    v.push((
        "Ammonia",
        Molecule::from_parts(
            222,
            vec![
                ("N", Vec3::new(0.00, 0.00, 0.00)),
                ("H", Vec3::new(0.94, -0.33, 0.00)),
                ("H", Vec3::new(-0.47, -0.33, 0.81)),
                ("H", Vec3::new(-0.47, -0.33, -0.81)),
            ],
            vec![(0, 1, 1), (0, 2, 1), (0, 3, 1)],
        ),
    ));

    {
        let mut atoms = Vec::new();
        let mut bonds = Vec::new();
        for k in 0..6 {
            let a = (k as f32) * PI / 3.0;
            atoms.push(("C", Vec3::new(1.4 * a.cos(), 1.4 * a.sin(), 0.0)));
            atoms.push(("H", Vec3::new(2.4 * a.cos(), 2.4 * a.sin(), 0.0)));
        }
        for k in 0..6 {
            bonds.push((k * 2, ((k + 1) % 6) * 2, 2)); // ring bonds
            bonds.push((k * 2, k * 2 + 1, 1)); // C-H bonds
        }
        v.push(("Benzene", Molecule::from_parts(241, atoms, bonds)));
    }

    v.push((
        "Ethanol",
        Molecule::from_parts(
            702,
            vec![
                ("C", Vec3::new(-1.25, 0.00, 0.00)),
                ("C", Vec3::new(0.25, 0.00, 0.00)),
                ("O", Vec3::new(0.90, 0.90, 0.00)),
                ("H", Vec3::new(1.80, 0.80, 0.00)),
                ("H", Vec3::new(-1.65, -0.90, 0.00)),
                ("H", Vec3::new(-1.65, 0.50, 0.85)),
                ("H", Vec3::new(-1.65, 0.50, -0.85)),
                ("H", Vec3::new(0.65, -0.60, 0.85)),
                ("H", Vec3::new(0.65, -0.60, -0.85)),
            ],
            vec![
                (0, 1, 1),
                (1, 2, 1),
                (2, 3, 1),
                (0, 4, 1),
                (0, 5, 1),
                (0, 6, 1),
                (1, 7, 1),
                (1, 8, 1),
            ],
        ),
    ));

    v
}

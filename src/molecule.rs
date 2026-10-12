use std::collections::HashMap;

use glam::Vec3;

#[derive(Clone)]
pub struct Atom {
    pub element: String,
    pub position: Vec3,
}

#[derive(Clone)]
pub struct Bonds {
    pub a: usize, // from atom[0]
    pub b: usize, // to atom[1]
    pub order: u8,
}

#[derive(Clone)]
pub struct Molecule {
    pub atoms: Vec<Atom>,
    pub bonds: Vec<Bonds>,
    pub cid: usize,
}

pub const KNOWN_NAMES: &[(usize, &str)] = &[
    (962, "Water"),
    (2519, "Caffeine"),
    (446220, "Cocaine"),
    (155903693, "Vitamin C"),
    (297, "Methane"),
    (222, "Ammonia"),
    (241, "Benzene"),
    (702, "Ethanol"),
];

impl Molecule {
    pub fn from_sdf_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

        let mut lines = content.lines();
        let cid: usize = lines
            .next()
            .and_then(|l| l.trim().parse().ok())
            .unwrap_or(0);

        // go to the 3rd line
        lines.next();
        lines.next();

        let counts_line = lines.next().unwrap_or("");
        let mut counts_part = counts_line.split_whitespace();

        let num_atoms: usize = counts_part.next().and_then(|s| s.parse().ok()).unwrap_or(0);

        let num_bonds: usize = counts_part.next().and_then(|s| s.parse().ok()).unwrap_or(0);

        let mut atoms = Vec::new();

        for _ in 0..num_atoms {
            if let Some(line) = lines.next() {
                let mut parts = line.split_whitespace();

                let x: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let y: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let z: f32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0.0);

                let element: String = parts.next().unwrap_or("X").to_string();

                atoms.push(Atom {
                    element,
                    position: Vec3::new(x, y, z),
                });
            }
        }

        let mut bonds = Vec::new();

        for _ in 0..num_bonds {
            if let Some(line) = lines.next() {
                let mut parts = line.split_whitespace();

                let atom1: usize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let atom2: usize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                let order: u8 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1);

                bonds.push(Bonds {
                    a: atom1.saturating_sub(1),
                    b: atom2.saturating_sub(1),
                    order,
                });
            }
        }

        Ok(Molecule { atoms, bonds, cid })
    }

    pub fn from_parts(
        cid: usize,
        atoms: Vec<(&str, Vec3)>,
        bonds: Vec<(usize, usize, u8)>,
    ) -> Self {
        Self {
            cid,
            atoms: atoms
                .into_iter()
                .map(|(e, p)| Atom {
                    element: e.to_string(),
                    position: p,
                })
                .collect(),
            bonds: bonds
                .into_iter()
                .map(|(a, b, order)| Bonds { a, b, order })
                .collect(),
        }
    }
}

pub fn atom_name(element: &str) -> String {
    let name = match element {
        "H" => "Hydrogen",
        "C" => "Carbon",
        "N" => "Nitrogen",
        "O" => "Oxygen",
        "S" => "Sulfur",
        "P" => "Phosphorus",
        _ => "Unknown",
    };
    name.to_string()
}

pub fn cpk_color(element: &str) -> (u8, u8, u8) {
    match element {
        "H" => (255, 255, 255),
        "C" => (60, 60, 60),
        "N" => (0, 0, 255),
        "O" => (255, 0, 0),
        "S" => (255, 255, 0),
        "P" => (255, 165, 0),
        _ => (255, 0, 255),
    }
}

pub fn molar_mass(element: &str) -> f32 {
    match element {
        "H" => 1.008,
        "C" => 12.011,
        "N" => 14.007,
        "O" => 15.999,
        "S" => 32.06,
        "P" => 30.974,
        _ => 0.0,
    }
}

pub fn vdw_radius(element: &str) -> f32 {
    match element {
        "H" => 1.20,
        "C" => 1.70,
        "N" => 1.55,
        "O" => 1.52,
        "S" => 1.80,
        "P" => 1.80,
        _ => 1.70,
    }
}

pub fn molecule_name(cid: usize) -> String {
    for (known_cid, name) in KNOWN_NAMES {
        if *known_cid == cid {
            return name.to_string();
        }
    }
    format!("CID {}", cid)
}

pub fn total_molar_mass(mol: &Molecule) -> f32 {
    mol.atoms.iter().map(|a| molar_mass(&a.element)).sum()
}

pub fn molecule_formula(mol: &Molecule) -> String {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for a in &mol.atoms {
        *counts.entry(a.element.as_str()).or_insert(0) += 1;
    }

    let mut keys: Vec<&str> = counts.keys().copied().collect();
    keys.sort_by(|a, b| {
        let rank = |s: &str| match s {
            "C" => 0,
            "H" => 1,
            _ => 2,
        };
        rank(a).cmp(&rank(b)).then_with(|| a.cmp(b))
    });

    let mut out = String::new();
    for k in keys {
        let n = counts[k];
        if n == 1 {
            out.push_str(k);
        } else {
            out.push_str(&format!("{}{}", k, n));
        }
    }
    out
}

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

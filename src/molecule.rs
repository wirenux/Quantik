use std::collections::HashMap;

use glam::Vec3;

pub struct Atom {
    pub element: String,
    pub position: Vec3,
}

pub struct Bonds {
    pub a: usize, // from atom[0]
    pub b: usize, // to atom[1]
    pub order: u8,
}

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

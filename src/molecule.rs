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

pub const KNOWN_NAMES: &[(usize, &str)] =
    &[(962, "Water"), (2519, "Caffeine"), (446220, "Cocaine")];

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

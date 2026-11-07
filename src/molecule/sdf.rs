use glam::Vec3;

use super::model::{Atom, Bonds, Molecule};

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
                    charge: 0,
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

        for line in lines {
            if line.starts_with("M  END") {
                break;
            }
            if line.starts_with("M  CHG") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let n: usize = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                for k in 0..n {
                    let idx: Option<usize> = parts.get(3 + 2 * k).and_then(|s| s.parse().ok());
                    let chg: Option<i32> = parts.get(4 + 2 * k).and_then(|s| s.parse().ok());
                    if let (Some(i), Some(c)) = (idx, chg) {
                        if i >= 1 && i <= atoms.len() {
                            atoms[i - 1].charge = c;
                        }
                    }
                }
            }
        }

        Ok(Molecule { atoms, bonds, cid })
    }
}

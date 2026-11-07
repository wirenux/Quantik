use glam::Vec3;
use std::collections::HashMap;
use std::f32::consts::TAU;

use super::element::{molar_mass, valence_electron};
use super::model::Molecule;

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

pub fn lone_pairs(mol: &Molecule) -> Vec<u32> {
    let mut bond_sum = vec![0i32; mol.atoms.len()];
    for b in &mol.bonds {
        bond_sum[b.a] += b.order as i32;
        bond_sum[b.b] += b.order as i32;
    }
    mol.atoms
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let electrons = valence_electron(&a.element) as i32 - a.charge - bond_sum[i];
            (electrons / 2).max(0) as u32
        })
        .collect()
}

pub fn lone_pair_dirs(mol: &Molecule, i: usize, count: u32) -> Vec<Vec3> {
    let pos = mol.atoms[i].position;

    // unit vectors from atom i towards each bonded neighbour
    let u: Vec<Vec3> = mol
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
            Some((mol.atoms[other].position - pos).normalize_or_zero())
        })
        .collect();

    if u.is_empty() || count == 0 {
        return Vec::new();
    }

    // opposite of the bonds
    let sum: Vec3 = u.iter().copied().sum();
    let axis = if u.len() >= 3 && sum.length() < 0.3 {
        // planar atom
        u[0].cross(u[1])
            .try_normalize()
            .unwrap_or_else(|| u[0].any_orthogonal_vector().normalize())
    } else {
        (-sum)
            .try_normalize()
            .unwrap_or_else(|| u[0].any_orthogonal_vector().normalize())
    };

    match (u.len(), count) {
        (_, 1) => vec![axis],

        (2, 2) => {
            let n = u[0]
                .cross(u[1])
                .try_normalize()
                .unwrap_or_else(|| axis.any_orthogonal_vector().normalize());
            let (s, c) = 54.75_f32.to_radians().sin_cos();
            vec![axis * c + n * s, axis * c - n * s]
        }

        _ => {
            let e1 = axis.any_orthogonal_vector().normalize();
            let e2 = axis.cross(e1);
            (0..count)
                .map(|k| {
                    let phi = TAU * k as f32 / count as f32;
                    axis / 3.0 + (e1 * phi.cos() + e2 * phi.sin()) * 0.9428
                })
                .collect()
        }
    }
}

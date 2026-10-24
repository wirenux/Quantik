use std::collections::HashMap;

use super::element::molar_mass;
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

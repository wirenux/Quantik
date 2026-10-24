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

impl Molecule {
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

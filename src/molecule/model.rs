use glam::Vec3;

#[derive(Clone)]
pub struct Atom {
    pub element: String,
    pub position: Vec3,
    pub charge: i32,
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
        atoms: Vec<(&str, Vec3, i32)>,
        bonds: Vec<(usize, usize, u8)>,
    ) -> Self {
        Self {
            cid,
            atoms: atoms
                .into_iter()
                .map(|(e, p, c)| Atom {
                    element: e.to_string(),
                    position: p,
                    charge: c,
                })
                .collect(),
            bonds: bonds
                .into_iter()
                .map(|(a, b, order)| Bonds { a, b, order })
                .collect(),
        }
    }
}

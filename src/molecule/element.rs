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

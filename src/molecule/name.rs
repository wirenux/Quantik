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

pub fn molecule_name(cid: usize) -> String {
    for (known_cid, name) in KNOWN_NAMES {
        if *known_cid == cid {
            return name.to_string();
        }
    }
    format!("CID {}", cid)
}

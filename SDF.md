# The SDF File Format

SDF (Structure-Data File) is a chemistry file format for storing molecules: 3D atom coordinates and explicit bonds, in one plain-text block per molecule.

## Example — water (CID 962)

```sdf
962
  -ISIS-  03119923412D

  3  2  0  0  0  0  0  0  0  0999 V2000
    0.0000    0.0000    0.1173 O   0  0  0  0  0  0  0  0  0  0  0  0
    0.0000    0.7572   -0.4692 H   0  0  0  0  0  0  0  0  0  0  0  0
    0.0000   -0.7572   -0.4692 H   0  0  0  0  0  0  0  0  0  0  0  0
  1  2  1  0  0  0  0
  1  3  1  0  0  0  0
M  END
> <PUBCHEM_COMPOUND_CID>
962

$$$$
```

## Breakdown

| Section | Lines | What it is |
| --- | --- | --- |
| Header | 1 | Molecule CID |
| Header | 2 | Software/timestamp line (usually ignorable) |
| Header | 3 | Blank comment line |
| Counts line | 1 | First number = atom count (N), second = bond count (M). Rest are legacy fields, ignore |
| Atom block | N lines | `x y z element <10 more fields you ignore>` — this is your 3D coordinate data |
| Bond block | M lines | `atom1_idx atom2_idx bond_order <ignore rest>` — 1-indexed atom references. `bond_order` is 1/2/3 for single/double/triple |
| End marker | 1 | `M END` — terminates the structure block |
| Property block | varies | Optional `> <TAG_NAME>` blocks with metadata (CID, molecular weight, etc.) — not needed for rendering |
| Molecule terminator | 1 | `$$$$` — ends this molecule; also separates multiple molecules if the file has more than one |

Everything from `M END` onward is metadata you can skip when parsing for rendering purposes.

## Parsing logic

1. Read the counts line → get `n_atoms`, `n_bonds`
2. Read the next `n_atoms` lines → split on whitespace, parse fields 1–3 as `f32` (x, y, z), field 4 as the element symbol
3. Read the next `n_bonds` lines → parse fields 1–2 as atom indices (subtract 1 for 0-indexing), field 3 as bond order
4. Stop — ignore everything from `M END` onward

```rust
struct Atom {
    element: String,
    pos: glam::Vec3,
}

struct Bond {
    a: usize,   // index into atoms
    b: usize,
    order: u8,  // 1, 2, or 3
}

struct Molecule {
    atoms: Vec<Atom>,
    bonds: Vec<Bond>,
}
```

## Getting SDF files from PubChem

```bash
https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/{CID}/SDF?record_type=3d
```

`record_type=3d` is required — the default record can be 2D-only (z always 0). If a compound has no 3D conformer, fall back to the 2D SDF and run it through `obabel --gen3d`.

```bash
CID=962
curl "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/${CID}/SDF?record_type=3d" -o water.sdf
```

> [!NOTE]
> this file have been AI generated to gain time rewriting all my paper note (double check has been done)

#!/usr/bin/env python3

# Usage:
#     python3 gen_element.py # print to stdout
#     python3 gen_element.py -o src/molecule/element.rs # save to .rs file
#     python3 gen_element.py -i PeriodicTableJSON.json -o element.rs # save to .rs file but from local .json file

import argparse
import json
import sys
import urllib.request

URL = (
    "https://raw.githubusercontent.com/Bowserinator/"
    "Periodic-Table-JSON/master/PeriodicTableJSON.json"
)

INDENT = "        "
MAX_WIDTH = 100

COLOR_OVERRIDES = {
    "H": (255, 255, 255),
    "C": (60, 60, 60),
    "N": (0, 0, 255),
    "O": (255, 0, 0),
    "S": (255, 255, 0),
    "P": (255, 165, 0),
}

VDW = {
    "H": 1.20, "He": 1.40, "Li": 1.82, "C": 1.70, "N": 1.55, "O": 1.52,
    "F": 1.47, "Ne": 1.54, "Na": 2.27, "Mg": 1.73, "Si": 2.10, "P": 1.80,
    "S": 1.80, "Cl": 1.75, "Ar": 1.88, "K": 2.75, "Ni": 1.63, "Cu": 1.40,
    "Zn": 1.39, "Ga": 1.87, "As": 1.85, "Se": 1.90, "Br": 1.85, "Kr": 2.02,
    "Pd": 1.63, "Ag": 1.72, "Cd": 1.58, "In": 1.93, "Sn": 2.17, "Te": 2.06,
    "I": 1.98, "Xe": 2.16, "Pt": 1.75, "Au": 1.66, "Hg": 1.55, "Tl": 1.96,
    "Pb": 2.02, "U": 1.86,
}

VDW_DEFAULT = 1.70

def load_elements(source):
    if source.startswith(("http://", "https://")):
        with urllib.request.urlopen(source) as resp:
            data = json.load(resp)
    else:
        with open(source, encoding="utf-8") as f:
            data = json.load(f)
    elements = data["elements"] if isinstance(data, dict) else data
    return sorted(elements, key=lambda e: e["number"])


def valence_electrons(el):
    number, group = el["number"], el.get("group")
    if number == 2: # helium: group 18, but only 2 electrons
        return 2
    if group in (1, 2):
        return group
    if group is not None and 13 <= group <= 18:
        return group - 10
    return 0  # transition metals, lanthanides, actinides


def hex_to_rgb(h):
    h = h.lstrip("#")
    return int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16)


def float_literal(x):
    s = repr(float(x))
    return s if any(c in s for c in ".e") else s + ".0"


def wrap_arm(symbols, value):
    pats = [f'"{s}"' for s in symbols]
    lines = []
    cur = INDENT + pats[0]
    for p in pats[1:]:
        candidate = f"{cur} | {p}"
        if len(candidate) > MAX_WIDTH:
            lines.append(cur)
            cur = f"{INDENT}| {p}"
        else:
            cur = candidate
    lines.append(f"{cur} => {value},")
    return lines


def fn_block(signature, arms, default, tail=None):
    out = [signature]
    if tail:
        out.append("    let name = match element {")
    else:
        out.append("    match element {")
    out.extend(arms)
    out.append(f"{INDENT}_ => {default},")
    if tail:
        out.append("    };")
        out.append(f"    {tail}")
    else:
        out.append("    }")
    out.append("}")
    return "\n".join(out)


def generate(elements):
    # atom_name
    names = [f'{INDENT}"{e["symbol"]}" => "{e["name"]}",' for e in elements]
    atom_name = fn_block(
        "pub fn atom_name(element: &str) -> String {",
        names,
        '"Unknown"',
        tail="name.to_string()",
    )

    # cpk_color
    colors = []
    for e in elements:
        sym = e["symbol"]
        if sym in COLOR_OVERRIDES:
            rgb = COLOR_OVERRIDES[sym]
        elif e.get("cpk-hex"):
            rgb = hex_to_rgb(e["cpk-hex"])
        else:
            continue  # falls back to the magenta default
        colors.append(f'{INDENT}"{sym}" => ({rgb[0]}, {rgb[1]}, {rgb[2]}),')
    cpk = fn_block(
        "pub fn cpk_color(element: &str) -> (u8, u8, u8) {", colors, "(255, 0, 255)"
    )

    # molar_mass
    masses = [
        f'{INDENT}"{e["symbol"]}" => {float_literal(e["atomic_mass"])},'
        for e in elements
        if e.get("atomic_mass") is not None
    ]
    mass = fn_block("pub fn molar_mass(element: &str) -> f32 {", masses, "0.0")

    # valence_electron
    by_value = {}
    for e in elements:
        v = valence_electrons(e)
        if v > 0:
            by_value.setdefault(v, []).append(e["symbol"])
    arms = []
    for v in sorted(by_value):
        arms.extend(wrap_arm(by_value[v], v))
    valence = fn_block("pub fn valence_electron(element: &str) -> u32 {", arms, "0")

    # vdw_radius
    radii = [
        f'{INDENT}"{e["symbol"]}" => {VDW[e["symbol"]]:.2f},'
        for e in elements
        if e["symbol"] in VDW
    ]
    vdw = fn_block(
        "pub fn vdw_radius(element: &str) -> f32 {", radii, f"{VDW_DEFAULT:.2f}"
    )

    return "\n\n".join([atom_name, cpk, mass, valence, vdw]) + "\n"


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("-i", "--input", default=URL, help="URL or local path of the JSON")
    ap.add_argument("-o", "--output", help="output file (default: stdout)")
    args = ap.parse_args()

    code = generate(load_elements(args.input))
    if args.output:
        with open(args.output, "w", encoding="utf-8") as f:
            f.write(code)
        print(f"wrote {args.output}", file=sys.stderr)
    else:
        sys.stdout.write(code)


if __name__ == "__main__":
    main()

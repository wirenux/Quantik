# Roadmap

## Math / Core

- [x] Vector/matrix/quaterninon library (`glam`)
- [x] Model -> View -> Projection transform pipeline
    - [x] Model matrix: places each individual atom/bond mesh in the scene
    - [x] View matrix: reposition the scene relative to the camera
    - [x] Projection matrix: project camera-relative 3D points onto the 2D screen (perspective)

## Molecule data

- [x] Get `.sdf` file from "`PubChem`"
    - Download : `curl "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/962/SDF?record_type=3d" -o water.sdf`
    - Find CID : `curl https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/water/property/IUPACName/JSON`
- [x] Parse `.sdf`
    - [SDF File Format](./SDF.md)
- [x] Element table : symbol, atomic radius, CPK Color

## Meshes

- [x] Procedural cube generation (for dev)
- [x] Procedural sphere generation (icosphere or UV-sphere) for atoms
- [x] Procedural cylinder generation for bonds

## Rendering

- [x] Rasterizer: 3D triangle -> 2D pixel buffer (color + depth per pixel)
- [x] Depth buffer (z-buffer) for corrent occlusion between atoms/bonds
- [x] Basic shading (diffuse light + ambient light)
- [x] ID buffer (alongside the depth buffer): tags each pixel with the atoms it belongs to (for tooltip)

## Camera

- [x] Camera position / orientation state
- [x] Orbit control: drag to rotate, scroll to zoom
- [x] Press `Space` to make if rotate automatically
    - [x] `+` and `-` to speed the rotate speed
- [x] Add pan with right click drag
- [x] Press `R` to reset camera positon angle etc...

## minifb

- [x] How to display the view
    - [x] Use framebuffer
- [x] Window title with the name of the molecule / atoms and the CID (e.g: "`Quantik - Water: 962`")
- [x] macOS menu with keyboard shortcut
- [x] Add cmd+o / ctrl+o
- [x] Menu to select molecule (or just a file opener for .sdf)
- [-] Use GPU

### Mouse click / integration

- [x] Tooltip :
    - [x] Atom name + ID on hover (e.g: `O #1`)
    - [x] Trivial name : `Carbon (C)`
    - [x] VDW radius
    - [x] Molar mass
    - [x] Different color for atom name
    - [x] Indentation for the info
    - [x] White border

### UI

- [x] A box with the name and info of the molecule (top-left corner)
    - Important Info
        - [x] Molecule name & CID : `Water (CID: 962)`
        - [x] Formula
            - [x] Classic Way
            - [x] With height offset
        - [x] Molar mass of molecule
    - Other info (open with a key press or a menu item on macos and windows)
        - [x] Number of bound
        - [x] Number of atoms

### Startpage

- [x] Title : `Quantik`
- [x] No 3d render for now
- [x] Cmd+O (or Ctrl+O) / File > Open...
- [x] Erase FB
- [x] Change title to : `Quantik - Water (CID: 962)`
- [x] Render molecule and everything else

## Order

- [x] Render a static sphere
- [x] Add camera (control)
- [x] Add z-buffer
- [x] Add 2nd sphere + cylinder
- [x] Add ID Buffer + hover tooltip
- [x] Parse a .sdf
- [x] Render a .sdf

## BUG

- [x] When the base of a bound get outside of the screen the cylinder move (or look like it's moving)
- [x] When a point of a triangle was off screen on the bottom or on the right side of the screen the whole triangle was removed

## Idea

- [x] Maybe use minifb instead of Ratatui to get a simple window like "black" (on github)
- [x] The minifb advantage is the macOS top bar can be switch to a windows/linux "in window" top bar
- [x] since minifb can't add top bar to linux -> rely on shortcut
- [x] Add a text in the buffer to say that a file must be selected to run the program (behind the file selector) (with the draw_text func)

- [x] Reorganise the project in more file (like the math part of drawing triangle etc... in a different file, instead of `window.rs`)

- [?] French/English toggle (?)
- [x] Make a menu or something else to get the molecule file and open it
- [x] Also maybe create a list of molecule implemented in the program
    - [x] Open with Cmd+E / Ctrl+E

- [x] Don't render anything behind the UI

- [x] Add @wirenux on the startpage on the bottom left or right

- [x] Antialiasing (SSAA)

## Other


Fun fact: when you press `Space` the molecule rotate (in reality it's the camera) and it also help me to know how much much i want having (since i calculate FPS when the framebuffer/screen is changing)

Fun fact: I choose the name Quantik because Atomik was already taken

Keyboard shortcut don't work on Linux (at least xfce w/ X11)

Source :

https://www.scratchapixel.com/lessons/3d-basic-rendering/rasterization-practical-implementation/overview-rasterization-algorithm.html?utm_source=gemini
https://schneide.blog/2016/07/15/generating-an-icosphere-in-c/

Logo credit:

@wirenux with the help of @c3a5m7

To build for the windows version (need to be on a x86-64 windows to build the windows version):

```bash
set RUSTFLAGS=-C target-feature=+crt-static
cargo build --release
```

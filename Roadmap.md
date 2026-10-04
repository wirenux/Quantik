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
- [ ] Element table : symbol, atomic radius, CPK Color

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
- [ ] Press `Space` to make if rotate automatically
    - [ ] `+` and `-` to speed up or slow down the rotate speed
- [ ] Add pan with right click drag
- [ ] Press `R` to reset camera positon angle etc...

## minifb

- [x] How to display the view
    - [x] Use framebuffer
- [x] Window title with the name of the molecule / atoms and the CID (e.g: "`Quantik - Water: 962`")
- [ ] macOS menu with keyboard shortcut
- [ ] Menu to select molecule (or just a file opener for .sdf)
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

- [ ] A box with the name and info of the molecule (top-left corner)
    - Important Info
        - [ ] Molecule name & CID : `Water (CID: 962)`
        - [ ] Molar mass
    - Other info (open with a key press or a menu item on macos and windows)
        - [ ] Number of bound
        - [ ] Number of atoms

## Order

- [x] Render a static sphere
- [x] Add camera (control)
- [x] Add z-buffer
- [x] Add 2nd sphere + cylinder
- [x] Add ID Buffer + hover tooltip
- [x] Parse a .sdf
- [x] Render a .sdf

## Idea

- [x] Maybe use minifb instead of Ratatui to get a simple window like "black" (on github)
- [ ] The minifb advantage is the macOS top bar can be switch to a windows/linux "in window" top bar
- [ ] since minifb can't add top bar to linux -> rely on shortcut
- [ ] Add a text in the buffer to say that a file must be selected to run the program (behind the file selector) (with the draw_text func)

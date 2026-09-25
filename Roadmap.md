# Roadmap

## Math / Core

- [x] Vector/matrix/quaterninon library (`glam`)
- [x] Model -> View -> Projection transform pipeline
    - [x] Model matrix: places each individual atom/bond mesh in the scene
    - [x] View matrix: reposition the scene relative to the camera
    - [x] Projection matrix: project camera-relative 3D points onto the 2D screen (perspective)

## Molecule data

- [ ] Get `.sdf` file from "`PubChem`"
    - Download : `curl "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/962/SDF?record_type=3d" -o water.sdf`
    - Find CID : `curl https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/water/property/IUPACName/JSON`
- [ ] Parse `.sdf`
    - [SDF File Format](./SDF.md)
- [ ] Element table : symbol, atomic radius, CPK Color

## Meshes

- [x] Procedural cube generation (for dev)
- [ ] Procedural sphere generation (icosphere or UV-sphere) for atoms
- [ ] Procedural cylinder generation for bonds

## Rendering

- [ ] Rasterizer: 3D triangle -> 2D pixel buffer (color + depth per pixel)
- [ ] Depth buffer (z-buffer) for corrent occlusion between atoms/bonds
- [ ] Basic shading (diffuse light + ambient light)
- [ ] ID buffer (alongside the depth buffer): tags each pixel with the atoms it belongs to (for tooltip)

## Camera

- [x] Camera position / orientation state
- [ ] Orbit control: drag to rotate, scroll to zoom, (OPTIONAL ?) pan

## minifb

- [x] How to display the view
    - [x] Use framebuffer
- [ ] Window title with the name of the molecule / atoms and the CID (e.g: "`Quantik - Water: 962`")
- [ ] macOS menu with keyboard shortcut
- [ ] Menu to select molecule (or just a file opener for .sdf)

### Mouse click / integration

- [ ] Crossterm mouse event handling wired into ratatui event loop
- [ ] Tooltip
    - [ ] Atom name + ID on hover (e.g: `O(1)`)

## Order

- [ ] Render a static sphere
- [ ] Add camera (control)
- [ ] Add z-buffer
- [ ] Add 2nd sphere + cylinder
- [ ] Add ID Buffer + hover tooltip
- [ ] Parse a .sdf
- [ ] Render a .sdf

## Idea

- [x] Maybe use minifb instead of Ratatui to get a simple window like "black" (on github)
- [ ] The minifb advantage is the macOS top bar can be switch to a windows/linux "in window" top bar
- [ ] since minifb can't add top bar to linux -> rely on shortcut

# Roadmap

## Math / Core

- [ ] Vector/matrix/quaterninon library (`glam`)
- [ ] Model -> View -> Projection transform pipeline
    - [ ] Model matrix: places each individual atom/bond mesh in the scene
    - [ ] View matrix: reposition the scene relative to the camera
    - [ ] Projection matrix: project camera-relative 3D points onto the 2D screen (perspective)

## Molecule data

- [ ] Get `.sdf` file from "`PubChem`"
    - Download : `curl "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid/962/SDF?record_type=3d" -o water.sdf`
    - Find CID : `curl https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/water/property/IUPACName/JSON`
- [ ] Parse `.sdf`
    - [SDF File Format](./SDF.md)
- [ ] Element table : symbol, atomic radius, CPK Color

## Meshes

- [ ] Procedural sphere generation (icosphere or UV-sphere) for atoms
- [ ] Procedural cylinder generation for bonds

## Rendering

- [ ] Rasterizer: 3D triangle -> 2D pixel buffer (color + depth per pixel)
- [ ] Render at 2x vertical resolution (each terminal "cell" = 2 stacked pixels via "`▀`", top = fg, bottom = bg)
- [ ] Depth buffer (z-buffer) for corrent occlusion between atoms/bonds
- [ ] Basic shading (diffuse light + ambient light)
- [ ] (LATER) Color approximation for non-truecolor terminal
- [ ] ID buffer (alongside the depth buffer): tags each pixel with the atoms it belongs to (for tooltip)

## Camera

- [ ] Camera position / orientation state
- [ ] Orbit control: drag to rotate, scroll to zoom, (OPTIONAL ?) pan
- [ ] Shadow

## Ratatui

- [ ] How to display the view
    - [ ] Use half-block char "`▀`"
- [ ] Top line (or "div" title) with the name of the molecule / atoms and the CID (e.g: "`Water - 962`")
- [ ] Bottom line with keybind
- [ ] Menu to select molecule

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

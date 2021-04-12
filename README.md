# BevyCAD

A simple MVP of a 3d CAD System leveraging an ECS for its data model and running in the browser/native.

## Milestones

### MVP

For the MVP I want to achieve some core functionality:
 - [x] CAD camera with zooming and panning
 - [ ] 3d primitives
 - [ ] 2d primitives
 - [ ] working WASM version running in the browser
 - [ ] CAD-like tags like *Layer* and *Name*

### Additional functionality

Additional functionality that would be nice to have and can be implemented after the MVP:
 - [ ] BoundingBox Storage for optimized drawing (maybe bevy already does this?)
 - [ ] Commands API (preferably scriptable)
 - [ ] expose projection mode and general camera settings
 - [ ] serialization
 - [ ] import/export of typical CAD file formats
 - [ ] undo/redo mechanism (possible related to how commands will be implemented)

## Build and run BevyCAD

Base implementation of the WASM/native hybrid from [https://github.com/mrk-its/bevy_webgl2_app_template](wasm_template)

### Prerequisites

```
cargo install cargo-make
```

### Build and serve WASM version
```
cargo make serve
```
then point your browser to http://127.0.0.1:4000/


### Build and run native version
```
cargo make run
```

![Screenshot](https://mrk.sed.pl/bevy-showcase/assets/bevy_webgl2_app_template.png?v=3)

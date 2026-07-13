# OpenMW provenance

The save slice follows the public OpenMW save architecture as a reference:

- `components/esm3/savedgame.hpp` and `savedgame.cpp` for versioned save
  metadata and tagged record/subrecord framing;
- `apps/openmw/mwstate/statemanagerimp.cpp` for subsystem-owned records and
  serializing to memory before replacing a save file;
- `apps/openmw/mwworld/worldmodel.cpp` for cell-owned persistent state.

No OpenMW source file is copied into this directory. The implementation is a
project-specific Rust reimplementation of the framing boundary and is not
binary-compatible with OpenMW or Fallout `.fos` files. The surrounding
project remains governed by its existing GPL-3.0 license.

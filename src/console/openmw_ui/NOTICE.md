# OpenMW provenance and notice

Source snapshot: `openmw-master`, identified by the supplied source tree as
OpenMW 0.52.0. The snapshot has no nested Git metadata, so an upstream commit
is not claimed here.

The Rust behavior adaptation is based on these sources and whole-file SHA-256
hashes:

- `apps/openmw/mwgui/console.cpp`: command history, draft restoration,
  completion, repeated-Tab candidate listing, selected-object title, and
  history persistence; SHA-256
  `723E77AFDE9070593CD8D4C77C3C4A99AFD062FF3B67D8634A1B7B5B263A5CB1`.
- `apps/openmw/mwgui/console.hpp`: console UX state and public behavior;
  SHA-256
  `2E5DA717285780F465852A09A36E4C0E69A0550E1C82E37E557F033F83F8E3D6`.
- `apps/openmw/mwinput/bindingsmanager.cpp`: grave-key default console binding;
  SHA-256
  `81FD2665189B6104896AE192395D474DB24051706C29496F49FFBCB1095D0F4E`.

Only behavior was adapted into new Rust code. No OpenMW C++ is compiled.
OpenMW is GPL-3.0; bevyout is also distributed under GPL-3.0. The complete
bevyout license text is at the repository root, and the supplied OpenMW
license remains in `openmw-master/LICENSE`.

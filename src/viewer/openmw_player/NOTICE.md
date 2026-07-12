# OpenMW provenance and notice

Source snapshot: `openmw-master`, OpenMW 0.52.0. The snapshot has no nested
Git metadata, so an upstream commit is not claimed here.

The Rust adaptation is based on these source regions and whole-file hashes:

- `apps/openmw/mwmechanics/character.cpp`, jump/air/landing handling around
  lines 2194-2292; SHA-256
  `9174CC5544CCFE327F20FDF250388F8FEA215969DE408C15D590F43263944C4E`.
- `apps/openmw/mwphysics/movementsolver.cpp`, actor ground and gravity rules;
  SHA-256
  `5C0CB9B243F324EB14E28099ADA20900C2D3F0D70027C543B3ABE063F17F73CE`.
- `components/misc/constants.hpp`, metric scale and gravity constants;
  SHA-256
  `984AD3684CBB9F838DE11ED4AA32272B9A21741F78B9B8158206856A193C684D`.
- `apps/opencs/model/world/defaultgmsts.cpp`, default jump movement values;
  SHA-256
  `3ECCB30614B0FA21A772E01AB36845F550A2581BB7BD24882A653F03629FB63E`.

Only behavior and numeric rules were adapted into new Rust code; no OpenMW
C++ or Bullet implementation is compiled. OpenMW is GPL-3.0. This project is
also distributed under GPL-3.0; the complete license text is at the repository
root and in `openmw-master/LICENSE`.

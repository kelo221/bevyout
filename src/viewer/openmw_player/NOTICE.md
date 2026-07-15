# OpenMW provenance and notice

Source snapshot: `openmw-master`, OpenMW 0.52.0.

The Rust adaptation is based on these source regions and whole-file hashes:

- `apps/openmw/mwmechanics/character.cpp`, jump/air/landing handling around
  lines 2194-2292; SHA-256
  `9174CC5544CCFE327F20FDF250388F8FEA215969DE408C15D590F43263944C4E`.
- `apps/openmw/mwphysics/movementsolver.cpp`, actor ground and gravity rules;
  SHA-256
  `5C0CB9B243F324EB14E28099ADA20900C2D3F0D70027C543B3ABE063F17F73CE`.
- `apps/openmw/mwphysics/stepper.cpp`, upward-horizontal-downward stair sweep;
  SHA-256
  `98EC4DA832FD3E9F15B708C9103578479A709B4B02BF1CF3D3D29DBB08A02300`.
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

## Adapted File Contributors

Upstream Git history identifies the following contributors for these source files:

- ζeh Matt
- Abdu Sharif
- Aesylwinn
- Alexander Perepechko
- Alexei Dobrohotov
- Alexei Kotov
- Allofich
- Andrei Kortunov
- AnyOldName3
- Arthur Moore
- Bo Svensson
- Bret Curtis
- Capostrophic
- Chris Robinson
- cody glassman
- Dave Corley
- Digmaster
- dteviot
- Elias Howell
- elsid
- Emanuel Guevel
- Evgeny Kurnevsky
- Evil Eye
- florent.teppe
- Frederic Chardon
- fredzio
- fteppe
- Glorf
- gus
- Harald H
- James-Deciutiis
- jeremy
- jvoisin
- kpp
- Lukasz Gromanowski
- Mads Buvik Sandvei
- Marc Zinnschlag
- Max
- Max Yari
- Miloslav Číž
- MiroslavR
- mrcheko
- Nelsson Huotari
- Niek Wilting
- Perry Hugh
- Petr Mikheev
- psi29a
- Rafael Moura
- Ragora
- rexelion
- Rohit Nirmal
- scrawl
- Shi Han
- Telvanni 4Life
- Torben Carrington
- tri4ng1e
- uramer
- Vincent Heuken
- vorenon
- wareya
- Zackhasacat

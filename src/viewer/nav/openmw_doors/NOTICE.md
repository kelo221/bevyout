# OpenMW provenance and notice

Source snapshot: local `openmw` checkout at revision `openmw-51-rc2-331-g6b5f6aa5f1`
(commit `6b5f6aa5f1fba4be436bb9816043c53398e5a4ff`).

The Rust adaptation is based on this source region and whole-file hash:

- `apps/openmw/mwmechanics/aipackage.cpp`, `AiPackage::openDoors()`, lines
  283-315; SHA-256
  `AA1EB379423C032EBD20C21D1A49D6F266E58839D0AAA41FACE600D486F48F34`.

`apps/openmw/mwmechanics/aiavoiddoor.cpp` (`AiAvoidDoor`, the stuck-against-a-
moving-door reaction) was read for reference but is deliberately **not**
adapted here -- issue #185 asks only to verify the existing navmesh-side
fail-fast (a locked/blocked door reaches a deterministic `Failed` ->
`Unreachable` repath terminal) covers the same case, not to port
`AiAvoidDoor`'s back-away-and-turn geometry. Its whole-file hash is recorded
for provenance in case a future issue does adapt it: SHA-256
`804E0FE81D9721F99354C4DAE1622E8D8E88D9B53EFC01CC539AF836DF501DF0`.

Only the decision rule (untrapped-and-unlocked passes; locked passes only
with the actor's own key; trapped never passes) was adapted into new Rust
code -- no OpenMW C++ is compiled, and bevyout's divergence from the literal
C++ (treating *any* trapped door as unconditionally non-openable, rather than
falling through to a key check the way `openDoors()` literally does) is
intentional: bevyout has no trap-spring/damage system at all yet, so silently
opening a trapped door would be strictly worse than refusing, per the
issue's own instruction ("we do not model traps; until we do, a door with a
trap should be treated as not-openable"). OpenMW is GPL-3.0. This project is
also distributed under GPL-3.0; the complete license text is at the
repository root.

## Adapted File Contributors

Upstream Git history identifies the following contributors for these source files:

- ζeh Matt
- Alexei Dobrohotov
- Alexei Kotov
- Allofich
- Andrei Kortunov
- Bret Curtis
- Capostrophic
- dteviot
- elsid
- Emanuel Guevel
- Evil Eye
- florent.teppe
- fteppe
- jvoisin
- Marc Zinnschlag
- Matt
- mrcheko
- Petr Mikheev
- psi29a
- Rohit Nirmal
- Roman Siromakha
- scrawl
- terrorfisch
- Thomas

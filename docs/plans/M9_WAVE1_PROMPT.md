# M9 wave 1 kickoff — S.P.E.C.I.A.L., skills, derived stats, GMST, leveling

Requested on 2026-08-21:

- complete all 10 waves of M9 (epic #19) per the roadmap
  [M9_Start.md](M9_Start.md); this wave is the first;
- establish the pure mathematical kernels for character stats: SPECIAL
  attributes, the 13 Fallout 3 skills, derived statistics, the leveling
  and XP engine, and skill checks;
- decode `GMST` (game settings) and `AVIF` (actor value info) ESM4
  records during `prepare` into a versioned `PreparedGmstCatalog` with
  Fallout 3 GOTY default fallbacks;
- expose the stats at runtime on the player with `getav`, `setav`,
  `modav`, `player.advlevel`, and `player.rewardxp` console commands so
  the wave is visible in the viewer and through the agent bridge;
- follow the feature-first invariant and the M9 determinism contract
  (integer milliseconds, basis points, ordered collections, core-owned
  PRNG — no floats in persisted probability state).

The approved slice is Wave 1 only: no perks (wave 2), no active
effects/chems/radiation (wave 3), and no save-format change (wave 10
introduces v9; nothing here persists new save records).

Tracked work:

- #308 — GMST/AVIF preparation and `PreparedGmstCatalog`
- #309 — pure SPECIAL, skills, derived-stats, and leveling kernels
- #310 — player stats runtime and console surface

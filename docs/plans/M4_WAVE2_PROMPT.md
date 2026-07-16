# M4 wave 2 — NAVM/NAVI decode into a project-native nav graph

Request (2026-07-16): after closing #103 (merged as PR #126) and ticking it
in the epic #9 roadmap, continue M4 with the next unblocked task. Selected
issue: #111 — "Decode FO3 NAVM/NAVI into a project-native polygon
navigation graph".

Selection rationale:

- #111 is pure decode + pure graph construction with synthetic-fixture
  tests: fully executable by one executor on the wave branch.
- #112 (bevy_landmass spike) consumes #111's graph ("one synthetic
  project-native NAVM graph and one real prepared interior graph"), so it
  is sequenced after this wave.
- #104 (NifTools KF spike) needs Blender runs over a local licensed asset
  corpus and human-judged playback evidence; it runs as its own wave.

Current state confirmed from the code: `reader.rs` captures per-cell NAVM
records but only catalogues chunk signatures/lengths plus the raw payload
(`parse_navmesh`), `stage_navmeshes` writes the raw bytes and
`PreparedNavMeshSource` metadata, and NAVI is not captured at all. This
wave adds the missing decode and the backend-neutral graph.

Single-issue wave: one Sonnet executor directly on branch `m4-wave2`, per
AGENTS.md model routing.

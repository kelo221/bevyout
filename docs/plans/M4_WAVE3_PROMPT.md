# M4 wave 3 — bevy_landmass navigation backend spike

Request (2026-07-17): after merging PR #127 (#111 nav graph + #128 `tnm`
overlay) and ticking both in the epic #9 roadmap, continue with the mesh
solutions: pick the next items, search crates/Bevy solutions if necessary,
and choose one that can move a crude actor from A to B, preferably with
doors.

Selected issue: #112 — "[Spike] Evaluate bevy_landmass as the navigation
and local-avoidance engine".

Selection rationale:

- #112 is the epic's designated next step: #111's `PreparedNavGraph` is
  its declared input, and #113/#114/#115 are all sequenced behind the
  backend decision.
- Crate research re-verified 2026-07-17: bevy_landmass 0.12.0 (released
  2026-06-20, landmass 0.9.0) targets Bevy 0.19, matching this repo's
  `bevy = "0.19.0"`. Alternatives remain worse fits: oxidized_navigation
  generates navmeshes from collider geometry via Recast, which discards
  the authoritative decoded FO3 NAVM polygons; vleue_navigator (polyanya)
  accepts explicit meshes but has no built-in agent/steering/local
  avoidance/off-mesh-link layer, all of which landmass provides and #112
  requires.
- The user's acceptance bar — a crude actor moving A→B with a door — maps
  exactly onto #112's "required integration proof" (one agent on a real
  prepared interior graph plus one animation link exercised as a Fallout
  door). The full Fallout adapter (travel doors, flags/costs, repathing
  policy) stays #113; grounded physics movement stays #114.

Single-issue wave: one Sonnet executor directly on branch `m4-wave3`, per
AGENTS.md model routing.

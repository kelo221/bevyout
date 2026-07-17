# M4 wave 4 — kickoff prompt

Recorded from the orchestrating session, 2026-07-17, immediately after the
wave 3 PR (#129, issue #112) merged.

> https://github.com/kelo221/bevyout/issues/9 112 was just merged. take the
> next items regarding nav mesh. Most likely, many items can be taken at
> once. I am most interested in: Doors ie dynamic mesh (or however it is
> implemented) and intercell agents

## Orchestrator reading

- "Doors ie dynamic mesh (or however it is implemented)": in the adopted
  bevy_landmass backend the mesh itself stays static; doors are off-mesh
  links whose usability is gated by door state. That is #113's scope
  (travel-door links, blocked/locked policy, NAVI cross-mesh merges,
  repath).
- "Intercell agents": an agent whose route crosses a travel door must
  actually leave the cell and be present on the other side when that cell
  becomes active. #113's "cell-boundary handoff policy" wording stops at
  the door; the continuity mechanism is new scope and became sub-issue
  #134 under epic #9.
- "Many items can be taken at once": the wave takes #113 + #134. #114
  (grounded movement, collision, local avoidance) is deliberately left
  out — it rewrites the same agent-locomotion seam both wave issues touch
  and neither named interest depends on it; it is the natural next wave.

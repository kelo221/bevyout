# Dialogue Wave 3 prompt — Yarn host bridge

Expose the existing Bevyout authorities to Yarn without creating a second
game-state API.

- Register namespaced pure functions such as `bo_condition`, inventory/global
  queries, quest queries, and actor checks.
- Register deferred `bo_*` commands for action sets, quest stages, item
  movement, reference state, script events, and dialogue completion.
- Add task-backed completion for commands that must wait on camera, voice,
  animation, or scene work.
- Produce a deterministic registration/support report.

The bridge consumes M7's typed registry and deterministic adapter; it does not
receive arbitrary Bevy `World` access.

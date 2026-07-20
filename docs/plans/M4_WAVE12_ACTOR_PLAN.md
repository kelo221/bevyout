# M4 wave 12 — actor animation game-flow integration (#104, #106)

Sequential two-item wave under epic #9. Issue #104's compatibility matrix and
native-parser decision shipped in PR #183; its remaining work is repository
housekeeping. Issue #106 owns the runtime seam and is implemented directly on
the wave branch because catalog loading, actor projection, target binding,
playback, and console acceptance all converge on the same actor components.

**Execution model recommendation (Codex runtime): Luna X-High.** The work
crosses FO3 record semantics, prepared revisions, Bevy glTF animation targets,
cell residency, transition policy, and real-data acceptance. Codex executes
directly without subagents, per the repository's runtime guidance.

## Fixed feature list

### 1. Close the compatibility decision (#104)

- Treat PR #183's humanoid, Radroach, door, and weapon matrix as the completed
  gate; retain every measured failure and the decision not to trigger #105.
- Close #104 only after confirming the merged PR and its CI gates. Do not add a
  second converter or modify the external Nifty repository.

### 2. Preserve the FO3 weapon animation selector

- Decode `WEAP.DNAM`'s animation type into the prepared item catalog and map
  the authored FO3 values to their third-person prefixes (`h2h`, `1hm`, `2hm`,
  `1hp`, `2hr`, `2ha`, `2hh`, `2hl`, `1gt`, `1lm`, `1md`).
- Use the equipped canonical weapon's authored prefix for equip/unequip and
  weapon-idle selection. Unknown/missing values fall back explicitly to the
  movement set; they never silently assume a pistol.
- Bump the item-catalog revision because the serialized prepared shape changes.

### 3. Deterministic gameplay animation policy

- Define the presentation states `Idle`, `Walk`, `Run`, `TurnLeft`,
  `TurnRight`, `Equip`, and `Unequip` behind a narrow actor request API.
- Resolve canonical locomotion clips by source path and sequence metadata, not
  merely the first colliding GLB name. Humanoids respect male/female locomotion
  variants; creatures use their own mapped set.
- Cross-fade state changes, loop locomotion according to prepared loop mode,
  and return one-shot equip/unequip states to idle when complete.
- Missing requested clips use a deterministic idle fallback and retain a
  diagnostic. An animation set without a compatible idle remains inert rather
  than playing an unrelated clip.

### 4. Runtime catalog, skeleton, and target binding

- Load and hash/revision-validate the per-cell actor-animation catalog for the
  startup cell and in the existing background neighbor-preload task.
- Resolve each living actor by reference, base, and kind; require the prepared
  actor skeleton path to match the set before loading its clip pack.
- Bind the external pack hierarchy to the native actor hierarchy using the
  proven zoo name canonicalization and global-rest-pose retargeting. Reject
  missing required targets explicitly.
- Hold authored accumulation-root motion at the presentation boundary so the
  gameplay actor root remains the world-transform authority.
- Share clip assets/graphs by animation-set identity, while each actor owns its
  player, transitions, intent, and binding state.

### 5. Cell lifecycle and visible acceptance

- Only active-cell actors advance and retarget; hidden preloaded actors remain
  paused. Eviction/despawn must not leave stale player entities or bindings;
  reactivation resolves the new actor entity deterministically.
- Add `actoranim <reference> <state>` as a developer producer of the same
  request API future AI/package/combat slices consume. Extend `actorinspect`
  with set, state, clip, target-binding, loop/root-motion, and error data.
- Keep animation-zoo as an inspection tool. No gameplay system depends on its
  resources, UI, or playback policy.

## Tests first

1. Add Cucumber scenarios for FO3 weapon-type decoding/prefix mapping, clip
   resolution, missing-clip fallback, one-shot completion, and cell lifecycle
   decisions.
2. Add pure policy unit tests for aliases, sex-specific locomotion, creature
   selection, loop/root-motion rules, and transition outcomes.
3. Add minimal-App/World tests for catalog validation, actor mapping, target
   compatibility, shared graph ownership, request playback, inactive-cell
   pause, and stale-entity replacement.
4. Add console tests for `actoranim` and the animation block in
   `actorinspect`.

## Gates and real-data acceptance

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Reprepare Super-Duper Mart with the native actor animation backend using the
  shared configuration and a cache outside the repository checkout.
- Launch the normal `view` command, not `animation-zoo`, and use the agent
  bridge/console to drive idle, walk, run, turns, equip, and unequip on humanoid
  `00041600` (or another prepared humanoid).
- Prepare/view Vault 101's Radroach `0005443b` (or another proven creature) and
  drive idle, walk, run, and turns.
- Record bound/missing targets, selected source paths, transition state, loop
  behavior, reload/revisit behavior, and a bounded runtime frame probe.

## Non-goals

- Combat attacks, hit/death/reload breadth, AI path-following, or player-body
  animation.
- A second animation-event audio/script dispatcher. Text keys remain prepared
  metadata available to later authoritative consumers.
- Changes to animation-zoo UI, navmesh behavior, scene preparation outside the
  catalog fields required by this wave, or the external Nifty repository.

## Shipped amendments

- Real Super-Duper Mart assembly proved that requiring the union of every KF
  target rejects valid actors: Gamebryo `##` controller pins are clip metadata,
  not deform bones in the assembled appearance. Binding now validates the
  canonical idle deform targets first and validates each requested clip before
  playback; an incompatible request reports a diagnostic and falls back to the
  already-proven idle.
- The converter pack and actor appearance have separate rest poses. Retargeting
  therefore applies each animated source-global delta to the actor's
  root-relative rest pose, blends locally across state changes, and holds only
  accumulation-root translation. This removed the elevated/floating result
  without taking world-transform authority from gameplay.
- Fixed-tick/root-transform sampling briefly reports zero movement between
  updates. A 150 ms locomotion hold and run-speed hysteresis prevent that
  cadence from repeatedly selecting idle and producing an animation chain.
- Shared pack and graph handles are cached per animation-set identity and
  pruned when no resident catalog references that set. Failed pack loads become
  stable actor diagnostics instead of leaving actors in `Loading` forever.
- Real-data acceptance used Super-Duper Mart `00017f37`, female Raider
  `00041600` (67 targets; pistol prefix `1hp`), and Vault 101 Atrium `00024511`
  Radroach `0005443b` (48 targets). The Super-Duper Mart Protectron has no
  native pack in the prepared catalog, so it remains inert with an explicit
  diagnostic rather than borrowing humanoid or Radroach animation.
- A warm Super-Duper Mart probe with six animated humanoids measured three
  120-frame windows at 1920x1080 on Apple M5 Max/Metal: average frame times
  15.61, 15.22, and 15.08 ms; p95 17.22, 17.04, and 17.14 ms; maxima 27.13,
  17.84, and 19.73 ms. This is bounded steady-state evidence, not an isolated
  animation A/B benchmark. The cooled collision-cook canary was 383 ms on the
  final launch.
- Final gates passed: `cargo fmt --check`, strict all-target Clippy, 1,059 unit
  tests (one ignored), four architecture tests, six CLI-contract tests, one
  command smoke test, and all 435 Cucumber scenarios/2,124 steps.
- Manual acceptance is recorded in
  [M4_WAVE12_ACTOR_MANUAL.md](M4_WAVE12_ACTOR_MANUAL.md).

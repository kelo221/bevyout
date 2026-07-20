# M4 walking-actors wave — manual acceptance

## What this wave shipped (plain language)

Before this wave, **nothing in the game walked**. Two half-systems existed
side by side but never met: nav agents could route and move but had no body,
and projected NPCs had a body and animation but no navigation. This wave binds
them, so a real NPC now routes across the navmesh with its walk/run/turn
animations driven by how fast it is actually moving.

Three issues:

- **#188** — bind a projected actor to a nav agent; drive its animation from
  the agent's *achieved* motion (idle/walk/run/turn), with hysteresis so it
  does not flap at the walk/run boundary. The KCC stays the single movement
  authority — animation never moves the actor.
- **#199** — a pre-existing crash: Vault101a (`00024512`) panicked on launch
  because a neighbour cell's placements were parented to a root that had not
  spawned yet. Fixed so the cell opens. (This unblocked #186's test cell.)
- **#186** — an Activator "door" (the vault gear door) animated open in the
  world but navigation still treated it as a wall forever, because only
  `Door`/`Container`/`Corpse` records ever recorded an open state. Now *any*
  solid blocker carries an open state nav can observe.
- **#189** — hardening: four nav "checks that agreed with the bug" from the
  post-mortem are now real failures — the landmass-rejection guard has a test,
  walkable geometry inside a closed blocker is a hard prepare failure, the
  interior-polygon invariant no longer shares its primitive with the code it
  checks, and the four point-in-polygon copies are one module. This part has
  no viewer surface; it is checked via `prepare` output and `cargo test`.

---

## One-time setup

Prepare both test cells (once each; skip if `.bevyout/cache/scenes/<id>/scene.ron`
already exists and is current):

```
cargo run-dev -- prepare --cell 000151e3    # Megaton player house (walk demo)
cargo run-dev -- prepare --cell 00024512    # Vault 101a (gear-door + #199 crash cell)
```

Expected on each: prepare completes with `nav doors: … unreported interior
polygons 0` (the #189 gate is now fatal, so a non-zero count would abort here).

---

## Part 1 — #189, deterministic (no viewer)

1. **Run the suite.**
   ```
   cargo test
   ```
   Expected: green. The named guards live in `issue_199_tests` (preload),
   `landmass_acceptance_tests` + `interior_polygon_gate_tests` (navmesh), and
   the containment-epsilon pin in `crates/bevyout-core/src/geometry.rs`.

2. **Prove the interior-polygon gate is fatal, not cosmetic.** Both cells
   above already prepare green with `unreported interior polygons 0`. The gate
   only escapes via the documented `BEVYOUT_NAV_ALLOW_INTERIOR_POLYGONS` env
   var — leave it unset; a stale cache carrying an interior polygon now aborts
   `prepare` instead of printing a number and continuing.

---

## Part 2 — #188 walking actor (Megaton, `000151e3`)

Launch with the agent bridge:

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron \
    --agent-bridge --agent-port 15702 --trace-seconds 30
```

Drive it (each is a `bevyout.console.exec` `{"line": "..."}` over BRP, or type
the line into the in-viewer console):

1. `tna bind 0x0008f6ae`
   → `nav agent 0 bound to actor 0008f6ae at (…)`. This puts the Mister Handy
   creature onto the nav roster with a real body.
2. `tna status`
   → the bound agent reports `grounded=true` (on the navmesh).
3. `tna goto 0x0008f6ae` *(or `tna goto <x> <y> <z>` to a reachable point)*
   → `status=moving` then `status=reached`. In the window the Mister Handy
   travels the route; the trace log shows
   `nav actor locomotion … idle -> run -> idle` — the animation state following
   the achieved motion, not a fixed clip.
4. **One-authority check:** the actor's logged position sequence is the KCC's;
   playing/stopping clips does not change it. (Covered deterministically by
   `agent_transform_is_bit_identical_with_and_without_clip_playback`.)
5. **Regression:** `tna spawn` (a bare debug capsule) still routes with
   `tna goto`; `actorinspect 0x0008f6ae` and `actoranim 0x0008f6ae walk` still
   work. The debug-capsule harness is untouched.

Expected: Idle↔Walk↔Run transitions happen at speed thresholds without
flapping when the agent hovers near the walk/run boundary.

---

## Part 3 — #199 + #186 gear door (Vault 101a, `00024512`)

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron \
    --agent-bridge --agent-port 15702 --trace-seconds 30
```

1. **#199:** the cell now reaches a live session with **no panic** during
   spawn (before this wave it crashed with `Entity not yet spawned`). The
   neighbour cell logs `preload ready … (616 placements)`.

2. **#186 — the gear door blocks nav until activated.** `VaultGearDoor` is
   reference `00024710` (an Activator, authored `openable=false`, 41/41
   blocking associations).

   a. Put an agent on one side and target the other (e.g. spawn near z≈−107,
      goto a point at z≈−35 across the door):
      ```
      tna bind 0x0005443b          # or: tna spawn, then player.setpos to side A
      tna goto <x> <y> <z>         # a point on the far side of the gear door
      ```
      **Door closed → `status=unreachable`** (the agent stays put; nav prices
      the door's polygons at infinity).

   b. Open it through the interaction boundary:
      ```
      activate 00024710
      ```
      → `activator 00024710 opened` (before this wave: `not_a_door`). The gear
      door plays its opening animation.

   c. Re-issue the route:
      ```
      tna goto <x> <y> <z>
      ```
      **Door open → `status=moving`**, and the agent walks straight through the
      gear-door zone to the far side.

   d. Close and re-check:
      ```
      activate 00024710            # → activator 00024710 closed
      tna goto <x> <y> <z>         # → status=unreachable again
      ```

Expected: the route result flips purely on the blocker's open state, in both
directions, with no FormID- or coordinate-specific logic in the fix.

---

## Gate summary the wave must satisfy

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`
  all green on the merged branch.
- `prepare` green on `000151e3` and `00024512` with `unreported interior
  polygons 0`.
- Parts 2 and 3 above pass on real prepared data.

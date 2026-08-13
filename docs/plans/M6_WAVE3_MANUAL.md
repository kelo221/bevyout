# M6 wave 3 — actor residency and NAVM policy manual

This manual exercises the policy contracts shipped by W3-A and W3-B, and
records the W3-C runtime acceptance (#278) run after gate #10 closed. It
proves one gameplay actor binds, crosses a cell border, survives source
eviction/reload, and retains one canonical state — not the downstream gate
#87 or gate #14.

## 1. Run the executable policy scenarios

From the repository root, run:

```text
cargo test --test features -- --input features/m6_wave3_policy.feature --concurrency 1
```

Expected: all five M6 wave 3 scenarios pass. The scenarios show:

- one actor reference and one canonical `HolderId::Actor` through bind and
  handoff;
- stale-generation and duplicate-projection rejection;
- unload/restore decisions that keep the canonical actor state location;
- cross-cell NAVM links appearing only when both sides are valid and
  navigation-ready; and
- eviction removing old links before a reload accepts a new-generation portal.

## 2. Run the focused policy tests

```text
cargo test --lib actor_residency
cargo test --lib resident_
```

Expected: the actor policy tests and resident-topology tests pass. These are
pure decisions over caller-owned observations; they do not create an ECS
actor store or mutate the save/item authorities.

## 3. Runtime acceptance (#278, after #10 closed)

Prepare the frozen 14-cell W6-B route (needed once; already done in the
`m6-wave3-w3c` cache, add `--force` to redo):

```text
cargo run --release -- prepare 00000c49 00000c4a 00000c4b 00000c4c 000010d5 \
    00001245 00000c67 00000c68 00000c69 00000c6a 00000c6b 000010eb 000010ec \
    000010ed --jobs 8 --force
```

(`cargo run-dev`/`check-dev` are broken repo-wide right now — issue #300,
unrelated bevy_dylib/crates.io mismatch — use `--release` instead.)

Launch the bridge on the start cell and drive it with raw BRP `curl`
(`bevyout.console.exec`) or the bevyout MCP server if connected:

```text
cargo run --release -- view --manifest .bevyout/cache/scenes/00000c49/scene.ron \
    --agent-bridge --agent-port 15737 --trace-seconds 600
```

The tracked actor is `CrEyebotEnclave`, reference `000638e8` (407784), base
`0001cf73` (118643), prepared in cell `00000c67` (grid 5,-6).

1. `worldstream trace 1`, then `worldstream cells` — expect grid (5,-6) /
   `00000c67` to report `actors >= 1`.
2. `actorresidency` — expect `000638e8` owned by `00000c67` at its current
   generation.
3. `tna bind 000638e8` — binds it to the resident exterior NAVM; expect
   `actorresidency` to now show `nav_bound: true`.
4. Force a cross-cell handoff: `prid 000638e8` then `setpos x 285` (crosses
   grid (5,-6) into (4,-6)/cell `00000c68`). Expect `actorresidency` to show
   the same reference now owned by `00000c68`, `handoffs` incremented,
   `nav_bound: true` (a fall-guard settle-and-rebind cycle at this exact
   forced coordinate is expected and harmless — see #304). A live
   `scene_snapshot` query for `reference_form_id == 407784` must return
   exactly one entity throughout — this is the core no-duplicate guarantee.
5. Force eviction: move the player 3+ cells away, e.g. `tp 120 175 175`.
   Expect `worldstream cells` to drop `00000c67`/`00000c68` from the
   resident window and `actorresidency` to no longer list `000638e8`
   (`unloads` incremented).
6. Restore: move back, e.g. `tp 300 165 340`. Expect `actorresidency` to
   list `000638e8` again (`restores` incremented, `nav_bound: true`), owned
   by its originally-prepared cell `00000c67` — restore respawns from the
   prepared source, it does not persist the console-forced mid-air
   teleport.
7. `worldstream summary` — expect `stale_completions`, `cancellations`,
   `invalid_unload_count` at 0. `failed` may be nonzero near the edge of
   this 14/15-cell frozen route (neighbouring grid cells outside the
   prepared set report `Failed` lifecycle by design) — that is a route
   boundary artifact, not a residency defect.

Package start (`runpackage`) is expected to fail with `unresolved_point` for
this actor until #301 lands, and to fail with `catalog_unreadable` unless an
interior cell is also present in the same prepare run until #302 lands — both
are tracked follow-ups, not part of this manual's pass/fail.

`capture_viewport` returns black frames when the window is occluded on macOS
(confirmed both by the implementer and the independent verifier); the console
transcript above is the load-bearing evidence, not screenshots.

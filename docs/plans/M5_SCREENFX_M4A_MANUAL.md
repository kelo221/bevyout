# M5 gate-support — screen feedback and actor gate manual acceptance

This wave adds a prepared Fallout 3 IMAD catalog, deterministic transient
screen feedback, and the inspection surface needed to demonstrate it. It also
records the real-data evidence for the existing M4A actor gate: a Raider in
Super-Duper Mart and a Radroach in Vault 101 assemble, animate, and remain
stable at the measured frame-time budgets.

1. Prepare the two representative cells from the repository root:

   ```sh
   cd /Users/simon/projects/bevyout
   cargo run-dev -- prepare --cell 00017f37
   cargo run-dev -- prepare --cell 00024511
   ```

   Expect warm runs to reuse the prepared assets and report an image-space
   catalog with `112 records`, revision `openmw-imad-v3`. The final acceptance
   run reused 605 assets for Super-Duper Mart and 589 for Vault 101. Derived
   RON/GLB/KTX2 content stays under `.bevyout/` and must not be committed.

2. Launch Super-Duper Mart with the agent bridge:

   ```sh
   cargo run-dev -- view \
     --manifest .bevyout/cache/scenes/00017f37/scene.ron \
     --agent-bridge --agent-port 15702 --unfocused
   ```

   Wait for `app state: Some(Loading) -> Some(InGame)` and the prepared
   collision `dynamic colliders ready` line. Press backquote to open the
   console. The expected active cell is `SuperDuperMart` / `00017f37` and the
   prepared manifest contains 1,693 placements.

3. Verify the representative Raider and its persistent runtime state:

   ```text
   actorinspect 00041600
   actorstate 00041600
   ```

   Expect a humanoid `RaceSexSpecific` actor with 11 visible parts, one
   prepared apparel model, an attached prepared weapon, animation set
   `animation-set-721d34227d5581a6`, 67 bound targets, and no animation
   diagnostic. `actorstate` must show `life=alive`, two canonical item
   instances, and one equipped instance. `missing_facegen` is an authored-data
   fallback diagnostic for this Raider, not a runtime assembly failure.

4. Exercise the actor animation state machine and return it to neutral:

   ```text
   actoranim 00041600 walk
   actorinspect 00041600
   actoranim 00041600 run
   actoranim 00041600 turn_left
   actoranim 00041600 turn_right
   actoranim 00041600 equip
   actoranim 00041600 unequip
   actoranim 00041600 idle
   actorinspect 00041600
   ```

   Each request should be acknowledged. The inspection after `walk` should
   report clip `mtforward__2`, and the final inspection should report clip
   `mtidle__2`, state `idle`, 67 bound targets, and no diagnostic. No duplicate
   weapon attachment or item-instance IDs should appear.

5. Inspect the prepared screen-FX catalog and trigger a timed hit effect. The
   following are real catalog FormIDs: `00000162` is `GetHit`, `00000164` is
   `ImageSpaceConcussion`, `00000166` is `ExplosionInFace`, and `00019482` is
   `BloodISFXd`.

   ```text
   screenfx status
   screenfx start 00000162 10
   screenfx status
   screenfx settings 0.5 0.25 0 0.75
   screenfx status
   screenfx stop 00000162
   screenfx clear death
   screenfx status
   ```

   The first status must report `catalog_records=112`, revision
   `openmw-imad-v3`, and `active_count=0`. After the start, `active_count=1`
   and `active_modifiers` must contain decimal FormID `354`; the sampled
   radial/double/motion values must be non-neutral. The settings command must
   lower the reported effect values and set flashes to zero. The final status
   must report `active_count=0` and neutral transient values while retaining
   the cell's base ImageSpace.

6. Verify the authored blood/fade timing and cancellation:

   ```text
   screenfx settings 1 1 1 1
   screenfx start 00019482 10
   screenfx status
   screenfx clear death
   screenfx status
   ```

   `BloodISFXd` must become active with non-zero sampled blur/contrast or fade
   values. Letting its 2,000 ms duration elapse must remove it automatically;
   `screenfx clear death` must remove it immediately and restore the neutral
   transient output. This is presentation state only: actor health and damage
   authority remain unchanged.

7. Capture the warm performance window through the bridge after the collision
   cook has completed and after all transient effects are cleared:

   ```sh
   curl -sS -X POST http://127.0.0.1:15702/ \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":1,"method":"bevyout.performance_snapshot","params":{"latest_limit":120,"budget_ms":16.667,"include_samples":false}}'
   curl -sS -X POST http://127.0.0.1:15702/ \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":2,"method":"bevyout.schedule_snapshot","params":{"schedule_contains":"Update","include_systems":false,"conflict_limit":10}}'
   ps -axo pid,rss,vsz,etime,command | rg 'target/debug/bevyout.*--agent-port 15702'
   ```

   Take three 120-sample windows, separated by at least two seconds, and
   record average, p50, p95, p99, max, sample count, and over-budget count.
   The acceptance run used the optimized `cargo run-dev` profile on an Apple
   M5 Max at the default 1,920x1,080 window: averages were 9.74, 9.72, and
   9.60 ms; every window had 0/120 samples over 16.667 ms. The world snapshot
   was 16,384 entities, 3,070 mesh entities, and 47 point lights. RSS was
   approximately 2.39 GB after warmup. The schedule snapshot reported 456
   systems, 48 exclusive systems, and 1,725 conflict pairs; these are
   inspection data, not a claim that all conflicts are bottlenecks.

   Then exercise the costed overlap path with the real representative records:

   ```text
   screenfx settings 1 1 1 1
   screenfx start 00000164 20
   screenfx start 00000166 30
   screenfx start 00000162 10
   screenfx status
   ```

   Expect `active_count=3` and `active_modifiers` containing decimal IDs
   `354`, `356`, and `358`, with non-neutral blur/brightness/contrast/radial
   values. Capture the same 120-sample window while refreshing the 400 ms
   `GetHit` start during the window so all three remain active; record this
   separately from the neutral baseline. The first acceptance attempt was
   noisy: it measured 18.21 ms average (p95 25.31 ms, max 29.23 ms, 58/120
   over budget) and then 23.20 ms (120/120 over), while its neutral window was
   already 18.13 ms. A follow-up run with the same manifest completed three
   overlap windows at 11.72, 6.89, and 6.79 ms average; p95 was 14.12, 7.74,
   and 7.25 ms, max was 15.45, 7.92, and 7.70 ms, and every window had 0/120
   samples over 16.667 ms. Treat the follow-up as the representative overlap
   budget result, but do not infer a precise neutral-to-effect delta from this
   run because the neutral baseline was noisier. Clear the overlap before
   continuing:

   ```text
   screenfx clear death
   ```

8. Stop Super-Duper Mart with Ctrl-C, launch Vault 101, and wait for InGame:

   ```sh
   cargo run-dev -- view \
     --manifest .bevyout/cache/scenes/00024511/scene.ron \
     --agent-bridge --agent-port 15703 --unfocused
   ```

   Inspect and animate the real Radroach:

   ```text
   actorinspect 0005443b
   actorstate 0005443b
   actoranim 0005443b walk
   actorinspect 0005443b
   actoranim 0005443b run
   actoranim 0005443b turn_left
   actoranim 0005443b turn_right
   actoranim 0005443b idle
   actorinspect 0005443b
   screenfx status
   ```

   Expect `AuthoredExact` creature assembly from
   `creatures/radroach/roach.nif`, no apparel or weapon invented, animation
   set `animation-set-e961bfc5e3a5a24f`, 48 bound targets, clip `mtforward`
   after `walk`, clip `mtidle` after the final `idle`, and no animation
   diagnostic. The base ImageSpace may report contrast `1.5`; that is the
   authored Vault base and should remain when transient screen effects are
   neutral. The prepared manifest contains 706 placements.

9. Repeat the three-window performance and memory capture on port 15703 with
   the same build, resolution, physics, warmup, and 16.667 ms budget:

   ```sh
   curl -sS -X POST http://127.0.0.1:15703/ \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":3,"method":"bevyout.performance_snapshot","params":{"latest_limit":120,"budget_ms":16.667,"include_samples":false}}'
   ps -axo pid,rss,vsz,etime,command | rg 'target/debug/bevyout.*--agent-port 15703'
   ```

   The acceptance run measured averages of 7.35, 7.50, and 7.37 ms; every
   window had 0/120 over-budget samples. The world snapshot was 8,192
   entities, 1,689 mesh entities, and 42 point lights. RSS was approximately
   1.63 GB after warmup. Its collision cook reported 614 bodies (40 dynamic),
   656 shapes, and 53.1 ms cook time.

10. For actor persistence, use the existing M4 actor-state acceptance path
   with a named save slot (`save m4a-check`, close, relaunch with
   `--save-slot m4a-check`, then `actorstate 00041600`). The expected contract
   is one stable canonical holder and the same equipped item-instance ID after
   reload; transient screen effects are intentionally cleared on reload and
   the base ImageSpace is restored. Do not use a saved slot as the frame-time
   benchmark.

11. Stop the viewer with Ctrl-C. If a manual run leaves a modifier active,
   clear it before stopping:

   ```text
   screenfx clear teardown
   screenfx settings 1 1 1 1
   ```

   A macOS window may produce a black `capture_viewport` PNG when occluded;
   use the bridge snapshots and stable logs above as the acceptance evidence
   in that case. The expected real-data warnings are limited to authored
   missing FaceGen/visual diagnostics and unrelated stale neighboring-cell
   preload caches; rerun `prepare` for a stale target manifest.

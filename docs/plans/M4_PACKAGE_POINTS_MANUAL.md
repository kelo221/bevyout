# M4 package-points wave — manual acceptance (#213)

## What this wave shipped

Before this wave, every Patrol (`near-linked-reference`, PLDT type 6) and
Sandbox (`near-editor-location`, PLDT type 3) NPC package resolved to nothing:
`showpackages` reported `near-linked-reference location has no linked
reference` and `runpackage` refused to start. The #195 resolver already knew
how to turn both types into world points — the runtime just never filled the
two inputs it read.

This wave:

- Decodes the REFR/ACHR `XLKR` subrecord (a reference's linked reference) into
  a new `PreparedPlacement.linked_reference_form_id`, bumping the prepare
  revision.
- Builds the AI resolution context from the full manifest placement list, not
  just spawned entities, so asset-less patrol markers are visible — and fixed
  a related real-data gap found during acceptance: those markers (the game's
  built-in `XMarkerHeading`) were previously dropped from the manifest
  entirely by the "skip non-rendering editor marker" path, not merely
  unspawned. They are now kept whenever they are part of a linked-reference
  chain.
- Walks a Patrol package's linked-reference chain into an ordered waypoint
  list (cycle-safe, capped), so `runpackage` drives the actor through every
  marker in authored order instead of a single point.
- Wires `NearEditorLocation` to the actor's own authored placement position,
  so Sandbox roams around where it was actually placed.

No new console command is needed: `showpackages <ref>` shows the resolved
point and source; `runpackage <ref> status` shows `marker=<i>/<n>` advancing.

## One-time setup (prepare revision bumped v5 → v6)

Both cells below need re-preparing once for the new `linked_reference_form_id`
field:

```sh
cargo run-dev -- prepare --cell 00017f37   # SuperDuperMart (Patrol demo)
cargo run-dev -- prepare --cell 00024511   # Vault 101 Atrium (Sandbox demo)
```

Expect `prepared SuperDuperMart (00017f37) (1693 placements, ...)` — placement
count went **up** versus a pre-wave cache (1670 → 1693): the patrol markers
that used to be dropped entirely now survive as asset-less placements.

## Part 1 — Patrol: SuperDuperMart, NPC `00041600` (LvlRaiderGun)

Real FormIDs confirmed against the prepared catalog during acceptance. NPC
`0005cf10` — the FormID named in the original issue — genuinely has **no**
authored `XLKR` in this data (a script-driven case, out of scope); `00041600`
is a sibling raider in the same cell that does, and is enabled by default so
it is actually spawned. Its chain: `00041600` → marker `00041601` → marker
`00041602` (terminal, 2 markers).

1. Launch the viewer:
   ```sh
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron \
       --agent-bridge --agent-port 15702
   ```
2. `showpackages 0002f6e2` (the actor's *base* FormID works even before it's
   bound). Expect:
   - Package #1 `DefaultPatrolWeaponDrawn` (Patrol, type 13) **selected**.
   - `... location resolved via linked-reference to (17.81,96.46,-89.05)
     entity=none radius=0.0` — this is marker `00041601`'s real translation;
     previously this line read `unresolved: near-linked-reference location
     has no linked reference`.
3. `tna bind 0 00041600` → `nav agent 0 bound to actor 00041600 at
   (17.95, 96.46, -92.85)`.
4. `runpackage 00041600` → `started patrol package 00023619 -> target
   (17.81,96.46,-89.05)` (marker `00041601`).
5. Wait a few seconds, then `runpackage 00041600 status`. Expect
   `marker=1/2 target=(19.04,96.46,-89.83)` — the actor has reached marker
   `00041601` and moved on to marker `00041602` in order. `00041600.getpos`
   shows the actor's live position has actually moved off its spawn point,
   between the two markers.
6. Optional: `cam follow 00041600` to watch it walk the route.
7. `runpackage 00041600 stop` when done.

## Part 2 — Sandbox: Vault 101 Atrium, radroach `0005443b` (CG04Radroach)

No `NPC_`-kind actor in either prepared cell happens to have a Sandbox
`near-editor-location` package as its *first* (highest-priority) package — the
patrol/travel packages authored ahead of it always win selection, which is
correct game behavior, not a bug. This radroach creature does have Sandbox
first, and the resolver treats NPC/creature actors identically, so it is the
clean real-data example for this location type.

1. Launch the viewer on the second cell:
   ```sh
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00024511/scene.ron \
       --agent-bridge --agent-port 15702
   ```
2. `showpackages 000522db` (the radroach's base FormID). Expect
   `DefaultSandbox…`-style `CG04RadroachSandbox1024` (Sandbox, type 12)
   selected, and `... location resolved via editor-location to
   (-47.64,120.69,12.37) ... radius=1024.0` — exactly the radroach's own
   authored placement translation. Previously this required a live spawned
   entity at that exact position; now it comes from the manifest regardless.
3. `tna bind 0 0005443b`, then `runpackage 0005443b` → `started sandbox
   package 0002e707 -> roam center (-47.64,120.69,12.37) radius 1024.0` — the
   roam center matches the resolved editor location exactly.
4. `runpackage 0005443b status` — see the known limitation below before
   expecting a clean, bounded wander here.

## Known limitation found during acceptance (pre-existing, not introduced by
this wave)

`PackageLocation.radius`/`PackageTarget.count_or_distance` are never scaled by
`FO3_SCALE` (1/70) the way positions are (`viewer::ai::resolution::
resolve_location`/`resolve_target`, unchanged since #195). A package authored
with `radius: 1024` (a common, sane ~14.6 m radius in Bethesda's native
units) is read back as **1024 metres**, so `runpackage`'s Sandbox roam in
step 4 above will very likely route outside the small interior cell and
report `phase=failed` rather than a smooth bounded wander. This reproduces
exactly the same way every time (`start_wander_package`'s roam seed is
deterministic), is orthogonal to #213's own scope (editor-location/linked-
reference *resolution*, which is verified correct above — the roam **center**
matches exactly), and is recommended as its own fast follow-up issue (scale
both radius/distance fields by `FO3_SCALE` in `resolve_location`/
`resolve_target`).

## Gates run before this manual

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo
test`, `cargo test-dev` all green; `cargo run-dev -- prepare --cell 00017f37`
and `--cell 00024511` both succeed with `prepare-v6-...-linked-ref`.

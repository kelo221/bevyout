# M4 package-points wave — plan (#213)

**Execution model recommendation:** **Sonnet** (Claude runtime), one executor,
sequential on the `m4-package-points` branch. Orchestrator (Opus) evaluates.

## The actual gap (investigated at plan time)

The #195 resolution layer (`src/viewer/ai/resolution.rs`) *already* resolves
every location/target type, **including** type 3 `NearEditorLocation` and type
6 `NearLinkedReference`. It just reads two `ResolutionContext` fields that the
runtime adapter never fills:

- `actor_editor_location` (type 3, Sandbox)
- `linked_reference` (type 6, Patrol)

`build_resolution_context` (`src/viewer/console/ai_package_commands.rs:1123`)
builds the context purely from spawned `PlacementRoot` entities and leaves both
fields `None` via `..default()`. So the prepared PACK catalog is fine; the two
missing pieces are **placement-side data**, not PACK subrecords:

1. **Editor location** = the actor's own authored spawn point = its
   `PreparedPlacement.translation`. **Already in the manifest — zero decode**,
   pure wiring.
2. **Linked reference** = the actor's REFR linked-reference FormID (currently
   dropped into `ReferenceRecord.ignored_subrecords`). Needs decode + a new
   `PreparedPlacement` field + a **prepare-revision bump** + threading.

### Critical constraint found at plan time

Asset-less placements (XMarkers / IdleMarkers — patrol markers have no GLB) are
**skipped at spawn** (`scene.rs:463`), so they are never `PlacementRoot`
entities. They *do* survive `prepare` into `LoadedSceneManifest.0.placements`
(144 asset-less placements in the cached `0001a273/scene.ron`). Therefore the
Patrol marker chain-walk and the editor-location lookup **must read the
manifest placements**, not only spawned entities. Building the resolution
context from the manifest (folding in markers the spawn pass skipped) is the
core design change.

### Genuine unknown → empirical discovery first

The exact FO3 REFR **linked-reference subrecord signature and byte layout** is
not assumed here. **Task F1 begins by discovering it against real data**
(`Fallout3.esm` at the configured Steam path): dump the ignored REFR subrecord
signatures for the SDM NPC `0005cf10` and its patrol markers, cross-check the
tag/offset against xEdit's FO3 REFR definition, *then* decode. Candidate tag is
`XLKR`; confirm before decoding. Do not hard-code a layout you have not seen in
the bytes.

## Features (fix the list → tests → implement, in order)

### F1 — Decode the REFR linked reference (prepare slice)
- **Discover** the linked-ref subrecord tag/layout empirically (above).
- In `src/vsa/openmw_esm4/records.rs` REFR parser: add
  `ReferenceRecord.linked_reference_form_id: Option<u32>` (resolver-adjusted,
  `!= 0` filtered), and move the discovered signature out of the
  `ignored_signatures(..)` supported list.
- **Unit test** (openmw_esm4 tests): a synthetic REFR carrying the subrecord
  decodes to the expected FormID; absent → `None`; a zero ref → `None`.

### F2 — Thread it through the prepared placement (+ revision bump)
- Add `PreparedPlacement.linked_reference_form_id: Option<u32>`
  (`#[serde(default)]`) in `crates/bevyout-core/src/manifest.rs`.
- Populate it in `src/vsa/prepare/placements.rs` from the decoded record.
- **Bump `CURRENT_PREPARE_REVISION`** (`src/vsa/manifest/mod.rs`) — the
  serialized placement shape changed; a serde-defaulted field is exactly the
  AGENTS.md "prepared asset revisions" trap. Add a suffix noting linked-refs.
- **Test**: a prepared placement round-trips the field; the revision constant
  is the new value.

### F3 — Build the resolution context from the manifest (runtime wiring)
- Extend `build_resolution_context` to fold in **manifest placements**
  (`LoadedSceneManifest.0.placements`) so meshless markers are present as
  `ResolvedReference`s, and to carry each reference's linked ref.
  - Add `linked_reference: Option<u32>` to `ResolvedReference` in
    `resolution.rs` and populate it (spawned entities keep live positions;
    manifest-only markers use the manifest translation).
  - Set `actor_editor_location = Some(actor placement translation)` for the
    actor whose package is being resolved.
  - Set `linked_reference = <actor placement's linked_reference_form_id>`.
- **Test**: with an actor placement that has an editor location + a linked ref,
  the built context exposes both; a type-3 location and a type-6 location now
  resolve through it (unit test against the pure resolver + a bare-`World`
  context-builder test).

### F4 — Patrol marker chain-walk (runtime)
- The Patrol branch of `runpackage` currently builds a **single** waypoint
  (`ai_package_commands.rs:560-563`). When the selected family is
  `PackageFamily::Patrol`, build the **ordered marker list** by following the
  linked-reference chain from the actor's `linked_reference`, resolving each
  marker to a world point and appending a `Waypoint`, with **cycle detection**
  (stop on revisit or a `None` link; cap length defensively).
- Put the chain-walk in a **pure** helper (std/serde-only, e.g.
  `resolution::linked_reference_chain(context, start) -> Vec<ResolvedPoint>`)
  so it is unit-testable via `tests/features.rs`.
- Non-patrol families keep the existing single-point behavior.
- **Unit tests**: a 3-marker cycle yields 3 ordered waypoints and terminates; a
  single marker yields one; a broken/missing link terminates cleanly (no
  infinite loop, no panic).

### F5 — Cucumber feature + manual visibility
- `features/ai_package_points.feature`: (a) a type-3 package resolves to the
  authored editor location; (b) a type-6 patrol builds N ordered waypoints from
  a linked-ref marker chain. Steps appended to the `tests/features.rs` merge
  seam (World fields at end of struct, delimited step section at end of file).
  `fail_on_skipped()` — every scenario line needs a step.
- No new console surface needed: `showpackages <ref>` already prints the
  resolved point/source, and `runpackage <ref> status` already prints
  `marker=<i>/<n>` — both are the human-visible proof (AGENTS.md "human must
  see it"). Drive them in the manual.
- `docs/plans/M4_PACKAGE_POINTS_MANUAL.md`: plain-language summary + numbered
  steps — one-time `prepare --cell 0001a273` (rebuild for the revision bump),
  `view` the cell, `tna bind <patrol-npc>` / `player.setpos` onto the mesh,
  `showpackages <npc>` (now resolved, not "no linked reference"),
  `runpackage <npc>` → Patrol, `runpackage <npc> status` showing `marker=`
  advancing, cinema follow to watch it walk the markers in order; then a
  Sandbox NPC roaming within its editor-location radius. Use real FormIDs from
  the prepared catalog (the executor confirms the NPC ref during acceptance —
  the issue's `0005cf10` is the seed).

## Gates
`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`,
and a real-data `cargo run-dev -- prepare --cell 0001a273` + viewer acceptance
(the manual script). Commit in logical increments as each feature goes green.

## Out of scope (do not expand the wave)
Autonomous binding/routing from a gameplay system (that is #218); the ECS agent
roster (#215); persistence of nav runtime state (#217). This wave only makes
Patrol/Sandbox *resolve and drive* under the existing `runpackage` command.

## Shipped amendments

- **Cell identity correction.** The prompt's `0005cf10`/"SuperDuperMart"
  reference is cell `00017f37`, not `0001a273` (an unrelated Metro cell found
  during a false-start byte scan before the real cell was confirmed via
  `scene.ron`'s `editor_id`). All real FormIDs in the manual are re-verified
  against `00017f37`.
- **`0005cf10` itself has no static `XLKR`.** Confirmed against real bytes:
  this specific actor's linked reference is set by a quest script at runtime
  (out of scope — no script VM here), not authored statically. A sibling
  raider in the same cell, `00041600`, does carry a static chain and is used
  for the manual/acceptance instead. `0005cf10` still correctly reports the
  same deterministic "no linked reference" diagnostic as before — that part
  was never wrong, just unresolvable for this one actor by design.
- **Real gap beyond the plan's premise: patrol markers were dropped from the
  manifest entirely, not merely unspawned.** The plan assumed every
  asset-less placement survives into `LoadedSceneManifest.0.placements` (based
  on a different cell's stale placement count). In fact FO3 patrol markers are
  plain `ReferenceKind::Object` placements of the engine's `XMarkerHeading`
  base (`markerxheading.nif`), and `prepare::placements`'s "skip non-rendering
  editor marker" branch dropped them **before any placement was ever
  created** for a plain Object-kind reference. `build_resolution_context`'s
  manifest fold-in (F3) could never have found them no matter how it was
  written. Fixed in `stage_placements`: a marker that is a linked-reference
  link source or link target keeps its placement now (`prepare-v6` bump
  covers this too, since it changes the manifest's *placement set*, not just
  a field). See the F1 commit's `editor_marker_needs_placement` for the pure,
  unit-tested decision.
- **Follow-up filed as #222 (not fixed here, out of scope):** `PackageLocation.radius`
  / `PackageTarget.count_or_distance` are never scaled by `FO3_SCALE` in
  `viewer::ai::resolution` (pre-existing since #195/#198, unrelated to this
  issue's editor-location/linked-reference resolution work). Found while
  acceptance-testing the Sandbox roam live: a `radius: 1024` package (a sane
  ~14.6 m radius in Bethesda's native units) is read back as 1024 *metres*,
  so `runpackage`'s Sandbox family routes outside the interior cell and
  reports `phase=failed` rather than a bounded wander. The roam *center*
  (this issue's own scope) is verified correct regardless. Recommended fix:
  multiply both fields by `FO3_SCALE` in `resolve_location`/`resolve_target`.
- **Real-data acceptance (cinema/console, both cells re-prepared to
  `prepare-v6-...-linked-ref`):**
  - SuperDuperMart (`00017f37`) `00041600`: `showpackages` resolved
    `linked-reference` to marker `00041601`'s real position
    `(17.81,96.46,-89.05)`; nav-bound + `runpackage` walked marker 0→1,
    landing on marker `00041602`'s real position `(19.04,96.46,-89.83)`,
    confirmed via both `runpackage status` (`marker=1/2`) and the actor's own
    live `getpos`.
  - Vault 101 Atrium (`00024511`) radroach `0005443b`: `showpackages` resolved
    `editor-location` to the actor's own authored placement position exactly;
    `runpackage`'s Sandbox roam center matched the same point exactly (roam
    radius itself hits the pre-existing radius-scale gap above).

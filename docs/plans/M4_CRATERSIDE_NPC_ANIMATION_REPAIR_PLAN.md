# M4 Craterside NPC animation repair — execution plan

Status: approved implementation plan. This document is the complete handoff for
a fresh execution session. Do not reopen product/design decisions unless current
repository evidence directly contradicts a fact recorded here.

## Execution routing

- **Orchestrator:** Luna Max.
- **Executors:** Luna Max `bevy-engineer` subagents, one per issue, in isolated
  git worktrees.
- **Orchestrator responsibilities:** refresh current repository/GitHub state,
  create and assign issues, create the integration branch/worktrees, preserve
  this plan, dispatch briefs, merge in the order below, resolve shared seams,
  review every diff, run all gates, perform real-data acceptance, write the
  manual, push, open the PR to `master`, and verify hosted checks.
- **Executor responsibilities:** tests first, then owned implementation, focused
  checks, a commit, and a concise evidence report. Executors are not alone in
  the repository: they must not revert unrelated or concurrent changes and
  must adapt to already-landed integration changes.
- **Branch:** create `m4-craterside-npc-animation` from current `master` after
  confirming a clean worktree.
- **Epic:** M4 epic #9. Create four assigned sub-issues with appropriate
  `area/*`, `bug`/`enhancement`, priority, and milestone labels. Link each via
  the sub-issue API and add them to the epic checklist.
- **Documentation:** create a matching kickoff prompt at wave start and write
  `M4_CRATERSIDE_NPC_ANIMATION_REPAIR_MANUAL.md` before the PR. Acceptance
  amendments append to this plan under `Shipped amendments`; do not rewrite
  the approved design.

### Agent and merge order

1. Run **Lane A (render/cache)** and **Lane B (package repair)** concurrently;
   their owned production files do not overlap.
2. Merge and review A and B. The orchestrator resolves any collision in
   `tests/features.rs`; executors append delimited World fields and step blocks
   at the end rather than reorganizing that shared file.
3. Start **Lane C (idle preparation)** only after B is merged because it extends
   the package record/catalog shapes revised by B.
4. Merge and review C, including every revision bump.
5. Start **Lane D (idle runtime/console)** from the integration branch after C.
6. The orchestrator performs final integration, documentation, full gates, and
   real-data acceptance. Do not delegate the final diff/revision audit.

If an executor discovers that an owned edit must cross into another lane's
files, it must stop and report the exact seam. The orchestrator either expands
ownership after the other lane is merged or performs the small integration edit
itself; two agents must never edit the same production file concurrently.

## Goal and visible success criteria

The exact command

```powershell
cargo run-dev -- render MegatonCratersideSupply
```

must no longer produce visually static NPCs because of an animationless cache.
After one accepted repair:

- Mercenary `0001ff18` and Moira Brown `0002d2bc` bind native clip packs and
  visibly play base idle animations while stationary.
- Navigation-driven movement visibly selects walk/run animation and returns to
  idle at rest.
- The mercenary's authored Eat, Sleep, and Guard packages are all present and
  selected in their correct hourly windows.
- Moira's 08:00–20:00 service package resolves its authored marker and can
  drive Sandbox movement at noon.
- Authored package and global IDLE special animations are prepared and can run
  when their Fallout conditions admit them.
- No generic personality/fidget pool is invented. When an actor has no eligible
  authored special idle, the correct behavior is the normal base idle.
- A console command can force a compatible prepared special idle for visible
  verification without changing automatic eligibility rules.
- A second render of the ready cache performs no unnecessary reprepare or bake.

## Ground truth already established

Reconfirm cheaply before coding; do not redo broad discovery unless a value has
changed.

### Target and cache

- `MegatonCratersideSupply` resolves to cell `00003a2a`.
- The inspected cache used manifest schema 24 and actor-animation catalog
  revision `actor-animations-v3-normalized-runtime-contract`.
- Actor mappings exist for:
  - Mercenary reference `0001ff18`, base `0001ff12`;
  - Moira Brown reference `0002d2bc`, base `0002d3c0`;
  - Radroach reference `0003e5d1`.
- The humanoid set was `animation-set-721d34227d5581a6`, but its
  `clip_pack_asset_path` and `clip_pack_hash` were `None`; every clip was
  `NotConverted`.
- Live `actorinspect` for Mercenary and Moira reported `bound_targets=0`, no
  selected clip, and `prepared animation set has no clip pack`.

### Confirmed render bug

- `ActorAnimationConverter::default()` is `Native` in `src/cli.rs`.
- `PrepareArgs` already exposes `--actor-animation-converter`.
- `RenderArgs` does not expose the option.
- `src/viewer/mod.rs::prepare_for_render` hardcodes
  `ActorAnimationConverter::Disabled`.
- `next_render_cache_action` checks manifest/bake compatibility only. It does
  not inspect the actor-animation sidecar, so a compatible animationless cache
  is treated as ready indefinitely.
- Render must not rely on the resumable prepare-job manifest to detect this;
  the inspected direct-render cache was usable while the jobs file reported it
  as not prepared.

### Confirmed Craterside package bugs

Mercenary base package list, in authored priority order:

| Package | EditorID | Schedule | Current defect |
| --- | --- | --- | --- |
| `0001ff1f` | `MegMercCratersideSleep5x3` | 05:00 for 3 hours | Valid 8-byte `PKDT` rejected |
| `0001ff1e` | `MegMercCratersideEat3x2` | 03:00 for 2 hours | Valid 8-byte `PKDT` rejected |
| `0001ff20` | `MegMercCratersideGuard0x24` | unscheduled/continuous | Parsed; Travel to `0001ff17` |

The raw 8-byte layouts contain `general_flags: u32`, package `type: u8`, one
unused byte, and Fallout behavior flags `u16`. The current parser accepts only
4 and 12 bytes, removes the malformed winner, and leaves actor-catalog
`unresolved package link` diagnostics. Preserve legacy 4-byte support, add 8,
and retain 12.

Moira's important packages:

- `00004153` `MegMoiraOfferServices8x12`: Sandbox, 08:00 for 12 hours,
  location marker `00076f52` (`MoiraServiceMarker`).
- `00004155` `MegMoiraEatHome21x2`: Eat, 21:00 for 2 hours, chair `00015882`.
- `00004156` `MegMoiraSleep0x2`: Sleep, 00:00 for 2 hours, bed `00003d75`.
- `00004157` `MegMoiraDefaultPackage`: unscheduled Sandbox near XMarker
  `00003fdf`.

`PackageSchedule::evaluate` currently divides positive `PSDT.duration` by 60
and therefore reads Moira's 12-hour service shift as 12 minutes. Real EditorIDs
and GECK schedule granularity establish that positive duration is in hours.

The service/default points are real persistent REFR records whose base is an
editor marker. Preparation drops editor markers unless they participate in a
linked-reference chain, so valid package points can disappear from the runtime
manifest. Package-linked marker retention must be added explicitly.

### Confirmed authored-idle data

- Craterside's Moira and mercenary packages do not themselves contain `IDLA`
  collections. Do not fabricate one.
- Fallout package records elsewhere contain:
  - `IDLF`: collection flags (`0x01` run in sequence, `0x04` do once; preserve
    and diagnose unknown `0x02`);
  - `IDLC`: animation count, observed as 1 byte and documented variants as a
    wider struct; accept 1 or 4 bytes and use the first byte;
  - `IDLT`: `f32` timer in seconds;
  - `IDLA`: packed little-endian IDLE FormIDs.
- Fallout `IDLE` records provide EditorID, model/KF path, CTDA conditions,
  parent and previous-sibling links (`ANAM`), and `DATA` group/loop/replay
  metadata.
- Preserve the raw group byte. Real values include `0x47`/`0x87` for Special
  Idle and `0x54` for Whole Body; derive the canonical group section from the
  low six bits (`raw & 0x3f`) while retaining the high bits.
- `NPCGeneralIdles` is an authored tree. Its child order is reconstructed from
  previous-sibling links, not FormID sorting:
  `Swatting -> PickingNose -> PickingNose02 -> BitingNails -> ScratchingSelf -> ScratchingSelf02`.
- Relevant CTDA functions in that path are `IsWeaponOut` (101),
  `GetInFaction` (71), `GetRandomPercent` (77), `GetEquipped` (182), and
  `IsLastIdlePlayed` (451).
- The existing humanoid KFFZ discovery already includes corresponding
  `meshes/characters/_male/idleanims/*.kf` clips. The missing work is authored
  record preparation, selection, conversion readiness, and runtime playback;
  do not create a second animation-conversion pipeline.

## Locked product and architecture decisions

1. **Native conversion is the normal render behavior.** `disabled` remains an
   explicit diagnostic/performance opt-out.
2. **Prompt and repair existing caches.** Interactive render explains missing
   actor animations and offers native reprepare. Declining continues with an
   explicit static-actor warning. Agent-bridge launches cannot prompt and must
   fail with an exact repair command.
3. **Rebake only when necessary.** Reprepare may invalidate bake metadata; use
   the existing bake-compatibility policy after repair rather than always
   baking.
4. **Fallout data is authoritative.** Decode package idle collections and the
   global IDLE tree. No generic fallback pool.
5. **Automatic v1 playback supports Special Idle (7) and Whole Body (20).**
   Preserve other group sections but report them unsupported for automatic or
   forced playback. Upper-body masking/layering is a follow-up, not hidden
   scope.
6. **Package collections override the global Idle Manager.** The package
   `No idle anims` general flag (`0x01000000`) disables both.
7. **Special idles run only while alive and stationary.** Movement, package
   transition, equipment transition, death, ragdoll, or unload interrupts the
   special clip immediately and returns authority to normal locomotion state.
8. **Core remains Bevy-free.** Pure contracts/policies may live in
   `bevyout-core`; Bevy systems only adapt facts and execute selected intents.
9. **No Big Brain dependency or fork in this wave.** Model the future utility
   AI boundary in documentation only; do not add speculative source skeletons.
10. **Preserve current contracts.** Do not rename existing console commands,
    public manifest fields, plugin sets, serialized actor fields, or runtime
    components unless explicitly listed below.

## Public interfaces and prepared data

### CLI

Add to `RenderArgs`:

```text
--actor-animation-converter <native|disabled>
```

It uses `ActorAnimationConverter::default()` (`native`) and is passed unchanged
to `PrepareArgs`. Correct any help text claiming prepare defaults to disabled.

### Actor-animation catalog

Extend `PreparedActorAnimationCatalog` in `bevyout-core` with a defaulted,
deterministically FormID-sorted list equivalent to:

```rust
PreparedActorIdleDefinition {
    form_id: u32,
    editor_id: Option<String>,
    source_kf_path: Option<String>,
    clip_name: Option<String>,
    parent_form_id: Option<u32>,
    previous_sibling_form_id: Option<u32>,
    conditions: Vec<Vec<u8>>,
    group_section_raw: u8,
    group_section: u8,
    loop_min: u8,
    loop_max: u8,
    replay_delay_seconds: i16,
    flags: u8,
}
```

Folder/root IDLE nodes legitimately have no KF path or clip name. Normalize KF
paths using the existing actor-animation canonicalization. Do not add a new
sidecar or new manifest pointer; the existing actor-animation sidecar owns
these definitions.

Bump `ACTOR_ANIMATION_CATALOG_REVISION` from v3 to a descriptive v4 value. The
field is serde-defaulted for backwards parsing, but the revision bump remains
mandatory.

### Package catalog

Extend `PackageRecord`, its pure input mirror, and `PreparedPackageEntry` with
one defaulted optional collection equivalent to:

```rust
PreparedPackageIdleCollection {
    flags: u8,
    timer_seconds: f32,
    animation_form_ids: Vec<u32>,
}
```

Validate `IDLC` against the decoded IDLA length. Preserve the animations and
emit a stable diagnostic on mismatch; do not truncate valid IDs to a bad count.
Resolve each raw IDLA FormID through the plugin's `FormIdResolver`.

Bump `PACKAGE_CATALOG_REVISION` from `openmw-packages-v2` to a descriptive v3
value.

### Runtime and console

Add a per-actor idle runtime authority containing at least:

- selection epoch;
- current/last IDLE FormID;
- next eligible evaluation time;
- per-IDLE replay cooldowns;
- active package FormID and collection cursor;
- do-once exhaustion state;
- current source (`package`, `idle_manager`, or `forced`);
- stable rejection/diagnostic reason.

Add:

```text
playidle <actor-reference> <idle-formid>
```

The command bypasses conditions and cooldown but still requires a prepared,
Ready clip compatible with the actor's set and an automatically supported
group section (7 or 20). It must return structured console data and a clear
error for unknown actor, unknown IDLE, unavailable clip, or unsupported group.

Extend `actorinspect` additively with idle source, selected IDLE FormID,
cooldown/collection state, and last rejection reason. Do not break existing
text or structured keys.

## Lane A — render and cache readiness

### Ownership

- `src/cli.rs`
- `src/viewer/mod.rs`
- `src/viewer/tests/mod.rs` and narrowly related viewer test modules
- no preparation, package, or actor-runtime files

### Tests first

Add pure/unit coverage for:

1. `RenderArgs` defaults to native and parses explicit disabled.
2. `prepare_for_render` passes the requested backend rather than hardcoding.
3. Actorless catalogs are ready and never prompt.
4. A valid mapped set with a hash-valid pack, Ready idle, and Ready forward
   locomotion is ready.
5. Missing catalog/path/hash, stale revision, wrong sidecar hash, missing pack,
   wrong pack hash, missing mapping set, no Ready idle, or no Ready forward
   locomotion returns `RepairRequired` with stable reason text.
6. Explicit disabled returns `IntentionallyDisabled` even for an animationless
   cache.
7. Interactive acceptance of repair invokes forced native reprepare and then
   re-runs bake compatibility.
8. Interactive refusal continues with one explicit warning.
9. Agent bridge returns an error containing the exact command:
   `cargo run-dev -- prepare <selector> --actor-animation-converter native --force`.
10. A ready cache remains `Ready`; no reprepare/rebake action is scheduled.

### Implementation

- Add an engine-independent readiness result such as `Ready`, `NoActors`,
  `IntentionallyDisabled`, or `RepairRequired(reason)`.
- Resolve and hash-check both the catalog and clip-pack paths relative to
  `manifest.asset_root`; do not trust serialized hashes without reading files.
- Share or extract the current pure clip-scoring rules so readiness and runtime
  agree on what constitutes base Idle and Walk. Do not duplicate filename
  heuristics in `viewer/mod.rs`.
- Require Idle and Walk. Run/turn/equip remain optional; improve locomotion
  fallback to `Run -> Walk -> Idle` and `Walk -> Idle` so a missing optional run
  clip cannot leave a moving actor frozen.
- Insert animation repair between manifest compatibility and bake readiness.
  A generic manifest incompatibility still uses the existing reprepare path.
- After repair, reload the manifest and sidecar and prove readiness before
  launch. If native conversion still cannot produce base locomotion, return the
  concrete diagnostic rather than prompting forever.

### Lane A focused gates

```powershell
cargo fmt --check
cargo test-dev viewer::tests
cargo test-dev actor_animation
cargo check-dev
```

Use the narrowest valid test filters available; report any filter that does not
actually execute tests.

## Lane B — package parsing, schedules, and marker retention

### Ownership

- `src/vsa/openmw_esm4/actor_support.rs` and its separate tests
- `src/vsa/prepare/package_catalog.rs` and separate tests
- `src/viewer/ai/selection.rs` and separate tests
- `src/vsa/prepare/orchestrator.rs` only for package-linked marker retention
- relevant existing Cucumber feature files and append-only `tests/features.rs`
- no actor-animation core/runtime files

### Feature list and tests first

Append scenarios to the existing package feature files rather than inventing a
parallel suite:

- `ai_packages.feature`: 4-, 8-, and 12-byte PKDT; 8-byte Eat/Sleep types
  survive into the catalog; package idle collection round-trip.
- `ai_package_selection.feature`: positive duration is hours; boundaries are
  half-open; midnight wrap; duration >= 24; unscheduled and non-positive
  behavior preserved.
- `ai_package_points.feature`: an editor marker referenced by a current-cell
  actor package is retained as a resolvable nonvisual point; an unrelated
  editor marker remains omitted.

Add parser/unit cases pinning the actual layouts:

- `0001ff1e`: type 3 Eat from 8-byte PKDT;
- `0001ff1f`: type 4 Sleep from 8-byte PKDT;
- 12-byte type and tail remain unchanged;
- malformed lengths still diagnose and remove the winner;
- deleted/overridden record policy remains load-order correct.

### Implementation

- Decode 8-byte PKDT without reading absent type-specific bytes. Preserve the
  existing 4-byte compatibility behavior and 12-byte behavior.
- Change positive duration conversion from `duration / 60` to
  `duration as hours`. A non-positive duration keeps the current open-ended
  policy. A positive span >= 24 is active all day once calendar gates pass.
- Do not broaden this issue into a full GECK calendar rewrite. Preserve current
  month/date/day gates unless a test proves a directly related defect.
- Decode `IDLF`/`IDLC`/`IDLT`/`IDLA` into the collection contract above. Remove
  those names from deferred-subrecord diagnostics once supported.
- Before `stage_placements`, collect reference FormIDs used by location type
  `NearReference` and target type `SpecificReference` from packages linked by
  actors placed in the selected cell. Pass that set explicitly into placement
  eligibility.
- Retain matching XMarker/editor-marker references regardless of render-model
  eligibility. They receive no GLB, visible entity, combined static mesh,
  collision, or shadow role; their transform remains available to package
  resolution.
- Scope retention to packages reachable from current-cell actor blueprints,
  not every content-wide package.
- Bump package and prepare revisions. Because the manifest shape does not
  change, do not bump the manifest schema solely for this lane.

### Lane B focused gates

```powershell
cargo fmt --check
cargo test-dev package
cargo test-dev --test features
cargo check-dev
```

The executor must report the number of Cucumber scenarios executed and confirm
that no scenario is skipped.

## Lane C — authored IDLE preparation

Start only after Lane B is merged.

### Ownership

- `crates/bevyout-core/src/actor_animation.rs` and separate core tests
- a capability-named IDLE decoder module under `src/vsa/openmw_esm4/`
- the minimal `openmw_esm4` reader/state wiring needed for winning IDLE records
- `src/vsa/prepare/actor_animation.rs` and separate tests
- actor-animation catalog/conversion Cucumber features and append-only shared
  steps
- no viewer runtime or console command files

### Feature list and tests first

Append to `actor_animation_catalog.feature` and
`actor_animation_conversion.feature`:

1. IDLE winners obey plugin override/deletion policy.
2. Root/folder nodes with no KF remain valid.
3. Model paths normalize case and separators and match existing set clips.
4. `ANAM` parent/previous links preserve the authored tree.
5. Sibling order follows the previous-sibling chain, not FormID or parse order.
6. Real-style raw group bytes map through `raw & 0x3f` while raw values survive.
7. DATA lengths of 6 and 8 bytes decode safely; truncated data diagnoses rather
   than panics.
8. CTDA payloads remain byte-exact and stream ordered.
9. Catalog ordering and output hashes are deterministic.
10. Existing clip conversion converts IDLE-referenced KFs once in the same
    shared set; there is no duplicate pack or second converter invocation.
11. Revision constants are pinned.

### Implementation

- Add a content-wide winning-record map for `IDLE`, using the same
  override/deletion semantics as existing package/faction collectors.
- Decode `EDID`, model data (at minimum authoritative `MODL` path), repeated
  `CTDA`, `ANAM`, and `DATA`. Preserve unknown bytes/signatures as diagnostics;
  do not make one malformed optional field fatal to the whole content set.
- Resolve parent/sibling FormIDs through the plugin resolver.
- Build the prepared idle list while building the actor-animation catalog.
  Resolve a leaf's normalized KF path to the existing clip definition/name.
  Missing/incompatible clips remain prepared with diagnostics so inspect tools
  can explain them.
- Reconstruct child order by finding the zero-previous sibling and following
  the chain. Diagnose missing predecessor, duplicate successor, cycle, or
  disconnected children. For malformed chains, append disconnected nodes in
  stable FormID order after the valid chain; never loop indefinitely.
- Bump the actor-animation catalog revision and prepare revision. Do not add a
  Bevy dependency to core and do not add a new manifest field.

### Lane C focused gates

```powershell
cargo fmt --check
cargo test -p bevyout-core actor_animation
cargo test-dev actor_animation
cargo test-dev --test features
cargo check-dev
```

## Lane D — idle selection, playback, and console surface

Start only after Lane C is merged.

### Ownership

- a pure idle-policy module under `src/viewer/actor_animation/`
- `src/viewer/actor_animation/mod.rs` and its separate test modules
- the narrow shared CTDA policy seam extracted from AI selection if required
- `src/viewer/console/world_commands.rs` or a capability-named actor-animation
  console provider/module, plus separate console tests
- `features/actor_animation_gameflow.feature` and append-only shared steps
- no preparation decoder/orchestrator files

### Feature list and tests first

Pin these behaviors in pure policy tests/Cucumber before Bevy systems:

1. A moving, dead, ragdolled, unloaded, or equipment-transitioning actor is not
   eligible for a special idle.
2. Package `No idle anims` disables package and global selection.
3. A nonempty active package collection overrides the global tree.
4. Run-in-sequence advances in authored IDLA order; random collections use a
   deterministic actor/package/epoch seed; do-once exhausts without restarting.
5. Package timer starts on stationary package entry and uses seconds.
6. Global selection walks parent conditions and authored sibling order.
7. One deterministic random-percent roll is used per sibling group/depth and
   reused for cumulative thresholds at that depth. A child level receives its
   own roll.
8. The supported condition functions read only authoritative runtime facts:
   weapon-out state, actor factions, equipped item IDs, last idle, and the
   deterministic roll.
9. Unsupported or malformed conditions make the branch ineligible with a
   stable reason; they never become implicitly true.
10. Per-leaf replay delay prevents immediate replay.
11. Loop count is selected deterministically inside inclusive min/max; invalid
    max below min clamps to min; zero/zero plays once.
12. Section 7 and 20 leaves play; other sections return `unsupported_group`.
13. Special completion resumes base Idle; locomotion interruption selects Walk
    or Run in the same update ordering without one static frame.
14. Forced `playidle` bypasses conditions/cooldown only, not compatibility,
    actor lifecycle, or supported-group checks.
15. `actorinspect` remains backwards-compatible and adds stable idle fields.

### Shared CTDA policy

The package selector already decodes CTDA comparison headers behind a
`ConditionFunctions` boundary. Extract only the reusable, std/serde-only
decode/comparison policy rather than copying it. Keep package behavior
unchanged. Implement these IDLE boundary functions:

| Index | Function | Runtime fact |
| --- | --- | --- |
| 101 | `IsWeaponOut` | actor animation/equipment presentation state |
| 71 | `GetInFaction` | prepared/canonical actor faction membership |
| 77 | `GetRandomPercent` | current sibling-group deterministic roll |
| 182 | `GetEquipped` | canonical equipped item base FormIDs |
| 451 | `IsLastIdlePlayed` | per-actor idle runtime authority |

Do not implement a general GECK VM. Preserve OR/operator semantics already
pinned by package tests.

### Runtime scheduling and playback

- Base Idle begins immediately whenever locomotion becomes stationary.
- Package collections evaluate when their `IDLT` timer expires.
- Global Idle Manager evaluates at each completed base-idle loop. If clip
  duration/completion metadata is unavailable, use a bounded one-second retry;
  do not evaluate every frame.
- On special selection, resolve the leaf's `clip_name` inside the actor's
  already-loaded shared animation set. Never synchronously read/convert assets
  in an Update system.
- Special/Whole Body v1 uses the existing graph/player and crossfade path as a
  full-body state. Do not create per-frame graph assets or mutate a shared graph
  differently for individual actors.
- Movement and other higher-authority states cancel special playback before
  choosing the next base clip. Keep scheduling inside the viewer's established
  `ViewerSet` order.
- Runtime diagnostics use `tracing`, never `println!`, with a stable prefix such
  as `actor-idle select/play/stop/reject <reference-formid>`.

### Lane D focused gates

```powershell
cargo fmt --check
cargo test-dev actor_animation
cargo test-dev console
cargo test-dev --test features
cargo check-dev
```

## Big Brain future compatibility decision

Do not add `big-brain`, a fork, adapter types, empty components, or placeholder
plugins in this wave.

At the time this plan was written:

- the GitHub repository was archived and development pointed to Codeberg;
- Codeberg `main` identified itself as big-brain 0.23.0 and depended on Bevy
  0.17;
- bevyout depends on Bevy 0.19;
- no Bevy 0.19 branch was found, and the latest observed upstream activity was
  October 2025.

Amend the Wave 8 section of `M5_COMBAT_ARCHITECTURE_ROADMAP.md` with this gate:

1. At Wave 8 kickoff, re-check current Codeberg upstream state.
2. Create a local compatibility spike updating upstream `main` to bevyout's
   exact Bevy version. Run upstream tests/examples and a bevyout minimal-App
   scheduling/cancellation test.
3. `bevyout-core` remains authoritative for world facts, utility score values,
   deterministic ordered tie-breaking, stable tactic/candidate IDs, replay,
   and persistence.
4. A Big Brain adapter may own Bevy scheduling and action lifecycle/cancellation
   only. Fallout packages remain authoritative for authored schedules; utility
   AI handles unscripted combat decisions such as pursuit, cover, range,
   reload, flee, and surrender.
5. No `big_brain` type may enter prepared assets, saves, or core contracts.
6. Prefer an upstream-compatible patch. If upstream cannot support the current
   Bevy version, show the human the maintained-fork proposal and maintenance
   diff before creating or publishing a fork/PR under their identity.
7. Reject adoption if deterministic selection requires depending on ECS
   iteration/entity IDs, if the adapter violates viewer schedule ownership, or
   if the port becomes a rewrite larger than the small utility-AI surface
   bevyout needs.

Sources for the future spike:

- <https://codeberg.org/zkat/big-brain>
- <https://raw.githubusercontent.com/zkat/big-brain/main/Cargo.toml>
- <https://docs.rs/big-brain/latest/big_brain/>

The animation wave should only preserve a clean fact -> intent -> execution
boundary naturally required by its own behavior. It must not manufacture
speculative AI abstractions solely to resemble Big Brain.

## Revision and compatibility audit

The orchestrator must explicitly inspect the final diff for serialized shape
changes.

Required bumps:

- `PACKAGE_CATALOG_REVISION`: package idle collection added.
- `ACTOR_ANIMATION_CATALOG_REVISION`: idle definitions added.
- `CURRENT_PREPARE_REVISION`: prepared placement meaning and catalog contents
  changed.

Do not bump `CURRENT_MANIFEST_SCHEMA_VERSION` unless the implementation
actually changes manifest serialized shape, which this plan does not authorize.
Update every pinned revision assertion, synthetic fixture, preload path, and
test manifest affected by the owning revision. A serde default is not a reason
to omit a revision bump.

## Full verification

After merging all lanes, the Luna Max orchestrator runs, in order:

```powershell
cargo fmt --check
cargo check-dev
cargo test -p bevyout-core
cargo test --test features
cargo test --test architecture
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

If the repository aliases reject workspace-style arguments, use the exact
AGENTS.md gate commands and record the substitution. No skipped Cucumber step,
warning, or stale revision is acceptable.

Also run:

```powershell
cargo run-dev -- prepare MegatonCratersideSupply --list-only
```

Expected selection:

```text
00003a2a MegatonCratersideSupply
```

## Real-data acceptance

Never commit `.ron`, GLB, KF, KTX2, NIF, DDS, WAV, screenshots containing
derived game data, or anything under `.bevyout/`. Use the user's installed
Fallout data only as untracked local evidence.

### 1. Existing-cache repair

Run:

```powershell
cargo run-dev -- render MegatonCratersideSupply
```

On a compatible but animationless cache, expect a prompt that names incomplete
actor animations and offers native reprepare. Accept it. If the bumped prepare
revision causes the generic refresh prompt first, accept it and still verify
that the forwarded backend is native and the resulting catalog passes the new
readiness check. Accept a bake prompt only if compatibility reports it stale.

Pass criteria:

- actor catalog mappings remain present;
- humanoid clip-pack path/hash are populated and hash-valid;
- Ready base idle and forward locomotion clips exist;
- no `conversion_not_requested` diagnostic remains for mapped sets;
- render reaches the viewer.

### 2. Bridge inspection

With the repaired/baked cache, launch a bounded bridge session:

```powershell
cargo run-dev -- render MegatonCratersideSupply --agent-bridge --agent-port 15702 --trace-seconds 120
```

Use bevyout MCP when registered; otherwise use BRP JSON-RPC. Execute:

```text
actorinspect 0001ff18
actorinspect 0002d2bc
showpackages 0001ff18 4
showpackages 0001ff18 6
showpackages 0001ff18 12
showpackages 0002d2bc 12
runpackage 0001ff18 status
runpackage 0002d2bc status
```

Expected package evidence:

- mercenary 04:00 selects `0001ff1e` Eat;
- mercenary 06:00 selects `0001ff1f` Sleep;
- mercenary noon selects `0001ff20` Guard/Travel;
- no `not found in package catalog` line for `0001ff1e`/`0001ff1f`;
- Moira noon selects `00004153` and resolves `00076f52` rather than falling
  through to missing `00003fdf`;
- actorinspect reports bound animation targets and a selected base clip.

Observe Mercenary travelling to `0001ff17` and Moira roaming within her service
Sandbox. While translation velocity is sustained, actorinspect/logs must report
Walk or Run; after arrival/dwell they must report Idle. Capture a visible
viewport while the window is unobscured plus scene snapshots/logs.

### 3. Authored special-idle surface

Execute:

```text
playidle 0002d2bc 00067941
actorinspect 0002d2bc
```

`00067941` is `ScratchingSelf02`, a prepared Special Idle leaf. Expect visible
playback and structured source `forced`. Then start/provoke navigation and
confirm the special idle stops immediately and locomotion resumes.

Also inspect automatic selection. Moira and the mercenary are not guaranteed to
qualify for `NPCGeneralIdles` because Fallout data controls faction/equipment
conditions. Remaining on base Idle with a concrete authored-condition rejection
is a pass; inventing or silently forcing a generic idle is a failure.

### 4. Ready-cache and opt-out regression

Rerun the normal render command. Expect no animation repair and no unnecessary
bake.

Run or cover synthetically:

```powershell
cargo run-dev -- render MegatonCratersideSupply --actor-animation-converter disabled
```

The explicit opt-out may use static actors but must state that animation
conversion was intentionally disabled and must not enter a repair loop.

### 5. Evidence recording

Record on each issue:

- exact command and commit;
- focused test counts;
- catalog mapping/set/Ready clip counts;
- package selections and resolved real FormIDs;
- actorinspect bound-target/current-clip evidence;
- whether bake was reused or rebuilt and why;
- what was visually verified versus only structurally verified.

Write the manual so a human can repeat the same numbered commands verbatim.

## Failure handling and traps

- Do not solve the symptom by forcing an Idle state at actor spawn; the missing
  clip pack must be repaired.
- Do not delete/recreate prepared caches as the normal recovery policy. Repair
  in place and preserve compatible converted assets.
- Do not make render always rebuild shadows, probes, or irradiance.
- Do not add Blender animation conversion; native Nifty/KF conversion is the
  supported backend.
- Do not render or collide editor markers retained only as package points.
- Do not sort IDLE siblings by FormID.
- Do not treat unevaluable CTDA as true.
- Do not evaluate global idles every frame.
- Do not create per-actor animation graphs every frame or mutate a shared graph
  in actor-specific ways.
- Do not introduce full upper-body animation masks in this wave.
- Do not add Bevy to `bevyout-core` or viewer imports to `src/vsa/`.
- Do not put `#[cfg(test)] mod tests { ... }` inline in implementation files;
  use separate test modules/files.
- Do not use broad root searches that traverse `.bevyout`, `target`, or other
  large caches. Use CodeGraph first, then scoped `rg`.
- Do not use `git add .` or `git add -A`; stage explicit paths.
- Do not commit Bethesda-derived data.
- Do not publish a Big Brain fork or upstream PR without showing the human the
  complete proposal and receiving explicit approval.

## PR completion

- One integration PR to `master` closes all four new sub-issues.
- The body briefly states the original static-NPC failure, the render/cache
  repair, Craterside package fixes, authored idle support, and validation.
- Link the manual and include exact local gates plus real-data evidence.
- Wait for hosted checks and review. Verify every finding against current code;
  fix confirmed issues and answer incorrect/out-of-scope findings with evidence.
- Tick epic/manual items only after the real-data gates above pass.

## Shipped amendments

Append acceptance-driven changes here. Do not edit the approved sections above
to hide implementation discoveries.

- None yet.

# M4 wave 7 — actor assembly and deterministic appearance fallbacks (#107, #108)

Wave under epic #9 on the single integration branch `m4-wave7`, based on
master after PR #159. Issues #107 and #108 share the same prepared actor
identity and viewer presentation seam, so they execute sequentially on this
branch instead of in parallel worktrees.

**Execution model recommendation (Codex runtime): Sol X-High.** The wave
crosses pure core contracts, ESM4 preparation, converter cache identity,
canonical item state, Bevy hierarchy synchronization, and real Fallout data;
the higher-reasoning execution model is appropriate for keeping those
contracts aligned.

## Baseline after PR #159

- Humanoid preparation already converts one assembled GLB containing the
  shared skeleton, sex-correct RACE body/head meshes, and deterministic
  non-conflicting ARMO meshes. Weighted apparel follows the ragdoll.
- CREA preparation already attempts `MODL` plus `NIFZ`, but it has no explicit
  creature fallback policy.
- Appearance preparation resolves templates separately from `actors.ron`.
  That duplicate resolver ignores per-group ACBS inheritance flags and can
  disagree with the catalog about sex, inventory, and concrete leveled actor.
- `HAIR`, `EYES`, and `HDPT` links are decoded on NPC/RACE records but their
  target records are not retained as supported bases, so their meshes cannot
  be assembled.
- Missing any NIF in an actor conversion input drops the whole placement.
  `PreparedActor` carries only `base_template_form_id`, the viewer has no actor
  plugin, and generic scene spawning skips every placement without an asset.
- Canonical item state is one global `ItemLedger`; adding a per-actor ledger
  component would create a second authority and is explicitly rejected.

## Fixed feature list

### 1. One resolved actor identity

- Extend the pure actor catalog result with the selected concrete identity and
  prepared assembly blueprint. Appearance, inventory, sex, scale, conversion,
  manifest semantic, diagnostics, and runtime all consume that one result.
- Remove/bypass the independent appearance template resolver. ACBS field-group
  inheritance and seeded leveled selection remain owned by
  `actor_catalog.rs`.
- Preserve source base, resolved base, and placement/reference FormIDs
  separately.

### 2. Pure assembly and fallback contracts

- Add engine-independent actor contracts to `bevyout-core`: actor kind,
  attachment points, typed mesh roles, apparel/weapon selections, FaceGen
  policy, five ordered fallback levels, structured fallback reasons, proxy
  kind, and `ActorAssemblyBlueprint`.
- Add pure preparation policies for deterministic part ordering, NPC override
  versus RACE default selection, apparel visibility, starting-weapon choice,
  compound root scale, creature routing, and the fallback ladder.
- A missing optional hair/eye/apparel/weapon records its own reason and never
  removes an otherwise valid actor. Required skeleton/body/head failures drive
  the assembly tier.
- Authored FaceGen bytes remain preserved but exact morph reconstruction stays
  #109. When authored coefficients cannot be applied, the prepared policy is
  an explicit rest-pose race/sex fallback, never a malformed morph.

### 3. Humanoid and creature preparation

- Retain `HAIR`, `EYES`, and `HDPT` bases and resolve NPC overrides first,
  then sex-specific RACE defaults/candidates. Hair is omitted/hidden when the
  equipped apparel occupies Hair or Hat slots.
- Keep RACE body-part indices and selected apparel metadata deterministic and
  duplicate-free. Existing weighted apparel/body masking remains intact.
- Select one deterministic starting WEAP from the resolved inventory without
  baking it into the actor GLB; preserve the active right-hand attachment
  decision even when its model is missing.
- CREA uses an explicit primary-root then compatible-secondary policy and
  never enters humanoid race/sex rules or silently promotes an arbitrary
  attachment to a skeleton.
- NPC scale is the finite-positive product of reference `XSCL`, sex-specific
  RACE height, and NPC `NAM6`; CREA scale uses `XSCL` and `BNAM`. Weight is
  preserved for diagnostics/future FaceGen/body morph work, not misused as
  uniform scale.

### 4. Serialized diagnostics and cache correctness

- Embed the assembly blueprint in `PreparedSemantic::Npc/Creature` and in the
  per-cell actor catalog. Prepared diagnostics include source/base/reference
  identity, selected assets, missing/incompatible reasons, fallback tier, and
  FaceGen policy in stable order.
- Bump every affected prepared revision, including the actor catalog, scene
  manifest/prepare revision, and actor converter/cache revisions. The actor
  assembly policy/metadata participates in the cache fingerprint, so two
  different selections cannot alias only because their NIF bytes match.
- Do not change the physics sidecar schema unless its serialized shape changes.

### 5. Runtime actor presentation

- Add `viewer::actor::ActorPlugin` and run reconciliation in
  `ViewerSet::WorldSync`.
- Every prepared actor root retains `PlacementRoot`/reference registration and
  receives actor runtime identity/assembly state. An actor with no GLB still
  spawns: a generated humanoid fallback or bounds proxy keeps it visible,
  selectable, persistable, and available to later navigation/physics slices.
- Add `HolderId::Actor { reference_form_id }`; seed the resolved initial
  inventory into the existing global canonical ledger and bind the selected
  weapon by stable `ItemInstanceId`. Actor ECS state is a projection of that
  ledger, never an independent item authority.
- After the assembled scene hierarchy exists, bind the selected weapon GLB to
  the exact `Weapon` node, falling back deterministically to `Bip01 R Hand`
  with a structured missing-node diagnostic. Reconcile attach/detach when the
  canonical binding changes.
- Add a dedicated `actorinspect <reference>` console command. It reports
  source/resolved identity, tier/reasons, parts, hidden slots, canonical
  holder/equipped item, attachment node/state, and proxy state. `tna` remains
  the test-nav-agent command family and is not overloaded.

## Tests first

1. Add `features/actor_assembly.feature` before production changes. Scenarios
   cover shared concrete resolution, sex/RACE hair and eyes, deterministic
   part order, Hair/Hat masking, weapon selection/node decision, compound
   scale, CREA bypass, and missing optional equipment.
2. Add `features/actor_fallback.feature` before production changes. Scenarios
   cover all five tiers, missing FaceGen rest pose, distinct missing-model /
   skeleton / incompatible-skin / equipment reasons, deterministic ordering,
   and proxy identity retention.
3. Add byte-level HAIR/EYES/HDPT parser tests; actor catalog revision and
   serialization tests; actor cache-key tests; and focused converter tests for
   required attachment names.
4. Add minimal-App viewer tests for actor root/proxy spawning, canonical actor
   holder seeding, delayed bone discovery, weapon attach/detach, and inspector
   output. Preserve existing actor conversion/ragdoll tests.

## Gates and real-data acceptance

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- representative `cargo run-dev -- prepare` for:
  - `00017f37` Super-Duper Mart (raiders plus Protectron; known raider
    `00041600`, Protectron `0006d921`);
  - `00024511` Vault 101 Atrium (humans plus radroaches);
  - `000151e3` Mister Gutsy creature/template path.
- Launch a prepared representative cell with the agent bridge and use
  `actorinspect`, scene snapshots, canonical equipment state, and viewport
  capture to prove one humanoid and one creature retain identity, selected
  parts, fallback diagnostics, and mounted equipment. Re-run from a warm cache
  and verify the same blueprint/tier counts.
- Record preparation/build/cache counts, actor/fallback coverage, load time,
  memory, and visual limitations in the shipped amendments/manual and on both
  issues. Any generic/proxy result in representative data remains visible and
  becomes a follow-up rather than being hidden.

## Non-goals

- Exact FaceGen morph reconstruction (#109).
- Combat firing/reload presentation, full animation-state transitions, or
  general AI packages.
- Full actor lifecycle/corpse policy and arbitrary runtime apparel re-skinning.
- Replacing the canonical item transaction or navigation backends.

## Shipped amendments

### A1. RACE and EYES semantics corrected from real records

RACE body index 3 is a texture slot, not required geometry; only body 0/1/2
and head 0 gate a complete humanoid. RACE head indices 6 and 7 are retained as
distinct left/right eye geometry while EYES supplies the selected diffuse
texture. Conversion applies that texture only to the selected eye meshes,
preserving the authored materials on the rest of the assembly. Exact FaceGen
remains out of scope.

### A2. Creature primary selection is content-ranked, not name-only

The suggested “primary root” rule could not require the NIF basename to match
its directory: real Radroach uses `Roach.NIF`, and Mister Gutsy uses
`MisterHandy.NIF`. Preparation now prefers an authored basename match, then
the largest available visual NIF with a stable path tie-break. This is
independent of NIFZ order and keeps all compatible secondary parts. PyNifly
failures for bone-parented creature attachments retry the existing NIFTools
compatibility importer while retaining skeleton-derived physics.

### A3. Actor cache identity owns the complete blueprint

Acceptance found that two actor references with identical model paths but
different blueprint metadata could calculate different output hashes while
the placement-stage de-duplication still keyed only by paths. That skipped a
conversion job and later requested a nonexistent physics sidecar. Actor
de-duplication now uses the content-addressed actor asset name, so every unique
blueprint has a matching GLB/physics pair.

### A4. Windows-only comparison boundary

Blender itself is cross-platform, but the PyNifly actor route exercised here
depends on native Windows DLLs. It is retained only as a Windows comparison
tool. Native `nifty` actor assembly is the production path and must carry the
portable acceptance result.

### A5. Historical acceptance results (superseded by A8)

These measurements established catalog and runtime plumbing, but they did not
prove the player-visible actor result. Metadata counters and embedded GLB
fields are not visual acceptance for gender, armor coverage, or hair.

- Real preparation: `000151e3`, `00024511`, and `00017f37` completed `3 done,
  0 failed`; immediate rerun reported `3 cells valid, 0 stale` and skipped all
  three. The final converter revision reused 1,755 assets and built 11 revised
  actor assets. Raider `00041600`'s content-addressed GLB embeds its selected
  `eyeblue` image.
- Real catalogs: 1 + 17 + 11 prepared actors, with zero unresolved,
  unsupported, or skipped entries. Humanoid `00054432` retained body/head,
  hair, two eyes, selected texture, four apparel pieces, and canonical holder.
  Wadsworth `0008f6ae` and Protectron `0006d921` were `AuthoredExact` without
  proxies and retained integrated weapon geometry plus explicit optional-model
  diagnostics.
- Runtime probe (Vault 101, 600 frames): average 16.670 ms, p95 18.283 ms,
  p99 18.985 ms, max 19.673 ms; 8,192 entities, 1,647 mesh entities; local dev
  process 2,863 MiB working set / 5,529 MiB private bytes.
- Gates: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and
  `cargo test` pass. Cucumber: 42 features, 320 scenarios, 1,451 steps.

### A6. Nested actor leveled lists resolve to concrete leaves

Real Super-Duper Mart raider `00041600` is placed from NPC shell `0002f6e2`,
whose template path reaches a nested actor leveled list. Actor preparation now
recursively walks `LVLN`/`LVLC` records to concrete matching `NPC_`/`CREA`
leaves with a 32-level bound and active-path cycle detection. Candidate order
and duplicates cannot change selection. Invalid, missing, cyclic, and
wrong-kind branches are diagnosed while valid leaves remain eligible.

`template_candidates` deliberately retains the root list's immediate entries
for diagnostics. `resolved_base_form_id` owns the selected concrete actor, and
all delegated template groups plus inventory/apparel consume that same seeded
identity. `00041600` therefore keeps source shell `0002f6e2` while resolving
to concrete female raider `0002f6d8`.

### A7. Native actor skins retain authored inverse bind matrices

The corrected descriptor exposed a native-only deformation/flicker regression:
the adapter recomputed every merged part's inverse bind matrices from the
shared skeleton. Bethesda actor parts carry authoritative part-local bind
matrices, so that reconstruction stretched triangles across the intact armor.
Native assembly now remaps joint nodes while retaining each part's authored
matrices. A focused `nifty` merge regression locks that contract, and the root
pins the corrected `native-fo3-glb` revision. Both native actor converter
revisions were bumped so ordinary preparation cannot reuse affected GLBs.

### A7b. Head parts keep their authored coordinate frames

Live close-up acceptance found that rigid head parts cannot share one blanket
transform. Eyes, mouth, teeth, and tongue already carry the rotation that
cancels the animated head bone's authored frame, while hair roots are authored
directly in `HeadAnims`. Native assembly now parents only `Hair` through
`HeadAnims`; the remaining face parts retain their own transforms under
`Bip01 Head`. This keeps hair on the scalp and eyes/mouth in the face. The
staging-only assembly descriptor records this distinction without changing the
prepared manifest schema.

The same acceptance run exposed per-frame weapon attachment churn: an attached
weapon could be reset before its deferred spawn became queryable, creating an
unbounded stream of duplicate weapon entities, log spam, overdraw, and visible
flicker. Attached state is now stable across that deferred-command boundary,
with a focused regression test.

### A8. Corrected visual acceptance

Acceptance is now native-first and player-visible. For `00041600`, preparation
must report source `0002f6e2`, concrete resolved actor `0002f6d8`, `female=true`,
nonempty canonical apparel/inventory, and native armor input
`armor/raiderarmor02/outfitf.nif`. The viewport must show the female raider in
complete armor with continuous body coverage and no stretched/flickering
triangles. Vault utility worker `00054432` separately proves authored hair is
visible when apparel does not mask the Hair/Hat slots. Blender/PyNifly may be
used only on Windows to compare a disagreement; it cannot satisfy the gate.

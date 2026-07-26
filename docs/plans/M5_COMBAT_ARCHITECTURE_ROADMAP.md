# M5 combat systems architecture and wave roadmap

## Purpose and status

This document preserves the cross-wave decisions for M5 combat after the
functional weapon slice and canonical-ammunition foundation landed.

| Wave | Deliverable | Status | Recommended Codex model |
| --- | --- | --- | --- |
| 1 | Functional modular 10mm pistol | Shipped in #239 | Sol X-High |
| 2 | Canonical ammunition, magazines, reloads, loadout, save v5, inspection | Shipped in #248 | Sol X-High |
| 3 | Condition, degradation, jams, deterministic combat RNG | Planned | Sol X-High |
| 4 | Spread, range, hitscan/projectile delivery, ADS accuracy | Planned | Sol X-High |
| 5 | Armor, DR/DT mitigation, armor condition | Planned | Sol X-High |
| 6 | Hit locations, limb health, crippling, dismemberment | Planned | Sol X-High |
| 7 | AP economy and complete V.A.T.S. execution | Planned | Sol X-High |
| 8 | Hostility, awareness, combat AI, cover and range tactics | Planned | Sol X-High |
| 9 | Integration, persistence, deterministic replay, release hardening | Planned | Sol X-High |

Wave 9 completes the combat architecture track. It does not by itself close
M5: compressed audio, melee/explosive breadth, screen feedback, corpses/loot,
HUD, and the player-facing gates remain separate tracks in epic #11.

This is the durable architecture baseline. At each wave kickoff, create the
wave's GitHub sub-issues plus `M5_WAVE<N>_PROMPT.md`,
`M5_WAVE<N>_PLAN.md`, and `M5_WAVE<N>_MANUAL.md`. The kickoff plan may refine
implementation details against current `master`, but must record any departure
from this roadmap as a shipped amendment rather than silently changing policy.

## Governing architecture

**Core owns causality. Viewer owns measured evidence and presentation.**

`bevyout-core` decides:

- whether an action is permitted and what state it consumes;
- deterministic probability draws and their ordering;
- spread, range, damage, armor, limb, AP, V.A.T.S., hostility, and AI policy;
- authoritative mutations and semantic outcomes;
- persisted combat state and typed inspection snapshots.

Bevy/VSA adapters may:

- translate input or AI requests into stable combat intents;
- measure ray hits, projectile impacts, distances, hit zones, visibility,
  hearing, path costs, and cover candidates;
- apply core-approved state snapshots at the persistence boundary;
- display animation, audio, HUD, camera, decals, gore, and debugging evidence.

Adapters must not independently consume rounds, roll probability, calculate
damage, decide hostility, or convert a core miss into a hit.

The direction remains:

```text
ESM/BSA records
  -> VSA decode and preparation
  -> serde core catalog DTOs
  -> core combat decisions
  -> Bevy spatial evidence
  -> core outcomes
  -> Bevy presentation and typed inspection
```

`src/main.rs` remains command routing only. `src/vsa` must not depend on
`src/viewer`. `bevyout-core` remains free of Bevy, physics, rendering, audio,
input, navigation, and `serde_json`.

### Stable combat protocol

A normal shot follows one authoritative sequence:

1. Input or AI submits a fire intent using stable actor and item identities.
2. Core validates action phase, life state, weapon state, jam state, and ammo.
3. Core performs the deterministic jam draw when applicable.
4. Core consumes one loaded round, applies condition loss, advances combat RNG,
   and assigns a stable `ShotId`.
5. Core returns the shot direction/delivery directive.
6. The spatial adapter raycasts or advances the projectile.
7. The adapter returns explicit impact or miss evidence.
8. Core resolves range, ammo modifiers, hit location, armor, health, limbs,
   death, and hostility exactly once.
9. Core emits semantic outcomes; presentation reacts without rewriting them.

Once a shot is accepted, its round is spent. A miss, world hit, despawned
target, missing VFX, or cell transition never refunds it.

For V.A.T.S., the adapter supplies a stable targeting snapshot before
execution. Core owns the probability roll and selected limb. Cinematic
projectiles are presentation and cannot change the result.

## Stable boundary and determinism contracts

### Inputs

The combat facade evolves around:

- `CombatIntent`: fire, reload, ammo switch, clear jam, enter/queue/cancel
  V.A.T.S., and AI-requested equivalents;
- `SpatialEvidence`: ray miss/hit, projectile impact, target reference FormID,
  hit location, origin/direction/impact/distance, visible limb fraction, LOS,
  hearing, and stable cover candidates;
- explicit integer elapsed milliseconds.

No public core operation accepts a Bevy `World`, `Entity`, query, resource,
timer, asset handle, physics type, or audio type.

### Outputs

- `CombatDirective`: accepted/blocked action, spatial query request,
  projectile launch data, action phase/progress, V.A.T.S. phase, and AI tactic;
- `CombatOutcome`: consumed round, committed reload, jam transition, damage,
  armor degradation, cripple, death, dismemberment, hostility/awareness
  transition, and V.A.T.S. completion/invalidation;
- typed, serializable, read-only inspection snapshots.

### Determinism

- Time is explicit integer milliseconds.
- Percentages use basis points `0..=10_000`.
- Stable identities use FormIDs, `ItemInstanceId`, `ShotId`, and explicit
  sequence IDs—never ECS entity IDs.
- Stable collections use `BTreeMap`/`BTreeSet`; no decision depends on ECS
  query iteration order.
- Combat uses a core-owned, versioned PRNG with an inspectable draw index; do
  not add runtime `rand`.
- Authoritative scalar results round at a documented boundary such as
  milli-damage before mutation and inspection.
- Invalid FormIDs, probabilities, durations, NaN, and infinity are rejected at
  core boundaries.
- Duplicate intents, transactions, and impact evidence are idempotent.

## Wave roadmap

### Wave 3 — condition, degradation, jams, and combat RNG

Goal: make each weapon instance's condition affect firing without introducing
non-deterministic runtime randomness.

- Add versioned combat RNG state, domain separation from noncombat randomness,
  stable draw indices, and deterministic seed/migration policy.
- Resolve per-shot degradation from prepared maximum condition and degrade
  rate; preserve exact item-instance identity.
- Define condition-to-damage scaling and minimum effectiveness.
- Add fire/reload jam policies, jam state, clear-jam intent, and blocked-action
  reasons.
- Persist only implemented condition/jam/RNG state and bump save/catalog/
  prepare/combat-policy revisions as required.
- Extend `combatstate` with condition, jam, RNG revision, draw index, and last
  decision terms.

Acceptance includes deterministic repeated runs, no RNG draw on rejected
actions, save/load of jammed and partially degraded weapons, zero-condition
behavior, and visible/inspectable jam-clear presentation.

### Wave 4 — ballistics and delivery

Goal: replace fixed center-ray policy with authoritative firearm accuracy,
range, hitscan, and projectile delivery.

- Prepare base/min/max spread, stance/movement/ADS modifiers, full-damage and
  maximum range, range-end multiplier, delivery kind, projectile speed,
  gravity, lifetime, and payload.
- Core computes deterministic spread directions and range/damage terms.
- Viewer performs only the requested raycast or projectile simulation and
  returns measured evidence.
- Physical projectiles carry stable `ShotId`; duplicate impacts and unloaded
  actors resolve once.
- Preserve a benchmark switch for comparing delivery cost without changing
  gameplay policy.

Acceptance covers standing/crouched/moving/ADS spread, range boundaries,
world blocking, hitscan/projectile parity, projectile save/cell-unload policy,
and inspectable modifier terms.

### Wave 5 — armor and mitigation

Goal: implement one shared player/NPC damage-mitigation pipeline.

- Decode Fallout 3 `ARMO` and `ARMA` data; do not use vanilla `AMAT`.
- Treat FO3 protection as DR-oriented. Support DT as an engine superset whose
  default for FO3 records is zero.
- Prepare covered body parts, biped slots, DR, DT, maximum condition,
  condition-protection curve, and degradation policy.
- Resolve ammo penetration/modifiers, DT, DR, minimum-damage policy, armor
  degradation, breakage, and final health damage in a documented order.
- Use the canonical loadout; apparel is never a separate combat authority.

Acceptance pins numeric mitigation vectors, armor coverage, broken-armor
behavior, ammo variants, player/NPC parity, and save/load of armor condition.

### Wave 6 — body, limbs, crippling, and dismemberment

Goal: make semantic hit locations authoritative while keeping mesh/collider
details in the spatial adapter.

- Prepare body-part semantics, mesh/V.A.T.S. nodes, damage multipliers, limb
  maximums, V.A.T.S. modifiers, actor values, and severable/explodable flags.
- Spatial adapters map collider/node evidence to a semantic part with a
  deterministic torso fallback.
- Core resolves location multiplier, limb damage, cripple thresholds,
  stagger/death interaction, and dismemberment eligibility.
- Persist limb mutations and dismembered masks; dead or severed parts cannot
  resume incompatible actions.
- Activate `hitboxdebug` with stable semantic IDs and visible overlays.

Acceptance covers every body part, unknown-node fallback, simultaneous lethal
and crippling damage, save/load, and player-visible hitbox/dismemberment proof.

### Wave 7 — AP and V.A.T.S.

Goal: implement a complete deterministic V.A.T.S. state machine rather than a
presentation-only targeting mode.

- Add AP maximum/current/regeneration, weapon AP costs, target snapshots,
  stable queue entries, and enter/queue/cancel/execute phases.
- Core calculates hit chance in basis points from skill, range, visibility,
  limb, weapon, and actor terms, then owns every hit/miss draw.
- Validate queue entries again at execution; define target death, unload,
  insufficient AP/ammo, jam, and weapon-switch invalidation.
- Normal and V.A.T.S. attacks share ammo, condition, armor, limb, death, and
  hostility pipelines.
- Viewer owns slow motion, camera selection, UI, and cinematic projectiles
  without changing outcomes.

Acceptance includes queue ordering, AP accounting, deterministic chances,
mid-sequence save/load, invalidation, multi-target execution, and stable
`vatsstate` inspection.

### Wave 8 — hostility, awareness, and combat AI

Goal: connect existing disposition/perception/navigation foundations to the
same combat engine used by the player.

- Resolve faction relations, disposition, aggression/confidence, damage
  provocation, friendly fire, and reputation into explicit hostility changes.
- Advance awareness from measured LOS/hearing facts with deterministic
  hysteresis and search/loss behavior.
- Prepare preferred range, cover bias, retreat threshold, search duration, and
  tactical weights.
- Spatial/navigation adapters report stable cover candidates, path costs, and
  exposure; core selects tactics and candidate IDs.
- NPC fire, reload, ammo, condition, mitigation, limbs, and death use the same
  core policies as player combat.
- Keep dead, surrendered, unloaded, and non-hostile actors out of incompatible
  AI actions.

Acceptance includes detection, pursuit, ranged positioning, cover, melee
fallback, flee/surrender, friendly-fire transitions, deterministic candidate
selection, and save/load of awareness/hostility.

### Wave 9 — integration and hardening

Goal: prove the complete combat loop is coherent, persistent, deterministic,
and ready for the wider M5 gate.

- Remove remaining duplicate combat policy from viewer/VSA adapters.
- Record ordered intents, evidence, RNG draws, directives, and outcomes in a
  bounded deterministic replay stream with a stable hash.
- Reconstruct and compare item, actor, AP, limb, hostility, projectile, and RNG
  state in core-only replay tests.
- Define ordering/idempotency for two actors firing in one frame, target death
  during V.A.T.S., armor break on lethal impact, disappearing reload ammo,
  weapon drop during an action, actor unload in flight, and duplicate BRP
  delivery.
- Add bounded tracing and a deterministic batch/benchmark baseline; avoid
  unstable percentage performance claims.
- Verify migration from every supported save version and persistence at every
  pending action phase.

Final combat-track acceptance requires ordinary and V.A.T.S. exchanges, enemy
cover selection, forced reload/jam/armor/cripple cases, save during active
combat, reload, completion, and an identical replay hash from the same seed.

## Persistence and revision policy

Wave 2 established save v5 for implemented ammo/loadout state. Later waves do
not reserve speculative dormant persisted structures.

- Any later serialized field, enum shape, or changed meaning bumps the save
  version immediately; `serde(default)` is not a substitute.
- Any prepared serialized shape or decoded meaning change bumps its owning
  `*_REVISION` and the preparation revision.
- The scene manifest version changes only when its serialized shape or meaning
  changes; catalog-only changes use catalog/prepare revisions.
- Every revision has a pinned test.
- Migrations are deterministic and conservative: preserve existing canonical
  state, never manufacture or silently consume items, and reject irreconcilable
  references rather than dropping them.

Expected revision ownership:

| Wave | Likely revision owners |
| --- | --- |
| 3 | Save, item catalog, prepare, combat RNG, combat policy |
| 4 | Item catalog, prepare, combat policy; save if projectiles persist |
| 5 | Item catalog, prepare, combat policy |
| 6 | Actor catalog, prepare, combat policy; manifest if hit-zone metadata enters it |
| 7 | Save, item/actor catalogs, prepare, combat policy |
| 8 | Save, actor/faction catalogs, prepare, combat policy; manifest only for prepared cover meaning |
| 9 | No planned shape change; any new serialized field still requires a bump |

## Inspection and test contract

Wave 2 registered the durable command family:

```text
ammostate [player|reference-form-id]
combatstate [player|reference-form-id]
vatsstate [player|reference-form-id]
hitboxdebug state|on|off [reference-form-id]
```

Results remain structured values inside `ConsoleOutput.value` using schema
`bevyout.m5.inspect`. Unimplemented capabilities return successful typed
`available=false` results with their planned wave. Breaking wire changes bump
the inspection schema; additive fields do not. Arrays sort by stable ID, times
are milliseconds, percentages are basis points, and ECS entity IDs never
appear.

Every wave follows this order:

1. Fix the feature list in GitHub and the wave plan.
2. Write Cucumber scenarios and pure core/unit tests.
3. Add serialization/migration and decoder/preparation tests when applicable.
4. Implement the core policy until pure suites pass.
5. Add thin Bevy/VSA adapters and minimal `App`/`World` tests.
6. Add structured console/BRP golden tests.
7. Run real-data acceptance and finish the manual.

Required repository gates:

```text
cargo fmt --check
cargo test -p bevyout-core
cargo test --test features
cargo test --test architecture
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Real-data acceptance uses Super-Duper Mart `00017f37`, 10mm Pistol `0000434f`,
10mm Round `00004241`, and Raider reference `00041600` where applicable.
Controlled AP/HP/surplus ammo, armor, hitbox, projectile, faction, and cover
cases require synthetic checked-in fixtures; Bethesda-derived RON, GLB, DDS,
WAV, NIF, or other game data must remain untracked.

Each manual records exact revisions, load order/fingerprints, preparation and
launch commands, FormIDs, setup/reset commands, before/after inspection,
visible expectations, save/reload steps, cleanup, and evidence paths.
Placeholder IDs, approximate numeric assertions, or screenshot-only policy
claims are merge blockers.

## GitHub wave procedure

- Every task is a sub-issue of epic #11, assigned to `@me`, labeled, and placed
  in the M5 milestone before execution.
- Add the wave checklist to #11 when scope becomes active; tick it only after
  tests, real-data acceptance, merged PR, and issue evidence hold.
- Use one integration branch/PR per wave with `Closes #NN` for every issue.
- Work sequentially when issues edit the same runtime/serialization seam.
- Verify review findings against actual code; fix confirmed findings and answer
  incorrect/out-of-scope findings with evidence.
- Create follow-up issues for acceptance discoveries rather than silently
  expanding a wave.
- Do not post externally under the user's identity without an explicit draft
  approval.

# M4 wave 13 — actor state and persistence (#110)

Single-issue wave under epic #9. The prepared actor catalog already resolves
NPC/creature templates, base values, factions, packages, appearance, inventory,
and stable ACHR/ACRE references. M3's canonical item ledger already supports
actor holders. This wave connects those foundations to one authoritative,
versioned mutable actor state used by normal game flow.

**Execution model recommendation (Codex runtime): Sol X-High.** Save-format
migration, actor-value precedence, canonical item ownership, cell eviction,
and a concurrent animation/navigation seam make correctness more important
than turnaround. Codex executes directly without subagents, per repository
guidance.

## Fixed feature list

### 1. Pure actor definition and value policy

- Separate immutable actor definition from mutable actor instance state, both
  keyed by stable base/reference FormIDs.
- Represent FO3 base values, S.P.E.C.I.A.L., skills, faction memberships/ranks,
  race/class identity, and authored package order without Bevy types.
- Resolve values in the explicit order template fallback, actor-base override,
  race/class/faction additive modifiers, then persisted runtime mutation.
- Keep derived/effective values computed and non-persistent. Reject non-finite
  values and invalid stable identity deterministically.
- Keep faction membership/rank separate from relationship or hostility policy.

### 2. One mutable state authority

- Store per-cell actor instance state by reference FormID in
  `PersistentWorldState`; seed an actor once and never reset an existing
  instance on cell revisit.
- Persist lifecycle (`Alive`/`Dead`), actor-value mutations, and the minimal
  active-package checkpoint needed by the later package executor.
- Keep transform, enabled, activated, and enable-parent state in the existing
  reference delta. Do not duplicate those fields in actor state.
- Keep inventory and equipment exclusively in the canonical `ItemLedger`
  actor holder. Repeated projection/revisit must neither duplicate inventory
  nor replace stable item-instance/equipment bindings.

### 3. Save format v4 and migration

- Add a deterministic actor-state record to format v4 with strict duplicate,
  identity, finite-number, lifecycle, and package-state validation.
- Migrate v1-v3 saves with an empty actor-state map while preserving existing
  reference, player, dropped-item, and canonical item data exactly.
- Unknown records remain skippable; malformed known actor records fail with a
  stable diagnostic. Encoding order is cell then reference FormID.

### 4. Runtime projection and visible diagnostics

- Load and revision/hash/fingerprint validate the per-cell actor catalog for
  startup and neighbor preload, sharing it by resident cell like animation
  catalogs.
- Join prepared definition, persisted actor state, generic reference delta,
  canonical holder/equipment, and live projection in `actorstate <reference>`.
- Add a narrow developer mutation surface for actor-value deltas and lifecycle
  state; it writes the same state future scripts/combat consume.
- Fix placement capture so assetless NPC/creature proxies are treated as
  spawnable and retain transform/presence across eviction.
- Actor state must remain available when its visual entity is evicted or was
  never renderable.

## Tests first

1. Add Cucumber scenarios for value precedence, faction/rank separation,
   one-time instance seeding, lifecycle/package persistence, and canonical
   actor inventory conservation.
2. Add core unit/property tests for deterministic resolution, finite-value
   validation, stable identity, and revisit idempotence.
3. Add save v4 round-trip/migration/corruption/order tests, including v3
   fixtures and unknown records.
4. Add minimal-World tests for proxy placement capture, actor catalog
   lifecycle, console mutation/inspection, canonical equipment projection,
   eviction, and loaded-save restoration.

## Gates and real-data acceptance

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Reprepare Super-Duper Mart and Vault 101 Atrium with the native scene and
  actor-animation converters into an external cache.
- In the normal viewer, inspect/mutate/save/revisit/restart Raider `00041600`
  and Radroach `0005443b`; verify stable reference/base identity, values,
  factions, canonical equipped instance, lifecycle/package checkpoint, and no
  inventory duplication.
- Record save-size growth, actor restore count/latency, actor count, and stable
  diagnostics. Use the agent bridge and exact manifests, not animation-zoo.

## Non-goals

- Combat damage, death animation, ragdoll-to-corpse conversion, respawn timing,
  actor effects/progression, script mutations, companions, or hostility policy.
- AI package selection/execution; this wave persists only a narrow checkpoint
  owned later by #115.
- Navmesh, locomotion, animation selection, animation-zoo, or external Nifty
  changes.

## Shipped amendments

- Fallout 3 `RACE.DATA` contributes its seven signed skill/boost pairs to the
  race modifier layer. The prepared actor catalog revision was bumped to
  `openmw-actors-v6-runtime-values-race-skill-modifiers`; class tag-skill IDs
  and faction membership remain identity data rather than invented numeric
  bonuses.
- Assetless actor proxies are now included by generic reference persistence.
  This closes a pre-existing hole where the visual proxy existed in gameplay
  but its transform/presence was omitted from cell capture.
- Real-data acceptance used the native Nifty converter in normal gameplay.
  Super-Duper Mart prepared 11 actors and 1,380 clips; Vault 101 Atrium
  prepared 17 actors and 1,396 clips. Raider `00041600` restored lifecycle,
  health mutation, package procedure/time, and item instance IDs `1`/`2`
  after a process restart. Radroach `0005443b` resolved creature-only values
  and applied health `5 -> 4` without invented humanoid data.
- The real Super-Duper Mart save was 61,159 bytes at neutral state and 61,201
  bytes with one health mutation plus package checkpoint: 42 bytes of mutable
  actor payload. An unmodified actor record costs 41 bytes; all seven spawned
  actors restore in the first gameplay update after save application.
- Combat death presentation and locomotion suppression remain non-goals here.
  The persisted `ActorLifeState` is exposed through `ActorStateRuntime` for the
  concurrent #188 navigation/animation integration to consume after its seam
  lands.

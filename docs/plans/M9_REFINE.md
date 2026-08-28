# Review of `M9-Work`

## Verdict

**The branch is a substantial architectural implementation, but it is not ready to merge as “M9 complete.”** I would block that merge on four correctness issues: shot-identity collisions, destructive/premature cell resets, fast-travel validation bypass, and the complete absence of Wave 8/V.A.T.S.

The branch contains eight commits touching roughly 123 files. Six commits implement M9 Waves 4, 5, 6, 7, 9, and 10; Wave 8 is skipped, while the first two commits contain unrelated GPU-validation and Raylib-export work that should be split out. ([GitHub][1])

That matters because the M9 baseline explicitly defines a complete RPG foundation encompassing limb combat, repair/crafting/barter, stealth/crime, minigames, V.A.T.S., lifecycle, fast travel, and Pip-Boy integration. 

I reviewed the source and branch diff statically. I could not execute the Cargo quality gates in this environment, so test and Clippy status remain unverified.

---

# Merge blockers

## 1. Shot IDs collide after changing or re-equipping weapons

`LimbState` keeps every applied `ShotId` in a persistent `BTreeSet`, and duplicate IDs cause the complete impact to be rejected. The viewer constructs that ID directly from `WeaponState.shots_fired`. However, synchronizing a newly equipped weapon reconstructs `WeaponState`, resetting its shot counter. ([GitHub][2])

A realistic failure sequence is:

```text
Equip weapon A
Shoot raider X -> ShotId(1), accepted

Equip weapon B
Its counter starts over
Shoot raider X -> ShotId(1), rejected as duplicate
```

This can make legitimate damage silently disappear. The persistent set also grows without a bound for every actor that is repeatedly hit.

**Required correction:** use a globally unique or composite impact identity, for example:

```rust
struct CombatImpactId {
    combat_generation: u64,
    weapon_instance: ItemInstanceId,
    shot_sequence: u64,
}
```

A bounded transaction-receipt or replay-window mechanism would be preferable to an indefinitely growing per-actor set.

---

## 2. `resetcell` can reset a cell 72 hours early

When `resetcell` targets an unregistered cell, the command registers it with a reset deadline of `now + 72 hours`, then immediately passes that future deadline to `apply_cell_reset`. The core reset operation checks that the supplied deadline equals the registered deadline, but it does **not** check whether the deadline has actually been reached. ([GitHub][3])

Therefore, a freshly registered cell can immediately satisfy the equality check and reset despite not being due.

The reset implementation is also destructive rather than restorative: eligible container and actor holders are replaced with empty item vectors, corpses are removed, and counters named `reset_containers` or `respawned_actors` are incremented without rebuilding loot tables or actor state from prepared reference data. ([GitHub][4])

This falls short of the roadmap requirement to regenerate unowned container loot, revive eligible non-unique actors, and clean temporary entities. 

**Required corrections:**

```rust
if reset_due_game_ms > clock.absolute_game_ms {
    return Err(CellResetError::NotDue);
}
```

Then replace the empty-holder behavior with an atomic reset plan based on prepared base/reference templates:

```text
Prepared reset template
    -> classify preserved references/items
    -> generate replacement contents
    -> validate all holder mutations
    -> commit holder + actor reset together
    -> record reset-generation receipt
```

The current implementation risks permanently deleting canonical inventories.

---

## 3. Runtime fast travel bypasses all meaningful validation

The core defines appropriate fast-travel blockers, but the console/runtime adapter constructs evidence with:

```text
discovered = true
in_combat = false
danger = false
interior = false
over_encumbered = false
continuous_damage = false
continuous_radiation = false
```

Those values are hardcoded rather than gathered from the actual player, location, perception, and destination state. ([GitHub][3])

The command commits the time advancement before emitting `DoorTravelRequested`. It does not first prove that the destination is prepared and loadable or resolve a valid arrival transform. If loading then fails, game time has already advanced. ([GitHub][3])

The baseline calls for real checks against combat/Danger, exterior state, encumbrance, continuous damage or radiation, and destination eligibility. 

**Required correction:** divide the operation into a true preflight and an atomic commit:

```text
Resolve destination
    -> prove discovered/prepared/loadable
    -> gather current player evidence
    -> core validates and returns FastTravelPlan
    -> reserve destination load
    -> advance time and lifecycle
    -> commit location/arrival
```

A console command may have an explicit `--force` form, but the ordinary command must not manufacture passing evidence.

---

## 4. Wave 8 is entirely absent

The branch sequence jumps from Wave 7 to Wave 9. The Wave 10 manual itself acknowledges that V.A.T.S. remains unavailable and identifies Wave 8 as blocked or missing. ([GitHub][1])

The roadmap requires:

* AP authority and costs,
* per-limb hit-chance calculation,
* FIFO tactical queue,
* targeting/playback/resolution state machine,
* interruption handling,
* runtime targeting and cinematic adapters. 

Wave 10 also advertises a `vats_probe` capability even though the method returns an unavailable placeholder. That is misleading to clients that use capability discovery. ([GitHub][5])

This branch can reasonably be described as **M9 Waves 4–7 and 9–10 foundations**, but it cannot be accepted as the M9 milestone until Wave 8 is implemented and included in the replay/save gate.

---

# High-priority correctness and architecture findings

## 5. Anatomical node interpretation is in the wrong layer

`BodyPartId::from_node_name` lives in the pure core and interprets strings such as head, skull, arm, and leg node names. The viewer retrieves Bevy `Name` values and passes them to this core heuristic. ([GitHub][6])

That reverses the project’s stated boundary: VSA/viewer code should convert engine-specific evidence into semantic values, while core should receive a `BodyPartId`. 

The current approach also creates fragile behavior across skeletons, creatures, modded meshes, naming conventions, and localized or generated nodes.

**Correction:** map nodes during actor preparation or collider construction:

```text
mesh/node metadata + actor skeleton
    -> viewer/VSA BodyPartMarker
    -> hit evidence containing BodyPartId
    -> pure limb kernel
```

Keep only semantic fallback policy—such as `Unknown -> Torso`—inside core.

---

## 6. Limb damage is not tied to final combat damage

The integration applies limb damage from the weapon’s base `damage` value. It does not appear to consume one authoritative final-damage receipt after armor, condition, criticals, and other combat modifiers. ([GitHub][7])

That can produce cases where:

* health loses heavily mitigated damage,
* the limb receives unmitigated base damage,
* a critical or condition modifier affects health but not the limb,
* normal fire and future V.A.T.S. fire disagree.

The roadmap explicitly says limb damage and V.A.T.S. must feed the existing ammunition, condition, and combat RNG pipelines rather than form a parallel path. 

Limb maximum health is also normalized around a fixed `100_000` value and forcibly raised to at least that value, while the roadmap specifies a pool proportional to actor maximum health. ([GitHub][2]) 

Finally, the implementation uses a 2,500-basis-point spread penalty per crippled arm, whereas the baseline says `+50%` for an arm. That may be an intentional rebalance, but it needs to be recorded as a shipped amendment rather than silently differing from the accepted formula. ([GitHub][2]) 

---

## 7. Repair requests can manufacture repair authority

`RepairRequest` accepts `max_condition` from the caller. The operation validates only that it is nonzero, and missing item-condition state is treated as zero through `unwrap_or(0)`. The supplied maximum then controls the repair result and cap. ([GitHub][8])

That means a core caller can potentially:

* repair an item with no condition model,
* supply a false maximum,
* supply repair skill above 100,
* calculate results that disagree with the prepared item definition.

The console currently derives some values from catalogs, but the canonical operation itself remains open to forged inputs.

**Correction:** pass a prepared, versioned repair definition or have the transaction service retrieve it:

```rust
struct RepairDefinition {
    base_form_id: FormId,
    compatibility_group: RepairGroupId,
    max_condition_milli: u32,
    revision: u32,
}
```

Reject conditionless items and repair skills outside the allowed range at the core boundary.

---

## 8. Crafting tiers are represented but have no effect

The crafting transaction carries `SchematicTier`, but output items are created with `ItemState::default()` regardless of whether the schematic is V1, V2, or V3. Tier is copied into the receipt without altering starting condition. ([GitHub][9])

The console also always submits V1, leaving V2 and V3 unreachable from the runtime adapter. ([GitHub][10])

The wave plan records this as a deferred amendment, so it is documented rather than hidden. Nevertheless, it is still an incomplete roadmap deliverable because schematic tiers are required to provide bonus starting condition. ([GitHub][11]) 

There is also an arithmetic hardening issue: duplicated ingredient requirements use checked multiplication but subsequently accumulate with unchecked `+=`, allowing extreme recipe quantities to overflow. ([GitHub][9])

---

## 9. Barter commits trust caller-constructible quotes

`BarterQuoteInput` accepts a caller-provided base value. `quote_barter` derives the price from that value, and `commit_barter` primarily validates holder revisions before applying the quoted unit price. The quote structure is publicly constructible. ([GitHub][12])

This means the commit boundary does not independently prove that:

* the base value matches the item definition,
* the direction and quantity match the current transaction,
* the modifiers match the current player and merchant,
* the quote was actually issued by the core.

**Correction:** either recompute the quote during commit or return an opaque/core-verifiable quote token containing a hash of all canonical inputs and relevant revisions.

Holder revisions protect against stale inventory, but they do not protect against a fabricated price.

---

## 10. Merchant “restock” advances metadata without restocking inventory

`restock_if_due` increments generation and timestamps and consumes an RNG draw, but it does not mutate merchant stock. Its catalog argument is unused. Wave 9 invokes it with an empty/default stock catalog. ([GitHub][12])

The result is a restock scheduler in name only:

```text
deadline reached
generation increments
next deadline advances
merchant inventory remains unchanged
```

The baseline requires a merchant inventory restock ledger every 72 in-game hours. 

Restocking should produce an explicit atomic receipt listing removed, preserved, and generated `ItemInstanceId`s, the catalog revision, generation, and RNG draw range.

---

## 11. Faction ownership is not connected to the runtime player

The ownership core supports factions, but the runtime claim classifier always passes `TakerFactions::default()`, which is empty. ([GitHub][13])

Consequently, faction membership or rank can never make a faction-owned item legal for the player. Every faction-owned claim is effectively judged as if the player belongs to no faction. The wave plan acknowledges this runtime limitation. ([GitHub][14])

This should be connected to the existing faction membership/rank authority before Wave 6 is accepted.

---

## 12. Witness evidence contains fabricated legal facts

The viewer constructs witness candidates with hardcoded:

```text
alive = true
enabled = true
hostile_to_victim = false
```

The core trusts those fields when deciding witness eligibility. ([GitHub][13])

Therefore dead, disabled, or victim-hostile actors may report a crime. The required victim-faction alliance test is not actually supplied.

The runtime must gather these facts from canonical actor and faction state. Spatial LOS and distance may be measured by the viewer, but legal allegiance and actor life/enabled state should not be invented there.

Other Wave 6 scope gaps include:

* no `Pickpocket` crime kind despite its presence in the roadmap,
* assault and murder mostly existing as core/test concepts rather than being wired to combat outcomes,
* owner/faction FormIDs being converted into actor-style target identity in portions of the adapter,
* duplicate crime handling updating stolen provenance before the duplicate-report check. ([GitHub][15]) 

---

## 13. Stealth evidence is only a partial implementation of the promised kernel

The core sums light, movement, armor noise, and observer perception, but distance and angle operate mainly as pass/fail gates rather than attenuation terms. The viewer uses movement speed without posture and assigns a fixed armor-noise value whenever apparel is present instead of deriving it from weight or sound footprint. The default field of view is 180°, not the specified 190°. ([GitHub][16])

The roadmap requires light, movement and posture, equipped armor weight/sound, distance/LOS attenuation, observer Perception, and a 190° cone. 

There is also a state-quality bug: last-known position is populated as `[distance, 0, 0]`, because detection evidence contains no actual world position. That is not a location that AI can meaningfully search after losing the player. ([GitHub][16])

Add a quantized world-space position to evidence and preserve the project boundary by validating floats before converting to integer millimetres.

---

## 14. Lockpick inventory consumption is not atomic with session mutation

`apply_torque` mutates the lockpick session—torque state, sequence number, phase, and stress—before attempting to consume a bobby pin. If ledger consumption then fails, the function returns an error while retaining the already-mutated session. ([GitHub][17])

This violates the intended transaction invariant:

```text
session transition + pin consumption
either both commit or neither commits
```

The function should validate against cloned state and commit only after both the session and ledger mutation succeed.

Two additional problems deserve tests:

* `delta_ms.max(1)` creates stress even for a zero-time input.
* Difficulty/skill gates permit some unexpectedly under-skilled attempts—for example, the checks are not a straightforward `skill >= difficulty` rule and need either justification or correction. ([GitHub][17])

---

## 15. Hacking generation is not meaningfully seeded and cannot generate bracket tricks

The hacking constructor filters and sorts the provided words, consumes an RNG draw without using it to select or place anything, accepts a caller-selected password, and creates an empty bracket collection. ([GitHub][17])

As implemented:

* changing the seed does not meaningfully change the board,
* the promised selection of `N` thematic words is absent,
* 4–12 character constraints are not properly established by generation,
* bracket-pair reset/dud mechanics cannot arise from a generated board,
* the console starts an explicitly synthetic session. ([GitHub][3])

This implements a useful likeness/session kernel, but not the complete deterministic RobCo minigame described by Wave 7. 

Terminal access also lacks the ownership/trespass reporting path that lock interaction is expected to share with Wave 6.

---

## 16. The authoritative game clock is frozen during ordinary play

`GameTimeRuntime` defaults to a timescale of zero, and the viewer plugin explicitly keeps that default. The previous active-effect ticker exits when the new lifecycle runtime exists. Together, this means effects do not decay during ordinary gameplay; they advance only through explicit time-jump operations such as `passtime` or fast travel. ([GitHub][18])

The wave plan documents zero timescale as an amendment, so the behavior appears intentional. ([GitHub][19])

It is still unsuitable as the default for a complete gameplay milestone. A fixed-point/rational conversion from real elapsed milliseconds to game milliseconds can preserve determinism without freezing time.

---

## 17. Lifecycle tasks are ordered, but effects are not processed at their true deadlines

For every time advance, Effects, Radiation, and Death tasks are scheduled at the end timestamp. Effects are then ticked once using the full interval. Intermediate merchant or cell-reset tasks can execute before an effect that should have expired earlier in the interval. Radiation and Death task handlers are currently no-ops. ([GitHub][4])

For example:

```text
Start: 10:00
Effect expires: 11:00
Merchant restock: 12:00
Advance directly to: 13:00

Current ordering can process restock at 12:00
before ticking the effect at the 13:00 task.
```

The scheduler needs actual effect/tick/withdrawal deadlines, or it must segment a large advancement at every due timestamp.

There are two related scheduling defects:

* restored restock/reset deadlines that are already earlier than the loaded clock may never execute because due-range processing excludes the starting timestamp,
* default tasks scheduled exactly at the current time are similarly excluded and can remain orphaned. ([GitHub][4])

---

## 18. Save v9 decoding accepts incomplete RPG state too readily

The `RPGS` decoder requires `HEAD`, but missing `STAT`, `PERK`, `EFCT`, `RADS`, `ADDI`, `LIMB`, or RNG data can silently fall back to defaults. Duplicate subrecords other than `HEAD` can overwrite prior values instead of being rejected. ([GitHub][20])

For a v9 save, a truncated or duplicated RPG section could therefore turn into apparently valid default progression rather than a corruption error.

`validate_save` also does not appear to perform semantic validation of the RPG state comparable to its validation of other save domains. ([GitHub][20])

Before the v9 format is frozen:

* define required versus optional subrecords for each `RPG_SAVE_REVISION`,
* reject duplicate singleton records,
* validate probabilities, durations, health, limb-map completeness, RNG state, and finite health values,
* add malformed and truncation fixtures,
* decide whether Wave 6 and Wave 9 additions require a section-revision bump or whether branch history will be squashed into the first public revision.

The roadmap requires deterministic migrations and pinned round-trip tests for every revision. 

One wording correction is also warranted: migration from old saves can deterministically initialize newly introduced RPG fields, but it cannot be literally lossless for information those save versions never contained.

---

## 19. Pip-Boy and inspection are not yet one shared projection

The shared `RpgInspectionSnapshot` direction is good, but the Pip-Boy body meters read `progression.limbs` directly rather than consuming the limb values from the shared inspection snapshot. ([GitHub][21])

That allows UI and BRP/console output to diverge if inspection applies normalization, fallback, revision reporting, or actor selection logic.

The branch also has only a partial Wave 10 presentation:

* V.A.T.S. remains a placeholder.
* The full SPECIAL/Skills/Perks navigation is not implemented as specified.
* Data-view quests, objectives, notes, holotapes, and world-map travel remain unavailable or deliberately deferred. ([GitHub][14])

Those are legitimate cross-milestone dependencies, but they should be reflected in status as incomplete rather than treating Wave 10 as closed. The target Wave 10 scope is explicit in the roadmap. 

---

# What was done well

Despite the blockers, several foundations are worth retaining.

### Core-first decomposition

The new work generally puts calculations and state machines in `bevyout-core`, uses typed outcomes, keeps runtime adapters relatively thin, and supplies dedicated modules for limbs, repair, crafting, barter, crime, minigames, lifecycle, and inspection. That aligns with the governing architecture. 

### Deterministic representation

There is broad use of integer milliseconds, basis points, stable item and target identities, ordered maps/sets, explicit RNG state, and versioned snapshots. This follows the baseline’s determinism rules and is a much stronger foundation than implementing these systems as arbitrary Bevy timers and components. 

### Documentation and test scaffolding

The branch adds wave plans, manuals, feature specifications, unit-test coverage, and inspection-oriented commands. The commit structure makes the intended wave boundaries reasonably inspectable, even though unrelated work and the missing Wave 8 weaken the final integration branch. ([GitHub][6])

### Early save integration

Introducing RPG persistence before the final presentation wave was the right architectural move. It lets limb, crime, time, and progression features develop against actual persistence rather than attempting a large retrofit at the end. The decoder now needs stricter validation, but the sectioned `RPGS` direction is sound.

### Honest amendments

The plans explicitly identify several deferred or altered behaviors, including schematic tiers, runtime faction membership, synthetic hacking, zero default timescale, and missing V.A.T.S. That honesty makes the remaining work measurable instead of hiding it behind nominally completed wave labels. ([GitHub][11])

---

# Recommended correction order

## Phase 1 — Prevent state corruption and false gameplay outcomes

1. Replace `ShotId(shot_index)` with globally stable impact identity.
2. Require `reset_due <= now`.
3. Stop emptying canonical holders until real reset templates exist.
4. Gather real fast-travel evidence and preflight destination loading.
5. Harden RPGS required-subrecord and duplicate validation.

## Phase 2 — Restore authority boundaries

1. Move node-name/body-part mapping out of core.
2. Route limb damage through one final combat-damage receipt.
3. Seal repair definitions and barter quotes against forged caller values.
4. Connect faction membership, actor life state, allegiance, and hostility to ownership/witness evaluation.
5. Make lockpick session and pin mutations atomic.

## Phase 3 — Complete the promised mechanics

1. Implement actual merchant stock generation.
2. Implement schematic-tier condition bonuses.
3. Implement seeded hacking board/bracket generation.
4. Finish stealth posture, armor weight/noise, attenuation, and world-space last-known position.
5. Enable real-time deterministic game-clock advancement.
6. Process lifecycle events at their actual chronological deadlines.
7. Implement the shared M5/M9 V.A.T.S. and AP state machine.

## Phase 4 — Freeze and prove the milestone

1. Route every Pip-Boy value through shared inspection snapshots.
2. Remove or accurately mark unavailable BRP capabilities.
3. Complete the required Wave 10 views.
4. Separate the unrelated infrastructure/export commits.
5. Run and archive the full repository gates:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run-dev -- report --input Fallout3.esm
```

These are the baseline merge gates, not optional cleanup. 

## Merge recommendation

**Do not merge `M9-Work` as the completed M9 epic.** Preserve it as a strong WIP/integration branch, split unrelated commits, fix the state-corruption blockers, and reclassify completed work as:

```text
Wave 4: substantial, combat integration corrections required
Wave 5: core foundations, several mechanics incomplete
Wave 6: core foundations, runtime evidence incomplete
Wave 7: partial minigame kernels
Wave 8: not implemented
Wave 9: clock/scheduler foundation, runtime lifecycle incomplete
Wave 10: partial inspection/Pip-Boy integration
```

The branch has a good deterministic core foundation, but its current runtime adapters frequently supply placeholders or synthetic evidence where the milestone requires canonical game state. That gap—not code volume—is the main issue preventing acceptance.

[1]: https://github.com/kelo221/bevyout/compare/master...M9-Work "https://github.com/kelo221/bevyout/compare/master...M9-Work"
[2]: https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/combat/limbs.rs "https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/combat/limbs.rs"
[3]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/console/world_commands.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/console/world_commands.rs"
[4]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/lifecycle.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/lifecycle.rs"
[5]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/agent_bridge.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/agent_bridge.rs"
[6]: https://github.com/kelo221/bevyout/commit/5f1d24c1aa733c26ffa2d36d4f6044d20a9ddc71 "https://github.com/kelo221/bevyout/commit/5f1d24c1aa733c26ffa2d36d4f6044d20a9ddc71"
[7]: https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/weapon.rs "https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/weapon.rs"
[8]: https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/repair.rs "https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/repair.rs"
[9]: https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/crafting.rs "https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/crafting.rs"
[10]: https://github.com/kelo221/bevyout/blob/M9-Work/src/viewer/console/repair_commands.rs "https://github.com/kelo221/bevyout/blob/M9-Work/src/viewer/console/repair_commands.rs"
[11]: https://github.com/kelo221/bevyout/blob/M9-Work/docs/plans/M9_WAVE5_PLAN.md "https://github.com/kelo221/bevyout/blob/M9-Work/docs/plans/M9_WAVE5_PLAN.md"
[12]: https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/barter.rs "https://github.com/kelo221/bevyout/blob/M9-Work/crates/bevyout-core/src/barter.rs"
[13]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/crime.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/crime.rs"
[14]: https://github.com/kelo221/bevyout/compare/master...M9-Work.patch "https://github.com/kelo221/bevyout/compare/master...M9-Work.patch"
[15]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/crime.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/crime.rs"
[16]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/detection.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/detection.rs"
[17]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/minigames.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/crates/bevyout-core/src/minigames.rs"
[18]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/game_time.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/game_time.rs"
[19]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/docs/plans/M9_WAVE9_PLAN.md "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/docs/plans/M9_WAVE9_PLAN.md"
[20]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/save/mod.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/save/mod.rs"
[21]: https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/pipboy/stats.rs "https://raw.githubusercontent.com/kelo221/bevyout/M9-Work/src/viewer/pipboy/stats.rs"

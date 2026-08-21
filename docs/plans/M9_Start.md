# M9 complete Fallout RPG systems — architecture and wave roadmap

## Purpose and status

This document defines the cross-wave architecture and delivery roadmap for
**Epic #19 (`[Epic] M9 — Complete Fallout RPG systems`)**.

M9 establishes the complete, headless-first RPG systems foundation for
`bevyout`: SPECIAL attributes, skills, derived statistics, perks, active
effects, radiation, chems/addiction, limb health and crippling, field repair,
schematic crafting, barter economics, stealth and crime detection, lockpicking
and hacking minigames, tactical V.A.T.S. queuing and execution, cell respawn
lifecycles, fast travel, and Pip-Boy data integration. Refer to OpenMW if you can for reference, don't reinvent the wheel.

| Wave | Deliverable | Status | Recommended Codex model | Recommended Claude model |
| --- | --- | --- | --- | --- |
| 1 | S.P.E.C.I.A.L., skills, derived stats, GMST foundation, leveling curves | Planned | Sol X-High | Opus |
| 2 | Perk and trait catalog, requirements evaluator, active perk modifiers | Planned | Sol High | Opus |
| 3 | Active effects engine, ingestibles (chems/aid), radiation, addiction | Planned | Sol X-High | Opus |
| 4 | Anatomical hit locations, limb health, crippling penalties, medical aid | Planned | Sol High | Sonnet |
| 5 | Item condition repair math, schematic crafting, merchant barter economics | Planned | Sol High | Sonnet |
| 6 | Environmental stealth detection, ownership, crime, witness LOS, Karma | Planned | Sol X-High | Opus |
| 7 | Headless minigames (hairpin lockpicking, RobCo terminal hacking) | Planned | Sol High | Sonnet |
| 8 | Tactical V.A.T.S. calculation kernel, AP queue, execution state machine | Planned | Sol X-High | Opus |
| 9 | World lifecycle, cell respawn clock, encounter zones, fast travel | Planned | Sol High | Sonnet |
| 10 | Pip-Boy UI views, save v9 migration, agent bridge probes, milestone gate | Planned | Sol X-High | Opus |

This document serves as the durable architecture baseline. At each wave
kickoff, create the wave's assigned GitHub sub-issues plus
`M9_WAVE<N>_PROMPT.md`, `M9_WAVE<N>_PLAN.md`, and `M9_WAVE<N>_MANUAL.md`. Any
departure from this roadmap discovered during real-data acceptance must be
recorded as a shipped amendment in the corresponding wave plan.

## Governing architecture

**Core owns causality. Viewer owns measured evidence and presentation.**

All RPG mechanics follow the strict Vertical Slice Architecture (VSA) and
domain-core boundaries defined in `AGENTS.md` and enforced by `tests/architecture.rs`:

```text
ESM4 records (GMST, AVIF, PERK, ALCH, INGEST, ENCH, SPEL, FACT, GLOB)
  -> VSA decode and preparation (src/vsa/prepare/)
  -> serde core catalog DTOs (PreparedRpgCatalog, PreparedPerkCatalog, etc.)
  -> core RPG decisions & math kernels (crates/bevyout-core/)
  -> Bevy spatial evidence & player input (src/viewer/)
  -> core outcomes & canonical mutations (ItemLedger, ActorState, SaveGame)
  -> Bevy presentation, HUD, Pip-Boy, audio, & typed inspection (src/viewer/, src/console/)
```

### Architectural invariants

1. **Pure Domain Core (`crates/bevyout-core`):**
   - Pure Rust structs, enums, calculations, state machines, and policies.
   - Depends **strictly on `std`, `serde`, and `glam`** (`tests/architecture.rs`).
   - Zero dependencies on Bevy, filesystem IO, `serde_json`, or rendering.
   - 100% testable headlessly against vanilla Fallout 3 GECK / Gamebryo formulas
     and `GMST` settings.
2. **VSA Preparation Layer (`src/vsa/`):**
   - Decodes ESM4 records (`GMST`, `AVIF`, `PERK`, `ALCH`, `INGEST`, etc.) and
     produces immutable, versioned prepared catalogs (`*_REVISION`).
   - Must never depend on `src/viewer/`.
3. **Bevy Presentation & Runtime Adapters (`src/viewer/`):**
   - Implements thin Bevy ECS plugins (`ViewerPlugins`), components, queries,
     systems, UI/HUD, interaction modals, and audio triggers.
   - Respects file modularity and root line caps: interaction coordinator capped
     at 250 lines, console coordinator capped at 150 lines.
4. **Console & Inspection Layer (`src/console/` & `src/viewer/console/`):**
   - Thin Bevy adapters over engine-agnostic command grammar implementing
     `ConsoleCommandProvider`.
5. **Canonical Persistence (`src/save/`):**
   - Authoritative save records (`SaveGame`, `ItemLedger`, `ActorState`).
   - Save version increments (e.g. Save Format v9) with deterministic migrations
     from legacy formats (v1–v8).
   - No dormant or speculative uninitialized structures.

## Stable boundary and determinism contracts

### Inputs contract

- `RpgIntent`: request stat mutation, spend AP, consume chem/aid item, attempt
  field repair, initiate barter transaction, attempt lockpick manipulation, submit
  terminal guess, enter/queue/execute/cancel V.A.T.S., or request fast travel.
- `SpatialEvidence`: ambient light level at actor position, movement speed /
  posture (standing/walking/running/crouched), equipped armor weight, line-of-sight
  occlusion, distance to observer, observer field-of-view cone (190°), and witness
  candidate IDs.
- Explicit integer elapsed milliseconds (`delta_ms: u32`).

No public core operation accepts a Bevy `World`, `Entity`, query, resource,
timer, asset handle, or rendering type.

### Outputs contract

- `RpgDirective`: accepted or blocked action, AP consumption confirmation,
  minigame state transition, or V.A.T.S. execution step.
- `RpgOutcome`: attribute delta, radiation accumulation delta, limb condition
  delta, active effect addition/expiration, chem addiction status transition,
  item condition delta, barter ledger delta, crime event broadcast, Karma delta,
  or XP gain / level-up event.
- Typed, serializable, read-only inspection snapshots.

### Determinism rules

- **Time:** All durations, ticks, and timers use explicit integer milliseconds.
- **Percentages & Probabilities:** Represented in basis points (`0..=10_000`
  representing `0.00%..=100.00%`), avoiding non-deterministic float rounding.
- **Identities:** Stable identities use `FormId`, `ItemInstanceId`, `ShotId`,
  or explicit string keys—never Bevy `Entity` IDs or pointer addresses.
- **Collections:** Ordered collections use `BTreeMap`/`BTreeSet` so execution and
  serialization order never depend on ECS query iteration order or hash seeds.
- **Randomness:** Non-deterministic `rand` is prohibited. All probability draws
  (speech checks, force lock odds, critical hits, chem addiction rolls, weapon
  jams, V.A.T.S. hit checks) consume a core-owned, versioned PRNG state with an
  inspectable draw index (`RpgRngState` / `CombatRngState`).
- **Validation:** Invalid FormIDs, out-of-range probabilities, negative durations,
  NaN, and infinity are rejected at core boundaries.

## Existing codebase foundations vs M9 scope

M9 builds upon and integrates with existing subsystems rather than duplicating them:

- **Actor Values & Mutations (`crates/bevyout-core/src/actor_state.rs`):**
  Extends existing `ActorValue`, `SpecialAttribute`, and `ActorSkill` enums into
  the full derived stats formula kernel and leveling engine.
- **Item Ledger & Transactions (`crates/bevyout-core/src/item_transaction.rs`):**
  Barter transactions, repair consumption, and chem consumption operate strictly
  through the canonical `ItemLedger` and `ItemInstanceId` invariants (#95).
- **Combat Pipeline (`crates/bevyout-core/src/combat/`):**
  V.A.T.S. and limb crippling feed directly into the authoritative weapon
  condition (`combat/condition.rs`), ammunition (`combat/ammo.rs`), and combat
  RNG (`combat/rng.rs`) pipelines.
- **Factions & Perception (`crates/bevyout-core/src/{faction,disposition,perception}.rs`):**
  Stealth detection and crime systems connect directly to the existing faction
  matrix, disposition calculations, and perception awareness levels.
- **Save Game Engine (`src/save/`):**
  Integrates RPG state into Save Format v9 via an explicit `RPGS` record.
- **Interaction & Modals (`src/viewer/interaction/`, `src/app_state/`):**
  Lockpicking, hacking, barter, and container repair UI plug into existing
  `GameplayModal` states and interaction capabilities.

---

## Wave-by-wave delivery sequence

### Wave 1 — S.P.E.C.I.A.L., skills, derived stats, GMST foundation, leveling curves

Goal: Establish the pure mathematical calculation kernels for character stats,
skills, derived attributes, leveling progression, and GMST game settings.

- **ESM4 & Preparation (`src/vsa/prepare/`):**
  - Decode `GMST` (Game Settings) records into `PreparedGmstCatalog`.
  - Decode `AVIF` (Actor Value Info) records for skill and attribute metadata.
  - Expose default fallback constants matching Fallout 3 GOTY defaults.
- **Core Domain (`crates/bevyout-core/src/stats/`):**
  - SPECIAL attributes (`Strength`, `Perception`, `Endurance`, `Charisma`,
    `Intelligence`, `Agility`, `Luck`), clamped to `1..=10` base, effective `1..=10`.
  - 13 Fallout 3 skills (`Barter`, `BigGuns`, `EnergyWeapons`, `Explosives`,
    `Lockpick`, `Medicine`, `MeleeWeapons`, `Repair`, `Science`, `SmallGuns`,
    `Sneak`, `Speech`, `Unarmed`), clamped `0..=100`.
  - Base skill formula:
    $$\text{Base} = 2 + (2 \times \text{Primary SPECIAL}) + \lceil \text{Luck} / 2 \rceil + \text{Tag Bonus (+15)}$$
  - Derived attributes calculation kernel:
    - **Max HP:** $\text{Base} + (\text{Endurance} \times \text{fAVDHealthEnduranceMult}) + (\text{Level} \times \text{fAVDHealthLevelMult})$ ($100 + \text{END}\times 20 + \text{LVL}\times 10$).
    - **Max AP:** $\text{Base} + (\text{Agility} \times \text{fAVDActionPointsAgilityMult})$ ($65 + \text{AGI}\times 3$).
    - **Carry Weight:** $\text{Base} + (\text{Strength} \times \text{fAVDCarryWeightStrengthMult})$ ($150 + \text{STR}\times 10$).
    - **Critical Chance:** $(\text{Luck} \times 100\text{ bps}) + \text{PerkBps}$.
    - **Damage Resistance (DR):** Armor DR + Perks + Chems (hard cap $85\%$).
    - **Poison/Rad Resistance:** $(\text{Endurance} - 1) \times 5\%$ base + gear (hard cap $85\%$).
  - Leveling and XP engine:
    - Level 1 to 30 XP requirement curve:
      $$\text{XP}(N) = \frac{(N-1) \times N}{2} \times 150$$
    - Skill points awarded per level: $10 + \text{Intelligence}$ (plus *Educated* $+3$).
  - Skill check formulas (percentile speech check, hard skill gates).
- **Runtime & Console Adapter (`src/viewer/`, `src/console/`):**
  - Components: `ActorStats`, `DerivedAttributes`, `Experience`.
  - Systems: `recalculate_derived_stats`, `experience_award_listener`.
  - Console commands: `getav`, `setav`, `modav`, `player.advlevel`, `player.rewardxp`.

### Wave 2 — Perk and trait catalog, requirements evaluator, active perk modifiers

Goal: Build the perk definition catalog, requirement predicate engine, and
declarative modifier pipeline.

- **ESM4 & Preparation (`src/vsa/prepare/`):**
  - Decode `PERK` records: FormID, EditorID, Name, Description, Ranks, Level
    requirement, SPECIAL/Skill requirements, and PRKE effect subrecords.
  - Generate `PreparedPerkCatalog` with `PERK_CATALOG_REVISION`.
- **Core Domain (`crates/bevyout-core/src/perks/`):**
  - Perk evaluation predicate: `can_take_perk(actor_stats, perk_id, current_rank) -> bool`.
  - Perk modifiers model: flat stat modifiers, multiplier modifiers, VATS cost
    discounts, XP bonuses (*Swift Learner*), and special flags (*Educated*,
    *Comprehension*, *Mysterious Stranger*, *Ninja*).
  - Perk rank container and progression history.
- **Runtime & Console Adapter (`src/viewer/`, `src/console/`):**
  - Component: `ActorPerks`.
  - Systems: evaluate available perks on level-up; apply active perk modifiers to
    stat derivation and combat resolution.
  - Console commands: `addperk`, `removeperk`, `hasperk`, `showperks`.

### Wave 3 — Active effects engine, ingestibles (chems/aid), radiation, addiction

Goal: Implement time-decay active effects, chem consumption, radiation poisoning,
and addiction state machines.

- **ESM4 & Preparation (`src/vsa/prepare/`):**
  - Decode `ALCH` / `INGEST` records (Stimpaks, RadAway, Rad-X, Med-X, Jet, Psycho,
    Buffout, Mentats, food/drink) with effects and addiction chances.
  - Generate `PreparedEffectCatalog` with `EFFECT_CATALOG_REVISION`.
- **Core Domain (`crates/bevyout-core/src/effects/`, `src/radiation.rs`, `src/chems.rs`):**
  - Active effects ledger: duration ticking, magnitude application, source tagging
    (`Item`, `Chem`, `Environment`, `Withdrawal`, `Perk`).
  - Radiation dosage tracker ($0..=1000$ Rads):
    - Absorption rate: $\text{RadsTaken} = \text{EnvironmentRadRate} \times (1.0 - \text{RadResistance})$.
    - Thresholds: Minor ($200\text{ Rads}: -1\text{ END}$), Advanced ($400: -2\text{ END}, -1\text{ AGI}$),
      Critical ($600: -3\text{ END}, -2\text{ AGI}, -1\text{ STR}$), Deadly ($800: -3\text{ END}, -2\text{ AGI}, -2\text{ STR}, -1\text{ INT}$), Fatal ($1000\text{ Rads}$: Death).
  - Chems and addiction engine:
    - Chem consumption applying temporary buffs and rolling addiction:
      `roll_addiction(chem_chance_bps, chem_resist_perks, rng) -> bool`.
    - Expiration triggers withdrawal debuffs until cured by doctor/item or re-dosed.
- **Runtime & Console Adapter (`src/viewer/`, `src/console/`):**
  - Components: `RadiationPool`, `ActiveEffectsList`, `Addictions`.
  - Systems: `tick_active_effects`, `update_radiation_penalties`, `chem_duration_system`.
  - Console commands: `rads`, `addrads`, `removerads`, `addchem`, `cureaddiction`.

### Wave 4 — Anatomical hit locations, limb health, crippling penalties, medical aid

Goal: Implement discrete body limb health pools, crippling penalties, concussion,
and surgical restoration.

- **Core Domain (`crates/bevyout-core/src/limbs.rs`):**
  - Limbs: `Head`, `Torso`, `LeftArm`, `RightArm`, `LeftLeg`, `RightLeg`.
  - Limb health pool proportional to actor max health.
  - Crippling thresholds and penalties:
    - `Head`: Concussion, Perception $-4$, visual blur request.
    - `LeftArm` / `RightArm`: Spread penalty $+50\%$, reload speed reduced, two-handed
      weapon penalty if both crippled.
    - `Legs`: Locomotion speed reduced to $60\%$ (one leg) or $40\%$ (both legs).
  - Limb damage application from combat impacts and fall damage.
  - Restoration rules: Stimpak direct application, Doctor treatment, sleeping in owned bed.
- **Runtime & Console Adapter (`src/viewer/`, `src/console/`):**
  - Component: `ActorLimbHealth`.
  - Systems: `process_limb_damage`, `apply_cripple_locomotion_penalties`.
  - Console commands: `damageactorvalue`, `healactorvalue`, `cripple`, `showlimbs`.

### Wave 5 — Item condition repair math, schematic crafting, merchant barter economics

Goal: Deliver two-item field repair calculations, custom schematic crafting, and
merchant barter transactions with restock cycles.

- **Core Domain (`crates/bevyout-core/src/repair.rs`, `src/crafting.rs`, `src/barter.rs`):**
  - Two-item field repair formula:
    $$\text{NewCond} = \text{Cond}_A + \text{Cond}_B + \left(\text{MaxCond} \times 0.25 \times \frac{\text{PlayerRepair}}{100}\right)$$
    $$\text{Cap} = \max(0.5, \frac{\text{PlayerRepair}}{100}) \times \text{MaxCond}$$
  - Schematic crafting engine: recipe ingredient verification against canonical
    `ItemLedger`, schematic tiers (v1, v2, v3) granting bonus starting condition.
  - Barter price factor equation based on Player Barter, Merchant Barter, and Charisma:
    $$\text{PriceFactor} = f(\text{PlayerBarter}, \text{MerchantBarter}, \text{PlayerCharisma})$$
  - Atomic two-holder transaction validation through `ItemLedger` (#95).
  - Merchant inventory restock ledger ($72$ in-game hours).
- **Runtime & Console Adapter (`src/viewer/interaction/`):**
  - Systems: `repair_interaction_system`, `barter_transaction_system`, `crafting_system`.
  - Console commands: `repairitem`, `barter`, `craftitem`.

### Wave 6 — Environmental stealth detection, ownership, crime, witness LOS, Karma

Goal: Establish the multi-factor detection kernel, ownership legality, crime
reporting, and Karma/disposition matrices.

- **Core Domain (`crates/bevyout-core/src/stealth.rs`, `src/crime.rs`):**
  - Detection calculation kernel combining:
    1. Light level sampled at actor position.
    2. Movement speed and posture (running, walking, crouching).
    3. Equipped armor weight and sound footprint.
    4. Distance and line-of-sight attenuation to observer.
    5. Observer perception skill and alertness cone (190°).
  - Detection states: `Hidden`, `Caution`, `Danger`.
  - Ownership model: `Ownership::Player`, `Ownership::Faction(FactionId)`, `Ownership::Actor(FormId)`, `Ownership::Unowned`.
  - Crime types: `Theft`, `Pickpocket`, `Trespass`, `Assault`, `Murder`.
  - Witness query: Observers with LOS and distance $< R_{\text{alarm}}$ allied with
    victim faction emit crime report and assign bounty.
  - Karma alignment scale: `Very Good`, `Good`, `Neutral`, `Evil`, `Very Evil`
    influencing faction reaction thresholds.
- **Runtime & Console Adapter (`src/viewer/`, `src/console/`):**
  - Components: `StealthState`, `CrimeBountyLedger`, `OwnershipTag`.
  - Systems: `evaluate_stealth_detection`, `witness_crime_listener`.
  - Console commands: `detectstate`, `crime`, `setownership`, `getkarma`, `modkarma`.

### Wave 7 — Headless minigames (hairpin lockpicking, RobCo terminal hacking)

Goal: Implement deterministic, headless state machines for hairpin lockpicking and
RobCo computer terminal decryption, with viewer interaction modal adapters.

- **Core Domain (`crates/bevyout-core/src/minigames/`):**
  - `LockpickSession`:
    - Inputs: `LockDifficulty` (`VeryEasy`=0, `Easy`=25, `Average`=50, `Hard`=75, `VeryHard`=100), `PlayerLockpickSkill`, `BobbyPinsRemaining`.
    - State: `sweet_spot_angle` ($[-90.0, 90.0]^\circ$), `pick_angle`, `cylinder_rotation` ($0^\circ \dots 90^\circ$), `bobby_pin_stress`.
    - Tolerance formula: $\text{Tolerance} = f(\text{Difficulty}, \text{Skill})$.
    - Force Lock odds calculation and pin breakage simulation.
  - `HackingSession`:
    - Inputs: `ScienceDifficulty`, `PlayerScienceSkill`.
    - Word generator selecting $N$ thematic dictionary words of length $4\dots 12$.
    - Likeness evaluator: `likeness(guess, correct) -> usize`.
    - Bracket trick parser (`()`, `[]`, `{}`, `<>`) resetting attempts or removing duds.
    - 4 attempts before lockout; output states: `Success`, `AttemptsRemaining(u8)`, `LockedOut`.
- **Runtime & Console Adapter (`src/viewer/interaction/`, `src/app_state/`):**
  - Modals: `GameplayModal::Lockpicking`, `GameplayModal::Hacking`.
  - Systems: `lockpick_interaction_system`, `terminal_hacking_system`.
  - Console commands: `lockpick`, `unlock`, `hackterminal`.

### Wave 8 — Tactical V.A.T.S. calculation kernel, AP queue, execution state machine

Goal: Build the deterministic V.A.T.S. targeting and tactical execution engine.

- **Core Domain (`crates/bevyout-core/src/combat/vats.rs`):**
  - Hit chance formula per target limb in basis points:
    $$\text{HitChance} = \text{Clamp}\Big(\text{BaseAccuracy} + (\text{WeaponSkill} \times 40\text{ bps}) + (\text{Perception} \times 200\text{ bps}) - (\text{Distance} \times \text{Decay}) - \text{CoverPenalty}, 0, 9500\text{ bps}\Big)$$
  - Action Point (AP) cost per shot:
    $$\text{ShotAP} = \text{WeaponBaseAPCost} \times \text{PerkAPMultipliers}$$
  - Action queue: FIFO queue of `{ target: FormId, limb: BodyLimb, ap_cost: u32 }`.
  - V.A.T.S. state machine: `Inactive` $\rightarrow$ `TargetingMode` $\rightarrow$ `Queued` $\rightarrow$ `PlaybackSequence` $\rightarrow$ `Resolution` $\rightarrow$ `Exit`.
  - Interruption handling: Target death, out of ammo, LOS obstruction, stagger.
- **Runtime & Console Adapter (`src/viewer/vats/`):**
  - Components: `VatsTargetSnapshot`, `VatsActiveQueue`.
  - Systems: `vats_targeting_input`, `vats_cinematic_camera`, `vats_playback_system`.
  - Console commands: `vatsstate`, `vatsexec`, `vatscancel`.

### Wave 9 — World lifecycle, cell respawn clock, encounter zones, fast travel

Goal: Implement in-game time tracking, 72-hour cell respawn policies, encounter zone
level locks, corpse cleanup, and fast travel calculations.

- **Core Domain (`crates/bevyout-core/src/world_lifecycle.rs`, `src/fast_travel.rs`):**
  - `GameTime` clock: Day, Month, Year, GameHour, Timescale.
  - Cell reset and respawn policy: 72 hours (`iHoursToRespawnCell`).
  - Respawn execution: resets unowned container loot tables, revives non-unique
    encounter actors, cleans up non-persistent corpses and temporary projectiles.
  - Encounter zones: level locking logic (min/max level clamp based on player level
    when first entered).
  - Fast travel validator:
    - Preconditions: No enemies in combat / Danger state, exterior cell, not
      over-encumbered, no continuous damage/radiation.
    - Time advancement formula: $\text{TravelHours} = \frac{\text{Distance}}{\text{fFastTravelSpeed}}$.
- **Runtime & Console Adapter (`src/viewer/`, `src/console/`):**
  - Resource: `GameTimeResource`.
  - Systems: `advance_game_time`, `cell_respawn_listener`, `fast_travel_system`.
  - Console commands: `passtime`, `fasttravel`, `resetcell`, `showgametime`.

### Wave 10 — Pip-Boy UI views, save v9 migration, agent bridge probes, milestone gate

Goal: Wire complete RPG models to Pip-Boy presentation views, upgrade save format
to v9, expose BRP inspection probes, and execute full milestone acceptance.

- **Pip-Boy Presentation (`src/viewer/pipboy/`):**
  - Status view: SPECIAL stats, Skills, Derived stats, Rads meter, Limb health paperdoll.
  - Items view: Weapons, Apparel, Aid, Misc, Ammo with live condition/weight/value.
  - Data view: Active quests, objectives, notes, holotapes, and world map fast travel.
- **Persistence (`src/save/`):**
  - Save format version 9: Add `RPGS` record storing player/actor stats, perks,
    radiation, limb damage, active effects, addictions, and game time.
  - Deterministic migration from Save Formats v1–v8.
- **Agent Bridge Probes (`src/viewer/agent_bridge.rs`):**
  - `bevyout.rpg_stats_probe`: JSON-RPC query for active player/actor RPG stats.
  - `bevyout.vats_probe`: Query returning limb hit chances for target entity.
  - `bevyout.active_effects_probe`: Query returning all active effects and durations.
- **Verification & Milestone Gate:**
  - Complete integration suite, determinism replay tests, and `M9_WAVE10_MANUAL.md`.

---

## Testing and verification strategy

Following the project's **feature-first testing invariant**:

```text
Fix feature list -> Write Cucumber features (*.feature) + Unit tests -> Implement until green
```

1. **Gherkin Feature Specifications (`features/`):**
   - `features/rpg_stats.feature`: Derived stat calculations, leveling curves, XP awards.
   - `features/rpg_effects.feature`: Radiation thresholds, chem duration, addiction rolls.
   - `features/rpg_repair_barter.feature`: Repair caps, schematic crafting, barter formulas.
   - `features/rpg_minigames.feature`: Lockpick tolerance, bobby pin stress, hacking likeness.
   - `features/rpg_vats.feature`: Limb hit-chance clamping ($0\dots 95\%$), AP queuing.
   - `features/rpg_world_lifecycle.feature`: 72-hour cell respawn, fast travel constraints.
   - Appended to `tests/features.rs` with `fail_on_skipped()`.
2. **Dedicated Unit Test Directories:**
   - Unit tests reside in dedicated `tests/` directories or `tests.rs` files (e.g.
     `crates/bevyout-core/src/stats/tests/`).
   - Implementation files remain clean and focused strictly on feature code.
3. **Headless ECS & Bare-World Tests:**
   - Systems and plugins tested using minimal headless `App` or bare `World` harnesses.
4. **Agent Bridge & Console Acceptance:**
   - Machine-readable BRP JSON-RPC methods and console commands exercise every
     subsystem headlessly during automated verification.
5. **Manual Acceptance Scripts:**
   - Each wave PR is accompanied by `docs/plans/M9_WAVE<n>_MANUAL.md` specifying
     exact prepared cells, console commands, FormIDs, and expected results.

## Persistence and revision policy

- Any serialized prepared shape change bumps its owning revision:
  - `RPG_CATALOG_REVISION`
  - `PERK_CATALOG_REVISION`
  - `EFFECT_CATALOG_REVISION`
  - `RECIPE_CATALOG_REVISION`
- Any serialized save state change increments `CURRENT_SAVE_FORMAT_VERSION`
  (Format v9 introducing the `RPGS` record).
- Every revision carries a pinned round-trip test.
- Legacy save migrations (v1–v8) are strictly deterministic and lossless.

## Repository quality gates

Before opening any wave PR or handing off milestone deliverables, all gates must pass:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run-dev -- report --input Fallout3.esm
```

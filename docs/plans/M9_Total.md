# M9 Waves 4–10 — surgical execution plan

I would treat the remaining work as **seven integration trains**, not seven isolated feature bundles.

The uploaded architecture is sound: core owns causality, the viewer supplies measured spatial evidence and presentation, and persistence stores canonical outcomes. Time must be integer-based, probabilities must use basis points, identities must be stable, and randomness must come from an inspectable core-owned PRNG.  

Current `master` already provides the important seams:

* Waves 1–3 live in the persistent-in-runtime `PlayerProgression`, including stats, perks, radiation, effects, addictions, and current health.
* The save format is still v8 and has no `RPGS` record.
* RCPE recipe preparation already exists.
* `ItemLedger` already owns stable item instances, holder revisions, caps, provenance, and atomic buy/sell/transfer operations.
* Perception already declares `AwarenessState` as the single target authority.
* The Pip-Boy already has Stats, Items, Data, and a positioned body-condition figure. ([GitHub][1])

## 1. Amend the roadmap before Wave 4

### Amendment A1 — Merge M9 combat ownership with M5

M9 Wave 4 overlaps M5 Wave 6 almost exactly, and M9 Wave 8 overlaps M5 Wave 7. M5 also already defines the authoritative shot sequence, semantic hit-location boundary, AP economy, V.A.T.S. execution, persistence, and inspection contracts. These must not become parallel implementations. ([GitHub][2])

| Capability                         | Authoritative implementation                         | M9 responsibility                                        |
| ---------------------------------- | ---------------------------------------------------- | -------------------------------------------------------- |
| Body parts, limb damage, crippling | Shared **M5 W6 / M9 W4** implementation in `combat/` | Medical restoration, RPG projection, Pip-Boy integration |
| AP and V.A.T.S.                    | Shared **M5 W7 / M9 W8** implementation              | Perks/effects/stats integration and milestone acceptance |
| Awareness and target acquisition   | Existing `perception.rs`, extended by M9 W6          | Stealth factors, ownership, crime, Karma                 |
| Combat hostility and tactics       | M5 W8                                                | Consume M9 crime/awareness outcomes                      |
| Damage delivery and armor          | M5 W4–W5                                             | Hard dependency for final Wave 4 combat integration      |

The shared issues should carry both epic labels and land through one implementation. In particular:

* Do not create both `crates/bevyout-core/src/limbs.rs` and `combat/limbs.rs`.
* Do not create a second `vats.rs` under an M9-specific namespace.
* Do not let M9 crime maintain one target while M5 combat AI maintains another.

### Amendment A2 — Introduce save v9 in Wave 4, not Wave 10

Waiting until Wave 10 would make limb damage, crime, AP, and world time nonpersistent for most of the roadmap. That conflicts with the wave-level save/load acceptance already expected by the combat roadmap.

Recommended structure:

```text
Save format v9, introduced by Wave 4

RPGS
  HEAD  envelope/revision metadata
  STAT  player stats/progression
  PERK  player perk progression
  EFCT  active effects
  RADS  radiation
  ADDI  addictions
  LIMB  player limb state

Later active sections:
  CRIM  bounty/Karma/global crime state
  VATS  player AP and active V.A.T.S. session
  TIME  authoritative game clock
```

Use a stable, sorted subrecord envelope with a revision per section. Only emit sections that are implemented; there should be no dormant placeholder payloads. The existing save implementation already relies on unknown-record skipping for forward compatibility, so the same mechanism can be reused for RPG sections. ([GitHub][3])

Per-actor state should **not** be duplicated in `RPGS`:

* NPC limb and awareness state belongs in the existing persistent `ActorInstanceState`.
* Item condition, ownership, and stolen provenance remain in `ItemLedger`.
* Merchant inventory remains an `ItemLedger` holder.
* `RPGS` owns player-specific and global RPG state.

Wave 10 then becomes the **v9 migration-hardening and schema-freeze wave**, rather than the first persistence implementation.

A related correction: legacy saves cannot losslessly recover M9 state that was never serialized. Since current `SaveGame` contains no `RPGS` while those values currently live in `PlayerProgression`, v1–v8 migration must initialize missing M9 sections from versioned defaults. It can be deterministic and conservative, but it cannot reconstruct absent historical radiation, effects, or perk changes. ([GitHub][1])

### Amendment A3 — Move all elapsed-world-time mechanics onto Wave 9’s clock

Three mechanics currently cross wave boundaries:

| Mechanic                             | Defining wave | Runtime activation |
| ------------------------------------ | ------------: | -----------------: |
| Owned-bed limb restoration           |             4 |                  9 |
| Merchant 72-hour restock             |             5 |                  9 |
| Active-effect and withdrawal ticking |             3 |      Migrated to 9 |

Waves 4 and 5 should implement pure policies accepting explicit `GameTime`, but should not introduce independent frame timers. Wave 9 wires all three to one scheduler.

### Amendment A4 — `Hidden/Caution/Danger` is a projection, not a second authority

The current perception implementation explicitly says `AwarenessState` and `ActorAwareness` are the single authoritative target-awareness state. M9 Wave 6 should enrich that model rather than add an independent `StealthState` with another target, timer, or confidence value. ([GitHub][4])

Recommended split:

* `AwarenessState`: authoritative per-observer detection progress and acquired target.
* `HostilityState`: authoritative faction/combat relationship, owned by M5.
* `Hidden/Caution/Danger`: player HUD projection derived from all relevant observers.

### Amendment A5 — Build one inspection projection for Pip-Boy, console, and BRP

Wave 10 should not separately calculate values in:

* Pip-Boy systems,
* console providers,
* JSON-RPC probes,
* save diagnostics.

Create one serializable `RpgInspectionSnapshot` family and feed all four consumers from it. The existing agent bridge already has a central registration point and current Pip-Boy code already has the three main surfaces needed for integration. ([GitHub][5])

## 2. Dependency graph

```text
M5 W4 ballistics
    -> M5 W5 armor
        -> shared M5 W6 / M9 W4 limbs
            -> shared M5 W7 / M9 W8 V.A.T.S.

M9 W4 save-v9 envelope
    -> M9 W5 repair/crafting/barter
    -> M9 W6 stealth/crime
    -> M9 W7 minigames

M9 W3 effects
M9 W4 bed-healing policy
M9 W5 restock policy
M9 W6 Danger/hostility evidence
    -> M9 W9 game time and lifecycle

W4–W9
    -> M9 W10 Pip-Boy, migration, probes, full gate
```

Wave 5 and most of Wave 6 can progress while M5 ballistics/armor are being finished. Wave 8 must not start its execution layer until M5 W4–W6 are authoritative.

---

# Wave 4 — Body parts, limb health, crippling, and medical aid

The source roadmap calls for six anatomical pools, crippling penalties, combat/fall damage integration, and Stimpak/doctor/owned-bed restoration. 

**Merge boundary:** a measured semantic body-part hit mutates canonical actor state exactly once, derives penalties exactly once, survives unload/save/load, and projects into gameplay and UI.

The viewer’s current hitscan path already finds the hit entity’s actor ancestor, retrieves the persistent actor state, and calls core `resolve_actor_impact`. Extend this path instead of building another damage listener. ([GitHub][6])

| Order | Issue-ready slice                                                 | Primary implementation                                                                                                                                                                       |
| ----: | ----------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   4.0 | **Converge M9 W4 with M5 W6 and probe real body-part data**       | Freeze `BodyPartId`, supported actor types, BPTD/node evidence, damage multipliers, fallback policy, and verified GMST inputs. Record unresolved real-data fields rather than guessing them. |
|   4.1 | **Introduce save v9 and active `RPGS` sections**                  | Persist Waves 1–3 player state plus player limbs. Extend `ActorInstanceState` for NPC limbs. Add v1–v8 deterministic defaults and pinned round trips.                                        |
|   4.2 | **Implement semantic anatomy and limb-state kernels**             | Add `combat/body.rs`, `combat/limbs.rs`, and a focused `medical.rs`. Store stable limb IDs, maximum/current condition, cripple transitions, and restoration outcomes.                        |
|   4.3 | **Map viewer geometry to stable body-part evidence**              | Add semantic markers to prepared actor nodes/colliders. Extend `weapon/hitscan.rs` to return `BodyPartId`; unknown or unmarked nodes deterministically fall back to torso.                   |
|   4.4 | **Integrate damage, penalties, and restoration**                  | Resolve health and limb damage in the shared combat pipeline. Project head, arm, and leg penalties into perception, weapon spread/reload, locomotion, and derived statistics.                |
|   4.5 | **Add inspection, Pip-Boy proof, console, and manual acceptance** | Add structured `showlimbs` output, debug overlays, live body meters, targeted Stimpak application, manual script, and agent-readable snapshots.                                              |

### Core contracts

```rust
pub enum BodyPartId {
    Head,
    Torso,
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

pub struct LimbState {
    // Ordered by BodyPartId.
    pub parts: BTreeMap<BodyPartId, LimbCondition>,
}

pub struct LimbImpact {
    pub shot_id: ShotId,
    pub target: TargetId,
    pub part: BodyPartId,
    pub final_damage_milli: u32,
}

pub enum MedicalSource {
    TargetedStimpak,
    Doctor,
    OwnedBed,
}
```

The core should never receive a mesh path, node name, collider handle, or Bevy `Entity`. Node-to-body-part mapping is viewer/preparation work.

### Ordering rules to freeze in tests

1. Validate stable target and impact identity.
2. Reject duplicate impact evidence.
3. Apply armor and final health-damage calculation through the shared M5 pipeline.
4. Resolve body-part multiplier.
5. Apply limb damage.
6. Determine new cripple transitions.
7. Resolve lethal state.
8. Emit semantic outcomes in a deterministic order.

A simultaneous lethal and crippling hit must have one pinned result; it must not depend on which Bevy system runs first.

### Acceptance matrix

* All six body parts are reachable through synthetic fixtures.
* Unknown child node and missing semantic metadata fall back to torso.
* Crossing a cripple threshold emits one transition; further damage does not repeatedly emit `Crippled`.
* One and two crippled legs produce the specified 60% and 40% movement projections.
* Arm penalties affect the existing weapon calculation path, not animation-only values.
* Head injury produces the Perception penalty and a presentation request; blur remains presentation-only.
* Targeted Stimpak heals the selected limb and consumes exactly one canonical item.
* Doctor healing uses the same core restoration policy with a different source.
* Owned-bed healing is core-tested in Wave 4 but activated by Wave 9’s clock/ownership integration.
* Actor cell unload/reload and save/load restore exactly the same limb values.
* Duplicate `ShotId`/impact evidence does not apply damage twice.
* The Pip-Boy’s existing six condition meters use live values instead of current fixed demo percentages. ([GitHub][7])

**Do not:** introduce per-limb Bevy health components as authority, key state by entities, implement a second damage pipeline, or allow UI code to decide crippling.

---

# Wave 5 — Repair, schematic crafting, and barter

The roadmap combines repair math, schematic crafting, merchant pricing, atomic transactions, and merchant restocking. 

**Merge boundary:** every repair, craft, purchase, and sale is an idempotent canonical `ItemLedger` mutation with a receipt and no partial inventory/caps changes.

The repository already has the correct foundations:

* Stable `ItemInstanceId`.
* `HolderId::Player`, actor, container, merchant, world, and corpse holders.
* Holder revisions.
* Atomic buy/sell/transfer requests.
* Per-instance condition and ownership provenance.
* An existing prepared RCPE recipe catalog whose comments explicitly anticipate future crafting transactions. ([GitHub][8])

| Order | Issue-ready slice                                                     | Primary implementation                                                                                                                                                                                                           |
| ----: | --------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   5.0 | **Probe FO3 repair compatibility, barter GMSTs, and schematic tiers** | Verify compatible repair-item grouping, condition units, cap rounding, merchant terms, recipe condition subset, and schematic tier representation. Do not fill the roadmap’s unspecified barter `f(...)` with a guessed formula. |
|   5.1 | **Add reusable atomic inventory mutation planning**                   | Introduce an internal batch plan capable of removals, count changes, condition updates, output creation, and caps deltas under expected holder revisions. Commit all or none.                                                    |
|   5.2 | **Implement two-item field repair**                                   | Validate target/donor IDs, compatibility, skill, condition cap, equipped state, and stack semantics. Consume exactly one donor and emit `RepairReceipt`.                                                                         |
|   5.3 | **Implement schematic crafting over the existing recipe catalog**     | Validate known schematic tier, skill/condition predicates, ingredients, outputs, and unsupported GECK conditions. Consume inputs in stable instance-ID order and create outputs atomically.                                      |
|   5.4 | **Separate barter quotation from transaction commit**                 | `quote_barter` calculates price terms and rounding. `commit_barter` requires the quote’s expected player/merchant revisions and rejects stale quotes.                                                                            |
|   5.5 | **Add repair/craft/barter adapters and restock policy**               | Add interaction capability modules, structured console providers, minimal-App tests, and pure restock scheduling data. Actual 72-hour scheduling waits for Wave 9.                                                               |

### Repair transaction

Recommended request:

```rust
pub struct RepairRequest {
    pub transaction_id: TransactionId,
    pub holder: HolderId,
    pub target: ItemInstanceId,
    pub donor: ItemInstanceId,
    pub repair_skill: u16,
    pub expected_holder_revision: u64,
}
```

The receipt should expose:

* target and donor IDs,
* condition before and after,
* calculated repair cap,
* donor count consumed,
* holder revision before and after,
* formula/settings revision.

The existing roadmap formulas become golden tests, but condition units and rounding must be frozen explicitly. Never calculate using `f32` and then let platform formatting choose the result.

### Craft transaction

```rust
pub struct CraftRequest {
    pub transaction_id: TransactionId,
    pub holder: HolderId,
    pub recipe_form_id: u32,
    pub count: u32,
    pub expected_holder_revision: u64,
    pub actor_snapshot: CraftingActorSnapshot,
}
```

Important rules:

* Reuse `PreparedRecipeCatalog`; do not decode RCPE a second time.
* Unknown recipe conditions return a typed `UnsupportedCondition`, not “true.”
* Ingredients are selected deterministically by `(base_form_id, ItemInstanceId)`.
* Either every ingredient/output mutation succeeds or none do.
* Output `ItemInstanceId` allocation is deterministic and included in the receipt.
* A replayed `TransactionId` returns the original receipt without consuming ingredients again.

### Barter quote/commit split

```text
Player/merchant snapshot
    -> pure BarterQuote
    -> UI presents quote
    -> commit with expected holder revisions
    -> existing atomic Buy/Sell ledger operation
```

A quote should include all calculation terms:

* base value,
* player Barter,
* merchant Barter,
* Charisma/disposition terms if verified,
* perk/effect modifiers,
* buy/sell direction,
* basis-point factor,
* rounded unit price,
* quantity and total price,
* source catalog/settings revisions.

The UI must never pass an arbitrary user-selected `unit_price` directly into the ledger.

OpenMW is useful here for decomposition rather than numbers: its trading mechanics calculate the decision separately from presentation and obtain randomness through world-owned state. Likewise, its security mechanics separate the mechanical attempt, tool use, world mutation, and unlock-attempt notification. Copy that separation, not Morrowind’s formulas. ([GitHub][9])

### Restock boundary

Wave 5 should define:

```rust
pub struct MerchantRestockState {
    pub generation: u32,
    pub last_restock_game_ms: u64,
    pub next_restock_game_ms: u64,
    pub seed_state: RpgRngState,
}
```

Wave 5 can unit-test `restock_if_due(now, state, catalog)`. It must not run from `Time::delta_secs()` or an independent Bevy timer. Wave 9 invokes it after game-time advancement.

### Acceptance matrix

* Repair rejects identical target/donor IDs, unknown items, incompatible items, insufficient skill, stale revision, and equipped donor.
* Repair condition never exceeds its calculated cap.
* A donor stack loses one unit rather than the entire stack.
* Crafting handles partial stacks, multiple ingredients, multiple outputs, and duplicate base IDs.
* Failed crafting changes neither quantities nor next item ID.
* Buy/sell conserve total caps and item quantity.
* Stale quotes fail after either holder changes.
* A repeated transaction ID cannot duplicate items or caps.
* Ownership/stolen provenance transformations are explicit and tested; no operation silently erases provenance.
* Save/reload preserves repaired condition, crafted output identities, merchant inventory, and transaction receipts.

**Do not:** add another inventory model, reimplement recipes, calculate prices in UI, ignore unsupported recipe conditions, or schedule restocks with wall-clock/frame time.

---

# Wave 6 — Stealth, ownership, crime, witnesses, bounty, and Karma

The roadmap calls for multi-factor stealth, Hidden/Caution/Danger, ownership, theft and violent crimes, witness LOS, bounty, and Karma. 

**Merge boundary:** viewer evidence updates the existing authoritative awareness state; an action is legally classified once; witnesses produce one idempotent crime report; and bounty/Karma mutations survive save/load.

The current ownership helper explicitly treats every non-player owner as theft because faction membership was not available when it was written. `ItemLedger` already has owner/faction/stolen provenance, so Wave 6 should replace that temporary classification without replacing the ledger. ([GitHub][10])

| Order | Issue-ready slice                                                     | Primary implementation                                                                                                                                                                                      |
| ----: | --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   6.0 | **Version the perception contract for deterministic stealth**         | Preserve `TargetId` and single-target authority, but quantize elapsed time, confidence, distance, angle, light, movement, and noise at the core boundary. Add migration tests for existing awareness state. |
|   6.1 | **Implement deterministic stealth evidence and hysteresis**           | Combine gameplay light, movement/posture, armor noise, distance, LOS, observer Perception, and field-of-view into ordered score terms.                                                                      |
|   6.2 | **Replace temporary take/steal classification with ownership policy** | Resolve actor/faction/player/unowned ownership against faction membership and ranks. On illegal transfer, update canonical stolen provenance.                                                               |
|   6.3 | **Implement crime, witness reporting, bounty, and Karma**             | Add stable `CrimeId`, crime severity, witness eligibility, one-time reporting, faction bounty ledgers, and Karma outcomes.                                                                                  |
|   6.4 | **Connect viewer evidence and downstream combat/AI**                  | Viewer computes LOS and deterministic gameplay-light/noise evidence. M5 hostility consumes crime and awareness events; Wave 6 does not directly choose combat tactics.                                      |
|   6.5 | **Persist and inspect stealth/crime state**                           | Persist actor awareness in actor state and player bounty/Karma in RPGS. Add `detectstate`, `crime`, `setownership`, `getkarma`, and `modkarma` structured outputs.                                          |

### Deterministic evidence contract

Recommended core evidence:

```rust
pub struct DetectionEvidence {
    pub observer: TargetId,
    pub subject: TargetId,
    pub distance_mm: u32,
    pub angle_millidegrees: u32,
    pub light_bps: u16,
    pub movement_noise_bps: u16,
    pub armor_noise_bps: u16,
    pub observer_perception: u16,
    pub has_line_of_sight: bool,
    pub delta_ms: u32,
}
```

The viewer may use floats to perform geometry, but it validates and quantizes once before entering core. NaN and infinity must be rejected rather than normalized into an accidental detection result.

Do not sample rendered screen brightness or GPU luminance. Gameplay light should be derived deterministically from prepared cell ambient values, weather/time projection, and CPU-known local light influences.

### Awareness and HUD relationship

```text
Per-observer AwarenessState
    + observer hostility
    -> aggregate HUD state

No aware hostile observers       -> Hidden
Suspicious/not fully acquired     -> Caution
At least one hostile acquisition  -> Danger
```

`Danger` is therefore not saved as a separate authoritative flag. It is reconstructed from canonical observer and hostility state.

### Crime flow

```text
Illegal action intent
    -> ownership/legal classification
    -> CrimeEvent with stable CrimeId
    -> viewer supplies sorted WitnessEvidence
    -> core selects eligible witnesses
    -> one CrimeReport
    -> one bounty/Karma mutation
    -> optional hostility/disposition outcomes
```

Witnesses must be sorted by stable reference FormID before evaluation. Multiple witnesses may be listed in the receipt, but they must not multiply the bounty unless the rules explicitly define that behavior.

Recommended crime identity:

```rust
pub struct CrimeId {
    pub actor: TargetId,
    pub sequence: u64,
}
```

### Acceptance matrix

* Each stealth factor has an isolated golden vector.
* Equivalent evidence in different ECS query orders produces the same result.
* Score oscillation around a threshold does not flicker due to hysteresis.
* Two equally valid targets resolve by stable priority and FormID.
* Unowned and player-owned items classify as legal.
* Actor- and faction-owned items respect membership/rank policy.
* An unwitnessed theft marks provenance but produces no bounty report.
* A witnessed theft reports once.
* A witness behind a wall, outside alarm range, dead, disabled, or hostile to the victim is rejected.
* Assault escalating to murder creates a documented sequence without double charging the assault.
* Save/load preserves suspicion, acquired target, last-known position, bounty, Karma, and crime sequence counters.
* M5 combat reads the same acquired target; there is no duplicate target component.

**Do not:** create a second `StealthState` authority, use ECS entities in crime records, read GPU light values, apply bounty once per witness, or let the viewer choose guilt.

---

# Wave 7 — Headless lockpicking and terminal hacking

The source requires deterministic state machines for hairpin lockpicking and RobCo terminal hacking, followed by thin interaction-modal adapters. 

**Merge boundary:** identical initial session state plus identical ordered input produces the same output, item consumption, lock/terminal mutation, and PRNG draw index without Bevy.

| Order | Issue-ready slice                                            | Primary implementation                                                                                                                                                     |
| ----: | ------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   7.0 | **Define shared minigame session protocol and RNG domains**  | Add stable session IDs, explicit input sequence numbers, terminal states, cancellation rules, inspection snapshots, and domain-separated noncombat PRNG draws.             |
|   7.1 | **Implement integer lockpicking state machine**              | Add difficulty/skill validation, sweet spot, pick angle, torque/cylinder rotation, stress, pin break, force-lock, success, and failure outcomes using integer angle units. |
|   7.2 | **Implement deterministic hacking session**                  | Add word-bank preparation, board generation, likeness, attempts, bracket-pair parsing, dud removal, attempt reset, success, and lockout.                                   |
|   7.3 | **Commit minigame outcomes to world/item/crime authorities** | Consume bobby pins through `ItemLedger`, mutate lock/terminal state idempotently, and emit illegal unlock/access attempts into Wave 6’s crime boundary.                    |
|   7.4 | **Add Lockpicking and Hacking gameplay modals**              | Extend `GameplayModal`, isolate input/cursor/audio/presentation, and guarantee clean cancellation and modal transitions.                                                   |
|   7.5 | **Add console, BRP snapshots, and manual acceptance**        | Add deterministic session setup/step inspection, `lockpick`, `unlock`, and `hackterminal` adapters plus synthetic and real-data manual steps.                              |

### Lockpick representation

Do not store gameplay angles as floats:

```rust
pub struct PickAngleMilliDegrees(i32);   // -90_000..=90_000
pub struct CylinderAngleMilliDegrees(u32); // 0..=90_000
pub struct PinStress(u16);
```

Suggested input protocol:

```rust
pub enum LockpickInput {
    SetPickAngle(PickAngleMilliDegrees),
    ApplyTorque { delta_ms: u32 },
    ReleaseTorque,
    ForceLock,
    Cancel,
}
```

A session step must either:

1. atomically update session and any required inventory mutation, or
2. leave both unchanged.

A broken pin cannot advance the session while failing to consume the pin. Prefer a core service that receives both `LockpickSession` and `ItemLedger` rather than an outcome followed by a fallible viewer-side removal.

### Hacking representation

Separate:

* word-bank preparation,
* deterministic board construction,
* pure likeness evaluation,
* session state,
* presentation grid.

Rules to pin:

* only board words are valid guesses,
* every bracket pair can be used at most once,
* duds are removed in stable board order after the PRNG chooses a valid candidate,
* attempt reset never exceeds the configured maximum,
* a fourth failed word reaches `LockedOut`,
* success/lockout world mutation is idempotent.

Bethesda-derived word banks or terminal text should remain local generated cache artifacts. Checked-in tests should use synthetic words.

### Save policy

The source does not require active minigame persistence. The lower-risk first implementation is:

* saving is explicitly unavailable while either minigame modal is active,
* cancellation returns to gameplay without consuming anything beyond already committed pin breaks,
* cell unload cancels the session deterministically,
* lock and terminal world state still persists after success/lockout.

Persisting active sessions should be a separate feature with its own `MINI` save section rather than an accidental partial serialization.

### Acceptance matrix

* Lockpick boundary angles and zero/full tolerance.
* Repeated torque produces identical stress/rotation sequences.
* Rejected inputs consume no PRNG draw.
* Pin break consumes exactly one canonical pin.
* Cancellation does not unlock or relock anything.
* Force-lock chance and draw index are inspectable.
* Likeness is exhaustive for unequal/equal synthetic words.
* Generated boards contain one solution and no invalid word lengths.
* Bracket pairs cannot be reused.
* Same seed and inputs yield byte-identical session snapshots.
* Owned lock/terminal attempts route through Wave 6’s crime mechanism.
* Modal close restores mouse, keyboard, pause, and camera state exactly once.

**Do not:** use runtime `rand`, use `f32` angle/stress state, mutate door state before success, consume pins in UI code, or silently reset an active session on save.

---

# Wave 8 — AP economy and deterministic V.A.T.S.

This must be the same implementation as M5 Wave 7. The uploaded M9 plan and the existing M5 roadmap both require a complete AP queue and execution state machine, not presentation-only targeting.  ([GitHub][2])

**Hard dependencies:**

* M5 W4 authoritative spread/range/delivery.
* M5 W5 armor and mitigation.
* Shared M5 W6/M9 W4 body and limb pipeline.
* M9 W2 perk modifiers.
* M9 W3 active effects.
* Canonical ammo, condition, jams, and combat RNG already present.

**Merge boundary:** V.A.T.S. queues stable actions, accounts for AP deterministically, rolls in core, and executes through exactly the same ammo/condition/armor/limb/death pipeline as normal attacks.

| Order | Issue-ready slice                                            | Primary implementation                                                                                                                                                 |
| ----: | ------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   8.0 | **Freeze one M5/M9 V.A.T.S. protocol**                       | Cross-link epics, define stable identities, phases, AP reservation/spend policy, snapshot validity, interruption semantics, and inspection schemas.                    |
|   8.1 | **Implement authoritative AP state**                         | Add current/max AP, regeneration, delay, reservations, perk/effect modifiers, clamping, actor/player persistence, and explicit millisecond ticking.                    |
|   8.2 | **Implement target and limb snapshots**                      | Viewer supplies stable candidate, distance, LOS, cover, and visible-limb evidence. Core calculates inspectable hit chances in basis points.                            |
|   8.3 | **Implement queue admission and cancellation**               | Add FIFO actions, stable action IDs, weapon/ammo/target/limb references, expected revisions, AP reservation, queue removal, and cancellation/refund policy.            |
|   8.4 | **Implement deterministic execution and interruption**       | Revalidate each entry and route accepted attacks through the normal combat facade. Handle death, unload, LOS change, ammo exhaustion, jam, stagger, and weapon switch. |
|   8.5 | **Add targeting UI, cameras, persistence, and replay tests** | Viewer owns overlays, slow motion, cameras, animation, and cinematic projectiles. Persist active queue/AP/RNG and prove mid-sequence reload determinism.               |

### AP authority

```rust
pub struct ActionPoints {
    pub current_milli: u32,
    pub max_milli: u32,
    pub reserved_milli: u32,
    pub regen_delay_remaining_ms: u32,
}
```

`max_milli` is derived from stats, effects, and perks. `current_milli` and reservations are mutable canonical state.

The AP policy must explicitly answer:

* Is AP reserved when an action enters the queue?
* When does reservation become spent?
* What cancellation phases refund?
* Does a target dying before execution refund?
* Does an action that fires but misses refund? It should not.
* What happens when a perk/effect changes maximum AP while a queue is active?

Freeze these before implementation. Do not let UI behavior accidentally define them.

### Snapshot and queue entry

```rust
pub struct VatsTargetSnapshot {
    pub target: TargetId,
    pub target_revision: u64,
    pub limbs: BTreeMap<BodyPartId, VatsLimbSnapshot>,
    pub captured_at_game_ms: u64,
    pub snapshot_hash: u64,
}

pub struct VatsQueueEntry {
    pub action_id: VatsActionId,
    pub target: TargetId,
    pub limb: BodyPartId,
    pub weapon: ItemInstanceId,
    pub ap_cost_milli: u32,
    pub targeting_snapshot_hash: u64,
}
```

The snapshot freezes targeting evidence for UI and initial admission. Execution still revalidates target life, loaded state, weapon binding, ammo, jam, and other hard blockers.

### Execution rule

```text
Targeting snapshot
    -> queue admission/AP reservation
    -> execution revalidation
    -> core hit/miss draw
    -> ordinary CombatIntent
    -> ordinary ammo/condition/armor/limb/death pipeline
    -> semantic V.A.T.S. outcome
    -> viewer cinematic playback
```

The cinematic projectile is evidence-free presentation. It cannot turn a core miss into a hit or select another limb.

### Persistence and replay

Save:

* AP current/max/reserved,
* active session ID and phase,
* ordered queue,
* current execution index,
* target/weapon stable IDs,
* combat RNG state and draw index,
* already committed action IDs.

On reload, the next action must neither repeat the previous shot nor skip an uncommitted action.

### Acceptance matrix

* Exact queue ordering and AP accounting.
* Queue admission rejects insufficient AP without mutation.
* Removing an entry applies the pinned refund policy once.
* Multi-target queues preserve insertion order.
* Hit chance clamps at the documented bounds.
* Same snapshot/seed gives the same hit/miss sequence.
* Target death before its entry follows the pinned invalidation/refund policy.
* Ammo depletion, jam, weapon switch, target unload, and LOS changes are independently tested.
* A normal attack and equivalent V.A.T.S. attack use the same condition, armor, limb, and death outcomes after hit determination.
* Saving after action N and reloading produces the same remaining outcomes and final replay hash.
* `vatsstate` and `bevyout.vats_probe` expose terms, queue, AP, phase, and draw indices without ECS entity IDs.

**Do not:** implement AP in UI resources, fork V.A.T.S. damage logic, use cinematic collision as authority, roll hit chance in Bevy, or key queue entries by `Entity`.

---

# Wave 9 — Game clock, cell lifecycle, encounter zones, and fast travel

The roadmap requires an authoritative game clock, 72-hour resets, encounter-zone locking, cleanup, and fast-travel validation/time advancement. 

The existing `time_of_day` core is a float-based 24-hour lighting interpolation policy. It is useful as a rendering projection, but it is not sufficient as the authoritative calendar/lifecycle clock. ([GitHub][11])

**Merge boundary:** one integer game-time advance drives effects, restocks, healing, lifecycle, and fast travel in stable chronological order.

| Order | Issue-ready slice                                      | Primary implementation                                                                                                                                                            |
| ----: | ------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|   9.0 | **Implement authoritative integer `GameTime`**         | Add absolute game-time representation, calendar conversion, rational/fixed-point timescale, wait/sleep/fast-travel advancement reasons, overflow checks, and v9 persistence.      |
|   9.1 | **Implement one ordered lifecycle scheduler**          | Schedule effect ticks/expiry, withdrawal, restocks, cell resets, owned-bed restoration, and future package deadlines by absolute due time.                                        |
|   9.2 | **Implement deterministic cell reset and cleanup**     | Add per-cell last-visited/reset-due/generation state, reset eligibility, container regeneration, non-unique actor respawn, corpse/projectile cleanup, and persistence exclusions. |
|   9.3 | **Prepare and lock encounter zones**                   | Decode verified ECZN data, lock level on first entry, apply min/max clamps, and persist the result independently of loaded ECS entities.                                          |
|   9.4 | **Implement fast-travel planning and commit**          | Validate conditions, calculate travel time, advance clock, process due tasks in pinned order, move the player, and load the destination.                                          |
|   9.5 | **Migrate Waves 3–6 onto game time**                   | Replace effect frame ticking, activate restock, activate owned-bed healing, and use authoritative Danger/combat evidence for travel blocking.                                     |
|   9.6 | **Add lifecycle inspection and full boundary testing** | Add `passtime`, `fasttravel`, `resetcell`, `showgametime`, scheduler snapshots, save/load tests, and real-data manual acceptance.                                                 |

### Authoritative clock

Recommended representation:

```rust
pub struct GameTime {
    pub absolute_game_ms: u64,
    pub fractional_timescale_remainder: u32,
}

pub enum TimeAdvanceReason {
    Realtime,
    Wait,
    Sleep,
    FastTravel,
    Console,
}
```

Calendar fields—year, month, day, hour, minute—should be derived from the absolute value under a versioned calendar policy. Rendering receives only a projection such as `hour_as_f32()`.

Do not continuously round scaled frame time independently in every subsystem. The clock advances once and emits one interval:

```rust
pub struct GameTimeAdvanced {
    pub from_game_ms: u64,
    pub to_game_ms: u64,
    pub reason: TimeAdvanceReason,
}
```

### Lifecycle scheduler

```rust
BTreeMap<GameTimeKey, Vec<LifecycleTask>>
```

Tasks at the same timestamp sort by a stable task-kind priority and stable owner ID. A single advancement may cross many deadlines; all due tasks must be processed in chronological order.

Example pinned order at one timestamp:

1. active-effect expiration/ticks,
2. radiation/withdrawal consequence,
3. actor or player death/state validation,
4. merchant restock,
5. cell reset eligibility,
6. destination load/arrival projection.

The exact order may be amended after acceptance, but it must never depend on Bevy scheduling.

### Cell reset model

```rust
pub struct CellLifecycleState {
    pub last_visited_game_ms: u64,
    pub reset_due_game_ms: Option<u64>,
    pub reset_generation: u32,
    pub occupied: bool,
}
```

Reset algorithm:

1. Reject reset while player-occupied or otherwise protected.
2. Load prepared base/reference reset metadata.
3. Diff against persistent world state.
4. Preserve persistent, unique, quest-owned, player-owned, and explicitly non-resettable references.
5. Replace eligible unowned container holders atomically.
6. Respawn eligible non-unique actors using stable reference IDs.
7. Remove eligible corpses and temporary projectiles.
8. Increment reset generation.
9. Record a receipt so the same due event cannot execute twice.

A container reset must coordinate with `ItemLedger`; it must not spawn a second holder alongside the old canonical holder.

### Encounter zones

Persist:

* zone FormID,
* first-entered game time,
* locked level,
* min/max settings revision.

Revisiting or reloading must not reroll the level. A cell associated with the same encounter zone must consume the same locked value.

### Fast travel commit order

```text
Build FastTravelEvidence
    -> validate
    -> produce FastTravelPlan
    -> commit plan once
       1. advance game time
       2. process due lifecycle tasks
       3. update player location
       4. request destination load
       5. emit arrival outcome
```

Evidence should explicitly include:

* hostile combat/Danger state,
* interior/exterior/current location,
* encumbrance,
* continuous damage/radiation state,
* destination discovery/eligibility,
* route distance,
* target location identity.

No viewer system should independently teleport first and advance time later.

### Acceptance matrix

* `71:59:59.999` does not trigger a 72-hour reset; the exact boundary does.
* A large time jump processes every crossed effect/restock/reset deadline.
* Saving just before a deadline and loading/advancing past it executes once.
* Loaded/occupied cells do not reset underneath the player.
* Unique actors and persistent references survive reset.
* Eligible non-unique actors respawn with the same stable reference identity.
* Player-owned/stolen items are not duplicated by container reset.
* Merchant restock has a stable generation and does not duplicate stock after reload.
* Fast travel is blocked independently by each precondition.
* Effects can expire and withdrawal can start during travel.
* Encounter-zone level is fixed at first entry and stable after save/load.
* Float lighting-hour projection does not feed back into authoritative time.

**Do not:** use `f32` hours as save authority, maintain independent subsystem timers, scan every cell every frame, reset an occupied cell, or recreate canonical holders without reset receipts.

---

# Wave 10 — Shared projections, Pip-Boy, migration hardening, probes, and milestone gate

The original roadmap places Pip-Boy views, save v9, BRP probes, and full acceptance here. Under the recommended amendment, Wave 10 freezes and proves the v9 design introduced in Wave 4. 

**Merge boundary:** all player-facing and machine-facing RPG views are projections of the same canonical state, all supported legacy saves migrate deterministically, and one end-to-end replay proves the milestone.

| Order | Issue-ready slice                                     | Primary implementation                                                                                                                                            |
| ----: | ----------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
|  10.0 | **Create shared RPG inspection/read models**          | Add serializable player status, limbs, effects, inventory, crime, V.A.T.S., and world-time snapshots with stable ordering and schema revisions.                   |
|  10.1 | **Bind Pip-Boy Stats to live state**                  | Replace current display defaults and demo condition values with live HP/AP/XP, SPECIAL, skills, perks, radiation, active effects, addictions, and limb condition. |
|  10.2 | **Complete Items and Data projections**               | Show live condition/weight/value; add world-map/fast-travel data; consume quest/objective state from M7 rather than inventing a second quest model.               |
|  10.3 | **Freeze save v9 and validate every migration path**  | Round-trip every active RPG section, test unknown/corrupt sections, migrate v1–v8, prove canonical item/actor preservation, and pin all revision constants.       |
|  10.4 | **Register BRP probes and structured console parity** | Add `bevyout.rpg_stats_probe`, `bevyout.vats_probe`, and `bevyout.active_effects_probe` using the same read models as Pip-Boy and console.                        |
|  10.5 | **Run full M9 acceptance and deterministic replay**   | Execute one scripted gameplay chain, save/reload at critical points, compare snapshots and replay hashes, record measured amendments, and close the epic gate.    |

### Shared snapshot family

```rust
pub struct RpgInspectionSnapshot {
    pub schema_revision: u32,
    pub player: PlayerRpgSnapshot,
    pub effects: ActiveEffectsSnapshot,
    pub limbs: LimbSnapshot,
    pub crime: CrimeSnapshot,
    pub vats: VatsInspectionSnapshot,
    pub world: WorldLifecycleSnapshot,
}
```

Required properties:

* no Bevy `Entity`,
* no asset handles,
* arrays sorted by stable ID,
* milliseconds for time,
* basis points for probabilities,
* explicit catalog and policy revisions,
* no calculated values that differ between UI, console, and BRP.

### Pip-Boy work

The current Pip-Boy already contains `Stats`, `Items`, and `Data`, and its Stats figure already positions six condition meters around a body image. Replace the fixed values rather than rebuilding the device. ([GitHub][12])

Recommended Stats subtabs:

```text
STATUS   SPECIAL   SKILLS   PERKS   EFFECTS
```

`STATUS` should display:

* player name and level,
* current/max HP,
* current/max AP,
* XP and next threshold,
* radiation stage,
* live six-part condition figure,
* active addictions,
* compact active-effect summary.

The Pip-Boy only formats snapshots. It should not calculate max HP, radiation stages, cripple status, item value, or fast-travel eligibility.

### Migration matrix

For each supported source version:

| Source | Required proof                                               |
| -----: | ------------------------------------------------------------ |
|  v1–v2 | World/player defaults, stable identities, no lost references |
|  v3–v4 | Canonical item-holder migration and ownership preservation   |
|  v5–v7 | Ammo/loadout/location/dialogue preservation                  |
|     v8 | Combat RNG state and draw index preservation                 |
|     v9 | Exact RPG section round trip                                 |

For new fields absent from an old save:

* initialize from a named, versioned default constructor,
* emit a migration diagnostic,
* never infer history from unrelated values,
* never silently consume or manufacture inventory,
* reject irreconcilable stable references rather than dropping them.

### BRP methods

```text
bevyout.rpg_stats_probe
bevyout.vats_probe
bevyout.active_effects_probe
```

Register them beside the current session, capability, snapshot, viewport, and console methods. Each response should include:

* schema revision,
* source/cached catalog revisions,
* stable target identity,
* canonical snapshot,
* availability reason when a subsystem is unavailable.

### Full milestone acceptance chain

Use one repeatable script with controlled synthetic fixtures plus the existing M5 real-data combat cell and IDs where applicable:

1. Load Super-Duper Mart.
2. Inspect initial RPG snapshot.
3. Apply radiation and consume a chem.
4. Damage and cripple a selected limb.
5. Heal the limb with a targeted Stimpak.
6. Repair a condition-damaged weapon.
7. Craft a synthetic verified recipe.
8. Quote and complete a merchant transaction.
9. Perform witnessed and unwitnessed theft fixtures.
10. Complete one lockpick and one terminal session.
11. Queue and execute multi-limb V.A.T.S. actions.
12. Save during a valid active V.A.T.S. phase and reload.
13. Fast travel or explicitly advance across an effect expiry and restock boundary.
14. Advance across the 72-hour cell reset boundary.
15. Reload the original cell.
16. Compare final save, ItemLedger, actor state, RPG snapshot, PRNG indices, and replay hash.

The existing M5 acceptance baseline names Super-Duper Mart `00017f37`, 10mm Pistol `0000434f`, 10mm Round `00004241`, and Raider reference `00041600`; continue using those where the test requires real combat evidence. Synthetic fixtures should cover controlled armor, body parts, recipes, locks, terminals, merchants, witnesses, and encounter zones. ([GitHub][2])

---

# 3. Per-wave issue and PR discipline

Keep **one integration PR per wave**, with the rows above as ordered sub-issues. Issues that edit the same authority—save encoding, `ItemLedger`, actor state, combat execution—should be implemented sequentially on the wave branch.

Every wave should create:

```text
docs/plans/M9_WAVE<N>_PROMPT.md
docs/plans/M9_WAVE<N>_PLAN.md
docs/plans/M9_WAVE<N>_MANUAL.md
```

The repository’s convention is to amend the plan rather than rewrite it when acceptance changes the implementation. ([GitHub][13])

Each plan needs these fixed sections:

| Section            | Required content                                                       |
| ------------------ | ---------------------------------------------------------------------- |
| Feature list       | Exact player-visible and headless behavior; explicit non-goals         |
| Ownership map      | Canonical state owner, adapters, persistence location                  |
| Contract           | Typed intents, evidence, directives, outcomes, snapshots               |
| Determinism        | Time units, probability units, stable IDs, ordering, RNG domains       |
| Data probe         | Records/GMSTs/FormIDs examined and unsupported findings                |
| Tests first        | Cucumber scenarios and core test vectors written before implementation |
| Revision table     | Every save/catalog/prepare/policy revision affected                    |
| Runtime proof      | Minimal `App`/`World` tests; no full viewer required for policy proof  |
| Inspection proof   | Console and BRP golden values                                          |
| Manual proof       | Exact cell, commands, IDs, expected values, save/reload steps          |
| Shipped amendments | Measured deviations and their linked issue/PR evidence                 |

The testing roadmap already mandates feature-first work, dedicated core tests, minimal headless Bevy harnesses, machine-readable inspection, and per-wave manual scripts. 

Suggested feature files:

```text
features/rpg_limbs.feature
features/rpg_repair_barter.feature
features/rpg_stealth_crime.feature
features/rpg_minigames.feature
features/rpg_vats.feature
features/rpg_world_lifecycle.feature
features/rpg_pipboy_save.feature
```

Required final gate for every wave:

```powershell
cargo fmt --check
cargo test -p bevyout-core
cargo test --test features
cargo test --test architecture
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo run-dev -- report --input Fallout3.esm
```

Every serialized change needs a pinned revision and round-trip test; no skipped Cucumber scenario should reach the wave PR. 

## Recommended execution order

1. Record Amendments A1–A5 in the durable M9 roadmap.
2. Open the shared M9 W4/M5 W6 contract and data-probe issue.
3. Introduce the active save-v9/RPGS envelope.
4. Land Wave 4 core anatomy and medical policy while M5 ballistics/armor finish.
5. Execute Waves 5 and 6 against the new persistence boundary.
6. Execute Wave 7 core after the crime contract is stable.
7. Finish shared limb combat integration after M5 armor.
8. Execute shared M9 W8/M5 W7 V.A.T.S.
9. Centralize time and lifecycle in Wave 9.
10. Use Wave 10 strictly for common projections, migration hardening, BRP/UI parity, and full milestone proof.

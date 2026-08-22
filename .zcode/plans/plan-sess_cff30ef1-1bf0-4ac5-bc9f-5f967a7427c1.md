## Implementation plan

**Execution model recommendation: Luna X-High.** This fix spans a prepared-asset schema, pure domain policies, Bevy scheduling, canonical inventory use, console/Pip-Boy integration, real-data CTDA semantics, and acceptance documentation. Per the Codex-runtime project rule, execution will happen directly on `m9-wave3`, not through implementation subagents.

### 1. Lock the regressions down first

Extend `features/rpg_effects.feature` and its existing `tests/features.rs` RPG-effects seam before production changes:

- projected max health and action points use chem-modified/radiation-penalized SPECIAL;
- active RadResist converts percentage points to capped basis points;
- real-shaped Stimpak conditions select 30 HP without Fast Metabolism and 36 HP with it;
- unsupported ingestible conditions remain skipped.

Add focused unit/runtime tests before implementation for:

- `getav` SPECIAL, action points, RadResist, and current health;
- Buffout/radiation changes to derived maximum health and AP;
- effect expiry restoring projections in the same update;
- current health clamping down when maximum health falls, without auto-healing when it rises;
- Rad-X reducing both `addrads` and irradiating ingestibles;
- Pip-Boy button and row use changing the canonical ledger, refreshing `PlayerInventory`, and applying effects;
- console `useitem` refreshing the projection and applying a real-shaped Stimpak;
- Stimpak behavior with and without Fast Metabolism;
- stale projection/canonical mismatch failing without consuming, applying effects, sound, or a success notice.

Update the stale Buffout expectation from base Strength `5` to projected Strength `7`.

### 2. Preserve the actual Stimpak condition in the prepared catalog

In `crates/bevyout-core/src/effects.rs`:

- replace `IngestibleEffect.conditioned: bool` with an optional serialized condition containing the already-decoded `oper`, `comparison_value`, `function`, and `param1` fields;
- add a small pure tri-state evaluator for only the supported condition family: equality `HasPerk` (`449` / `0x1C1`) against `0.0` or `1.0`;
- use existing `PerkProgression::rank` as the ownership fact;
- return `True`, `False`, or `Unsupported`; unsupported or malformed conditions remain conservatively skipped.

At the prepare boundary, copy the existing parser wire fields instead of collapsing them to a Boolean. Update conditioned-effect counters to test `condition.is_some()`.

Bump `EFFECT_CATALOG_REVISION` from `openmw-effects-v1` to `openmw-effects-v2`, update the pinned catalog/round-trip tests, and require re-prepare. No parser redesign or generic CTDA engine will be added: base Stimpak needs exactly one already-decoded condition per effect.

### 3. Make effect-aware derived attributes a pure core policy

Refactor the existing formulas in `crates/bevyout-core/src/stats.rs` into a reusable function that accepts effective Strength, Endurance, Agility, and Luck. Keep `CharacterSheet::derived` using it for base-sheet behavior.

Add `projected_derived` beside `projected_special` in core effects:

- calculate SPECIAL from the sheet, active ledger, and radiation;
- derive max health, max AP, carry weight, and critical chance from that projection;
- add direct active `ActorValue::ActionPoints` modifiers such as Jet;
- keep maxima nonnegative.

Do not treat instant Health restoration as a maximum-health modifier.

Add a pure `active_rad_resistance_bps` policy that converts active `RadResist` percentage points to basis points, floors negative totals at zero, and applies the existing 85% gameplay cap.

### 4. Correct runtime projection, ordering, and health clamping

Update `viewer/stats.rs` so `DerivedAttributes` reacts to changes in `ActorStats`, `ActiveEffectsList`, and `PlayerRadiation` and uses `projected_derived`.

Introduce a narrow effects runtime schedule ordering so mutation/ticking completes before projections, and projections complete before health clamping. This fixes the current one-frame stale expiry behavior while allowing SPECIAL and derived projections to run in parallel.

Add a health clamp system with these semantics:

- if projected maximum health falls, clamp current health down;
- if maximum health later rises, do not grant free healing;
- explicit healing remains the only way current health rises.

Use projected maximum health inside ingestible healing as well, so Stimpaks cannot heal above a radiation-reduced maximum.

Remove the uncommitted `UseIngestibleRequested` message and reader because it has no producer and cannot make inventory consumption plus effect application one synchronous operation.

### 5. Evaluate real Stimpaks narrowly and observably

Pass the existing player `ActorPerks` into the shared ingestible kernel.

For each effect:

- no condition: apply normally;
- supported condition true: apply;
- supported condition false: skip as a false branch;
- unsupported condition: skip conservatively.

Real Stimpak will therefore apply exactly one mutually exclusive effect:

- 30 HP when Fast Metabolism `00094EBF` is absent;
- 36 HP when it is owned.

Extend the application result to distinguish false conditions from unsupported conditions while retaining the existing aggregate skipped-conditioned count in console output for compatibility.

### 6. Give all radiation doses one resistance-aware runtime seam

Create one viewer effects helper that applies a radiation dose using the active-effects ledger and the core resistance conversion policy.

Route both current positive-radiation paths through it:

- `addrads`, the present environmental exposure seam;
- instant negative-Rads ingestible effects.

Update `addrads` help text so it no longer claims resistance is zero. Future environmental systems can reuse the same helper.

### 7. Make canonical Pip-Boy Aid use authoritative and synchronous

Add a narrow canonical lookup/use operation beside `CanonicalItemLedger` that deterministically resolves the matching player `StackKey`, calls `ItemLedger::use_item`, and then rewrites `PlayerInventory` from the canonical ledger.

Update both Pip-Boy activation routes to use that operation and invoke the same `apply_ingestible` kernel as console use. Preserve Pip-Boy-specific behavior at the UI edge:

- quest/inert classification;
- prepared pickup sound;
- existing notice wording and effect labels;
- tracing log.

Emit sound, success notice, and effects only after canonical consumption succeeds. Do not synchronize stale projection data back into the canonical ledger or resurrect missing items.

Update console `useitem` to use the same canonical operation and refresh `PlayerInventory`; preserve its current JSON/log contract. `addchem` remains a debug effect-only command and does not consume inventory.

### 8. Correct the console actor-value surface

Keep console reads synchronous from authoritative inputs:

- SPECIAL: projected value;
- Action Points: projected maximum including direct AP effects;
- Rad Resist: active percentage-point value;
- Health: `PlayerVitals.current_health`, because `getav health` is the observable current actor value needed to verify healing.

Make `setav health` and `modav health` mutate current health, clamped to `[0, projected max health]`, so manual acceptance can damage and heal the player deterministically. Maximum health remains available through the runtime `DerivedAttributes` projection and dedicated tests; it is not conflated with current health internally.

### 9. Amend shipped documentation and real-data acceptance

Amend `docs/plans/M9_WAVE3_PLAN.md` and `M9_WAVE3_MANUAL.md` rather than rewriting history:

- correct the prior claim that `0x1C1` is `GetHealthPercentage`;
- record that it is `HasPerk`, selecting the 30/36 HP Fast Metabolism branches;
- document that all other unsupported CTDA functions remain skipped;
- state that `openmw-effects-v2` requires re-prepare;
- replace the conditioned-skip Stimpak steps with exact `setav health`, `addchem`/`useitem`, `addperk 00094ebf`, `getav health`, inventory-count, Rad-X, and projected-stat checks.

### 10. Verification

Run focused tests as each seam lands, then the required gates:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `git diff --check`
- representative `cargo run-dev -- prepare --cell 000151e3` to regenerate and validate `openmw-effects-v2`

Finally run live real-data acceptance through the existing viewer/agent bridge for:

- Stimpak 30 HP without Fast Metabolism;
- Stimpak 36 HP with Fast Metabolism;
- canonical Pip-Boy or `useitem` consumption plus healing;
- Rad-X reducing `addrads` absorption;
- Buffout/radiation changing exposed SPECIAL and derived AP/health behavior.

No commits, pushes, or external posts are included.
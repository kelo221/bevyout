# M5 Wave 3 plan — condition, degradation, jams, and deterministic combat RNG

Roadmap source: [M5 combat architecture roadmap](M5_COMBAT_ARCHITECTURE_ROADMAP.md),
Wave 3. GitHub epic: [#11](https://github.com/kelo221/bevyout/issues/11).

## Execution recommendation

Use **Sol X-High** for execution in the Codex runtime. The wave crosses pure
combat policy, canonical item transactions, Bevy adapters, save migration, and
real-data verification; it needs sustained repository-wide reasoning while
keeping the final evaluation in the orchestrating session.

## Issue map and order

| Order | Issue | Scope | Depends on |
| --- | --- | --- | --- |
| 1 | [#262](https://github.com/kelo221/bevyout/issues/262) | Core RNG, condition effectiveness, and finite-input policy | Wave 2 / PR #248 |
| 2 | [#263](https://github.com/kelo221/bevyout/issues/263) | Canonical condition/degradation/jam transactions | #262 |
| 3 | [#266](https://github.com/kelo221/bevyout/issues/266) | Player/runtime integration and inspection/clear-jam surface | #262, #263 |
| 4 | [#265](https://github.com/kelo221/bevyout/issues/265) | Save/load, migrations, feature coverage, and real-data gate | #262, #263, #266 |

The issues are worked sequentially on `m5-wave3`: they share the combat and
weapon runtime seams, and the later issue must build on the earlier canonical
state rather than create a second authority.

## Fixed feature list

1. Add a versioned, domain-separated combat RNG state in the pure core. It
   carries a stable seed, policy revision, and draw index; deterministic draws
   use explicit domains/indices and finite validated inputs. Fire/reload policy
   reserves a draw only after the action is otherwise accepted. Rejected
   actions leave the draw index unchanged.
2. Extend prepared weapon data with the degradation inputs needed by the Wave 3
   policy. Condition is bounded by prepared maximum condition, decreases once
   per accepted shot, and remains attached to the same `ItemInstanceId` through
   inventory/container transactions. Define and test a non-zero minimum
   effectiveness floor and the exact condition-to-damage scaling curve in the
   core policy.
3. Add explicit jam state and stable jam reasons. Fire and reload decisions can
   enter a jam through the deterministic policy; a jam blocks subsequent fire
   and reload until the clear-jam intent succeeds. Clearing a jam is an atomic
   canonical transaction and is idempotent when no jam is present.
4. Route player fire/reload and the console adapter through the core decisions.
   Preserve Wave 2 ammo, recoil, screen feedback, audio, and actor-damage
   behavior. Extend `combatstate` with condition, jam, RNG revision/draw
   information, and the last decision terms; add a visible `weaponclearjam`
   command.
5. Persist only implemented Wave 3 condition, jam, and RNG state. Migrate old
   saves deterministically, reject malformed/non-finite values, and bump every
   affected `*_REVISION` constant. Planned policy identifiers are
   `m5-combat-v3`, save format v8 (the current branch already uses v7 for
   world-location persistence), item catalog
   `openmw-items-v10-combat-condition`, and prepare revision
   `prepare-v22-m6-worldspace-lod-imad-screen-fx-combat-condition`; keep these
   exact shipped strings synchronized in the amendments below.
6. Add a deterministic feature trace and dedicated unit tests, then run the
   real prepared Super-Duper Mart acceptance with the actual 10mm Pistol and
   10mm Round FormIDs. Record measured evidence on the issues and in the wave
   PR.

## Architecture boundaries

- `crates/bevyout-core` remains `std`/`serde`/`glam` only. Keep RNG, condition,
  jam, eligibility, and transaction decisions pure and testable.
- `src/vsa/prepare` owns prepared weapon inputs and revision changes. Do not
  put runtime state in prepared manifests.
- `src/viewer/weapon`, player, save, and console code are adapters. They may
  orchestrate core decisions and presentation, but must not become an
  independent condition/jam authority.
- Do not add armor, limbs, ballistics, VATS, AI, or multi-light/multi-weapon
  combat scope from later waves.
- Keep feature-driven modules free of Bevy imports and put unit tests in the
  repository’s dedicated test files/modules.

## Test-first sequence

1. Fix the feature list above and add `features/combat_condition.feature`.
   Append its `World` fields and a delimited step section at the end of
   `tests/features.rs`.
2. Add dedicated core tests for RNG draw reservation, finite values, condition
   scaling/degradation, jam transitions, ItemInstanceId preservation, and
   migration. Add prepare/save tests for catalog and serialized revisions.
3. Implement the core policy and canonical ledger operations until the pure
   tests and feature scenarios are green.
4. Implement the runtime/console adapter and the clear-jam inspection path;
   add Bevy-side harness tests for presentation/blocked reasons.
5. Run the full gates and the real-data manual script. Amend this plan with
   shipped revision strings or acceptance design changes before opening the PR.

## Acceptance gates

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- representative dynamic-linking prepare and viewer launch for cell `00017f37`
- two identical fresh combat traces produce the same decisions, condition
  values, jam transitions, and draw indices
- blocked/rejected actions do not advance the RNG draw index
- accepted fire degrades the weapon exactly once and damage follows the tested
  condition curve, including zero/minimum condition behavior
- a jam is observable, blocks fire/reload, and clears through the console
- save/load preserves partial condition, jammed state, and RNG continuation;
  old saves migrate deterministically
- the prepared-data revision/cache behavior is correct on real data
- issue comments contain measured deterministic/runtime evidence, and the
  manual script is linked from the wave PR

## Shipped amendments

- A1 (implementation): this checkout’s current save format is v7 because M6
  already owns world-location persistence, so Wave 3 uses save format v8 and a
  `CRNG` record rather than the roadmap’s stale v6 placeholder.
- A2 (implementation): the existing prepared `max_condition` field is enough
  to drive the fixed Wave 3 policy; no new prepared field was needed. Its
  decoded meaning is nevertheless revision-gated by item catalog
  `openmw-items-v10-combat-condition` and prepare revision
  `prepare-v22-m6-worldspace-lod-imad-screen-fx-combat-condition`.

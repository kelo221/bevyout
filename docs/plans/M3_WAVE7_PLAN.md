# M3 wave 7 — recipe preparation and corpse loot (#117, #118)

Wave under epic #7 on branch `m3-wave7` off the M3 wave 6 kickoff. Executors
run in isolated worktrees per AGENTS.md; the orchestrator owns GitHub
housekeeping, merges, conflict resolution, gates, and real-data acceptance.

## Fixed feature lists

### #117 — recipe records and deterministic crafting inputs

- Decode supported `RCPE` fields and ingredient/output FormIDs through the
  OpenMW-derived ESM4 importer.
- Emit a deterministic, content-fingerprinted prepared recipe catalog with
  serde-defaulted fields for older manifests/catalogs.
- Keep validation pure and std/serde-only: reject missing outputs, missing
  ingredients, duplicate ingredients, and non-positive quantities without
  partial state.
- No crafting execution or UI in this slice.

### #118 — corpse activation and loot transfer

- Represent a stable corpse loot holder with lossless stacks and reuse the
  existing container transfer policy/persistence boundary where possible.
- Extend `activate <FormID>` or the narrowest console seam needed to open a
  staged corpse through the same scripted/player path, with stable errors and
  logs.
- Support take-one, take-stack, take-all, close, and save/reload without loss
  or duplication.
- No actor death, AI, crime consequences, or barter/economy.

## Tests first

- #117: synthetic ESM4 recipe fixtures and pure catalog ordering,
  fingerprint/reuse, compatibility, and validation tests.
- #118: pure corpse transfer policy, console-harness activation tests, and
  save round-trip tests including old saves without corpse sections.

Each executor owns all production and test code for its issue. The shared
`tests/features.rs` seam is avoided unless an executor proves a cucumber
scenario is necessary; if it is needed, append only at the end as required by
AGENTS.md.

## File ownership

- #117 owns the OpenMW record importer, prepared recipe data/catalog modules,
  recipe-focused fixtures/tests, and only the manifest/preparation call sites
  required to publish the catalog.
- #118 owns corpse policy/runtime/console/save changes and corpse-focused
  fixtures/tests. It may touch existing interaction/transfer UI modules only
  for the corpse seam.
- Neither executor edits the other issue's files or the in-flight #98/#99
  implementation.

## Gates and acceptance

Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, and a representative `cargo run-dev` command after integration.
For #118, use the bevyout agent bridge on a prepared real cell when a corpse
reference exists; otherwise record synthetic/console evidence and explain the
real-data limitation. Viewer changes use tracing; CLI preparation output is
deterministic.

## Shipped amendments

(none yet — add acceptance-driven changes here rather than rewriting the
fixed feature lists.)

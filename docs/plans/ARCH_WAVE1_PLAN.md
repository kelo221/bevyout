# Architecture wave 1 — pure core contracts and policies (#143)

Wave under epic #142 on branch `Refactor`, based on `master` at `8d430cc`.

## Fixed feature list

1. Add `crates/bevyout-core` with only `std`, `serde`, and the exact `glam`
   version already shared with Bevy.
2. Move `ItemLedger` and its public transaction/value types into the core
   crate. Preserve `bevyout::item_transaction` as a compatibility re-export.
3. Add one `FormId`/`FormIdResolver` implementation in core and consume its
   master-index arithmetic from both the ESM parser and `ContentIndex`.
4. Move `PreparedSceneManifest` and all of its serialized value types to core.
   Runtime code uses a local Bevy `Resource` wrapper at the viewer boundary.
5. Remove the preparation import of `viewer::interaction::item_rules` by
   putting the shared quest-item decision in core.
6. Add executable architecture guardrails for the new dependency direction.

## Tests before implementation

- Core unit tests for local FormIDs, master remaps, invalid master indices,
  item conservation, stable IDs, and opaque state.
- Existing manifest RON round-trip and save byte-round-trip tests must pass
  unchanged through compatibility re-exports.
- `tests/architecture.rs` asserts that `bevyout-core` has no Bevy dependency
  and `src/vsa` contains no `crate::viewer` dependency.
- Existing Cucumber imports for moved core policies are replaced by normal
  crate imports where the public integration-test boundary permits it.

## Gate

`cargo fmt --all --check`, `cargo test -p bevyout-core`, `cargo check-dev`,
`cargo test-dev`, and `cargo clippy --all-targets -- -D warnings`.

## Shipped amendments

- `PreparedItemCatalog` stays as a small Bevy resource in the application
  crate. Its item definition/value types moved to core, but the catalogue is
  not part of `PreparedSceneManifest`'s serialized shape and moving it would
  add a second runtime wrapper with no domain-boundary benefit.
- `PreparedPhysicsSource`, which is embedded by prepared placements, moved
  with the manifest contract so the core transport graph is self-contained.

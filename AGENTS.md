# Agent guidance

## Project architecture

This project uses Vertical Slice Architecture (VSA):

- `src/main.rs` is a dispatcher only. Keep command routing there; do not add
  Fallout parsing, conversion, or Bevy systems to it.
- `src/cli.rs` owns clap command and option definitions.
- The Fallout cell feature owns its complete path from CLI input through plugin
  and BSA parsing, NIF-to-GLB conversion, manifest generation, and Bevy
  rendering. Keep those feature concerns together rather than creating a
  global layer for every parser, asset type, or system.
- `src/vsa/` contains the current Fallout cell slice internals; `viewer.rs`
  contains its Bevy presentation boundary and consumes only the prepared RON
  manifest.
- `PreparedSceneManifest` is the explicit hand-off contract inside the slice.

When adding a feature, add a new slice directory or module with its own input,
data, preparation, and runtime code. Expose only a narrow command/plugin API
to `main.rs`. Prefer Bevy `Plugin` values for new runtime feature groups
instead of growing `main.rs` or one large startup system.

## Local Bevy documentation

Use `BevyDocs/` as the local API and architecture reference before relying on
generic Bevy examples. In particular:

- `BevyDocs/programming/app-builder.md` and `plugins.md` describe app/plugin
  composition.
- `BevyDocs/programming/systems.md` describes system organization.
- `BevyDocs/setup/bevy-config.md` documents dynamic linking.
- `BevyDocs/setup/perf.md` covers development and release profiles.

If the local docs do not cover a version-specific detail, verify it against the
Bevy version in `Cargo.toml`.

## Build policy

Use dynamic linking for iterative desktop development:

```powershell
cargo check-dev
cargo test-dev
cargo run-dev -- prepare --cell 000151e3
```

These aliases expand to Bevy's `dynamic_linking` feature. Keep that feature
development-only; release builds must not depend on the Bevy DLLs unless they
are intentionally bundled and tested.

Before handing off changes, run `cargo fmt --check`, `cargo clippy --all-targets
-- -D warnings`, `cargo test`, and a representative `cargo run-dev` command.

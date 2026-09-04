# bevyout

An offline, modernized recreation of Fallout 3 built on Bevy. `bevyout` reads
the original GECK-authored `Fallout3.esm`, its masters, and the Fallout
mesh/texture/sound BSAs directly, converts interior cells into Bevy-native
scenes, and renders them with a from-scratch renderer, audio, and interaction
layer. No GECK or original engine runtime is involved.

## Legal and trademark disclaimer

This project is an unofficial, community driven fan tool. It is not affiliated
with, endorsed by, or approved by Bethesda Softworks, ZeniMax, or The Wand
Company. All related trademarks, including "Pip-Boy", "Vault-Tec", "Vault
Boy", "Fallout", and other references remain the property of Bethesda
Softworks.

## Requirements

- **Rust**, edition 2024 (rustc 1.85+). Install via [rustup](https://rustup.rs).
- **A licensed copy of Fallout 3 GOTY**, for `Fallout3.esm` and its BSAs. The
  game data is not redistributed here; point `game_root` at your installation
  in `.bevyout/config.toml`.
- No Blender installation is required. Native Rust NIF conversion and the Rust
  irradiance baker are the supported production paths. The old
  `src/vsa/assets/blender_script.py` remains only as an annotated historical
  comparison/reference script and is never invoked by preparation or runtime.
- **[ImageMagick](https://imagemagick.org/script/download.php)** (`magick` on
  `PATH`, or the default Windows install path). It converts staged DDS
  textures to PNG during `prepare`; preparation can continue without it, but
  textures remain unconverted.
- **[KTX-Software](https://github.com/KhronosGroup/KTX-Software/releases)**,
  using the unified `ktx` binary. It is required for a static point-shadow
  cache miss during `prepare` and for the default irradiance bake. It is not
  needed by `render` or `bake --quality preview` when their cached artifacts
  are already available.

On Windows, these tools are auto-detected at their default install locations.
Otherwise put them on `PATH` or set `[tools]` in
[`config.example.toml`](config.example.toml).

Without a project-local `.bevyout/config.toml`, a user-level config is also
read from `%APPDATA%\bevyout\config.toml` on Windows, or
`$XDG_CONFIG_HOME/bevyout/config.toml` (falling back to
`~/.config/bevyout/config.toml`) on macOS/Linux.

## First run

From the repository directory, use a Fallout GECK EditorID. During development,
prefer the dynamic-linking aliases so Bevy itself does not need to be
statically relinked on each iteration:

```powershell
cargo run-dev -- prepare SuperDuperMart
cargo run-dev -- bake SuperDuperMart
cargo run-dev -- render SuperDuperMart
```

`run-dev` is the compile-time path (`opt-level = 1` on this crate). For a
playable viewer FPS without a full release build, use `cargo run-play -- render SuperDuperMart` (`dev-opt`, `opt-level = 3`). GPU API validation is off by default; pass `--wgpu-validation` (or `WGPU_VALIDATION=1`) when diagnosing renderer bugs.

The equivalent direct command is `cargo run --features bevy/dynamic_linking`.
Dynamic linking is development-only; do not enable it for release builds
unless the Bevy runtime DLLs are deliberately bundled and tested.

`render` can offer to prepare or bake a missing cell. The explicit
`prepare`/`bake` commands remain the reproducible path for scripts and
debugging. The selector also accepts an eight-digit hexadecimal FormID; for
example, `SuperDuperMart` resolves internally to `00017f37`.

### Isolated ragdoll laboratory

Compare one prepared actor without loading or modifying the market scene:

```powershell
cargo run-dev -- ragdoll-lab SuperDuperMart --actor 00041606
cargo run-dev -- ragdoll-lab SuperDuperMart --actor 00041606 --backend boxddd
```

Avian3D is the laboratory default; BoxDDD remains the production viewer
backend. Press `Space` to pause/resume the drop and `R` to restore the intact
pose and rebuild it. Add `--agent-bridge` to expose
`bevyout.ragdoll_lab_probe` on the usual loopback bridge.

### Isolated animation zoo

Prepare the native KF clip pack, then cycle every compatible external KF on one
actor without loading the gameplay viewer:

```powershell
cargo run-dev -- prepare SuperDuperMart --actor-animation-converter native
cargo run-dev -- animation-zoo SuperDuperMart --actor 00041606
```

The zoo restores bind pose between clips and supports pause, previous/next,
restart, looping, and playback-speed controls. `--agent-bridge` exposes
`bevyout.animation_zoo_probe` and `bevyout.animation_zoo_control`.

### Experimental native NIF conversion

The OS-agnostic Rust converter is the supported standalone command for
FO3/FNV NIF `20.2.0.7` assets. It emits a self-contained GLB,
including supported controller-sequence animations, and can also emit the
authored Havok collision sidecar used by the prepared-physics schema:

```powershell
cargo run-dev -- nif-convert `
  --asset meshes/clutter/ammo/ammobox01.nif `
  --game-root "C:\Games\Fallout 3" `
  --output .bevyout/cache/native-nif/ammobox01.glb `
  --physics-output .bevyout/cache/native-nif/ammobox01.physics.json.gz `
  --report .bevyout/cache/native-nif/ammobox01.report.json
```

Use `--input <file.nif>` for a direct filesystem input, `--allow-lossy` for
actor/skinned assets whose ragdoll conversion is not yet in scope, and
`--force` to replace outputs. Unsupported native blocks are reported in the
conversion report; they do not route through Blender.

## Current scope

The current slice supports interior and exterior-cell preparation and
rendering, cached native NIF-to-GLB conversion, LAND terrain packages,
deterministic Rust irradiance baking, prepared static point shadows, first-person
movement and physics, staged audio, and an initial
pickup/container/door/activator interaction path.

Exterior LAND and worldspace streaming, NPC assembly and AI, runtime NAVM
pathfinding, compressed music and voice, dialogue, quests, scripts, complete
inventory/RPG systems, and persistent Fallout save games remain roadmap work.
See the [compatibility status](https://github.com/kelo221/bevyout/wiki/Compatibility-Status)
for the current capability boundaries and player-facing completion gates.

## Documentation

The root README owns installation and the first-run path. Detailed, durable
documentation lives in the project wiki:

- [Getting Started](https://github.com/kelo221/bevyout/wiki/Getting-Started)
- [Asset Pipeline](https://github.com/kelo221/bevyout/wiki/Asset-Pipeline)
- [Console and Agent Bridge](https://github.com/kelo221/bevyout/wiki/Console-and-Agent-Bridge)
- [Compatibility Status](https://github.com/kelo221/bevyout/wiki/Compatibility-Status)
- [Architecture](https://github.com/kelo221/bevyout/wiki/Architecture)
- [Testing and Troubleshooting](https://github.com/kelo221/bevyout/wiki/Testing-and-Troubleshooting)

Contributor and agent-specific guidance lives in
[`CONTRIBUTING.md`](CONTRIBUTING.md) and [`AGENTS.md`](AGENTS.md). The history
of milestone plans and wave acceptance work is recorded in
[`docs/plans/README.md`](docs/plans/README.md). Live status, dependencies, and
acceptance gates remain in [GitHub issues and milestones](https://github.com/kelo221/bevyout/issues).

## Checks

Before pushing, run:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run-dev -- render SuperDuperMart
```

Generated extracted game data and GLBs live under `.bevyout/`, which is ignored
by Git. Bethesda-derived assets must not be committed, published, or attached
to issues.

## Provenance

Code adapted or ported from OpenMW is isolated in attributed provenance
folders such as [`src/vsa/openmw_esm4/`](src/vsa/openmw_esm4/). Each such folder
contains its own `README.md` and `NOTICE.md` identifying the source snapshot,
files, hashes, licenses, and adaptations. Project-native parsing, preparation,
audio, and viewer systems remain outside those folders.

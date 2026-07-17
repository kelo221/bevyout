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
- **[Blender](https://www.blender.org/download/)** 5.2 with the
  **[Blender Niftools Addon](https://github.com/niftools/blender_niftools_addon)**
  installed and enabled (`io_scene_niftools`). Blender is used by `prepare`
  and by the optional `bake --quality preview` path.
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
[`config.example.toml`](config.example.toml). macOS and Linux users should set
the Blender path explicitly. The legacy `irradiance_blender` option is accepted
for configuration and CLI compatibility but is ignored by the Rust irradiance
baker.

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

To inspect the hybrid shadow path without Fallout data or KTX packaging, open
the hermetic procedural scene:

```powershell
cargo run-dev -- lighting-test
```

The orange pillar is traced once by the same CPU static-shadow kernel used by
`prepare`. The blue block moves every frame and casts through Bevy's realtime
point-shadow pass. The floor combines both visibility sources, making the two
paths directly observable in one view. The hermetic bake uses the same
high-quality 512 face resolution as `prepare` by default;
`--shadow-resolution 128|256` remains available for faster checks. Press Space
to pause or resume motion, and press 1 or 2 to toggle the baked-static and
realtime sources independently.

The test also includes a purple point light driven by the isolated
`src/vsa/dynamic_lighting` slice, a color-coded rack containing all 15 temporal
effects, and a second rack containing all eight spatial types. Each light has
an isolated receiver so intensity and pattern changes stay readable beside the
90,000-unit hybrid shadow light. Press 3 to toggle the custom HDR pass and F to
freeze or resume effect time. Press 4 to hide every ordinary Bevy point light
and 5 to toggle the separately identified shadow-only proxy. The proxy is a
black, fixed-intensity Bevy `PointLight`: Bevy owns its realtime cubemap, its
built-in direct contribution is zero, and the custom pass samples that cubemap
without driving the proxy from temporal effects. Prepared irradiance baking and custom-light
authoring both keep one diffuse bounce enabled by default
(`bounce_multiplier = 1.0`).

Henry-style volumetric fog is rendered by the same isolated custom path. The
scene demonstrates a temporally modulated sphere, a non-uniformly scaled box,
and rotated `ConeZ`/`ConeY` volumes. Press 6 to toggle only the fog pass. Fog
uses the shared effect runtime, reconstructs geometry from depth, and is
composed in HDR before tonemapping and UI; it does not require Bevy
`VolumetricLight` or `FogVolume` components.

For a fast Blender preview that leaves the prepared manifest unchanged:

```powershell
cargo run-dev -- bake SuperDuperMart --quality preview
```

The equivalent direct command is `cargo run --features bevy/dynamic_linking`.
Dynamic linking is development-only; do not enable it for release builds
unless the Bevy runtime DLLs are deliberately bundled and tested.

`render` can offer to prepare or bake a missing cell. The explicit
`prepare`/`bake` commands remain the reproducible path for scripts and
debugging. The selector also accepts an eight-digit hexadecimal FormID; for
example, `SuperDuperMart` resolves internally to `00017f37`.

## Current scope

The current slice supports interior-cell preparation and rendering, cached
NIF-to-GLB conversion, deterministic Rust irradiance baking, hybrid prepared
static/realtime point shadows, default-on one-bounce indirect lighting,
an isolated DynamicLighting effects/spatial/volumetric bridge, first-person
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

# bevyout

Offline Fallout 3 interior-cell preparation and a small Bevy viewer.

## Prepare and view a cell

From this directory, use a hexadecimal FormID. During development, prefer the
dynamic-linking alias so Bevy itself does not need to be statically relinked on
each iteration:

```powershell
cargo run-dev -- prepare --cell 000151e3
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron
```

The equivalent direct command is `cargo run --features bevy/dynamic_linking`.
This remains a development-only feature; do not enable it for release builds
unless the Bevy runtime DLLs are deliberately bundled.

`prepare` reads `Fallout3.esm`, indexes the loose files and the Fallout mesh/texture BSAs, stages the referenced NIFs and textures, converts DDS files to PNG with ImageMagick, and runs Blender headlessly through the installed Niftools addon. Copy `config.example.toml` to `.bevyout/config.toml` to configure the Fallout root, plugin, cache, Blender, and KTX paths. Explicit CLI flags override config values; Blender and KTX still have automatic detection fallbacks. Use `--config path.toml` for a different config file or `--force` after changing Blender or conversion settings.

## Bake lighting

Light baking is a separate step after `prepare`. It writes baked assets under
the scene's cache directory and updates the same manifest, so the normal viewer
command automatically loads the baked scene after a successful quick or final
bake:

```powershell
# Fast Eevee preview; writes preview.png and leaves the manifest unchanged.
cargo run-dev -- bake --manifest .bevyout/cache/scenes/000151e3/scene.ron --quality preview

# Low-cost Cycles direct-light bake; runtime Fallout lights remain enabled.
cargo run-dev -- bake --manifest .bevyout/cache/scenes/000151e3/scene.ron --quality quick --device optix

# Production Cycles bake with indirect light and denoising.
cargo run-dev -- bake --manifest .bevyout/cache/scenes/000151e3/scene.ron --quality final --device optix

cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron
```

The modes are intentionally different:

| Mode | Renderer and settings | Result |
| --- | --- | --- |
| `preview` | Eevee screen-space ray tracing and Fast GI | A quick `preview.png`; no KTX2 or manifest bake metadata |
| `quick` | Cycles, 512px page, 4 samples, 1 bounce, direct light only | KTX2 lightmap with runtime ambient/point lights retained for indirect fill |
| `final` | Cycles, 4096px page, 512 samples, 4 bounces, direct + indirect, OpenImageDenoise | Full KTX2 lightmap; runtime lights are disabled on lightmapped meshes |

`--device` accepts `cpu`, `optix`, `cuda`, or `hip` for Cycles modes. The
requested GPU backend must be available in Blender; `preview` always uses
Eevee. `--force` replaces an existing `baked` directory. `--keep-intermediate`
keeps the generated Blender job, Python script, result JSON, and EXR for
diagnostics. KTX-Software is required for `quick` and `final` unless you pass
`--keep-intermediate` to retain the EXR without producing KTX2.

The current slice handles interior cells and static geometry. The viewer uses the Fallout-to-Bevy coordinate conversion, starts near the prepared scene bounds, spawns the prepared GLB scenes and point lights, and provides free flight with WASD/QE plus mouse look. The mouse is captured on startup; press `Esc` to release it and click the window to capture it again. NIF alpha flags and texture alpha are exported as glTF `MASK`/`BLEND` materials, while non-rendering editor markers are omitted. Exterior LAND, animation playback, and collision are intentionally left for later passes.

## Checks

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Generated extracted game data and GLBs live under `.bevyout/`, which is ignored by git.

## VSA boundaries

The project follows Vertical Slice Architecture (VSA):

- `src/cli.rs` owns command-line configuration.
- The Fallout cell slice owns offline Fallout parsing, BSA access, asset staging, Blender conversion, and the prepared scene manifest. Its current implementation is in `src/vsa/`, with submodules for manifest types, plugin/BSA parsing, asset conversion, path/transform helpers, and orchestration.
- `src/viewer.rs` owns the slice's Bevy presentation: app setup, scene spawning, lighting, and camera input.
- `src/main.rs` only dispatches between the preparation and viewer entry points.

The prepared scene manifest is the hand-off contract between the offline and
real-time parts of the Fallout cell slice. Future features should add their own
vertical slice instead of expanding a shared global `main.rs`.

# Pipeline contract

## Contents

- Stage and artifact map
- Dependencies and backend selection
- Command behavior
- Cache recovery
- Runtime verification
- Failure classification

## Stage and artifact map

| Stage | Primary input | Durable output | Mutates `scene.ron` |
| --- | --- | --- | --- |
| `prepare` | EditorID/FormID selector, resolved plugin chain and game data | `.bevyout/cache/scenes/<formid>/scene.ron` plus converted assets, physics/audio/catalog/nav artifacts, and prepared point shadows | Rewrites it |
| `bake` irradiance | Prepared selector or `--manifest` | `baked/scene.glb`, `baked/irradiance.ktx2`, and bake fingerprint/revision metadata | Yes |
| `bake --quality preview` | Prepared selector or `--manifest` | `baked/preview.png` | No |
| `render` | EditorID/FormID selector | Viewer process; may interactively call prepare/bake first | Only through accepted recovery work |
| `view` | Exact manifest path | Viewer process | No |

Batch prepare writes `.bevyout/cache/prepare_jobs.ron` for resumable state.
Batch bake writes `.bevyout/cache/bake_jobs.ron`. Selector lookup stores scenes
by lowercase eight-digit FormID and resolves EditorIDs by reading cached
manifests; duplicate cached EditorIDs are reported as ambiguous.

Prepared point shadows belong to `prepare`, after GLB conversion and physics
classification. Irradiance and static batching belong to `bake`. Runtime
realtime point shadows belong to the viewer and are disabled unless opted in.

## Dependencies and backend selection

| Operation | Default implementation | External tool behavior |
| --- | --- | --- |
| NIF-to-GLB during `prepare` | Native Rust (`--converter native`) | Unified KTX-Software is required on a texture cache miss; Blender is not resolved |
| Compatibility NIF-to-GLB | `--converter blender` | Requires Blender/NIFTools and unified KTX-Software; actor comparison has platform limitations |
| Prepared point-shadow miss | Rust generation and KTX packaging | Resolves unified KTX-Software only on a cache miss or `--rebuild-shadows` |
| Irradiance bake | Deterministic Rust CPU ray tracer | Requires unified `ktx`; never invokes Blender |
| Preview bake | Blender Eevee | Requires Blender; does not create a usable irradiance bake |
| Viewer | Bevy | Does not generate prepared or baked artifacts |

Prepared material textures, item icons, and Pip-Boy sprites are stored as
UASTC+Zstd KTX2. ImageMagick is not a prepare fallback; unified KTX-Software
is required whenever these texture artifacts need to be generated.
The native converter is the CLI default through `PrepareConverter::default`
and `resolve_converter_backend(None)`. Both native and Blender prepared
converter revisions remain accepted by the viewer, so "supported" does not
mean "preferred."

The Rust irradiance path still uses legacy names such as `BakeJob`,
`blender_path`, `blender_bake.py`, and fields for `.blend`/result files. The
shared serialized job participates in bake fingerprinting, and current code
may write then remove legacy intermediates, but only the preview branch calls
`Command::new(blender)`.

## Command behavior

### Prepare

Use a GECK EditorID or eight-digit hexadecimal FormID:

```powershell
cargo run-dev -- prepare SuperDuperMart
cargo run-dev -- prepare 00017f37 --converter native
```

Useful selectors and controls:

- Repeat positional selectors for several cells.
- Use `--all-interiors`, `--worldspace <selector>`, or `--all` only for
  intentional batch work.
- Use `--list-only` to resolve and print selection before extraction or
  conversion.
- Use `--check-fingerprints` to report valid/stale recorded cells without
  preparing them.
- Use `--jobs <N>` to bound batch concurrency.
- Use `--retry-failed` to select recorded failures.
- Use `--shadow-resolution 512` by default; 128 and 256 are explicit quality
  reductions.

Switching converter backends changes the prepared converter fingerprint. Do
not compare native and Blender results without recording which backend last
wrote the manifest.

### Bake

Use either a selector or an exact manifest:

```powershell
cargo run-dev -- bake SuperDuperMart
cargo run-dev -- bake --manifest .bevyout/cache/scenes/00017f37/scene.ron
```

The default settings are Rust irradiance, 8 metre probe spacing, 64 samples,
and 64 metre static batch chunks. Parameter bounds are owned by `src/cli.rs`.
An ordinary single-cell bake already replaces known outputs; its legacy
`--force` flag is a no-op. Batch bake uses recorded validity and supports
`--all-interiors`, `--retry-failed`, and meaningful `--force` regeneration.

Preview is diagnostic only:

```powershell
cargo run-dev -- bake SuperDuperMart --quality preview
```

Do not treat `preview.png` as a baked manifest or as viewer-ready GI.

### Render

`render <selector>` locates the cached manifest, checks prepare compatibility,
checks bake state, and enters the viewer. In an interactive terminal it can
offer to prepare a missing scene, refresh an incompatible manifest, or bake
missing/stale irradiance. A missing bake may be declined and viewed unbaked;
an incompatible existing bake must be rebuilt or the launch fails.

Non-interactive input declines prompts, so use explicit prepare/bake stages in
scripts. `render --agent-bridge` is stricter: it refuses missing/incompatible
prepared content and requires a compatible irradiance bake instead of
prompting.

### View

`view --manifest <path>` performs no selector lookup and no recovery. It
validates prepared compatibility and rejects a stale existing bake. A manifest
with no bake metadata remains valid and can be inspected directly, including
with `--agent-bridge`.

Use `--disable-physics` only for an intentional render-cost comparison,
`--realtime-shadows` only for the bounded runtime shadow opt-in, and
`--trace-seconds` for unattended launches.

## Cache recovery

Use this order:

1. Resolve the intended cell with `prepare <selector> --list-only`.
2. Run `prepare <selector> --check-fingerprints` when the cell is already in
   the prepare job manifest.
3. Rerun normal prepare. Allow revision/fingerprint logic to invalidate stale
   work.
4. Add `--rebuild-assets` only for a converter-cache reproduction.
5. Add `--rebuild-shadows` only for a point-shadow-cache reproduction.
6. Rerun normal bake when bake metadata is absent/stale or bake parameters
   changed.
7. Preserve `--keep-intermediate` only while diagnosing bake/KTX output.

Do not delete `.bevyout/cache` as a first response. It destroys useful cache
evidence and expands a narrow invalidation problem into a full rebuild.

## Runtime verification

For a baked selector:

```powershell
cargo run-dev -- render SuperDuperMart --agent-bridge --trace-seconds 30
```

For an exact or unbaked prepared manifest:

```powershell
cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron --agent-bridge --trace-seconds 30
```

Use the `bevyout-mcp` skill for snapshots, console commands, BRP calls,
performance probes, schedule inspection, and viewport capture. Record stable
CLI/log lines and structured snapshots; a black occluded macOS capture is not
visual evidence.

## Failure classification

| Symptom | Classify first | Narrow response |
| --- | --- | --- |
| Selector not found/ambiguous | Content selection or cached EditorID lookup | Run `--list-only`; use FormID when needed |
| Manifest revision mismatch | Prepared cache compatibility | Rerun ordinary prepare with the intended backend |
| Native job unsupported/failed | Native converter coverage | Capture diagnostic and asset identity; do not silently fall back to Blender |
| KTX missing during prepare | Prepared shadow cache miss | Configure unified KTX or reuse a valid cache |
| KTX missing during bake | Irradiance packaging | Configure unified KTX; Blender is unrelated |
| `preview.png` exists but render requests bake | Preview/bake confusion | Run default Rust irradiance bake |
| `render --agent-bridge` refuses recovery | Automation contract | Run explicit prepare and bake, or use exact `view` for prepared-only inspection |
| Stale visual after source change | Cache identity/revision | Audit the relevant `*_REVISION` before forcing all assets |

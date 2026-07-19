# Blender history and current boundary

## Contents

- Historical sequence
- Current supported uses
- Legacy residue
- Investigation rules

## Historical sequence

- `6ae5e67` (2026-07-11, initial repository): the imported scene workflow was
  Blender-centered.
- `0275c3d` (2026-07-12): moved irradiance baking to Blender 4.5 Eevee
  irradiance volumes.
- `5f49e5f` (2026-07-12): made `render` the interactive orchestration entry
  point that could offer prepare and bake work.
- `03ec0e3` (2026-07-15): replaced Blender irradiance baking with the
  deterministic Rust CPU ray tracer. From this point, Blender remained for
  NIF conversion and optional preview, while KTX-Software packaged the Rust
  irradiance atlas.
- `82955d2` / PR #159 (2026-07-19): added native NIF conversion, wired
  `ConverterBackend::Native` as the default, integrated native conversion into
  prepare, and retained Blender as an explicit compatibility backend.
- `M4_WAVE7_MANUAL.md` then named native actor conversion the production and
  acceptance path. Blender/PyNifly actor conversion was retained as a
  Windows-only comparison tool.

The PR #159 README prose did not fully follow its code: it still described
native conversion as experimental and said prepare continued to use Blender.
The current `src/cli.rs`, `src/converter_policy.rs`, prepare orchestrator, and
wave-7 acceptance manual supersede that wording.

## Current supported uses

Use Blender only for:

1. `prepare <selector> --converter blender`, as an explicit NIFTools/PyNifly
   compatibility comparison.
2. `bake <selector> --quality preview`, as a fast Eevee preview that writes
   `preview.png` and does not produce irradiance metadata.

Do not require Blender for:

- default `prepare`;
- default `bake` irradiance;
- prepared point shadows;
- `render` or `view` when their required cache artifacts already exist;
- KTX packaging;
- runtime realtime shadows.

Both native and Blender prepared converter revision strings remain accepted
by viewer compatibility checks. That preserves existing caches and explicit
comparison work; it is not evidence that both backends are equally preferred.

## Legacy residue

Expect Blender names in current code even when the default operation is native
or Rust-owned:

- `blender` and `irradiance_blender` configuration fields remain for explicit
  compatibility and old CLI/config files. `irradiance_blender` is ignored by
  the Rust baker.
- `BakeJob`, `blender_path`, `blender_bake.py`, `result_json`, and
  `irradiance_blend` survive around the shared preview/fingerprint seam.
  Default irradiance may serialize/write then clean some of these legacy
  intermediates, but it does not spawn Blender.
- `src/vsa/assets/blender_script.py` and its tests remain substantial because
  `--converter blender` is still a supported compatibility backend.
- Comments and feature steps may say "Blender job" where the data structure is
  now shared with Rust bake planning.

Treat these as cleanup candidates, not proof of an active default dependency.
Before removing one, trace its callers, fingerprint participation, serialized
compatibility role, tests, and explicit fallback behavior.

## Investigation rules

1. Search source and history separately. A current symbol name may be legacy;
   an old README statement may no longer describe current routing.
2. Confirm process invocation. For baking, the decisive boundary is the
   `BakeQuality::Preview` branch containing `Command::new(blender)`.
3. Confirm default selection. The decisive boundary for prepare is
   `PrepareConverter::default` delegating to
   `resolve_converter_backend(None) -> Native`.
4. Confirm compatibility before deletion. Check
   `SUPPORTED_PREPARED_CONVERTER_REVISIONS`, config migration, Cucumber
   scenarios, and real-data acceptance instructions.
5. Keep a Blender comparison explicit in commands and evidence. Never allow a
   native failure to fall back silently and make the result look native.
6. If the goal is complete Blender removal, open it as a separate migration:
   first prove native parity for the remaining comparison cases, replace the
   preview surface, remove legacy bake job fields without breaking
   fingerprints, update config/CLI/docs, bump affected revisions, and test
   stale-cache behavior.

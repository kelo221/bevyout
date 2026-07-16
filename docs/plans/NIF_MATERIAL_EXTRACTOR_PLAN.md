# Native Rust NIF material extractor follow-up

Status: follow-up issue [#124](https://github.com/kelo221/bevyout/issues/124) to
the Fallout-faithful emissive calibration PR; do not mix this work into that
PR.

## Goal

Move extraction of authored NIF material emission metadata into the Rust
asset-preparation slice. The native extractor should provide the source color
and multiplier needed by the GLB material policy while the existing NIFTools
Blender conversion remains responsible for geometry, textures, animation, and
the final GLB export during the migration.

## Scope

- Add a std/serde-only extractor under `src/vsa/assets/` that reads the NIF
  record/property data needed for material emission.
- Support the Fallout-era material sources currently handled by the Blender
  shim:
  - `NiMaterialProperty.emissive_color` plus the versioned multiplier field
    (`emissive_mult` / `emit_multi`).
  - `BSLightingShaderProperty.emissive_color` and
    `emissive_multiple`.
  - `BSEffectShaderProperty.base_color` and `base_color_scale`.
- Preserve material identity and property ordering well enough to associate a
  native result with each Blender material slot without using asset-name
  allowlists or arbitrary brightness boosts.
- Keep explicit emission, glow-texture, and physical-bulb precedence in the
  existing material policy.
- Add deterministic diagnostics for unsupported NIF versions, malformed
  records, ambiguous material matches, and fallback use.

Out of scope: replacing NIFTools geometry import/export, implementing a native
NIF-to-GLB converter, changing the public manifest or CLI contract, or
introducing a second runtime emission pipeline.

## Fallback strategy

1. Try the native extractor and validate every returned color and multiplier
   for finiteness, nonnegative strength, and a unique material association.
2. If extraction is complete, pass the native metadata into the existing
   Blender job as internal preparation metadata and use it for the material
   policy.
3. If the file version or property layout is unsupported, malformed, or
   ambiguous, keep the current NIFTools-side path for that asset. The fallback
   must preserve authored color when NIFTools exposes it and must use strength
   `1.0` only when no trustworthy source multiplier is available.
4. Record fallback counts and reasons in deterministic preparation diagnostics;
   fallback is a compatibility path, not a silent success condition for strict
   extraction coverage.

## Migration path

1. Build the parser and pure policy tests against synthetic NIF records and
   small real-data probes, with no converter behavior change.
2. Run the native extractor in shadow mode beside the current Blender shim and
   compare per-slot color/strength results across the prepared asset batch.
3. Fix parity gaps and establish a zero-unexpected-difference gate for the
   supported Fallout property classes. Keep NIFTools as the per-asset fallback
   for unsupported inputs.
4. Switch the Blender job metadata source to the native result, bump the
   converter revision, rebuild the asset cache, and re-run scene acceptance.
5. After one stable release of parity and fallback telemetry, remove the
   NIFTools multiplier interception and its shadow-mode comparison code. Do not
   remove the geometry converter until a separately approved native converter
   plan exists.

## Acceptance tests

- Unit tests cover nonzero and zero authored colors, all three multiplier
  sources, invalid/negative values, duplicate/ambiguous matches, and explicit
  emission/glow/bulb precedence.
- Parser tests cover the actual Fallout `NiMaterialProperty.emissive_mult`
  spelling as well as `emit_multi`, plus the BSLighting and BSEffect fields.
- A real-data parity test covers RadAway, MS05NukaColaQtm,
  genericProtectronTerminalDesk, OffRmLightOFF02, OffRmLight02, OffRmLight01,
  and MetLight01b.
- A batch comparison over the Super-Duper Mart preparation set reports zero
  unexpected native-versus-NIFTools differences for supported properties and
  lists every fallback reason.
- Rebuild Super-Duper Mart and verify the target GLBs, including normalized
  glTF emission plus `KHR_materials_emissive_strength` where the source
  multiplier requires it; zero-color `OffRmLightOFF02` remains non-emissive.
- Verify the live viewer's Fallout bloom baseline and capture the same three
  probe targets without whiteout.
- Run `cargo fmt --check`, `cargo check-dev`, `cargo test-dev`,
  `cargo clippy --all-targets -- -D warnings`, and `cargo test`.

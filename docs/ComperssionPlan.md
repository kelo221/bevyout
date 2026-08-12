## Plan: Efficient Prepared-Cache Compression

Goal: reduce derived-cache size primarily through deduplication, while preserving visual output, runtime streaming, and preparation correctness.

### Wave 1 — Measure before changing formats

Use the existing 49-cell prepared patch as a representative sample.

- Record total size by category: GLB, KTX2 textures, shadows, audio, manifests, physics.
- Hash all converted assets and measure duplicate bytes across cells.
- Record preparation time, viewer load time, and peak residency.
- Establish targets only after this inventory; likely target is 40–70% reduction through deduplication.

### Wave 2 — Content-addressed shared assets

Implement one global prepared-asset store:

```text
cache/assets/<content-hash>/<artifact>
```

The hash must include:

```text
source content + converter revision + format/settings revision
```

Cell manifests reference shared assets instead of storing duplicate per-cell copies.

Requirements:

- Atomic writes and resumable cache misses.
- Case-insensitive path normalization.
- Preserve old cache readability or provide a controlled migration.
- Bump prepared revision constants when manifest references change.
- Add tests for deduplication, cache invalidation, and concurrent preparation.

This is the highest-value change.

### Wave 3 — Geometry compression

Evaluate runtime-compatible glTF compression:

- Mesh quantization.
- `EXT_meshopt_compression` or equivalent supported by the Bevy asset path.
- Preserve collision and physics geometry separately where needed.
- Compare visual error, load time, CPU decode time, and GPU upload time.

Only enable it after a representative viewer comparison.

### Wave 4 — Role-based texture compression

Keep the current KTX2 pipeline, but choose formats by texture role:

- BC5/BC4 for normals and grayscale maps.
- BC7 or Basis/ETC1S for suitable color textures.
- UASTC for UI, decals, and visually sensitive assets.
- Preserve mipmaps where runtime filtering needs them.

Do not recompress already-valid artifacts unless the new format policy requires it.

### Wave 5 — Shadow-cache audit

Measure how much space prepared point-shadow cubemaps consume.

- Keep the current 512 default unchanged.
- Retain 256/128 as explicit quality options.
- Deduplicate identical shadow payloads where fingerprints match.
- Do not reduce quality globally without a separate acceptance decision.

### Wave 6 — Optional packaging

Only if filesystem overhead remains significant:

- Package shared assets in a seekable Zstandard archive or chunk store.
- Keep random access by hash.
- Avoid one monolithic archive that prevents cell-level streaming and recovery.

### Acceptance gates

For each wave:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- Representative `cargo run-dev` preparation
- 49-cell cache comparison against the baseline
- No increase in failed preparations
- Fingerprints remain current
- Non-black runtime capture and unchanged residency behavior

Final acceptance should prove:

- At least the measured target cache reduction.
- No visual regression in representative cells.
- Preparation time increase stays within an agreed limit, preferably under 10%.
- Full exterior selection and preparation still report zero failed cells.
- Runtime traversal still crosses and returns across three cell boundaries within the residency bound.
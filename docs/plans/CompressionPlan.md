# Refined Plan: Prepared-Cache Deduplication, Compression, and Packing

## Strategic objective

Use the original Fallout 3 installation size—approximately **8 GB**—as a long-term reference point, not as the first hard acceptance gate.

The comparison is not one-to-one. Fallout 3 stores source archives, while bevyout generates runtime-oriented GLBs, mipmapped KTX2 textures, physics sidecars, prepared audio, static point-shadow cubemaps, catalogs, manifests, and optional bake products. These derived artifacts can legitimately exceed the source installation before deduplication and format optimization. ([GitHub][1])

Track three separate sizes:

| Budget                              | Contents                                                                        |
| ----------------------------------- | ------------------------------------------------------------------------------- |
| **Source game data**                | Licensed Fallout 3 installation; remains external                               |
| **Runtime-required prepared cache** | Assets required to prepare, load, stream, and play                              |
| **Optional cache**                  | Debug pages, validation outputs, bake intermediates, reports, migration backups |
| **Application package**             | bevyout executable, libraries, configuration, and non-Bethesda resources        |

The main optimization target should be the **runtime-required prepared cache**. Optional artifacts should not obscure that measurement.

---

# Repository-specific corrections to the original plan

## 1. GLB caching already exists, but it is recipe-addressed

The existing `content_addressed_glb_name()` hashes the converter/material-policy identity together with the source NIF bytes. That is useful deterministic caching, but technically it is an **input recipe identifier**, not a hash of the final GLB payload. ([GitHub][2])

The new design should retain two identities:

```text
RecipeId = hash(inputs + converter revisions + policy + settings)
ObjectId = hash(final artifact bytes)
```

This distinction matters because:

* Different recipes can occasionally produce identical bytes.
* A recipe can be invalidated without losing an already-known identical object.
* Corruption checks must validate the output payload, not merely the input key.
* Shadows may have different semantic fingerprints but identical output payloads.

Therefore, Wave 2 should not be described as “introduce GLB content addressing.” It should be:

> Generalize the existing single-artifact recipe cache into a global recipe-to-object prepared-artifact store.

## 2. Embedded textures are a major structural deduplication gap

The current texture pass reads embedded GLB image `bufferView`s, converts those images to KTX2, and places the converted bytes back into the GLB binary chunk. Therefore, the same texture used by ten different models can still occupy ten physical copies because each GLB contains its own embedded KTX2 payload. ([GitHub][3])

Whole-GLB hashing cannot remove that duplication. The plan needs a dedicated **embedded-texture extraction and shared loading wave**.

This may also reduce runtime residency if all models reference the same Bevy image asset rather than creating separate image subassets from separate GLBs. That must be instrumented and proven rather than assumed.

## 3. The current texture policy is safe but coarse

All current textures are converted to RGBA8, encoded as UASTC, Zstandard-compressed at level 9, and given mipmaps. The main role distinction during embedded-image rewriting is sRGB versus linear. ([GitHub][3])

That leaves several opportunities:

* One-channel masks are stored as four-channel source data.
* Two-channel normal data is stored as four-channel source data.
* UI, diffuse, normal, mask, emissive, and lightmap assets share one broad encoding policy.
* Repeated DDS-to-RGBA-to-PNG-to-KTX conversion adds preparation work.

However, Fallout normal-map alpha carries specular strength. Converting those textures directly to BC5 would discard the alpha channel and create a visible material regression. ([GitHub][4])

## 4. Meshopt and glTF quantization cannot be enabled directly

The repository currently uses Bevy 0.19. Bevy 0.19’s glTF loader explicitly lists both `KHR_mesh_quantization` and `EXT_meshopt_compression` as unsupported. ([GitHub][5])

Geometry work should therefore begin with **extension-free optimization**. Meshopt-compressed glTF should be an isolated custom-loader research track, not an ordinary converter switch.

## 5. The cache redesign can also improve preparation concurrency

The batch preparation session currently uses a global `asset_stage_lock` because native conversion and texture staging mutate the shared staging tree. Per-recipe temporary directories and atomic publication into the object store can narrow or remove that lock. ([GitHub][6])

This is important because the compression project should not merely avoid slowing preparation; it should remove avoidable serialization where possible.

---

# Target cache architecture

```text
.bevyout/cache/
  objects/
    glb/aa/bb/<payload-sha256>.glb
    texture/aa/bb/<payload-sha256>.ktx2
    physics/aa/bb/<payload-sha256>.ron
    audio/aa/bb/<payload-sha256>.<ext>
    shadow/aa/bb/<payload-sha256>.ktx2
    bake/aa/bb/<payload-sha256>.<ext>
    catalog/aa/bb/<payload-sha256>.ron

  recipes/
    glb/aa/bb/<recipe-sha256>.ron
    texture/aa/bb/<recipe-sha256>.ron
    physics/aa/bb/<recipe-sha256>.ron
    audio/aa/bb/<recipe-sha256>.ron
    shadow/aa/bb/<recipe-sha256>.ron

  scenes/
    <cell-form-id>/scene.ron

  indexes/
    live-roots.ron
    object-index.ron
    cache-generation.ron

  staging/
    <process-id>/<recipe-id>/...

  quarantine/
    corrupt/
    interrupted/
```

Two-level prefix sharding avoids putting hundreds of thousands of files into one directory. A directory per object should be avoided unless an artifact genuinely has multiple inseparable files.

## Object and recipe records

A minimal object reference:

```rust
struct PreparedObjectRef {
    kind: PreparedObjectKind,
    sha256: String,
    byte_len: u64,
    extension: String,
}
```

A recipe record:

```rust
struct PreparedRecipe {
    recipe_version: u32,
    kind: PreparedObjectKind,
    input_hashes: Vec<String>,
    converter_revision: String,
    format_policy_revision: String,
    canonical_settings: Vec<u8>,
    output: PreparedObjectRef,
}
```

The recipe identifier should use domain separation and length-prefixed or canonical serialization:

```text
SHA-256(
  "bevyout-prepared-recipe-v1" ||
  artifact-kind ||
  canonical-settings ||
  converter-revision ||
  ordered-input-hashes
)
```

Do not construct it by ambiguously concatenating arbitrary strings.

## Manifest migration strategy

Many current manifest fields are path strings. A single sweeping conversion to typed object references would have a large compatibility and review surface. The safer sequence is:

### Phase A — shared paths without schema replacement

Keep existing path fields and point them at canonical shared paths:

```text
objects/glb/ab/cd/<hash>.glb
objects/audio/ef/01/<hash>.ogg
```

The hash and kind are encoded in the path. This limits the first migration to path semantics and resolver behavior.

### Phase B — typed references

Once the store is stable, replace important path fields with `PreparedObjectRef` or a compatibility enum:

```rust
enum PreparedAssetLocation {
    LegacyPath(String),
    Object(PreparedObjectRef),
}
```

The normal viewer path should remain strict. Old caches should be handled by an explicit migration command rather than silently accepted as current. The project already treats both serialized shape and serialized meaning as revision-gated compatibility boundaries. ([GitHub][7])

---

# Revised implementation waves

## Wave 0 — Reproducible baseline and cache inventory

### Objective

Establish exactly where the bytes, processing time, and residency are going before changing storage or formats.

### Sample set

Retain the existing 49-cell exterior patch, but add smaller cohorts covering:

* Asset-heavy interiors.
* Actor- and animation-heavy locations.
* Audio-heavy cells.
* High-light-count cells with static shadows.
* Cells using worldspace LOD or bake outputs.
* A cold first prepare and a warm incremental prepare.

The 49 exterior cells alone may underrepresent interiors, voices, actor assemblies, and other content classes.

### Add a deterministic command

```text
bevyout cache stats \
  --cache .bevyout/cache \
  --manifest-set reports/sample-cells.ron \
  --json reports/cache-baseline.json \
  --csv reports/cache-objects.csv
```

Output should be stable and suitable for tests and issue comments, consistent with the repository’s deterministic CLI-output convention. ([GitHub][8])

### Measurements

| Area          | Required measurements                                                                                            |
| ------------- | ---------------------------------------------------------------------------------------------------------------- |
| Filesystem    | Logical bytes, allocated bytes, file count, directory count                                                      |
| Category      | GLB, embedded KTX2, external KTX2, physics, audio, shadows, manifests, catalogs, navigation, bake outputs        |
| Deduplication | Unique payload bytes, duplicate physical bytes, duplicate clusters, largest repeated objects                     |
| GLB internals | Geometry, animation, images, JSON, padding, other buffers                                                        |
| Textures      | Dimensions, mip count, channel count, role, color space, codec, encoded size                                     |
| Preparation   | Cold, warm, one-cell incremental, CPU time, wall time, bytes written                                             |
| Runtime       | Initial scene load, cell activation p50/p95, peak process residency, GPU texture/mesh residency where measurable |
| Streaming     | Three-boundary crossing and return, active/preloaded cell counts, eviction behavior                              |
| Reliability   | Failed preparations, invalid artifacts, stale fingerprints                                                       |

Measure both **logical file size** and **physical allocated size**. Packaging may reduce allocation overhead without materially changing logical bytes.

### Exit gate

* Baseline report is reproducible on the same machine.
* Every large file class is attributed.
* Exact duplicate bytes are known.
* The proposed 40–70% target is either supported by the duplicate inventory or reclassified as a later stretch target.

---

## Wave 1 — Global object store and recipe index

### Objective

Introduce storage semantics without changing artifact formats.

### Core operations

```rust
trait PreparedObjectStore {
    fn resolve_recipe(&self, recipe: &RecipeId) -> Result<Option<ObjectRef>>;
    fn publish(&self, recipe: &RecipeId, candidate: CandidateObject)
        -> Result<ObjectRef>;
    fn open(&self, object: &ObjectRef) -> Result<File>;
    fn verify(&self, object: &ObjectRef) -> Result<Verification>;
}
```

### Atomic publication algorithm

1. Create the temporary file inside the destination shard or on the same filesystem.
2. Stream converter output while calculating the payload SHA-256.
3. Validate the completed artifact:

   * Parse GLB.
   * Validate KTX2.
   * Parse physics/catalog RON.
   * Probe audio headers.
4. Flush the file and publish it atomically.
5. If the destination already exists:

   * Verify its length and hash.
   * Discard the duplicate temporary file.
6. Write the recipe record only after the object is durable.
7. On corruption, move the existing object to quarantine and rebuild.

Use either a short-lived per-recipe lock or optimistic concurrent publication. Do not use a global object-store lock.

### Source-path normalization

Apply case-insensitive normalization to **source identities**, not to payload-object paths:

* Convert `\` to `/`.
* Use invariant ASCII lowercase for Fallout asset paths.
* Remove redundant leading `Data/`.
* Collapse repeated separators.
* Reject absolute paths, drive prefixes, `.` and `..`.
* Preserve the original display path separately for diagnostics.
* Do not use locale-sensitive lowercasing.

Object paths use lowercase hexadecimal hashes and therefore need no source-path case rules.

### Tests

* Same source through the same policy resolves to one recipe.
* Two recipes producing the same payload resolve to one object.
* Source-byte change invalidates the recipe.
* Converter revision change invalidates the recipe.
* Format-policy change invalidates the recipe.
* Interrupted write leaves no visible partial object.
* Multiple workers publishing the same object succeed.
* Existing corrupt object is detected and replaced.
* Windows-style and Unix-style source paths normalize identically.

### Exit gate

No artifact bytes change relative to the baseline. Only their storage and references change.

---

## Wave 2 — Migrate complete artifact classes

### Objective

Move currently separate prepared products into the global store before introducing lossy or structural format changes.

### Recommended order

1. Existing GLB and physics pairs.
2. Prepared audio.
3. Actor animation and actor catalogs.
4. Navigation and other generated catalogs.
5. Static shadows.
6. Reflection probes, lightmaps, and bake products.
7. Worldspace LOD assets.

Scene manifests remain ordinary root documents initially. They identify the object graph that garbage collection must retain.

### Preparation concurrency

Replace shared mutable staging with:

```text
staging/<process-id>/<recipe-id>/
```

Converters receive isolated input and output directories. Publication is the only shared step.

This should allow the current broad `asset_stage_lock` to be replaced with narrow locks around genuinely shared indexes, while physics and other immutable objects remain reusable across workers. ([GitHub][6])

### Migration command

```text
bevyout cache migrate \
  --from .bevyout/cache \
  --backup-manifests \
  --verify
```

The command should:

1. Parse supported legacy manifests through migration-specific code.
2. Hash and publish every referenced artifact.
3. Rewrite manifest paths atomically.
4. Verify the rewritten manifest.
5. Preserve a migration journal.
6. Remove old copies only after the entire migration generation succeeds.

Do not make the runtime viewer a permissive legacy-cache parser.

### Exit gate

* At least **95% of measured exact duplicate physical bytes** are eliminated.
* Object-store metadata overhead remains below a proposed **2% of unique payload bytes**.
* Cold preparation increases by no more than 10%.
* Warm preparation remains flat or improves.
* Concurrent preparation produces deterministic manifests.

---

## Wave 3 — Extract embedded textures from GLBs

### Objective

Deduplicate texture payloads independently of model geometry.

### Required spike

Before converting the full pipeline, prepare a small test containing:

* Two GLBs using the same diffuse texture.
* Two GLBs using the same normal/specular texture.
* One unique texture.
* One alpha-cutout texture.
* One animated or skinned model.

Prove:

1. Both GLBs can resolve the same external KTX2 object.
2. Bevy loads the scene correctly.
3. A common canonical asset path produces one shared image handle or one GPU allocation.
4. Unloading one GLB does not prematurely evict a texture still used by the other.
5. The solution works with the cache outside the project source tree.

### Implementation choices

**Preferred:** a cache-root Bevy asset source or resolver that exposes canonical object paths.

```text
prepared://texture/<sha256>
prepared://glb/<sha256>
```

**Fallback:** a custom prepared-model asset containing:

```rust
struct PreparedModel {
    geometry_object: PreparedObjectRef,
    textures: Vec<PreparedTextureBinding>,
    material_bindings: Vec<PreparedMaterialBinding>,
}
```

The loader reads geometry and binds shared textures after loading.

Do not make hard links or symbolic links the semantic foundation. They can be optional migration optimizations, but they complicate Windows behavior, garbage collection, and portability, and they do not solve duplicated embedded bytes by themselves.

### Exit gate

* Duplicate embedded textures occupy one physical object.
* Texture hashes are stable across different referring GLBs.
* Runtime captures match the baseline.
* Peak GPU texture residency does not increase.
* Cell unload and reload behavior remains correct.

---

## Wave 4 — Role- and channel-aware KTX2 policy

### Objective

Reduce texture size without sacrificing Fallout material semantics.

The current Bevy KTX2 path can transcode one-channel UASTC layouts to BC4/EAC R11 and two-channel layouts to BC5/EAC RG11, while RGB/RGBA UASTC can become ASTC or BC7 depending on hardware support. ([GitHub][9])

### Introduce explicit texture roles

```rust
enum PreparedTextureRole {
    BaseColorOpaque,
    BaseColorAlpha,
    NormalSpecular,
    NormalXy,
    SpecularMask,
    GrayscaleMask,
    Emissive,
    Ui,
    Decal,
    Lightmap,
    OtherLinear,
}
```

Resolve roles from material slots first. Filename conventions should only be a fallback.

### Initial policy

| Role                           | Initial format policy                                          |
| ------------------------------ | -------------------------------------------------------------- |
| Base color, opaque             | sRGB UASTC with mipmaps                                        |
| Base color with alpha          | sRGB RGBA UASTC with alpha-preserving mipmaps                  |
| Normal plus specular alpha     | Keep RGBA linear UASTC initially                               |
| Normal XY after material split | Two-channel linear UASTC, runtime BC5/EAC                      |
| Specular or grayscale mask     | One-channel linear UASTC, runtime BC4/EAC                      |
| Emissive                       | Role-specific sRGB/linear decision based on shader contract    |
| UI and decals                  | RGBA UASTC; prioritize quality and alpha edges                 |
| Lightmaps                      | Separate profile after auditing channel type and dynamic range |

### Normal/specular split

The highest-value channel reduction for normal maps requires a deliberate material change:

```text
Original:
  RGBA = normal XYZ + specular strength in A

Prepared:
  RG   = encoded normal XY
  R    = specular strength
```

The shader reconstructs normal Z and samples the specular mask separately.

This requires:

* Material-schema revision.
* Converter revision.
* Shader revision.
* Tests for gloss/specular response.
* GPU-sampling cost measurement.
* Proof that the disk and residency savings exceed the additional texture binding cost.

Until that work is accepted, normal/specular textures should remain RGBA.

### ETC1S policy

Do not adopt ETC1S as the default under Bevy 0.19. Its KTX2 loader currently contains an unimplemented transcoding path and only accepts the encoded data directly when the target ETC2 format is available. ([GitHub][9])

UASTC is the safer portable profile for the current runtime.

### Mipmap correctness

* Generate normal-aware mipmaps that renormalize vectors.
* Preserve alpha coverage for cutout textures.
* Do not generate sRGB mips as linear byte averages.
* Preserve mipmaps where runtime filtering requires them.
* Store mip policy in the recipe identifier.

### Quality evaluation

For each texture role:

* Encoded bytes.
* Runtime GPU format and GPU bytes.
* Decode/transcode time.
* Upload time.
* SSIM/PSNR from the KTX tooling where applicable.
* Fixed-camera visual captures.
* Normal/specular highlight comparison.
* Alpha-edge and distant-shimmering comparison.

### Exit gate

No policy is enabled globally until its role-specific test set passes. Existing valid objects are reused unless the role-policy recipe changes.

---

## Wave 5 — Extension-free geometry optimization

### Objective

Reduce GLB geometry and animation bytes while remaining compatible with the existing Bevy loader.

### First production pass

Apply optimizations that retain ordinary glTF accessors and buffers:

* Remove unreferenced meshes, nodes, accessors, and buffer views.
* Deduplicate identical vertices.
* Remove attributes not consumed by the material or runtime.
* Reorder triangle indices for vertex-cache locality.
* Reorder vertex data for vertex-fetch locality.
* Use 16-bit indices where vertex counts permit.
* Deduplicate identical buffer ranges inside one asset.
* Remove redundant animation keys.
* Quantize animation time/value data only where standard accessor types and error tolerances permit.
* Canonicalize buffer alignment and eliminate excessive padding.

Collision and physics geometry remain separate and must not inherit visual simplification automatically.

### Optional research pass

`EXT_meshopt_compression` and `KHR_mesh_quantization` can only become production options after either:

1. Bevy gains verified support at the repository’s selected Bevy version, or
2. bevyout adds and maintains a custom decoder/loader.

Both extensions are currently unsupported by the Bevy 0.19 glTF loader. ([GitHub][5])

The research gate must measure:

* Compressed bytes.
* CPU decode time.
* Allocation count.
* Scene activation latency.
* GPU upload time.
* Animation and skinning correctness.
* Error recovery when an object is corrupt.

### Exit gate

* Static and actor assets pass visual comparison.
* Skinning and animation poses remain within agreed tolerances.
* Physics assets are byte-identical unless a separate physics change is approved.
* p95 cell activation does not regress beyond the agreed limit.

---

## Wave 6 — Audio deduplication and optional codec policy

### Objective

Eliminate duplicate clips first, then determine whether selective transcoding is worthwhile.

The current build enables Bevy WAV and Vorbis support, so a role-based audio policy is technically available without introducing a new runtime codec. ([GitHub][10])

### Sequence

1. Hash final audio payloads and deduplicate exact matches.
2. Detect source aliases and case-only path duplicates.
3. Record duration, channels, sample rate, bit depth, codec, and bytes.
4. Classify clips:

   * Short latency-sensitive effects.
   * Footsteps.
   * Interface sounds.
   * Ambient loops.
   * Dialogue.
   * Music.
5. Test optional Vorbis conversion only for categories where it produces material savings.

### Preserve

* Loop boundaries.
* Channel layout.
* Sample rate where resampling would be audible.
* Trigger latency.
* Dialogue intelligibility.
* Stable clip identity in manifests and animation cues.

Do not recompress media that is already suitably compressed unless a measured policy shows a clear benefit.

### Exit gate

* Exact duplicates are fully eliminated.
* No loop seams or missing events.
* Audio activation latency does not regress.
* Any lossy policy has listening-test evidence and per-role settings.

---

## Wave 7 — Static shadows and bake-output storage

### Objective

Deduplicate outputs while keeping the current quality contract.

Prepared point shadows are currently written beneath each scene’s `shadows` directory, use a semantic fingerprint, and are stored as Zstandard-compressed `D32_SFLOAT` KTX2 cubemap arrays. The documented default remains 512, with 256 and 128 retained only as explicit lower-quality options. ([GitHub][11])

### Revised identity model

```text
ShadowRecipeId:
  generator revision
  resolution and near plane
  caster object hashes
  caster transforms
  light identities and transforms

ShadowObjectId:
  SHA-256(final KTX2 bytes)
```

A semantic recipe can map to an existing identical payload even when the semantic identifiers differ.

### Keep unchanged initially

* 512 default.
* `D32_SFLOAT`.
* Current near plane.
* Light and caster eligibility.
* Runtime upload behavior.
* Existing visual quality.

A lower-precision depth representation should be a separate renderer-quality investigation, not part of the initial deduplication work.

### Runtime versus optional cache tiers

Classify bake products as:

* `runtime-required`
* `rebuildable-runtime`
* `validation`
* `debug`
* `intermediate`

Validation pages, temporary raw faces, diagnostic captures, and obsolete bake generations should not be retained indefinitely with the required runtime cache.

### Exit gate

* Exact duplicate shadow and bake payloads are shared.
* 512 remains the default.
* Non-black capture and shadow-receiver behavior remain unchanged.
* Rebuilding one corrupt object does not force unrelated cells to rebuild.

---

## Wave 8 — Cache operations, garbage collection, and optional packs

### Required commands

```text
bevyout cache stats
bevyout cache verify
bevyout cache explain <hash-or-path>
bevyout cache migrate
bevyout cache gc --dry-run
bevyout cache gc
bevyout cache compact
```

### Garbage collection

Use mark-and-sweep:

1. Acquire a cache-generation lock.
2. Read all current scene, catalog, job, and retained-generation roots.
3. Traverse recipe-to-object references.
4. Mark live objects.
5. Report unreferenced objects and their age.
6. Sweep only objects older than a configurable grace period.
7. Clean stale temporary directories and incomplete recipe records.
8. Never delete quarantine data automatically without an explicit retention rule.

Do not use modification time as the sole indication of liveness.

### Trigger for packaging

Do not add a pack format merely because loose files appear inelegant. Add it only when at least one of these is measured:

* Allocated bytes materially exceed logical bytes.
* File count causes costly directory traversal or antivirus overhead.
* Open/close activity materially affects cell activation.
* Metadata consumes a material percentage of cache size.
* Copying or distributing a prepared cache is impractically slow.

### Pack design

```text
packs/
  <generation>/
    small-runtime-0001.pack
    small-runtime-0001.index
    catalogs-0001.pack
    physics-0001.pack
```

Requirements:

* Immutable packs.
* Hash-to-offset index.
* Per-object length and checksum.
* Bounded pack sizes selected by benchmark.
* Range-readable and seekable.
* Recovery at pack granularity, not whole-cache granularity.
* Explicit compaction rather than in-place mutation.
* Loose objects remain usable during interrupted compaction.

Pack small metadata and high-count objects first. Do not assume another compression layer will materially shrink UASTC/Zstandard KTX2, compressed audio, or already-compressed GLBs. Benchmark each artifact class.

---

# Revision and invalidation matrix

The repository currently has strict manifest schema, preparation, converter, physics, and artifact revisions. The cache project should preserve that discipline rather than using one global “cache v2” switch. ([GitHub][2])

| Change                                                  | Required revision action                                             |
| ------------------------------------------------------- | -------------------------------------------------------------------- |
| Object-store path semantics only                        | Bump prepared meaning/revision and prepare-pipeline fingerprint      |
| New manifest fields or typed references                 | Bump manifest schema and prepared revision                           |
| GLB byte-generation change                              | Bump static or actor converter revision                              |
| KTX role/channel policy change                          | Bump dedicated texture-policy revision included in converter recipes |
| Normal/specular material split                          | Bump converter, material, shader, and prepared revisions             |
| Physics serialized shape                                | Bump physics schema and physics pipeline revision                    |
| Shadow-generation byte change                           | Bump static-shadow generator revision                                |
| Pack container format                                   | Bump pack/index revision; object hashes remain unchanged             |
| Storage implementation only, identical manifest meaning | Do not unnecessarily invalidate converter outputs                    |

A dedicated `CACHE_STORE_REVISION` should govern store metadata without forcing expensive asset reconversion.

---

# Acceptance matrix

| Dimension               | Gate                                                                                            |
| ----------------------- | ----------------------------------------------------------------------------------------------- |
| Exact deduplication     | Recover at least 95% of measured exact duplicate physical bytes                                 |
| Metadata overhead       | No more than 2% of unique payload bytes                                                         |
| 49-cell size            | 40–70% reduction remains a stretch gate until the baseline proves it is attainable              |
| Preparation             | Cold preparation increase under 10%; warm preparation flat or improved                          |
| Incremental preparation | Unchanged cells produce no new converter work                                                   |
| Reliability             | Zero increase in failed preparations                                                            |
| Manifest correctness    | All fingerprints and revision checks current                                                    |
| Runtime loading         | p95 cell activation regression preferably below 5%                                              |
| Streaming               | Cross and return across three cell boundaries within the existing residency bound               |
| Residency               | No increase in resident-cell count; shared texture/object handles do not multiply GPU residency |
| Visual output           | Fixed-camera captures pass agreed pixel/SSIM thresholds and human review                        |
| Normal/specular         | Highlights and normal orientation match representative baseline scenes                          |
| Alpha assets            | No new halos, cutout disappearance, or mip-related coverage loss                                |
| Shadows                 | 512 default retained; no new leaking, peter-panning, or missing receivers                       |
| Audio                   | No missing cues, loop seams, or material trigger-latency increase                               |
| Recovery                | Interrupted writes, concurrent publication, corruption, migration, and GC tested                |
| Manual evidence         | Non-black capture plus deterministic logs and cache-stat output                                 |

For lossless storage waves, the strongest visual gate is that the referenced final artifact bytes remain unchanged. Pixel tolerances are principally needed for texture and geometry compression waves.

---

# Estimating whether an 8 GB prepared cache is feasible

Do not extrapolate linearly from bytes per cell. Shared assets cause the marginal cost per additional cell to fall as coverage increases.

Use a saturation model:

```text
projected full cache =
    currently observed unique object bytes
  + estimated marginal unique bytes for uncovered content classes
  + store/index overhead
```

Report:

* Unique bytes after each additional cell.
* New-object count per additional cell.
* Duplicate ratio by sample cohort.
* A lower and upper projection for full exterior and full-game coverage.
* Runtime-required and optional-cache totals separately.

An 8 GB hard ceiling should be adopted only after texture extraction and role-based texture experiments indicate that it can be reached without quality or streaming regressions. Before that point, the correct hard goal is:

> No unnecessary physical duplication, no stale generations in the runtime tier, and measured format choices for every major byte category.

---

# Suggested PR and issue breakdown

| Work item                       | Scope                                                                           |
| ------------------------------- | ------------------------------------------------------------------------------- |
| **1. Cache inventory**          | Statistics model, GLB/KTX inspection, deterministic JSON/CSV report             |
| **2. Object-store core**        | Recipe IDs, payload hashes, atomic publication, verification, concurrency tests |
| **3. Resolver and migration**   | Shared paths, manifest rewriting, controlled legacy migration                   |
| **4. Whole-artifact migration** | GLB, physics, audio, shadow, catalog, bake objects                              |
| **5. Texture extraction spike** | External/shared KTX2 prototype and GPU-handle instrumentation                   |
| **6. Texture policy**           | Role classification, one-/two-channel UASTC, normal/specular preservation       |
| **7. Geometry optimization**    | Extension-free optimizer and representative asset comparisons                   |
| **8. Cache lifecycle**          | Verify, explain, GC, generation retention, quarantine                           |
| **9. Optional packs**           | Small-object pack prototype and random-access benchmark                         |
| **10. Full acceptance report**  | 49-cell comparison, supplemental cohorts, traversal, size projection            |

Likely code areas include:

```text
src/vsa/assets/
src/vsa/prepare/
src/vsa/manifest/
crates/bevyout-core/src/manifest.rs
src/viewer/ asset-loading units
CLI command and argument modules
features/ and dedicated test modules
```

Repository convention requires feature lists and tests before implementation, deterministic CLI evidence, a manual acceptance script for every wave, and an explicit execution-model recommendation. ([GitHub][8])

**Recommended execution model:** `Sol X-High` for a Codex runtime, or an `Opus` orchestrator with `Sonnet` executors for a Claude runtime. The object-store, manifest migration, and external-texture wave should be reviewed as architecture-sensitive changes rather than combined into one large implementation PR.

---

# Parallel track: application package size

This should remain separate from the prepared-cache project.

The current root `Cargo.toml` does not define explicit release `lto` or `strip` settings. A release-package audit can evaluate:

```toml
[profile.release]
lto = "thin"
codegen-units = 1
strip = "symbols"
```

`panic = "abort"` can also be evaluated, but only if crash-reporting and failure behavior remain acceptable. The exact settings should be benchmarked for build time, executable size, startup, and platform compatibility. ([GitHub][10])

Also exclude development-only dynamic-linking dependencies, test fixtures, reports, source maps, and debug symbols from release packages. Original Fallout data should remain external rather than being incorporated into the bevyout distribution. ([GitHub][1])

---

# Recommended order of attack

1. **Measure logical, physical, and internally embedded bytes.**
2. **Implement recipe-to-payload object storage.**
3. **Migrate existing whole artifacts without changing formats.**
4. **Extract and deduplicate embedded textures.**
5. **Introduce role- and channel-aware KTX2 policies.**
6. **Apply extension-free geometry optimization.**
7. **Deduplicate and selectively optimize audio, shadows, and bake outputs.**
8. **Add GC and cache generations.**
9. **Package small objects only when filesystem measurements justify it.**

The two changes most likely to produce the largest safe reduction are the global recipe/payload store and extraction of repeated KTX2 payloads from otherwise different GLBs. Archive packaging and unsupported glTF compression extensions should not be the starting point.
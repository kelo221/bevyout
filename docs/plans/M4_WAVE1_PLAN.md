# M4 wave 1 — actor record preparation (#103)

Wave under epic #9 on branch `m4-wave1` off master. Executors run per
AGENTS.md model routing; the orchestrator owns GitHub housekeeping, merges,
conflict resolution, gates, and real-data acceptance. One issue, three
executor tasks in two phases:

- **Phase 1 (parallel worktrees):** task A (NPC_/CREA decode) and task B
  (RACE/CLAS/FACT/PACK decode) — pure ESM4 subrecord decoding, no policy.
- **Phase 2 (wave branch):** task C — pure actor-catalog resolution
  (templates, diagnostics, counters) plus manifest/prepare wiring, after A
  and B are merged.

## Fixed feature lists

### Task A — NPC_ and CREA actor decode (`src/vsa/openmw_esm4/actors.rs`)

- Decode NPC_ actor subrecords into `ActorData` on `BaseRecord`: `ACBS`
  (flags, fatigue, barter gold, level/level-mult, calc min/max, speed
  multiplier, karma, disposition, template flags), `SNAM[]` faction
  memberships (form id + rank), `INAM` death item, `VTCK` voice, `RNAM`
  race, `EITM` actor effect/item, `SCRI` script, `AIDT` AI data
  (aggression, confidence, energy, responsibility, mood, services, teach
  skill, train level, assistance, aggro radius behavior), `PKID[]`
  packages, `CNAM` class, `DATA`/`DNAM` base stats and skills, `PNAM[]`
  head parts, `HNAM`/`ENAM` hair/eyes, `LNAM` hair length, `HCLR` hair
  color, `ZNAM` combat style, `NAM6`/`NAM7` height/weight, and
  `FGGS`/`FGGA`/`FGTS` FaceGen coefficients retained as opaque bytes.
- Decode CREA equivalents: `NIFZ`/`NIFT` model lists, `KFFZ` animation
  files, body-part `NIFZ` data, `ACBS`, `SNAM[]`, `INAM`, `PKID[]`,
  `AIDT`, `DATA` (creature type, combat skill, health, damage,
  attributes), attack reach, `ZNAM` combat style, turning speed, base
  scale, foot weight, `CSCR`/`CSDT`-style inherited-sound references.
- Field layouts are verified against the fopdoc Fallout 3 pages and the
  OpenMW `loadnpc`/`loadcrea`/`actor.hpp` sources, not assumed; unknown or
  malformed subrecords land in `ignored_subrecords` diagnostics exactly
  like existing parsers.
- FormID subrecords go through `sub_form_id`/`FormIdResolver` so
  load-order adjustment stays uniform.

### Task B — supporting records (`src/vsa/openmw_esm4/actor_support.rs`)

- `RACE`: male/female heights and weights, flags, older/younger race
  links, default hair per sex, default hair colors, hair/eyes candidate
  lists, head/body part model references (per-index groups kept minimal
  but deterministic), FaceGen base data retained opaque.
- `CLAS`: tag skills, class flags, services, teaches/max-training data.
- `FACT`: flags, interfaction relations (`XNAM`), ranks (rank number plus
  male/female titles, insignia), crime-related values present in FO3.
  Decoded from the fopdoc spec — OpenMW has no FACT loader to port.
- `PACK`: package type and flags (`PKDT`), location (`PLDT`), schedule
  (`PSDT`), target (`PTDT`), and `CTDA` conditions retained as opaque
  bytes the way `RecipeRecord` already does.
- New `ParsedState`/`ParsedPlugin` maps: `races`, `classes`, `factions`,
  `packages`, each `HashMap<u32, …Record>` with load-order override
  semantics identical to existing maps, plus `reader.rs` dispatch arms.

### Task C — actor catalog and template resolution (phase 2)

- Pure std/serde-only module `src/vsa/prepare/actor_catalog.rs`: resolve
  each NPC_/CREA base into an `ActorBlueprint` by applying `ACBS` template
  flags field-by-field (traits, stats, factions, actor effect list, AI
  data, AI packages, model/animation, base data, inventory, script)
  through `TPLT` chains, including chains that pass through `LVLN`/`LVLC`
  leveled actors (record the candidate set deterministically).
- Detect and diagnose cycles, missing templates, unresolved race/class/
  faction/package links, and unsupported inheritance flags — actionable
  diagnostics, never silent fallback to a generic static.
- Blueprint carries: stable base + reference FormIDs, display/model data,
  race and sex, actor values, faction memberships with ranks, initial
  inventory (reusing the existing item-catalog contract from #95/#98, not
  duplicating it), package ids, animation candidates, ACHR/ACRE placement
  identity with initial enable/transform state, and diagnostics.
- Versioned, serde-defaulted actor catalog on `PreparedSceneManifest` with
  deterministic ordering and `prepare` report counters: prepared,
  inherited, unresolved, unsupported, skipped.

## Tests first

- Tasks A/B: synthetic byte-level fixtures via the existing
  `tests/mod.rs` builders (`subrecord`/`record`/`group`/`tes4`); each task
  adds its own test module file (`tests/actors.rs`, `tests/actor_support.rs`)
  registered with one line in `tests/mod.rs`. Cover every supported
  subrecord, load-order overrides, deleted records, and
  malformed/truncated payload diagnostics.
- Task C: cucumber feature (`features/actor_catalog.feature`) exercising
  pure catalog resolution — template inheritance per flag, cycles, missing
  links, leveled templates, deterministic serialization — plus unit tests.
  `tests/features.rs` additions append-only per AGENTS.md.

## File ownership

- A owns `actors.rs`, `tests/actors.rs`, the `actor: Option<ActorData>`
  field on `BaseRecord`, and the minimal `parse_base`/records.rs hook that
  populates it for NPC_/CREA.
- B owns `actor_support.rs`, `tests/actor_support.rs`, the four new
  `ParsedState`/`ParsedPlugin` maps and their merge/select plumbing, and
  the new `reader.rs` dispatch arms.
- Shared-file conflicts (`mod.rs`, `tests/mod.rs`, `NOTICE.md`) are
  expected to be one-line-adjacent and are resolved by the orchestrator at
  merge. Both tasks update `NOTICE.md` for newly adapted OpenMW layouts.
- C owns `prepare/actor_catalog.rs`, manifest additions, prepare
  orchestrator call sites, the feature file, and its `features.rs` steps.

## Gates and acceptance

`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test`, and a representative `cargo run-dev -- prepare` after each
merge. Real-data acceptance prepares representative human, ghoul/super-
mutant-style, and creature records from the FO3 masters and reports
counts, unresolved links, cache size, and preparation time on #103.
Manual steps land in `docs/plans/M4_WAVE1_MANUAL.md` before the PR.

## Shipped amendments

- **A1 — per-cell actor catalog storage.** Real-data acceptance caught the
  initial task C artifact layout keying the actor catalog by the
  content-set source fingerprint (the item/recipe catalog precedent).
  Those catalogs are cell-independent; the actor catalog embeds per-cell
  ACHR/ACRE placements, so consecutive prepares overwrote one shared
  `catalogs/<fingerprint>/actors.ron` and left earlier cells' manifests
  pointing at the wrong actors. Fixed to `scenes/<cell>/actors.ron` next
  to `scene.ron` (self-cleaning with the scene directory);
  `actor_catalog_hash` covers exactly that cell's serialized catalog, and
  a regression test asserts two cells from one content set keep distinct,
  non-clobbered artifacts.
- **A2 — FO3 FACT has no crime fields.** The planned "FO3 crime-related
  values if present" resolved to none: fopdoc documents FACT `CNAM` as an
  unused float and no CRVA-style subrecord exists in FO3 (that is
  FNV/TES5). `CNAM` is accepted-and-ignored rather than diagnosed.
- **A3 — deliberate OpenMW divergence.** PACK `PTDT` FormID resolution is
  gated on `PTDT`'s own type field; OpenMW's `loadpack.cpp` gates it on
  `mLocation.type`, a copy-paste bug not reproduced here (documented in
  `NOTICE.md`). FO3's 20-byte `AIDT`, 11-byte `NPC_.DATA`, 28-byte
  `NPC_.DNAM`, and 17-byte `CREA.DATA` are decoded from fopdoc where the
  OpenMW snapshot skips or only handles TES4 layouts.

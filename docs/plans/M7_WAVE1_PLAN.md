# M7 wave 1 plan — shared record stream and script inventory

Cross-wave architecture and Waves 2-7 are preserved in
`M7_SCRIPTING_ARCHITECTURE_ROADMAP.md`.

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. This wave changes the shared ESM4
load-order traversal and report schema, must preserve existing content-index
semantics, and introduces script/attachment identities that constrain later IR
and save compatibility. In the Codex runtime, execute directly on one wave
branch; do not spawn implementation subagents.

## Fixed feature list

### #252 — reusable resolved record stream

- Expose an internal borrowed record envelope with resolved FormID, signature,
  flags/deletion state, source/provenance, and payload/subrecords.
- Preserve compression, load-order validation, master remapping, override,
  deletion, fingerprint, diagnostics, and deterministic ordering.
- Drive `ContentIndexBuilder` through a collector/fan-out seam without exposing
  arbitrary payloads from public index types.

### #253 — structural `ScriptCatalog`

- Model stable top-level and embedded script asset identities.
- Decode SCHR, SCDA, SCTX, SLSD, SCVR, and SCRO structurally from winning SCPT
  records; resolve FormIDs and retain variable/reference slots.
- Preserve unknown subrecords, provenance, presence combinations, and contextual
  parse failures without interpreting source or bytecode.

### #254 — attachment and embedded-script inventory

- Index SCRI attachments by resolved owner, record signature, and stable slot.
- Recognize embedded script collections such as current PACK script subrecords
  without requiring a top-level SCPT FormID.
- Preserve override/deletion behavior, missing-target diagnostics, provenance,
  and deterministic ordering.

### #255 — deterministic report and real-data gate

- Extend `src/vsa/report/`; do not add a parallel reporting framework.
- Emit sorted JSON and CLI totals for script kinds, SCDA/SCTX combinations,
  byte sizes, variable/reference counts, attachment signatures,
  top-level/embedded scripts, parse failures, provenance, and fingerprint.
- Keep opcode/function/event frequency explicitly deferred to the SCDA
  disassembler wave.
- Run the report twice against locally installed Fallout3.esm, compare output,
  and post measured non-Bethesda-derived counts/limitations to #255.

## Execution order and ownership

These issues share traversal/catalog/report seams and therefore execute
sequentially on the wave branch: #252, #253, #254, then #255. Keep the primary
ownership boundaries as:

- #252: `src/vsa/content_index/` plus the minimum shared ESM4 traversal seam;
- #253: `src/vsa/scripts/` catalog/record modules;
- #254: `src/vsa/scripts/` attachment modules and narrow existing record
  adapters;
- #255: `src/vsa/report/`, CLI wiring, report goldens, and acceptance docs.

`features/script_inventory.feature` and its delimited `tests/features.rs`
steps are the shared merge seam. Each issue appends only its own scenarios and
steps.

## Feature-first test order

1. Fix Wave 1 scenarios for compressed records, remapping, winning overrides,
   deletions, structural fields, attachments, unknown/malformed content, and
   deterministic reports.
2. Add focused content-index equivalence and parser/report unit tests before
   each production change.
3. Implement #252 and prove all existing content-index behavior unchanged.
4. Implement #253 and #254 against the shared stream.
5. Implement #255, update the report schema revision/goldens, and prove
   byte-identical repeated output.
6. Run formatting, clippy, tests, architecture checks, and local real-data
   acceptance. Write `M7_WAVE1_MANUAL.md` before the wave PR.

## Acceptance gates

- Existing `ContentIndex` results and diagnostics retain their semantics.
- One load-order scan can feed metadata, script, attachment, and report
  collectors; `ContentIndex` does not retain arbitrary record payloads.
- Synthetic compressed, overridden, remapped, and deleted SCPT records produce
  one deterministic winning catalog.
- Unknown subrecords and malformed scripts produce contextual diagnostics, not
  panics or whole-import failure.
- Embedded identity does not require an SCPT FormID, and SCRI targets are
  canonical resolved FormIDs.
- Two report runs over equal input are byte-identical.
- The local Fallout3.esm report completes and writes only to `.bevyout/`; no
  extracted source, bytecode, RON, or other Bethesda-derived artifact is
  staged.
- Repository gates pass: `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, and a
  representative `cargo run-dev -- report` command.

## Shipped amendments

- Fallout3.esm acceptance found every one of 3,021 PACK records carries all
  three action markers and an `SCHR`; 8,553 of those scripts are header-only
  with neither SCDA nor SCTX. They remain counted as embedded scripts under
  the explicit `neither` representation bucket. A bare action marker with no
  script subrecords is not counted; non-empty groups without `SCHR` remain
  retained and diagnosed.
- Real data also proved `SCHR.variable_count` is not the cardinality of SLSD
  entries (348 valid scripts differ, often by one). The report uses the
  authoritative header count and the catalog no longer emits that false
  mismatch diagnostic; decoded SLSD slots remain independently preserved.

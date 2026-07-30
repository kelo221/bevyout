# M7 wave 1 manual acceptance — script inventory reporting

Wave 1 ships the foundation of bevyout script support: one resolved ESM4
record stream shared by the content index and the report, a structural
`ScriptCatalog` decoding every top-level SCPT record and embedded package
script without interpreting their bytecode, and a deterministic script
inventory report over real Fallout 3 data. Nothing executes scripts yet; the
acceptance surface is the `report` CLI and its stable, repeatable output.

## Setup

1. Ensure the local Fallout 3 installation is configured (the `report`
   command resolves `Fallout3.esm` through `config.toml`'s `game_root`, as
   `prepare` does).

2. Run the report twice into two directories:

   ```powershell
   cargo run-dev -- report --out-dir .bevyout/reports/m7-a
   cargo run-dev -- report --out-dir .bevyout/reports/m7-b
   ```

3. Expect the summary totals to be identical in wording and numbers in both
   runs, ending with the script lines:

   ```text
   scripts: top-level=1257 embedded=9063 attachments=10653 compiled-bytes=427312 variables=5119 references=8091 diagnostics=0
   script kinds: effect=38 object=10181 quest=101
   script representations: neither=8553 scda_sctx=1752 sctx_only=15
   ```

4. Compare the two runs byte-for-byte:

   ```powershell
   diff .bevyout/reports/m7-a/Fallout3.report.json .bevyout/reports/m7-b/Fallout3.report.json
   diff .bevyout/reports/m7-a/Fallout3.summary.txt .bevyout/reports/m7-b/Fallout3.summary.txt
   ```

   Expect no output (identical files) and exit code 0.

5. Open `.bevyout/reports/m7-a/Fallout3.report.json` and confirm the new
   `script_inventory` section: `schema_version` is `2`, entries stay sorted,
   and the inventory carries `content_fingerprint`, `totals`, `by_kind`,
   `by_representation`, `attachment_owner_signatures`, and sorted
   `scripts`/`attachments`/`diagnostics` lists with provenance.

## What to look for

- Report schema version is bumped to `2` and `tests` goldens match.
- Embedded scripts (the `embedded=9063` majority) need no top-level SCPT
  FormID; header-only PACK scripts are counted under the explicit `neither`
  representation bucket.
- Attachment owners resolve to canonical FormIDs (10,653 SCRI attachments;
  `attachment_owner_signatures` groups them by record type).
- No extracted source, bytecode, RON, or other Bethesda-derived artifact is
  written anywhere except the gitignored `.bevyout/reports/` output.

## Recorded implementation acceptance

The Wave 1 implementation run executed `report` three times against the
locally installed `Fallout3.esm` (fingerprint
`d9fb0a33af495ddb43992b96ea74f2741b123fefdb1fcdcea28096f7649b0d06`). Runs two
and three, produced by the final code, were byte-identical for both the JSON
report and the text summary; run one predated the shipped amendments. Final
totals: 1,257 top-level and 9,063 embedded scripts, 10,653 attachments,
427,312 compiled bytes, 5,119 header-declared variables, 8,091 SCRO
references, zero diagnostics. Gates passed: `cargo fmt --check`,
`cargo clippy --all-targets -- -D warnings`, and `cargo test` (1,415 unit
tests, 586 cucumber scenarios).

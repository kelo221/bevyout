# All-Exterior Preparation and Free-Roam Handoff

Last refreshed: 2026-08-13

## Objective

Enable free exploration across every exterior cell in the resolved Fallout
plugin chain by adding an explicit `prepare --all-exteriors` batch selector
while preserving targeted preparation, `--all-interiors`, and existing `--all`
behavior.

Completion requires all three gates below. Do not weaken them:

1. The selector resolves exactly the catalog's exterior-cell set.
2. Every selected exterior cell is prepared to current fingerprints with zero
   failed cells.
3. Ordinary player movement crosses at least three adjacent exterior-cell
   boundaries and returns along the same route, with `failed=0`,
   `stale_completions=0`, `invalid_unload_count=0`, and exterior residency
   never above 25 cells.

If real data contains unsupported cells, stop and report the exact FormIDs and
causes. Do not silently omit cells or reduce the acceptance set.

## Repository State

- Repository: `C:\Users\V\Projects\Rust\bevyout`
- Branch: `WorldTravel`
- HEAD when this document was written: `8a355fc8 Update pre-push`
- Feature implementation commit: `3b815f40 Some cell stuff`
- The worktree was clean before adding this handoff document.
- Bethesda-derived prepared data remains under `.bevyout/` and must never be
  staged or committed.

The feature commit also contains unrelated project/tooling changes inherited
from the branch. Do not rewrite or revert those while finishing this work.

## Implemented Work

### `prepare --all-exteriors`

- Added `PrepareArgs::all_exteriors` in `src/cli.rs`.
- The option conflicts with positional cell selectors, legacy `--cell`,
  `--all`, `--all-interiors`, `--worldspace`, and `--exterior-radius`.
- It remains compatible with batch controls such as `--list-only`,
  `--check-fingerprints`, `--retry-failed`, and `--jobs`.
- `src/vsa/prepare/selectors.rs` selects every catalog cell with
  `interior == false`, then sorts and deduplicates by FormID.
- `src/vsa/prepare/orchestrator.rs` treats it as a batch selector and clears it
  when cloning per-cell worker arguments.
- Existing targeted, `--all-interiors`, `--worldspace`, `--exterior-radius`,
  and `--all` paths were retained.
- Viewer render-auto-prepare constructs `PrepareArgs` with
  `all_exteriors: false`.

### Selector tests

- `features/prepare_selectors.feature` has a Cucumber scenario proving that
  `--all-exteriors` returns exactly the exterior subset in deterministic,
  deduplicated FormID order.
- Step implementation is in `tests/features.rs`.
- Focused selector tests are in `src/vsa/prepare/tests/selectors.rs`.
- CLI batch-dispatch and conflict tests are in
  `src/vsa/prepare/tests/orchestrator.rs`.

### Runtime defects found during real-data acceptance

Three independent defects were fixed while testing the 49-cell Super-Duper
Mart neighborhood:

1. **Unstaged source NIF leaked into runtime exterior packages.**
   `src/vsa/exterior/package.rs::apply_staged_assets` now clears asset and
   physics paths when a source placement has no staged prepared placement.
   This prevents intentionally skipped editor markers such as
   `MarkerCOCHeading.nif` from reaching the runtime asset loader. A regression
   test is in `src/vsa/exterior/tests/mod.rs`.

2. **Viewer IO task-pool stack overflow while loading 25 packages.**
   `src/viewer/task_pools.rs` installs `ViewerIoTaskPoolPlugin` before
   `DefaultPlugins`, preserving Bevy's normal IO worker-count policy but using
   a 16 MiB stack. Tests are in `src/viewer/tests/task_pools.rs`. The viewer no
   longer requires an external `RUST_MIN_STACK` workaround.

3. **Exterior residency exceeded the 25-cell bound.**
   The startup cell had been permanently pinned, leaving it resident outside
   the new 5x5 window. It now starts resident but evictable; the existing
   collision handoff remains the temporary pin authority. In addition,
   eviction finalization now runs before package completion, and completed
   package tasks do not spawn a root without root-budget capacity. Regression
   tests are in `src/viewer/world/exterior/tests.rs`.

Because the prepared runtime-asset contract changed, the prepare fingerprint
was bumped to:

```text
prepare-pipeline-v10-static-facegen-reconstruction-corpse-nested-actors-actor-animations-weapon-animation-type-ktx2-textures-image-space-layouts-hud-sprites-staged-exterior-assets-only
```

No prepared serialized type changed, so no prepared schema revision was
needed.

## Current Evidence

### Selector gate: proven

The following command was rerun when this document was written:

```powershell
cargo run-dev -- prepare --all-exteriors --list-only
```

Output was saved to:

```text
.bevyout/all-exteriors-list-handoff.log
```

Parsed result:

```text
count: 41989
unique: 41989
sorted: true
first: 00000a96
last: 000cc685
exit: 0
```

Earlier comparison against `--all` and `--all-interiors` also established:

```text
all cells: 42410
interior cells: 421
exterior cells: 41989
exterior == all - interior: true
intersection(exterior, interior): 0
union(exterior, interior): 42410
```

The current `--list-only` output is the authoritative selector artifact.

### Full preparation gate: incomplete

The current fingerprint command was rerun:

```powershell
cargo run-dev -- prepare --all-exteriors --check-fingerprints
```

Output is saved to:

```text
.bevyout/all-exteriors-fingerprints-handoff.log
```

Current state across all 41,989 selected cells:

```text
valid:   49
stale:   915
pending: 41025
failed:  no failure result has been observed, but the full batch is unfinished
total:   41989
```

The command exits 1 because stale fingerprints exist. Its footer is:

```text
fingerprint: 49 cells valid, 915 stale
Error: 915 cell(s) have stale fingerprints
```

Note that the footer does not include pending cells; parse the per-cell lines
as well. Final acceptance requires 41,989 valid, zero stale, zero pending, and
zero failed.

An earlier full batch used:

```powershell
cargo run-dev -- prepare --all-exteriors --jobs 4 --progress off
```

Its filtered progress is in:

```text
.bevyout/all-exteriors-prepare-progress.log
```

It prepared 910 cells in 3,549 seconds with no failure lines, then was stopped
intentionally after the unstaged-NIF defect was found and pipeline v10 made
those outputs stale. The stop appears as exit 255; it was not a preparation
cell failure. Do not delete the cache or use `--force`: ordinary batch prepare
is resumable and should reuse valid conversion artifacts.

The 49-cell patch centered at `00000c49` was subsequently prepared under v10:

```powershell
cargo run-dev -- prepare 00000c49 --exterior-radius 3 --jobs 4
```

That run completed all 49 cells with zero failures in approximately 155
seconds.

### Runtime residency gate: partial proof only

The current 49-cell v10 patch has been launched successfully with:

```powershell
cargo run-dev -- view `
  --manifest .bevyout/cache/scenes/00000c49/scene.ron `
  --agent-bridge --agent-port 15702 --trace-seconds 600
```

Structured bridge evidence after relocating from startup grid `(4,-5)` to
grid `(7,-7)` showed:

```text
resident_cells: 25
peak_resident_cells: 25
resident_budget: 25
failed: 0
stale_completions: 0
invalid_unload_count: 0
collision_tracked: 25
evictions: 23
```

Artifacts:

```text
.bevyout/residency-status-start.json
.bevyout/residency-status-loaded.json
.bevyout/residency-status-moved.json
.bevyout/captures/all-exteriors-start.png
```

The capture was non-black and showed visible Wasteland terrain/objects. The
structured relocation proves the residency fixes but **does not satisfy the
ordinary-movement gate**, because console `player.setpos` was used.

An attempt to drive the game window with OS keyboard input was interrupted by
the user's physical Escape key. Do not count that attempt. At the time of this
handoff there is no accepted three-boundary round trip.

## Verification Already Run

Before the later runtime stability/residency fixes, these full gates were
green:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check-dev
cargo run-dev -- prepare --help
```

That `cargo test` run included 1,842 unit tests and the feature harness with 85
features, 690 scenarios, and 3,387 steps.

After the runtime fixes, the following focused tests were green:

```text
unstaged_source_model_is_not_published_as_a_runtime_exterior_asset
startup_cell_uses_the_same_evictable_residency_policy_as_streamed_cells
package_completion_cannot_spawn_above_the_resident_root_budget
viewer IO task-pool worker-count and stack-size tests
```

Because production code changed after the earlier full gate run, the complete
repository gate suite must be rerun before handoff is considered final.

## Exact Continuation Checklist

### 1. Reconfirm source and focused tests

```powershell
git status --short
cargo test-dev all_exteriors --lib
cargo test-dev unstaged_source_model_is_not_published_as_a_runtime_exterior_asset --lib
cargo test-dev startup_cell_uses_the_same_evictable_residency_policy_as_streamed_cells --lib
cargo test-dev package_completion_cannot_spawn_above_the_resident_root_budget --lib
```

Preserve unrelated worktree changes. This handoff document itself will be the
only new tracked file if nothing else changes.

### 2. Resume full preparation

Stop any viewer first to recover memory, then run:

```powershell
cargo run-dev -- prepare --all-exteriors --jobs 4 --progress off
```

- Do not delete `.bevyout/cache`.
- Do not use `--force`, `--rebuild-assets`, or `--rebuild-shadows` unless a
  specific failure proves one is required.
- Four jobs were chosen because the machine previously had roughly 10 GiB of
  available memory and the viewer itself can consume several GiB.
- Capture the stable completion footer and every failure line.
- If the process stops, rerun the same command; the batch job manifest is
  resumable.
- If a real cell fails, record its exact FormID and full diagnostic. Do not
  remove it from selection.

### 3. Prove all fingerprints current

```powershell
cargo run-dev -- prepare --all-exteriors --check-fingerprints
```

Require:

```text
41989 valid
0 stale
0 pending
0 failed
exit 0
```

Do not rely only on the footer because current reporting can omit pending
cells from its summary.

### 4. Run all repository gates

Follow `AGENTS.md` exactly:

```powershell
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check-dev
cargo run-dev -- prepare --help
```

Also run `git diff --check` and inspect the final diff/commit range.

### 5. Perform the ordinary player-movement round trip

Launch the exact prepared startup manifest with the bridge:

```powershell
$env:RUST_LOG='bevyout=info,warn'
cargo run-dev -- view `
  --manifest .bevyout/cache/scenes/00000c49/scene.ron `
  --agent-bridge --agent-port 15702 --trace-seconds 600
```

Then, in a stable MCP session:

```powershell
$env:BEVYOUT_MCP_ACCESS='runtime_write'
bun tools/bevyout-mcp/src/cli.ts console --line "worldstream trace 1" --session all-exteriors-acceptance
bun tools/bevyout-mcp/src/cli.ts console --line "worldstream status" --session all-exteriors-acceptance
```

Acceptance route:

1. Start at exterior grid `(4,-5)`.
2. Use ordinary FPS keyboard movement—not `player.setpos`, BRP transform
   mutation, `tfc`, or navigation-agent movement—to travel west through grids
   `(3,-5)`, `(2,-5)`, and `(1,-5)`.
3. Return east along the same sequence to `(4,-5)`.
4. Query `worldstream status` at each boundary and after returning.
5. Capture non-black viewport images at the start, far point, and return.

The final trace and every intermediate status must prove:

```text
failed=0
stale_completions=0
invalid_unload_count=0
peak_resident_cells<=25
resident_cells<=25
```

Also confirm visible terrain/objects throughout. If physical keyboard control
is unavailable, stop and report that the ordinary-movement proof is missing;
do not substitute console teleportation.

## Useful Diagnostics and Caveats

- Use `cargo run-dev`; launching `target/debug/bevyout.exe` directly can fail
  to locate Bevy's dynamic-linking DLL.
- The viewer should now run without setting `RUST_MIN_STACK`.
- `worldstream cells` explains exactly which grids own the residency set;
  `worldstream status` provides the acceptance counters.
- A healthy startup at `(4,-5)` settles at one `Resident` cell plus 24 `Ready`
  cells, for 25 total.
- The old defect appeared as 26 steady-state cells because startup `(4,-5)`
  survived outside the new window. The later race appeared as a transient
  peak of 34. Both are regression-tested and the current relocation proof
  peaked at 25.
- Expected BoxDDD warnings about rejected coplanar convex hulls are not by
  themselves stream failures. Judge the stable counters and cell-specific
  preparation diagnostics.
- Prepared artifacts and logs under `.bevyout/` are local evidence only. Never
  commit them.

## Completion Audit

Do not claim completion until all rows are proven:

| Requirement | Current state | Required final evidence |
| --- | --- | --- |
| Explicit `--all-exteriors` selector | Implemented | CLI/tests remain green |
| Exact exterior catalog set | Proven: 41,989 unique sorted FormIDs | Preserve list artifact |
| Existing selector behavior preserved | Implemented and previously tested | Rerun full tests |
| All exteriors current | Incomplete: 49 valid, 915 stale, 41,025 pending | 41,989 valid, zero stale/pending/failed |
| Three adjacent boundaries west | Not proven with ordinary input | Boundary-by-boundary statuses |
| Same-route return east | Not proven | Returned `(4,-5)` status |
| Visible outdoor content | Proven only for current 49-cell patch | Non-black start/far/return captures |
| Runtime counters clean | Proven for structured relocation only | Clean counters throughout ordinary route |
| Residency bound | Current relocation proof peaks at 25 | Ordinary route peak never above 25 |
| Final code gates | Earlier run predates final runtime edits | Fresh full gate outputs |

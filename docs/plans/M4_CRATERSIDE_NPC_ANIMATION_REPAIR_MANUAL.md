# M4 Craterside NPC animation repair — manual acceptance

## What this wave ships

This wave repairs the animationless Craterside cache instead of silently
launching static NPCs. Native actor clip packs are threaded through `render`,
real Craterside package records and package points are prepared, authored IDLE
records are retained in the existing actor-animation catalog, and runtime idle
selection/playback is visible through `actorinspect` and `playidle`.

## 1. One-time preparation

Run from the repository root with Fallout 3 data configured. Derived assets
remain under `.bevyout/` and must not be committed.

```powershell
cargo run-dev -- prepare MegatonCratersideSupply --actor-animation-converter native
```

The selector must resolve to:

```text
00003a2a MegatonCratersideSupply
```

The completion summary must show actor mappings and a native clip pack with
Ready clips. The humanoid catalog must no longer report
`conversion_not_requested`, and the manifest/catalog revisions must be current.

## 2. Existing-cache repair and viewer launch

Run the exact user-facing command:

```powershell
cargo run-dev -- render MegatonCratersideSupply
```

If the old compatible cache is animationless, accept the native actor-animation
repair prompt. Accept a bake prompt only when the existing bake is stale or
missing. The viewer must reach the scene without a repeated repair prompt.

A launch through the bridge cannot prompt. For a cache needing repair, verify
its error names this exact command:

```text
cargo run-dev -- prepare MegatonCratersideSupply --actor-animation-converter native --force
```

## 3. Bridge inspection

Launch a bounded bridge session after repair:

```powershell
cargo run-dev -- render MegatonCratersideSupply --agent-bridge --agent-port 15702 --trace-seconds 120
```

Run these commands through the local bridge/Bevyout MCP:

```text
actorinspect 0001ff18
actorinspect 0002d2bc
showpackages 0001ff18 4
showpackages 0001ff18 6
showpackages 0001ff18 12
showpackages 0002d2bc 12
runpackage 0001ff18 status
runpackage 0002d2bc status
```

Expected:

- Mercenary `0001ff18` reports bound animation targets and a selected base
  clip; Moira `0002d2bc` does likewise.
- At hour 04:00 Mercenary selects Eat `0001ff1e`; at 06:00 it selects Sleep
  `0001ff1f`; at noon it selects Guard/Travel `0001ff20`.
- The package catalog does not report `0001ff1e` or `0001ff1f` as missing.
- At noon Moira selects service package `00004153` and resolves marker
  `00076f52` (`MoiraServiceMarker`), not the missing default point
  `00003fdf`.
- While navigation translation is sustained, `actorinspect`/logs show Walk or
  Run. After arrival and dwell they return to Idle. Capture a scene snapshot
  and an unobscured viewport when possible.

## 4. Authored special idle

Force the prepared Moira leaf `ScratchingSelf02`:

```text
playidle 0002d2bc 00067941
actorinspect 0002d2bc
```

The command must return structured success with source `forced`, and the actor
must visibly play the prepared Special Idle. Start/provoke navigation and
confirm the special stops immediately and locomotion resumes.

Automatic global idle selection remains Fallout-condition driven. A concrete
condition rejection or a normal base Idle is a pass when the actor is not
eligible; inventing a personality/fidget pool is a failure.

## 5. Ready-cache and explicit opt-out regression

Rerun the normal command:

```powershell
cargo run-dev -- render MegatonCratersideSupply
```

A ready cache must not trigger actor-animation repair or unnecessary bake work.
The explicit diagnostic opt-out must be accepted without a repair loop:

```powershell
cargo run-dev -- render MegatonCratersideSupply --actor-animation-converter disabled
```

It may show static actors, but must state that conversion was intentionally
disabled.

## 6. Repository gates

Run before opening the integration PR:

```powershell
cargo fmt --check
cargo check-dev
cargo test -p bevyout-core
cargo test --test features
cargo test --test architecture
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Record exact test/scenario counts, skipped-step count, catalog mapping/Ready
clip counts, package selections, actorinspect evidence, bake reuse/rebuild
reason, and which behavior was structural versus visibly verified in the
wave plan and issue comments.

## Acceptance evidence

To be filled after the real-data run: cache repair output, native pack/hash
summary, bridge observations, screenshots/snapshots/log excerpts, and final
gate counts.

# bevyout — Pi Context

Work in `C:\Users\V\Projects\Rust\bevyout`, an offline recreation of Fallout 3
built with Rust 2024 and Bevy 0.19. The default branch is `master`; the remote
is GitHub. Never commit, push, open a PR, post externally, or alter unrelated
git state unless the user explicitly asks.

## Response style

CAVEMAN MODE ACTIVE. Rules: Drop articles/filler/pleasantries/hedging. Fragments
OK. Short synonyms. Pattern: [thing] [action] [reason]. [next step]. Not: "Sure!
I would be happy to help you with that." Yes: "Bug in auth middleware. Fix:".
Code/commits/security: write normal. User says "stop caveman" or "normal mode"
to deactivate.

## Workflow

- If `.codegraph/` exists, use `codegraph explore "<question>"` before grep or
  broad file reads. Keep fallback searches scoped and output bounded.
- Read local Bevy references in `BevyCheatSheet/`, `BevyDocs/`, and
  `bevy_markdown_docs/` before using generic or version-mismatched examples.
- Keep `src/main.rs` a dispatcher and `src/cli.rs` the clap boundary.
- Keep pure contracts and policy in `crates/bevyout-core`; it may depend only
  on `std`, `serde`, and `glam`, never Bevy.
- Keep Fallout-cell internals in `src/vsa/` and Bevy presentation in the
  viewer boundary. Prefer narrow plugins and pure policy modules.
- Use feature-first order: fix the feature list, write Cucumber and unit tests,
  then implement. Every Cucumber scenario must have non-skipped steps.
- Use dynamic-linking aliases while iterating: `cargo check-dev`,
  `cargo test-dev`, and `cargo run-dev -- ...`.
- Before handoff run `cargo fmt --check`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test`, and a
  representative `cargo run-dev` command when the touched path permits it.
- Use the bevyout MCP/BRP agent bridge for live viewer inspection,
  screenshots, console commands, bounded profiling, and runtime acceptance.
- Check the diff and report exact validation. Separate static/local proof from
  anything not verified with real prepared data or a live viewer.

## Project Rules

- Preserve Vertical Slice Architecture and the explicit viewer schedule:
  `Input -> Interaction -> WorldSync -> Ui`.
- Bump the relevant `*_REVISION` whenever a prepared serialized type changes,
  including new serde-defaulted fields.
- Runtime item movement goes through `bevyout-core`'s canonical `ItemLedger`;
  inventories, containers, and world drops are projections.
- CLI diagnostics use deterministic `println!` text. Viewer diagnostics use
  stable, grep-able `tracing` events, never `println!`.
- Prepared point shadows are generated during `prepare`. Do not move them into
  Blender or a per-frame runtime cubemap path. Realtime shadows remain explicit
  opt-in and independent of prepared shadows.
- Record-level container audio is authoritative; model cues may only fill
  missing open/close fields through the existing prepared-audio path.
- Put OpenMW-derived Rust code only in an isolated, attributed provenance
  folder.

## Landmines

- Never commit Bethesda-derived RON, GLB, DDS, WAV, NIF, or other game data.
  Derived content belongs under ignored `.bevyout/`; fixtures must be synthetic.
- Never use `git add -A` or `git add .` at the repository root. Stage explicit
  paths so worktrees, caches, and scratch files are not swept in.
- Prepared caches can parse successfully while silently missing new defaulted
  fields if their revision was not bumped.
- Viewers consume prepared manifests and do not regenerate shadow artifacts.
- WebGPU cannot upload CPU bytes directly to `Depth32Float`; the local
  `bevy_pbr` patch uses an `R32Float` staging texture and one GPU upload pass.
- Live capture can return black when the window is occluded, and frame timings
  are meaningful only on a cool machine. Treat runtime artifacts and logs as
  the acceptance evidence.
- On Windows, stale viewer or Cargo processes can retain file locks during
  final build gates.

For multi-issue milestone work, follow the wave, issue, model-routing, manual
acceptance, and PR conventions in `AGENTS.md` and `docs/plans/README.md`.
Load detailed guidance only when relevant from `.agents/skills/*/SKILL.md`,
the local Bevy docs, and the project wiki.

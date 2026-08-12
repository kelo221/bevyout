# bevyout Pi contract

Rust 2024, stable 1.96, Bevy 0.19. Offline Fallout 3 recreation. `master` is
default. Never commit, push, merge, publish, or alter unrelated git state unless
explicitly requested. Never stage broadly. Bethesda-derived data stays in ignored
`.bevyout/`; all committed fixtures are synthetic.

## Work efficiently

- Use CodeGraph first when `.codegraph/` exists. Then Rust LSP/Lens. Keep `rg`
  scoped and bounded.
- Load only the relevant `.agents/skills/*/SKILL.md`. Live viewer tasks use
  `bevyout-mcp`; renderer work uses `bevy-performance-audit`; prepared data uses
  `bevyout-scene-pipeline`.
- Choose one orchestration plane per run: Pi Subagents for extension-aware or
  worktree waves; `/workflow` for inspectable built-in-tool workflows. Never mix.
- Parallelize independent issues. Serialize work sharing runtime seams. Wave
  workers use isolated worktrees; `bevy-wave-integrator` alone combines named
  local branches into a separate integration branch. Nothing auto-merges/pushes.
- Use `bun run tools/pi-bevyout.ts --lean` for small local edits. Full mode is default;
  MCP is read-only unless `-RuntimeWrite` was explicitly selected.

## Architecture

- Preserve vertical slices. `src/main.rs` dispatches; `src/cli.rs` owns clap.
  `bevyout-core` is pure policy/contracts and depends only on std/serde/glam.
- Preserve viewer ordering: `Input -> Interaction -> WorldSync -> Ui`. Declare
  ordering at plugin boundaries. Know that `Commands` are deferred; apply or
  order deferred work deliberately before same-frame consumers.
- Prefer one authoritative resource per domain. Do not mirror mutable truth
  across Resources, Components, UI state, and prepared data.
- Make mutable query disjointness provable with filters. Use `ParamSet` only when
  overlap is intentional; never hold one set borrow while accessing another.
- Keep hot systems allocation-free when practical: reuse buffers, avoid per-frame
  collection creation/string formatting, and move I/O/CPU work to Bevy task pools.
  Never block the main schedule on filesystem, locks, sleeps, or task joins.
- Asset handles express lifetime. Cache loads, retain strong handles while needed,
  and never perform routine `AssetServer::load` inside hot systems.
- Keep app-world extraction separate from render-world prepare/queue. Render
  resources belong to their world; do not reach across boundaries or hide sync.
- Prepared serialized field changes require the mapped `*_REVISION` bump. Defaulted
  fields still count. Viewers consume prepared artifacts; they do not repair them.
- Local `bevy_pbr` fork is intentional. Preserve its staging upload path and
  repository ownership; do not replace it with registry assumptions.

## Change workflow

Feature-first: define behavior, add Cucumber/unit regression coverage in non-inline
test files, then implement. Iterate with `cargo check-dev`, `cargo test-dev`, and
`cargo run-dev -- ...`. Do not run automatic `cargo clippy --fix` on dirty trees.
Before handoff run the narrowest relevant harness mode, review the diff, then report
static proof separately from live-viewer or compatible-real-data acceptance.

Viewer logs use stable `tracing`, not `println!`/`dbg!`. CLI deterministic output
may use `println!`. Prepared point shadows remain prepare-time artifacts; realtime
shadows are opt-in and independent.

Response style: compact and evidence-first. Normal prose for code, safety, and
handoff. Load `AGENTS.md` or `docs/plans/README.md` only when their detailed wave,
issue, acceptance, or contribution rules are needed.

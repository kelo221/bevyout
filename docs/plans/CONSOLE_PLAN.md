# CONSOLE_PLAN — Gamebryo console and agent I/O framework

Goal: make the running game observable and scriptable through **one command core with
three thin frontends** — the in-game tilde console (humans), BRP/MCP (agents), and a
script runner + Rust harness (regression). Every command returns structured JSON;
scripts produce JSONL transcripts that are **byte-diffed against goldens**, extending
the existing determinism house pattern (M1 F37.3, F39.8). A repro found live pastes
1:1 into a committed test. Target outcome: "manual verification for Simon" shrinks to
reviewing transcript diffs plus a short real-data spot check.

## Decisions (brainstorm, 2026-07-13)

- **Grammar: Gamebryo-faithful.** `[reference.]command [args...]` per the
  [Gamebryo console command set](https://fallout.fandom.com/wiki/Gamebryo_console_commands).
  LLMs know `coc`/`prid`/`tcl`/`player.moveto` from training data — faithful syntax is
  zero-shot agent usability, and the wiki doubles as documentation.
- **Execution model: commands + in-process Rust harness.** No embedded Rhai/Lua.
  Arbitrary logic belongs in `#[test]` code via the harness. The parser stays a
  separable unit so console lines can later become fragments of a real ObScript
  interpreter (in the original engine the console *is* the script engine's REPL —
  this layer is a down payment on that).
- **I/O contract first.** Frontends never scrape strings; they consume
  `ConsoleOutput` JSON.

## Architecture

- New top-level VSA slice **`src/console/`**: grammar, registry, execution, output
  types, engine commands. No rendering dependency — the core must compile and run
  under `MinimalPlugins`.
- Frontends stay thin and live at their boundaries: tilde UI in `src/viewer/`,
  BRP custom method registration in app setup, script runner in `src/cli.rs` dispatch.
- Execution: parsed commands are queued, then drained by **one exclusive system**
  (`&mut World`) at a fixed schedule point — deterministic ordering, one place where
  mutation happens.
- Output type (serde, stable field order):

```rust
ConsoleOutput {
    ok: bool,
    value: serde_json::Value,   // machine-shaped, e.g. getpos -> {x, y, z}
    log: Vec<String>,           // human-readable lines for the UI
    frame: u64,                 // sim frame the command executed on
    error: Option<ConsoleError { code: String, message: String }>,
}
```

## Grammar

- `[reference.]command [args...]`; command names case-insensitive.
- References: `player`, an eight-digit hex FormID, or the current `prid` selection.
- Arguments: numbers, quoted strings, EditorIDs, FormIDs.
- `;` starts a comment; blank lines ignored.
- A **`.bscript` file is nothing but a sequence of console lines** — timing is
  expressed with the `advance N` command, assertions with `expect`. No second syntax.

## Issue overview

| Issue | Branch | Slice touched | Conflict surface | Blockers |
|-------|--------|---------------|------------------|----------|
| C1 BRP enablement | `codex/console-c1-brp` | `Cargo.toml`, `src/viewer/app.rs` | tiny | none |
| C2 Console core + engine commands | `codex/console-c2-core` | new `src/console/` | `lib.rs`, `viewer/app.rs` (plugin add) | none |
| C3 Deterministic sim clock + seeded RNG | `codex/console-c3-determinism` | new `src/sim_time/` | `viewer/app.rs` | none |
| C4 Game commands via ContentIndex | `codex/console-c4-game-commands` | `src/console/commands/` | `console/`, `app_state/` | #39, C2 |
| C5 Script runner, transcripts, harness | `codex/console-c5-scripts` | new `src/console/script/`, `tests/` | `cli.rs`, `main.rs` | C2, C3 |
| C6 BRP console method + agent workflow | `codex/console-c6-remote` | app setup, `AGENTS.md` | `viewer/app.rs` | C1, C2 |
| C7 Tilde console UI | `codex/console-c7-ui` | `src/viewer/console_ui/` | `viewer/`, `app_state/` | C2, #35 |
| C8 Screenshot + perf lanes | — | — | — | deferred |
| C9 GECK spec extraction + parity | `codex/console-c9-geck-spec` | new `src/console/spec/` | `console/` | C2 |

**Ordering rule inside every issue (unchanged from M1): feature list is fixed →
tests are written first → implementation makes them green.**

---

## Issue C1 — BRP remote access enablement

Bevy 0.19 ships `bevy_remote` (BRP): JSON-RPC 2.0 over HTTP with `world.query`,
component get/insert/mutate, `+watch` streams, `rpc.discover`, and custom method
registration. `bevy_brp_mcp` v0.19 exposes it as MCP tools for agents;
`bevy_brp_extras` adds screenshots and synthetic input. C1 turns this on — agents can
inspect and mutate the live game before any console code exists.

### Feature list

- **FC1.1** Cargo feature `remote` enabling `bevy/bevy_remote`; added to the
  `run-dev`/`check-dev`/`test-dev` aliases in `.cargo/config.toml`. Release builds do
  not include it by default.
- **FC1.2** `RemotePlugin` + `RemoteHttpPlugin` added in viewer app setup when the
  feature is on. Bind `127.0.0.1` only. CLI: `--remote-port <u16>` (default 15702),
  `--no-remote`.
- **FC1.3** AGENTS.md section: hitting BRP with curl (`rpc.discover`, `world.query`,
  `registry.schema` for type discovery, `world.trigger_event` — e.g. firing #35's
  `RequestStateTransition` remotely), installing `bevy_brp_mcp` + `bevy_brp_extras`
  for MCP-capable agents, version pins. Reference: bevy 0.19 `bevy::remote` docs;
  requests are processed in the `RemoteLast` schedule (fixed point per frame).
- **FC1.4** Security stance documented: localhost only, dev feature, never shipped
  enabled in release unless deliberate.

### Test list

- **TC1.1** Headless `App` with the remote plugins on an ephemeral port answers
  `rpc.discover` over a raw `std::net::TcpStream` HTTP request.
- **TC1.2** `world.query` over HTTP returns a component value from a test entity.
- **TC1.3** CLI contract (extend `tests/cli_contract.rs`): `--remote-port` and
  `--no-remote` parse; `--remote-port 0` rejected.

### Implementation steps

1. Feature + alias wiring; plugin registration behind `#[cfg(feature = "remote")]`.
2. TC1.1–TC1.3 first (they define the HTTP smoke path), then make green.
3. AGENTS.md workflow section.
4. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.

---

## Issue C2 — Console core slice and engine commands

### Feature list

- **FC2.1** Slice layout `src/console/`: `grammar.rs`, `registry.rs`, `exec.rs`,
  `output.rs`, `commands/engine.rs`, `plugin.rs`. Registered in `lib.rs`;
  `ConsolePlugin` added by the viewer. Core compiles under `MinimalPlugins`.
- **FC2.2** Grammar per spec above, parsed into
  `ParsedCommand { reference, name, args }`. All failures are structured errors
  (unknown command, bad arity, bad type, unresolvable reference) — never a panic,
  never a partial mutation.
- **FC2.3** Registry: `name -> { handler, signature, help, mutating: bool }`.
  Handlers are `fn(&mut World, &Invocation) -> ConsoleOutput`. Public registration
  API so any future slice adds its own commands without touching console internals
  (mirrors how every Gamebryo subsystem contributes script functions).
- **FC2.4** Command queue resource + exclusive drain system at a fixed point in the
  schedule; submission order preserved; results correlated by submission id.
- **FC2.5** `RefRegistry`: FormID → `Entity`, populated at scene spawn from manifest
  placements. `prid <formid>` selects a reference; `player` resolves to the active
  controller entity.
- **FC2.6** Engine command set v1 (Gamebryo names where they exist):
  `help [command]`, `prid`, `getpos [x|y|z]`, `setpos <axis> <v>`,
  `getangle`/`setangle`, `moveto <ref>`, `tcl` (capsule controller collision),
  `tm` (hide all UI/diagnostics — clean screenshots), `tdt` (existing diagnostics
  overlay), `sgtm <mult>` (`Time<Virtual>` relative speed), `dump [ref]`
  (components + values via reflection), `screenshot <path>` (windowed only in C2).
- **FC2.7** Every command returns `ConsoleOutput`; `value` is machine-shaped;
  `help` output is generated from the registry (self-documenting for agents).
- **FC2.8** Positions in command I/O are Bevy metres (the manifest contract);
  document the Fallout-units difference in `help getpos`.

### Test list

- **TC2.1** Grammar table test: `player.moveto ff0000a1`, `prid 0001a2b3`,
  `setpos z 1.5`, quoted strings, case-insensitivity, comments, blank lines.
- **TC2.2** Unknown command / bad arity / bad reference → `ok = false`, structured
  error code, world observably unchanged.
- **TC2.3** A test-only slice registers a custom command through the public API and
  invokes it.
- **TC2.4** `prid` + `player` targeting resolve against a fake `RefRegistry` on a
  headless world.
- **TC2.5** `getpos`/`setpos`/`moveto` round-trip on spawned `Transform`s.
- **TC2.6** `tcl`, `tm`, `tdt` flip their state resources; `sgtm 2` doubles virtual
  time speed.
- **TC2.7** Determinism: the same line sequence on two fresh headless apps yields
  byte-identical `ConsoleOutput` JSON.
- **TC2.8** Two commands queued in one frame execute in submission order.

### Implementation steps

1. Output + grammar types; TC2.1–TC2.2 first.
2. Registry + queue + exclusive executor; TC2.3, TC2.8.
3. `RefRegistry` + targeting; TC2.4.
4. Engine commands; TC2.5–TC2.7.
5. Viewer wires `ConsolePlugin`; fmt/clippy/test.

---

## Issue C3 — Deterministic sim clock and seeded RNG

Pulled forward from the M1 epic's "deterministic time/RNG" checklist item; scripts
and transcripts are only regression-solid on top of this.

### Feature list

- **FC3.1** `SimRng` resource: seeded deterministic RNG; seed logged at startup;
  console command `seed <n>` (registered via the C2 public API) resets it.
- **FC3.2** Frame-stepping API for headless runs: advance the app exactly N updates
  with a fixed virtual dt. Console command `advance <n>`; in windowed mode it
  pauses and steps.
- **FC3.3** Audit viewer systems for wall-clock and ad-hoc randomness (footstep
  variation is the likely offender); route through `SimRng` and `Time<Virtual>`.
- **FC3.4** Monotonic sim frame counter resource; the executor stamps it into
  `ConsoleOutput.frame`.

### Test list

- **TC3.1** Two headless apps, same seed, same command sequence → identical outputs.
- **TC3.2** `advance 7` advances the frame counter by exactly 7; systems gated on
  `Update` ran exactly 7 times (probe counter).
- **TC3.3** Different seeds diverge (guards against a silently unused seed).
- **TC3.4** A wall-clock probe proves gameplay systems don't observe `Instant::now`
  drift under fixed stepping (tolerance-free comparison of two runs).

### Implementation steps

1. TC3.1–TC3.4 with probe systems first.
2. `SimRng` + frame counter + stepping API; `seed`/`advance` commands.
3. Viewer audit + rewiring; fmt/clippy/test.

---

## Issue C4 — Game commands via ContentIndex (after #39 and C2)

### Feature list

- **FC4.1** EditorID accepted everywhere a reference/FormID is accepted, resolved
  through `ContentIndex::by_editor_id` → `RefRegistry`.
- **FC4.2** `coc <EditorID|FormID>`: sets `LoadingTarget` and requests the
  InGame→Loading transition through #35's `RequestStateTransition` — no bespoke
  loading path. Works from any InGame state; rejected elsewhere with a structured
  error.
- **FC4.3** `prid <EditorID>`; `getformid`/`getedid` on the selected reference.
- **FC4.4** Deferred, documented in help as such: `additem`/`removeitem` (needs #38
  inventory classes), `placeatme`, `disable`/`enable` (needs #38 enable-state
  mutability), `getav`/`setav` (needs actor values to exist).

### Test list

- **TC4.1** Synthetic two-cell fixture: `coc CellB` transitions Loading→InGame and
  spawns CellB's placements exactly once (reuses T35.6 machinery).
- **TC4.2** EditorID→entity resolution on a synthetic index; unknown EditorID →
  structured error, no state change.
- **TC4.3** `coc` from a modal state is rejected per the #35 legal-transition table.
- **TC4.4** Transcript determinism across `coc` (two runs byte-identical).

---

## Issue C5 — Script runner, transcripts, goldens, Rust harness

### Feature list

- **FC5.1** `bevyout script run <file.bscript> [--headless] [--transcript <out.jsonl>]`
  CLI subcommand; `main.rs` gains only a dispatch arm.
- **FC5.2** Transcript: one `ConsoleOutput` JSON object per line, byte-stable, each
  record extended with `{ line_no, input }`. Two runs on the same seed are
  byte-identical.
- **FC5.3** `expect` command: `expect (<value-returning line>) <op> <literal> [tol <t>]`,
  e.g. `expect (player.getpos z) > 0 tol 0.001`. Ops: `== != < > <= >=`. Failure is
  recorded in the transcript and sets a nonzero process exit; default stop-on-fail,
  `--keep-going` to continue.
- **FC5.4** `ConsoleHarness` Rust API for `#[test]`: builds the headless app
  (MinimalPlugins + selected plugins + synthetic fixtures), `exec(&str) -> ConsoleOutput`,
  `advance(n)`, `run_script(path) -> Transcript`. Same executor codepath as the CLI
  runner — parity is a test, not a promise.
- **FC5.5** Golden suite: `tests/console_scripts/*.bscript` + `tests/goldens/*.jsonl`,
  synthetic fixtures only (licensed-data hygiene). Byte-diff assert;
  `UPDATE_GOLDENS=1` regenerates.
- **FC5.6** Real-esm scripts live under `.bevyout/scripts/` (gitignored) and replace
  the standing "manual verification for Simon" checklist where possible.

### Test list

- **TC5.1** Runner exit codes: clean script 0, failed expect nonzero, parse error
  nonzero with the offending line number.
- **TC5.2** Transcript byte-determinism (run twice, compare bytes).
- **TC5.3** `expect` pass and fail paths, including tolerance edges.
- **TC5.4** Harness/runner parity: same script through both → identical transcripts.
- **TC5.5** Golden regen flag rewrites goldens; without it, a mismatch fails.
- **TC5.6** CLI contract: `script run` parses; `--transcript` path respected.

---

## Issue C6 — BRP console method and agent workflow

### Feature list

- **FC6.1** BRP custom method `bevyout/console.exec { line } -> ConsoleOutput`
  registered on `RemotePlugin` init; `bevyout/console.help` returns the registry
  dump. Wire-shape equals local shape — one contract.
- **FC6.2** Agents get the full loop documented in AGENTS.md: launch headless or
  windowed → connect (curl or `bevy_brp_mcp`) → poke via `console.exec` and BRP
  built-ins → capture the repro as a `.bscript` → commit script + golden.
  **House rule: a bug report is a script.**
- **FC6.3** `rpc.discover` reflects the custom methods.

### Test list

- **TC6.1** HTTP round trip: `console.exec` with `getpos` on a test entity returns
  the same JSON as local execution (byte compare).
- **TC6.2** Error shape parity over the wire (unknown command).
- **TC6.3** `rpc.discover` lists `bevyout/console.exec`.

---

## Issue C7 — Tilde console UI (after C2 and #35)

### Feature list

- **FC7.1** `~` opens the console as a new `GameplayModal::Console` variant —
  mutually exclusive with Paused/Dialogue/PipBoy by construction, pauses
  `Time<Virtual>` like the original engine, releases the cursor.
- **FC7.2** Monospace `bevy_ui` overlay: input line + scrollback rendering
  `ConsoleOutput.log` (and pretty-printed `value` on demand).
- **FC7.3** History (up/down), `Tab` autocomplete from the registry, plus EditorID
  completion once C4 is merged. (`Tab` is currently the controller toggle — move
  that binding to `tfc`-adjacent behavior or another key; decide in-issue.)
- **FC7.4** UI logic (history buffer, completion candidates, line editing) lives in
  pure structs, unit-testable without rendering.

### Test list

- **TC7.1** Modal gating: gameplay input systems do not tick while the console is
  open (reuses T35.3-style probes); virtual time paused and resumed.
- **TC7.2** History and completion behavior as pure-struct unit tests.
- **TC7.3** Open→exec→close round trip on a headless app with UI logic stubbed.

---

## Issue C8 — Deferred lanes (file when needed)

- **Screenshot goldens:** `screenshot` in headless via render-to-texture; few,
  tolerance-based perceptual diffs (dssim), lavapipe/llvmpipe GPU-less CI. Keep the
  suite small — transcripts are the primary gate, pixels the smoke check.
- **Perf budgets:** `perf` command surfacing the existing `render_timings.csv`
  diagnostics; budget assertions as `expect` lines.
- **Watch streams:** BRP `+watch` methods for agents observing change-over-time.
- **Input record/replay:** synthetic input capture for controller regression
  (complements, not replaces, command scripts).

---

## Issue C9 — GECK spec extraction and console parity (after C2)

`GECK-Notes/` (gitignored local scrape of the official GECK wiki, 6202 pages with
`page_id`/`revision_id` provenance) documents every Gamebryo function in a regular,
parseable convention: `_[Object]._SetAngle Axis:char{X, Y, Z} Angle:float`, short
aliases (`ToggleCollision` / `TCL`), console-only markers, and data tables like
`AV-Codes`. This issue turns faithfulness into a build artifact instead of a review
item.

### Feature list

- **FC9.1** Extractor parsing frontmatter + Syntax sections into
  `FunctionSpec { name, alias, ref_callable, args: [{name, type, enum_values, optional}],
  console_only, source_game, page_id }`. Invoked via a dev subcommand
  (`bevyout geck-spec extract --geck-notes <dir>`), not `build.rs`.
- **FC9.2** Scope filter: only pages marked as base **Fallout 3** enter the spec
  (~723); FNV/NVSE/JIP pages are excluded but counted in the extraction summary.
- **FC9.3** Committed `console_spec.ron`: extracted **facts only** (signatures,
  aliases, flags, page_id) — no wiki prose. `GECK-Notes/` itself stays gitignored;
  regeneration is deterministic (byte-identical for the same input set).
- **FC9.4** Parity test: every registered command whose name matches a spec entry
  must agree on alias, arity, argument types/enums, and ref-callability. Deliberate
  deviations (e.g. `getpos` returns Bevy metres) are declared in the registry with a
  reason string; undeclared deviation fails the test.
- **FC9.5** Console coverage report (the #37 pattern applied to functions):
  `Implemented` / `Partial` / `Unimplemented` / `OutOfScope` counts, deterministic
  and diffable, emitted by `bevyout geck-spec coverage`.
- **FC9.6** `help <cmd> --full` reads the Description prose live from local
  `GECK-Notes/` when the directory exists (dev machines); otherwise falls back to
  the terse, original-wording committed help.

### Test list

- **TC9.1** Extractor on **synthetic fixture pages** (mini pages mimicking the wiki
  format — never committed real wiki content) produces the expected `FunctionSpec`s.
- **TC9.2** Signature parsing details: enum args (`{X, Y, Z}`), optional args,
  ref-callability from the leading `_[Object]._`, alias capture from "Or:" blocks.
- **TC9.3** FO3 vs FNV/NVSE filtering on fixtures; excluded pages counted.
- **TC9.4** Parity test fails on an intentionally wrong test-registry signature and
  on an undeclared deviation; passes with a declared one.
- **TC9.5** Spec and coverage output determinism (generate twice, byte-compare).

### Implementation steps

1. Synthetic fixtures + TC9.1–TC9.3 first; extractor until green.
2. Commit generated `console_spec.ron`; wire the parity test into `cargo test`
   (skips gracefully if a command has no spec entry — new bevyout-only commands are
   legal and simply unlisted).
3. Coverage subcommand + `help --full` lookup; fmt/clippy/test.

---

## Swarm execution plan

**Wave 1 — 3 agents in parallel** (C1, C2, C3), isolated worktrees branched from
current `master`, branch names per the table. Each agent:

1. Reads `AGENTS.md` and its issue section here
   (`/Users/simon/projects/bevyout/CONSOLE_PLAN.md`).
2. Writes its test list first, then implements until green.
3. Sets `CARGO_TARGET_DIR=/Users/simon/projects/bevyout/target` (shared dep cache,
   as in M1).
4. Finishes with `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`, commits on its branch. No pushes.

**Merge order wave 1: C2 → C3 → C1** (C2 owns the new slice; C3's `viewer/app.rs`
and command registrations rebase cheaply on it; C1 is near-conflict-free).
Re-run the full check suite after each merge.

**Wave 2 — after wave 1 merges and #39 lands:** C4, C5, C6, C9 in parallel.
Merge order **C5 → C4 → C6 → C9** (C5 defines the transcript contract others' tests
lean on; C6 only wraps; C9 touches registry metadata and merges cheapest last).

**Wave 3:** C7 (needs #35 merged for the modal). C8 filed as follow-up issues only.

**Manual verification for Simon (licensed data, real window):**

- `cargo run-dev -- render SuperDuperMart`, open `~`: `prid` a shelf item,
  `moveto player`, `tcl`, `sgtm 4`, `coc` to a second prepared cell, `tm` +
  `screenshot`.
- Run one real-esm `.bscript` from `.bevyout/scripts/` and skim the transcript.
- Point an MCP-capable agent at the running game and watch it execute
  `bevyout/console.exec` end-to-end.
- Confirm nothing Bethesda-derived lands outside `.bevyout/`.

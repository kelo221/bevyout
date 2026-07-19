---
name: bevyout-scene-pipeline
description: Operate and troubleshoot bevyout's Fallout scene lifecycle from prepare through bake, render, direct view, and live verification. Use when Codex needs to import or refresh cells, inspect stale fingerprints, choose native versus Blender conversion, bake irradiance or a preview, launch a selector or exact manifest, automate a viewer through the agent bridge, diagnose cache/tool failures, or execute real-data acceptance of prepared scenes.
---

# Operate the Bevyout Scene Pipeline

Treat `prepare`, `bake`, `render`, and `view` as separate entry points over one
cache contract. Prefer the explicit stages for reproducible work; use `render`
as an interactive convenience command.

## Establish the contract

1. Read the repository `AGENTS.md` and inspect `git status --short`. Preserve
   unrelated worktree changes.
2. Read [pipeline-contract.md](references/pipeline-contract.md) before running
   or diagnosing a pipeline stage.
3. Read [blender-history.md](references/blender-history.md) when Blender,
   converter parity, old documentation, or legacy cleanup is involved.
4. Treat `src/cli.rs`, `src/converter_policy.rs`, `src/vsa/prepare/`,
   `src/vsa/bake/`, and `src/viewer/` as the current source of truth. Recheck
   them when the requested work follows recent pipeline changes.
5. Keep all extracted or converted game data beneath `.bevyout/`. Never stage,
   commit, attach, or publish Bethesda-derived RON, GLB, DDS, WAV, NIF, or other
   cache artifacts.

## Choose the entry point

| Need | Command | Contract |
| --- | --- | --- |
| Create or refresh prepared assets | `prepare <selector>` | Parse content, convert assets, generate prepared metadata and point-shadow caches, then write `scene.ron` |
| Compose static geometry and irradiance | `bake <selector>` | Consume a prepared manifest, run the Rust CPU bake, publish baked GLB/KTX2, and update the manifest |
| Produce a quick lighting image | `bake <selector> --quality preview` | Run the legacy Blender Eevee preview and leave bake metadata unchanged |
| Open by EditorID/FormID with recovery prompts | `render <selector>` | Resolve the cache, optionally offer prepare/bake, then enter the viewer |
| Open one exact artifact | `view --manifest <scene.ron>` | Bypass selector lookup and interactive recovery; validate and load that manifest |
| Inspect or drive the live scene | `render ... --agent-bridge` or `view ... --agent-bridge` | Expose the loopback bridge; then use the `bevyout-mcp` skill |

## Run a reproducible cell flow

Use dynamic linking for iterative desktop work:

```powershell
cargo run-dev -- prepare <selector> --converter native
cargo run-dev -- bake <selector>
cargo run-dev -- render <selector>
```

Omit `--converter native` for ordinary use because native is the default. Keep
it explicit in acceptance scripts when converter identity is part of the
evidence.

For exact-manifest or prepared-only inspection, launch directly:

```powershell
cargo run-dev -- view --manifest .bevyout/cache/scenes/<formid>/scene.ron
```

For deterministic bridge automation, prepare and bake explicitly before
`render --agent-bridge`; that command intentionally refuses interactive cache
recovery. Use direct `view --agent-bridge` when inspecting a compatible
prepared manifest without requiring an irradiance bake.

## Diagnose before rebuilding

Start with bounded, read-only checks:

```powershell
cargo run-dev -- prepare <selector> --list-only
cargo run-dev -- prepare <selector> --check-fingerprints
```

Then apply the narrowest recovery:

- Rerun ordinary `prepare` for a stale prepared manifest; revision and cache
  fingerprints should select the required work.
- Use `--rebuild-assets` only to bypass otherwise valid NIF-to-GLB cache hits.
- Use `--rebuild-shadows` only to replace prepared point-shadow cubemaps.
- Use `--force` only when the normal resumable/fingerprint path is not the
  behavior under test.
- Use `--retry-failed` for recorded prepare or bake batch failures.
- Do not start `--all` or `--all-interiors` work unless the request requires a
  batch; these are materially larger operations.

Do not install or invoke Blender to fix a native-default failure. First
classify whether the problem belongs to native conversion, Rust baking,
prepared shadows, KTX packaging, or runtime viewing. Use
`--converter blender` only for an explicit compatibility comparison, and use
Blender for baking only with `--quality preview`.

## Verify the result

1. Require the expected stable CLI completion lines and capture cache reuse,
   rebuild, failure, and timing counts when they are acceptance evidence.
2. Launch a bounded viewer with `--trace-seconds <seconds>` when unattended.
3. For live assertions, invoke the repository's `bevyout-mcp` skill, take a
   scene snapshot, run structured console commands, and re-query the state.
4. Treat bridge changes as runtime-only; they never repair `scene.ron` or
   converted assets.
5. If implementation or tests changed, run the repository gates in
   `AGENTS.md`, including the representative `cargo run-dev` command. Do not
   run the full code gate suite for a read-only operational diagnosis unless
   it is needed to answer the request.

Report the selector and resolved FormID, converter backend, cache state,
prepare/bake actions, exact launch command, runtime observations, and any
unverified visual behavior. Separate source-backed facts from live measured
evidence.

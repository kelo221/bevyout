# M7 scripting architecture roadmap

This roadmap keeps M7's script work behind one engine-owned, versioned
boundary:

```text
resolved ESM4 records -> ScriptCatalog -> ScriptProgramV1 -> resumable VM
                                               |
                                               v
                                  ScriptFunctionRegistry
                                               |
                                               v
                              typed host services and commands
```

SCDA is the shipping compatibility input. SCTX remains optional tooling and a
possible bootstrap frontend. Neither Bethesda bytecode, source-parser ASTs, nor
Bevy `World` access may become the runtime API.

## Permanent boundaries

- `bevyout-script` is a pure, Bevy-free domain crate for values, versioned IR,
  immutable programs, mutable instances, events, VM state, diagnostics, and
  snapshots.
- `src/vsa/scripts/` owns ESM4 script records, attachments, SCDA/SCTX
  frontends, prepared artifacts, and corpus reports.
- `src/viewer/scripts/` is a narrow Bevy adapter for bindings, ordered events,
  deterministic scheduling, typed host services, deferred commands, and
  diagnostics.
- Program identity supports both top-level SCPT records and embedded owner/slot
  scripts. Instance identity uses stable Fallout/Bevyout IDs, never Bevy
  `Entity` IDs.
- Locals and references use decoded slots. Names and source locations are
  diagnostic metadata; prepared references use canonical resolved `FormId`s.
- Authoritative instance state is central and survives entity/cell lifetime.
- Native calls use typed capabilities and deferred commands, never `&mut World`.
- Script-generated events enter a later processing round; execution is never
  recursively invoked by gameplay systems or coupled to render frequency.

Yarn dialogue is documented as a standalone adapter sequence in
`DIALOGUE_YARN_ROADMAP.md`. It consumes the typed registry, deterministic Bevy
adapter, save barrier, and condition/quest/effect authorities described below;
it does not create a second scripting runtime. The dialogue roadmap owns Yarn
presentation, prepared dialogue bundles, Fallout dialogue normalization, and
the replaceable Yarn host bridge.

## Delivery sequence

### Wave 1 — record stream and script inventory

Issues #252-#255 extract one resolved record stream, build the structural
catalog and attachment index, and expose deterministic corpus evidence through
the existing report slice. No instruction interpretation occurs in this wave.

### Wave 2 — `ScriptProgramV1`

Add versioned, serializable IR and hand-authored fixtures. Preserve source byte
offsets and optional source spans while normalizing execution to instruction
indices. Unknown instructions remain inspectable. Gate on stable round-trips,
hashes, and arithmetic/branch/call/event/wait fixtures.

### Wave 3 — decoder and VM kernel

The SCDA disassembler and headless VM can proceed independently after Wave 2.
The disassembler first measures opcode, function, event, and decode-failure
frequency. The VM implements persistent slots, stack/frames, jumps, native
calls, yield/resume, structured faults, and per-instance/global budgets against
synthetic IR. A separate SCTX feasibility spike compares a pinned TypeScript
oracle, the Rust NVSE parser, and a minimal native port without making any of
them a shipping dependency.

### Wave 4 — typed function registry

One descriptor table resolves SCDA numeric IDs and case-folded SCTX names,
aliases, targets, defaults, condition IDs, contexts, and extension providers.
It drives decoding, source lowering, validation, runtime dispatch, and coverage
reporting. Initial implementations are limited to authoritative services that
already exist.

### Wave 5 — deterministic Bevy adapter

Add one `ScriptRuntimePlugin` ordered as event collection, binding resolution,
instance scheduling, VM execution, command application, and diagnostics.
Prove a player-visible object-script slice with activation/inventory or enabled
state plus a yielding `GameMode` fixture. Locals survive unload/reload, commands
apply once, and repeated traces produce equal state hashes.

### Wave 6 — save/load

Persist a versioned runtime snapshot only at a post-command barrier. Include
locals, serializable continuation indices/stacks/frames, wake state, queued
events, sequence state, globals, and status. Old saves default empty; changed
program revisions are migrated explicitly or quarantined/reset, never resumed
blindly.

### Wave 7 — conditions, quests, dialogue, effects, and extensions

Route CTDA conditions through the shared registry, then add quest/dialogue and
effect ownership/events. Expand vanilla Fallout 3 functions from measured
coverage before FOSE; New Vegas/NVSE remains outside scope unless the project
explicitly expands platforms.

## Explicit rejections

- No `geckscript-lsp` shipping dependency or persisted third-party AST.
- No `esplugin` SCPT payload reader or second complete ESM4 parser.
- No string-keyed authoritative locals, per-scene program duplication, or
  assumption that every script owns a top-level FormID.
- No raw Bevy `World` native-function API, recursive execution, or render-frame
  `GameMode` scheduling.
- No broad FOSE/NVSE implementation before corpus evidence and vanilla needs.

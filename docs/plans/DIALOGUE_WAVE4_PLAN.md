# Dialogue Wave 4 plan — narrative variables and boundary saves

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute after the M7 save barrier is
defined so dialogue state cannot introduce a competing save authority.

## Fixed interfaces and wire contract

- `NarrativeVariableState` stores persistent, session, and temporary values in
  deterministic maps.
- Namespaces are `$global_`, `$session_`, and `$temp_`.
- `DialogueSnapshot` contains schema version, dialogue bundle hash,
  persistent variables, and `active: Option<ActiveDialogueCheckpoint>`.
- Add a top-level `DLOG` record and advance the save format from v5 to v6.
- Represent persistent values with a Bevy-free stable DTO rather than Yarn
  runtime types.
- Wave 4 always serializes `active = None` and clears session/temporary data
  at the documented boundaries.

## Save policy

- Persistent variables save before dialogue starts or after it completes.
- Manual save during active dialogue is rejected or deferred with a stable
  diagnostic.
- Loading always begins with no active Yarn instruction execution.
- A missing DLOG record means an older save with empty dialogue state.

## Test-first order

1. Add `@dialogue-wave4` scenarios for namespace lifetime, deterministic
   variable ordering, save rejection/defer, v5 compatibility, and corruption.
2. Add dedicated snapshot and binary save tests.
3. Integrate the storage with the runner builder and save barrier.
4. Verify encode/decode determinism and old-save behavior.

## Acceptance gate

- Persistent variables round-trip in sorted deterministic order.
- Session and temporary variables do not survive their boundaries.
- v1–v5 saves load with absent dialogue state.
- Corrupt or incompatible DLOG data fails safely without partial state.
- No active Yarn runner internals are serialized.

Depends on Waves 2–3 and M7 save/load barriers. Write
`DIALOGUE_WAVE4_MANUAL.md` before the PR with a save/load runtime trace.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->

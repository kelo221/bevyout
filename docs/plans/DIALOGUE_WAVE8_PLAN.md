# Dialogue Wave 8 plan — explicit checkpoint-node save/resume

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute after the first generated
conversation has proven that action identity and bundle provenance are stable.

## Fixed checkpoint contract

`ActiveDialogueCheckpoint` contains dialogue key, checkpoint node, stable
speaker/listener IDs, and sorted completed action keys. It is optional in
`DialogueSnapshot` and remains absent for ordinary boundary saves.

## Load behavior

1. Restore compatible persistent narrative variables.
2. Recreate the session context.
3. Start the authored checkpoint node.
4. Suppress completed action keys atomically.
5. Reset or quarantine the checkpoint on bundle-hash mismatch while retaining
   compatible persistent variables.

Do not serialize or restore arbitrary Yarn VM instruction state, runner entity
IDs, or Bevy entities.

## Test-first order

1. Add `@dialogue-wave8` scenarios for checkpoint creation, load, action
   deduplication, mismatch quarantine, and interrupted sessions.
2. Add snapshot migration and idempotency tests.
3. Integrate checkpoint metadata with generated/authored nodes.
4. Exercise save/restart/load in the viewer.

## Acceptance gate

- Resume occurs only at an explicit checkpoint node.
- A completed world mutation is never applied twice after load.
- Hash mismatch does not execute stale checkpoint actions.
- Older saves and saves without checkpoints remain valid.
- No arbitrary Yarn runner internals enter the save format.

Depends on Wave 4, Wave 7, and M7 save/load barriers. Write
`DIALOGUE_WAVE8_MANUAL.md` before the PR with a restart/load trace.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->

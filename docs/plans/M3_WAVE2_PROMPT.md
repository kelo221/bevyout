# M3 Wave 2 — Kickoff Prompt

Epic: [#7 — M3 Inventory, equipment, and Pip-Boy foundation](https://github.com/kelo221/bevyout/issues/7)

User request (2026-07-15): pick a few tasks from the M3 milestone epic #7,
avoiding double assignment — three sub-issues (#70 item catalog, #71
authoritative inventory + Pip-Boy Items UI, #72 dropped-item persistence /
save v2) are already taken and in flight (kelo221, wave 1). The picked tasks
must make sense together. Way of working per `AGENTS.md` (waves, sub-issues,
feature-first tests, worktree swarm, gates); model split: Fable orchestrates
and plans, Sonnet executes per-issue.

## Selection

The unclaimed checklist items cluster into the **container/loot pipeline**,
chosen because it complements rather than overlaps wave 1 and directly
serves gate #8 ("representative item lifecycle survives save and reload"):

- #74 — leveled-list records (LVLI/LVLN/LVLC) + pure deterministic resolver.
- #75 — container open + world-loot transfer interface (corpse and barter
  deferred: no actors or economy exist yet).
- #76 — persist container stacks + resolved leveled results.

Rejected alternative: the equipment cluster (slots/equip rules, consumables,
equipment persistence) — every task in it consumes #71's authoritative
stacks, which are unmerged wave-1 work; the loot cluster only touches that
seam at one narrow API point.

# Plans and wave records

Every multi-issue work wave leaves a paper trail here so it stays traceable:
**what was requested** (the `*_PROMPT.md` kickoff briefing), **what was
planned** (the `*_PLAN.md` with its fixed feature lists and tests-first
ordering), and **what actually landed, changed, or broke** (the plan's
"Shipped amendments" section plus the PR and issue comments it links to).

Convention for a new wave: the kickoff prompt and the plan both live in this
folder, named `M<milestone>_WAVE<n>_PROMPT.md` / `..._PLAN.md`. The plan is
amended — not rewritten — when acceptance testing forces changes, so the delta
between "planned" and "shipped" stays visible. Measured results go in the plan
and as comments on the issues; the PR closes the issues.

| Wave | Requested | Planned | Landed |
|------|-----------|---------|--------|
| M1 runtime foundation | (predates this convention) | [M1_PLAN.md](M1_PLAN.md) | PRs #40, #42, #43, #44 |
| Console + agent bridge | (predates this convention) | [CONSOLE_PLAN.md](CONSOLE_PLAN.md) | PR #42 |
| M2 wave 1 — import (cell map, batch selectors) | (predates this convention) | [M2_PLAN.md](M2_PLAN.md) | PR #50 (#45, #46) |
| M2 wave 2 — instant cell transitions | [M2_WAVE2_PROMPT.md](M2_WAVE2_PROMPT.md) | [M2_WAVE2_PLAN.md](M2_WAVE2_PLAN.md) — see "Shipped amendments" for what acceptance changed (A1–A5, bug #56, rejected experiment A4) | PR #54 (#47, #48, #51, #52, #56); follow-up #55 |
| M2 wave 3 — animations, fingerprints, reveal amortization | [M2_WAVE3_PROMPT.md](M2_WAVE3_PROMPT.md) | [M2_WAVE3_PLAN.md](M2_WAVE3_PLAN.md) — see "Shipped amendments" (A6–A9) | PR #58 (#57, #49; #55 partial, stays open) |
| M2 wave 4 — gate #6 remainder: failure recovery, state persistence, reveal fix, resumable bake, collider ownership | [M2_WAVE4_PROMPT.md](M2_WAVE4_PROMPT.md) | [M2_WAVE4_PLAN.md](M2_WAVE4_PLAN.md) — see "Shipped amendments" (A12–A17; #55's pre-warm became a one-line reload-loop fix) | PR #68 (#59, #60, #61, #62, #63, #55) |
| M3 wave 1 — OpenMW-based inventory, Pip-Boy Items, persistent dropping | [M3_WAVE1_PROMPT.md](M3_WAVE1_PROMPT.md) | [M3_WAVE1_PLAN.md](M3_WAVE1_PLAN.md) | PR #77 (#70, #71, #72) |
| M3 wave 2 — leveled loot, container transfer, container persistence | [M3_WAVE2_PROMPT.md](M3_WAVE2_PROMPT.md) | [M3_WAVE2_PLAN.md](M3_WAVE2_PLAN.md) | PR #79 (#74, #75, #76) |
| M3 wave 3 — caps, ownership, quest-item flags | [M3_WAVE3_PROMPT.md](M3_WAVE3_PROMPT.md) | [M3_WAVE3_PLAN.md](M3_WAVE3_PLAN.md) | PR #82 (#81) |
| M3 wave 4 — console additem, pickup activation | [M3_WAVE4_PROMPT.md](M3_WAVE4_PROMPT.md) | [M3_WAVE4_PLAN.md](M3_WAVE4_PLAN.md) | in flight (#84) |
| M3 wave 5 — canonical item instances, atomic holders, save v3, static merchant | [M3_WAVE5_PROMPT.md](M3_WAVE5_PROMPT.md) | [M3_WAVE5_PLAN.md](M3_WAVE5_PLAN.md) | in progress (#95) |

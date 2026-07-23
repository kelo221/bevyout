# M4 package-points wave — kickoff prompt

**Issue:** #213 — Decode patrol-marker & editor-location package subrecords so
AI packages resolve to movable points. Follow-up from the P1 AI-packages wave
(#196/#197/#198, merged as PR #214). Epic #9.

## What was requested

On real data (SuperDuperMart, cell `0001a273`), the typical NPC packages are
**Patrol** (`near-linked-reference`, location type 6) and **Sandbox**
(`near-editor-location`, location type 3). The #195 resolution layer already
knows how to turn both into world points — but the runtime `ResolutionContext`
never populates the two inputs those types read, so every Patrol/Sandbox NPC
stands rather than walks:

```
showpackages 0005cf10 -> Patrol 00023619 selected,
  location unresolved: near-linked-reference location has no linked reference
```

Make Patrol NPCs walk their authored markers in order and Sandbox NPCs roam
within their editor-location radius, driven through the existing `runpackage`
console command, verifiable with cinema.

## Acceptance (from the issue)

A prepared SuperDuperMart NPC with a Patrol package, once `runpackage`-driven,
walks its authored markers in order (verify with cinema); a Sandbox NPC roams
within its editor-location radius.

## Runtime / model

Claude runtime, single-issue wave → **sequential on the `m4-package-points`
branch**, one Sonnet executor. Orchestrator (Opus) plans, reviews, runs gates
and real-data acceptance, opens the PR.

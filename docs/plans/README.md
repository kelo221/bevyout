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
| M3 wave 4 — console additem, pickup activation | [M3_WAVE4_PROMPT.md](M3_WAVE4_PROMPT.md) | [M3_WAVE4_PLAN.md](M3_WAVE4_PLAN.md) | PR #85 (#84; merged into `m3-wave3`, reached master via PR #82's squash) |
| M3 wave 5 — canonical item instances, atomic holders, save v3, static merchant | [M3_WAVE5_PROMPT.md](M3_WAVE5_PROMPT.md) | [M3_WAVE5_PLAN.md](M3_WAVE5_PLAN.md) | in progress (#95) |
| M3 wave 6 — equipment, consumable use and reading, Pip-Boy Data views | [M3_WAVE6_PROMPT.md](M3_WAVE6_PROMPT.md) | [M3_WAVE6_PLAN.md](M3_WAVE6_PLAN.md) | PR #122 (#98, #99, #100; combined with wave 7); manual script [M3_WAVE6_MANUAL.md](M3_WAVE6_MANUAL.md) |
| M3 wave 7 — recipe preparation and corpse loot | [M3_WAVE7_PROMPT.md](M3_WAVE7_PROMPT.md) | [M3_WAVE7_PLAN.md](M3_WAVE7_PLAN.md) — see "Shipped amendments" (A10) | PR #122 (#117, #118; combined with wave 6); follow-up #120; manual script [M3_WAVE7_MANUAL.md](M3_WAVE7_MANUAL.md) |
| M4 wave 1 — actor record preparation | [M4_WAVE1_PROMPT.md](M4_WAVE1_PROMPT.md) | [M4_WAVE1_PLAN.md](M4_WAVE1_PLAN.md) | PR #126 (#103); manual script [M4_WAVE1_MANUAL.md](M4_WAVE1_MANUAL.md) |
| M4 wave 2 — NAVM/NAVI nav graph | [M4_WAVE2_PROMPT.md](M4_WAVE2_PROMPT.md) | [M4_WAVE2_PLAN.md](M4_WAVE2_PLAN.md) | PR #127 (#111); manual script [M4_WAVE2_MANUAL.md](M4_WAVE2_MANUAL.md) |
| M4 wave 3 — bevy_landmass navigation backend spike | [M4_WAVE3_PROMPT.md](M4_WAVE3_PROMPT.md) | [M4_WAVE3_PLAN.md](M4_WAVE3_PLAN.md) | PR #129 (#112); manual script [M4_WAVE3_MANUAL.md](M4_WAVE3_MANUAL.md) |
| M4 wave 4 — travel-door navigation and intercell agents | [M4_WAVE4_PROMPT.md](M4_WAVE4_PROMPT.md) | [M4_WAVE4_PLAN.md](M4_WAVE4_PLAN.md) | PR #135 (#113, #134); follow-ups #136, #137, #138; manual script [M4_WAVE4_MANUAL.md](M4_WAVE4_MANUAL.md) |
| M4 wave 5 — grounded movement, mid-route door gating, tnm path | [M4_WAVE5_PROMPT.md](M4_WAVE5_PROMPT.md) | [M4_WAVE5_PLAN.md](M4_WAVE5_PLAN.md) | PR #140 (#114, #137, #138; #136 measured, stays open); manual script [M4_WAVE5_MANUAL.md](M4_WAVE5_MANUAL.md) |
| M4 wave 6 — nav clearance, note text, real corpses, Pip-Boy click actions | [M4_WAVE6_PROMPT.md](M4_WAVE6_PROMPT.md) | [M4_WAVE6_PLAN.md](M4_WAVE6_PLAN.md) — see "Shipped amendments" (A1–A9) | PR #149 (#136, #123, #120, #121); follow-up #148; manual script [M4_WAVE6_MANUAL.md](M4_WAVE6_MANUAL.md) |
| M4 wave 7 — actor assembly and deterministic appearance fallbacks | [M4_WAVE7_PROMPT.md](M4_WAVE7_PROMPT.md) | [M4_WAVE7_PLAN.md](M4_WAVE7_PLAN.md) — see "Shipped amendments" (A1–A5) | PR-ready (#107, #108); manual script [M4_WAVE7_MANUAL.md](M4_WAVE7_MANUAL.md) |
| M4 wave 8 — nav correctness: portals, door topology, stuck progress, setlock | [M4_WAVE8_PROMPT.md](M4_WAVE8_PROMPT.md) | [M4_WAVE8_PLAN.md](M4_WAVE8_PLAN.md) — see "Shipped amendments" (A1–A8) | PR #166 (#154, #155, #157, #163); follow-ups #162, #164, #165; manual script [M4_WAVE8_MANUAL.md](M4_WAVE8_MANUAL.md) |
| M4 wave 9 — nav follow-ups: lock-respecting travel hand-off, fall-out guard + collision root cause, authored NAVM semantics | [M4_WAVE9_PROMPT.md](M4_WAVE9_PROMPT.md) | [M4_WAVE9_PLAN.md](M4_WAVE9_PLAN.md) — see "Shipped amendments" (A1–A5) | PR #170 (#165, #164, #156; #148 root-caused, stays open); follow-ups #168, #169; manual script [M4_WAVE9_MANUAL.md](M4_WAVE9_MANUAL.md) |
| M4 wave 10 — collision-derived navmesh rebuild; portal quarantine, preferred costs, setlock init | [M4_WAVE10_PROMPT.md](M4_WAVE10_PROMPT.md) | [M4_WAVE10_PLAN.md](M4_WAVE10_PLAN.md) — see "Shipped amendments" (A1–A6) | PR #173 (#162, #168, #169, #153; #148 carried by #171); follow-ups #171, #172; manual script [M4_WAVE10_MANUAL.md](M4_WAVE10_MANUAL.md) |
| M4 wave 10 — actor KF compatibility and animation zoo | [M4_WAVE10_ACTOR_PROMPT.md](M4_WAVE10_ACTOR_PROMPT.md) | [M4_WAVE10_ACTOR_PLAN.md](M4_WAVE10_ACTOR_PLAN.md) — see "Shipped amendments" | PR #183 (#104); runtime integration follow-up #106; manual script [M4_WAVE10_ACTOR_MANUAL.md](M4_WAVE10_ACTOR_MANUAL.md) |
| M4 wave 11 — finish the navmesh saga; start AI packages | [M4_WAVE11_PROMPT.md](M4_WAVE11_PROMPT.md) | [M4_WAVE11_PLAN.md](M4_WAVE11_PLAN.md) — see "Shipped amendments" (A1–A6) | PR #182 (#171, #175, #176, #180; #172 closed as premise-disproven, #148 carried by #177); follow-ups #177, #179, #181; manual script [M4_WAVE11_MANUAL.md](M4_WAVE11_MANUAL.md) |
| M4 wave 12 — actor animation game-flow integration | [M4_WAVE12_ACTOR_PROMPT.md](M4_WAVE12_ACTOR_PROMPT.md) | [M4_WAVE12_ACTOR_PLAN.md](M4_WAVE12_ACTOR_PLAN.md) — see "Shipped amendments" | PR #187 (#106; #104 closed by PR #183); manual script [M4_WAVE12_ACTOR_MANUAL.md](M4_WAVE12_ACTOR_MANUAL.md) |
| M4 wave 13 — actor state and persistence | [M4_WAVE13_ACTOR_PROMPT.md](M4_WAVE13_ACTOR_PROMPT.md) | [M4_WAVE13_ACTOR_PLAN.md](M4_WAVE13_ACTOR_PLAN.md) | In review (#110, PR #200); manual script [M4_WAVE13_ACTOR_MANUAL.md](M4_WAVE13_ACTOR_MANUAL.md) |
| M4 doors wave — doors as real blockers, agents that open them, ORCA stall | (see plan §"Why this wave existed") | [M4_DOORS_WAVE_PLAN.md](M4_DOORS_WAVE_PLAN.md) — see "Shipped amendments" (A1–A4) | PR pending (#177, #184; #148 root-caused, carried by #185/#186); post-mortem in [docs/postmortem/](../postmortem/VERDICT.md); manual script [M4_DOORS_WAVE_MANUAL.md](M4_DOORS_WAVE_MANUAL.md) |
| Architecture wave 1 — pure core contracts and policies | [ARCH_WAVE1_PROMPT.md](ARCH_WAVE1_PROMPT.md) | [ARCH_WAVE1_PLAN.md](ARCH_WAVE1_PLAN.md) | In progress (#143; epic #142) |
| Architecture wave 2 — typed viewer composition | [ARCH_WAVE2_PROMPT.md](ARCH_WAVE2_PROMPT.md) | [ARCH_WAVE2_PLAN.md](ARCH_WAVE2_PLAN.md) | Planned (#144; epic #142) |
| Architecture wave 3 — interaction capability modules | [ARCH_WAVE3_PROMPT.md](ARCH_WAVE3_PROMPT.md) | [ARCH_WAVE3_PLAN.md](ARCH_WAVE3_PLAN.md) | Planned (#145; epic #142) |
| Architecture wave 4 — viewer console capability modules | [ARCH_WAVE4_PROMPT.md](ARCH_WAVE4_PROMPT.md) | [ARCH_WAVE4_PLAN.md](ARCH_WAVE4_PLAN.md) | Planned (#146; epic #142) |
| Architecture wave 5 — production-backed extension traits | [ARCH_WAVE5_PROMPT.md](ARCH_WAVE5_PROMPT.md) | [ARCH_WAVE5_PLAN.md](ARCH_WAVE5_PLAN.md) | Planned (#147; epic #142) |
| M4 HeadAnims wave — actor animation stale-cache finding + regression tests | [M4_HEADANIMS_PROMPT.md](M4_HEADANIMS_PROMPT.md) | [M4_HEADANIMS_PLAN.md](M4_HEADANIMS_PLAN.md) — see "Outcome" | PR pending; #206 not-a-bug (stale actor-GLB cache pre-#160; HeadAnims regression tests kept), #205 closed as not-needed; manual script [M4_HEADANIMS_MANUAL.md](M4_HEADANIMS_MANUAL.md) |
| M4 AI-packages wave — autonomous AI: packages, families, perception, key-aware doors | [M4_AI_PACKAGES_PROMPT.md](M4_AI_PACKAGES_PROMPT.md) | [M4_AI_PACKAGES_PLAN.md](M4_AI_PACKAGES_PLAN.md) — see "Shipped amendments" | PR pending (#193–#198, #116, #185); follow-up #213 (deferred package-location subrecords); manual script [M4_AI_PACKAGES_MANUAL.md](M4_AI_PACKAGES_MANUAL.md) |
| M4 package-points — patrol linked-ref chains + editor-location resolution | [M4_PACKAGE_POINTS_PROMPT.md](M4_PACKAGE_POINTS_PROMPT.md) | [M4_PACKAGE_POINTS_PLAN.md](M4_PACKAGE_POINTS_PLAN.md) — see "Shipped amendments" | PR pending (#213); follow-up #222 (radius not FO3-scaled); manual script [M4_PACKAGE_POINTS_MANUAL.md](M4_PACKAGE_POINTS_MANUAL.md) |
| M4 autonomous-actors — on-load auto-bind + patrol + animation, ECS roster, native clips | [M4_AUTONOMOUS_ACTORS_PROMPT.md](M4_AUTONOMOUS_ACTORS_PROMPT.md) | [M4_AUTONOMOUS_ACTORS_PLAN.md](M4_AUTONOMOUS_ACTORS_PLAN.md) — see "Shipped amendments" | PR pending (#215, #218, #224, #225; stacked on #213); follow-ups #226 (creature clips), #227 (turn oscillation); manual script [M4_AUTONOMOUS_ACTORS_MANUAL.md](M4_AUTONOMOUS_ACTORS_MANUAL.md) |

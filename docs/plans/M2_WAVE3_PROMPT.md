# M2 Wave 3 — Kickoff Prompt

You are resuming work on bevyout (Fallout 3 → Bevy). Previous sessions
completed M2 waves 1 and 2. This file is your full briefing; the general way
of working (waves, sub-issues, feature-first tests, worktree swarm, gates,
BRP verification, logging) now lives in `AGENTS.md` — read it first and
follow it exactly.

## Read first (in this order)

1. `AGENTS.md` — architecture AND the way-of-working sections added after
   wave 2.
2. `docs/plans/README.md` — the traceability convention; then
   `docs/plans/M2_WAVE2_PLAN.md`, especially "Shipped amendments" (A1–A5).
3. GitHub: epic #5 (checklist state), gate #6 (milestone exit criteria),
   PR #54 (wave 2: batch session, resumable prepare, preloader, instant
   swap). **If #54 is not yet merged, ask the user whether to merge it
   first — wave 3 builds directly on it.**
4. Open M2 issues: #49 (fingerprints), #55 (first-reveal frame spike),
   #57 (door/activator animations — created for this wave).
5. Memory: `m2-import-wave` (session memory dir).

## State after wave 2

- Instant cell transitions work end-to-end on real data: Vault101a ↔
  Vault101b ↔ Vault101d chain gives 4/4 same-frame swaps, zero loading
  screens; neighbors preload via the door graph (`cellmap.ron`, dropped
  into `.bevyout/cache/` by every batch prepare). Swap telemetry:
  `swap <src>-><dst> instant max_frame_ms=<x>`; revisits 22–33 ms, first
  reveal of the largest cell ~84 ms (that spike is #55).
- `prepare` has a batch session, job manifest, resume, `--retry-failed`,
  `--jobs N`. Corpus prepared: Vault101a/b/d + 000151e3 + 00017f37.
- Console: `activate <ref>` drives door travel over the BRP bridge
  (`{"line":"activate 00028579"}` = the Vault101a→b door). V toggles the
  camera (bug #56 — the message advertised it, wave 2 bound it).
- Save-layer application on activation exists as a tested seam
  (`ActiveSaveState`), but nothing inserts real save data yet.
- Vault 101 FormIDs: a=00024512, b=00024511, d=00028138. Door refs:
  a→b 00028579, b→d 0005398b, d→b 00054285, b→a 00052196.

## Wave 3 goal — doors that look like doors, and the rest of the M2 gate

Priority order (the user explicitly asked for animations first):

1. **#57 door and activator controller animations** — activating any door
   visibly opens it; travel doors play their opening before/during the
   (already instant) swap. Importer side (NIF controllers → GLB animation
   or sidecar) + viewer side (play on activation, sync with
   `InteractionState::open` and door sounds). This is the wave's headline;
   validate on the Vault 101 route.
2. **#49 fingerprint validation** — plugin/converter/physics/prep
   fingerprints checked before reuse; the last open area/import item.
3. **#55 first-reveal amortization** — profile the reveal frame first
   (visibility vs pipeline cost), then pick the cheapest fix; the plan's
   acceptance bar (no frame > 33 ms on any preloaded hop, first visit
   included) becomes reachable.
4. **Remaining #5 checklist items, issue-per-claim as you take them**
   (create sub-issues the way wave 2 created #51/#52): fades + cancellation
   for the loading fallback; apply persistent/enable-parent state from a
   real save flow (the seam exists); preserve dynamic bodies across
   unload/reload; resumable `bake --all-interiors`; cell
   ownership/asset-barrier/unload lifecycle. Check gate #6 for which of
   these the milestone actually requires — do not gold-plate past the gate.

### Acceptance (real data, measured — not vibes)

- Vault 101 chain again over BRP, now with doors visibly animating
  (`scene_snapshot` before/after + the swap/preload log lines; remember
  `capture_viewport` is black when the window is occluded — if you need
  visual evidence, ask the user to watch a run).
- Frame telemetry re-measured on a cool machine (startup collision "cook"
  line ~10 ms is the canary) — #55's fix judged against the same
  `max_frame_ms` lines.
- Every issue commented with measured results; plan amended, not
  rewritten; PR(s) with `Closes #NN`; `docs/plans/README.md` table row
  updated; memory updated.

## Wave-3-specific cautions

- Blender/NIF animation export is new ground: decide early whether
  controller animations ride in the converted GLB or a sidecar, spike it on
  one door NIF (e.g. VDoorSliding01) before committing the wave plan, and
  record the decision in `M2_WAVE3_PLAN.md`.
- Asset-cache invalidation: changing the NIF converter output means bumping
  `NIF_CONVERTER_REVISION` — that invalidates every cached GLB, so budget
  one full re-prepare of the corpus (and note #49 makes this exact
  situation detectable).
- The `tests/features.rs` merge seam and file-ownership rules from
  `AGENTS.md` apply to every parallel agent.

# M2 Wave 4 — Kickoff Prompt

You are resuming work on bevyout (Fallout 3 → Bevy). Previous sessions
completed M2 waves 1–3 (wave 3: PR #58 — door animations #57, fingerprints
#49, partial first-reveal work on #55). This file is your full briefing; the
way of working (waves, sub-issues, feature-first tests, worktree swarm,
gates, BRP verification, logging, model split) lives in `AGENTS.md` — read
it first and follow it exactly.

## Read first (in this order)

1. `AGENTS.md` — architecture AND way-of-working.
2. `docs/plans/README.md`, then `M2_WAVE3_PLAN.md` — especially "Shipped
   amendments" A6–A9: the niftools corpus patches, the stale-`AnimationPlayer`
   rediscovery, and the #55 measurement story you must not re-derive.
3. GitHub: epic #5 (checklist state), gate #6 (exit criteria), PR #58.
   **If #58 is not yet merged, ask the user whether to merge it first —
   wave 4 builds directly on it.**
4. Open M2 issues, all assigned and linked under #5:
   #59 (fades/cancellation/failure recovery), #60 (apply persistent +
   enable-parent state), #61 (preserve dynamic bodies/containers),
   #55 (first-reveal render pre-warm — carries wave-3 measurements),
   #62 (resumable `bake --all-interiors`), #63 (cell ownership/asset
   barriers/lifecycle).
5. Memory: `m2-import-wave` (session memory dir).

## State after wave 3

- Doors and activators visibly animate: GLBs carry named `Open`/`Close`
  clips (converter `niftools-blender52-havok-sidecar-anim-v4`); travel
  doors defer the swap by a 600 ms open-lead; console/BRP
  `activate <ref>` goes through the same lead and reports `open_lead_ms`.
- Fingerprints (#49) validate on every batch run; `prepare
  --check-fingerprints` reports; targeted invalidation verified on real
  data.
- #55 is instrumented (`reveal <formid> entities= chunks= visflip_ms=
  frame_ms=`) and chunked (256/frame, arrival-door chunk first). Vault101d
  first reveal: 84 → ~36–39 ms; the ≤33 ms bar is NOT met. Known facts:
  visibility flipping is free (~0.1 ms); smaller chunks measure WORSE
  (longer window overlaps collider/preload work); first run over a fresh
  corpus measures 111–134 ms from cold caches — always measure the second
  run. Next candidate: render/pipeline pre-warm for preloaded-but-hidden
  cells. Do NOT retry below-world visible stashing (wave-2 A4) or smaller
  chunks (wave-3 A9).
- Save-layer application on activation exists as a tested seam
  (`ActiveSaveState`), but nothing inserts real save data yet — #60/#61
  fill that seam from both directions (apply on load, capture on unload).
- The V camera-mode key binding was removed in PR #58 (user decision —
  do not re-bind; console `tfc` is the only free-camera toggle).
- Vault 101 FormIDs: a=00024512, b=00024511, d=00028138. Door refs:
  a→b 00028579, b→d 0005398b, d→b 00054285, b→a 00052196. Corpus also has
  000151e3 (MegatonPlayerHouse) and 00017f37 (SuperDuperMart).

## Wave 4 goal — the rest of gate #6

Gate #6's two unmet criteria drive priority:

1. **#59 fades + cancellation + failure recovery** — gate: "Failed loading
   returns safely to the source cell." Fallback path only; instant swaps
   stay fade-free.
2. **#60 + #61 state persistence** — gate: "Dynamic, inventory, and
   enable-parent state survive revisits and restart." One shared save-layer
   seam, two issues (apply-on-load vs capture-on-unload); consider one
   agent for both with the seam as its internal boundary.
3. **#55 render pre-warm** — the remaining frame-budget miss; profile
   against the existing reveal telemetry before building (the wave-3 data
   on the issue is the baseline).
4. **#62 resumable bake / #63 ownership lifecycle** — P2, not gate-blocking;
   take them only if the wave has capacity after 1–3 are green on real data.

### Acceptance (real data, measured — not vibes)

- Gate #6 walk-through: drop/move a dynamic object and open a container in
  Vault101b, run the a→b→d→b→a chain, revisit — state persists; restart the
  viewer with a save and it still holds (#60/#61).
- Force a fallback failure (e.g. temporarily rename a destination
  manifest): player returns to the source cell at the door, with a notice,
  no stuck loading screen (#59).
- Frame bar re-measured on a cool machine (11 ms cook canary), second run
  after any corpus rebuild: the #55 fix judged against the same
  `reveal`/`swap max_frame_ms` lines, all preloaded hops ≤ 33 ms including
  first visits.
- Every issue commented with measured results; plan amended, not
  rewritten; PR(s) with `Closes #NN`; `docs/plans/README.md` row updated;
  memory updated; epic #5 ticked only for criteria that hold on real data.

## Wave-4-specific cautions

- `tests/features.rs` merge seam and file-ownership rules apply to every
  parallel agent (wave 3's boundaries worked; reuse the pattern).
- #59 and #55 both live in `src/viewer/world/**` — either sequence them or
  split ownership per file (`swap.rs`/fallback UI vs `preload.rs`/
  `reveal.rs` + any new pre-warm module).
- #60/#61 touch unload/reload paths that #63 would formalize — if both run
  in one wave, #63's design must come first or be deferred again.
- A converter/physics revision bump forces a full corpus re-prepare and a
  cold-cache first run — budget it and measure on the second run.

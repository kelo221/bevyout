# M9 wave 6 plan — stealth evidence, ownership, crime, Karma

## Execution model recommendation

Roadmap recommendation: **Sol X-High** (Codex runtime) / **Opus** (Claude
runtime) — the wave extends the existing awareness authority, ownership
policy, and save v9 optional blobs rather than adding a parallel RPG
stack. ZCode runtime: the orchestrating session executes directly on
`M9-Work`.

## Fixed feature list

### Core

- `detection.rs`: quantized `DetectionEvidence` (mm, millidegrees, bps,
  ms). `update_from_evidence` mutates existing `AwarenessState`
  (`confidence_milli` / `time_since_seen_ms` plus f32 copies).
  `DetectionConfig::golden()`: 400 ms of a maxed term does not acquire;
  800 ms does. Non-finite geometry is rejected. Hidden/Caution/Danger is
  `project_detection_hud`.
- `items.rs`: `OwnershipClaim` + `TakerFactions` + `classify_ownership`.
  Unowned / player `0x7` = Take. Known faction + taker rank ≥ required
  XRNK = Take. Else Steal. `classify_take` remains a compatibility
  wrapper with empty taker/factions.
- `crime.rs`: `CrimeId { actor, sequence }`, Theft/Assault/Murder with
  fixed bounty/karma (40/-5, 40/-10, 1000/-100). Witnesses sorted by
  class+FormID. Empty eligible → no report; theft still marks stolen.
  Replay of a reported id is a no-op. `escalate_assault_to_murder` keeps
  the same `CrimeId`.

### Persistence

- Save format stays **v9**. `RPG_SAVE_REVISION` stays **1**.
- Optional RPGS `CRIM` (bounty, karma, next sequence, reported ids).
- Optional ACTR `AWRS` (actor `AwarenessState`).
- HUD projection is not saved.

### Viewer

- Perception adapter quantizes once, then calls `update_from_evidence`.
  Gameplay light is prepared cell ambient sRGB, never GPU luminance.
- Pickup / scripted pickup / container take classify once and report
  through `maybe_report_theft` / `report_theft_in_world`.
- Runtime player faction membership is empty (`TakerFactions::default()`);
  faction-legal takes are covered in core/cucumber.
- Console: `detectstate`, `crime`, `setownership`, `getkarma`, `modkarma`
  as a 20th provider without exceeding the 150-line `console.rs` cap.

## Tests-first order

1. `features/rpg_stealth_crime.feature`.
2. Cucumber World fields after `rpg_restock_outcome`; steps at EOF.
3. Core unit tests (`detection.rs`, `crime.rs`, `items.rs`).
4. Save optional CRIM/AWRS + console adapters.
5. Implement until green; gates; `M9_WAVE6_MANUAL.md`.

## Acceptance gates

- Light/movement/armor/Perception alone acquire after 800 ms; darkness,
  occlusion, out-of-cone, and out-of-range never acquire.
- Oscillation around the acquire threshold does not flicker.
- Equidistant evidence prefers lowest FormID; player beats another actor.
- Non-finite geometry is rejected; legacy 0.6 confidence migrates to 600.
- Faction rank ≥ required is Take; below / non-member is Steal.
- Unwitnessed theft marks stolen, bounty 0. Two witnesses → bounty 40
  once. Ineligible witnesses produce no report. Assault→murder bills
  murder only.
- HUD Hidden / Caution / Danger from observer hostility + confidence.
- `cargo fmt --check`, clippy `-D warnings`, tests.

## Shipped amendments

- **A1.** Integer evidence sits beside the float `AwarenessState::update`
  path so `perception.feature` stays green.
- **A2.** `classify_take` remains for existing cucumber; faction scenarios
  call `classify_ownership`.
- **A3.** Destination item id after transfer/pickup is
  `latest_player_item` (max id of that base form on the player).
- **A4.** Player FACT membership is not prepared in this viewer; runtime
  takes use an empty taker snapshot.
- **A5.** `HudSneaking` is still a placeholder; nothing in this wave
  writes it. Detection labels still only render while that flag is true.

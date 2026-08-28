# M9 wave 7 kickoff — headless lockpicking and terminal hacking

Requested on 2026-08-28 (continuing remaining M9 work on `M9-Work` per
[M9_Total.md](M9_Total.md)):

- integer millidegree lockpick state machine and deterministic hacking
  board in `bevyout-core` (no Bevy, no `rand`, no `f32` angle/stress);
- shared session protocol with domain-separated `MinigameRngState`;
- bobby pins consumed through `ItemLedger::use_item` atomically with the
  session step that breaks a pin;
- doors/terminals mutate only on success or lockout; saving is blocked
  while a minigame is active; cell unload cancels the session;
- `GameplayModal::Lockpicking` / `Hacking`; console `lockpick`,
  `unlock`, `hackterminal` stay on the existing world provider.

The approved slice is wave 7 only: V.A.T.S. stays wave 8, TERM record
decode is a follow-up, schematic bonus and restock catalog mutation
stay out of this wave.

Tracked work:

- integer lockpick + force-lock chance in basis points
- synthetic hacking boards, likeness, brackets, lockout
- trespass through Wave 6 `resolve_crime`
- persist picked/locked-out lock overrides; block save in-session
- viewer adapter, activation start, console inspection

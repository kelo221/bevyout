# M9 wave 9 kickoff — integer game clock and lifecycle

Requested on 2026-08-28 (continuing remaining M9 work on `M9-Work` per
[M9_Total.md](M9_Total.md)):

- authoritative integer `GameTime` / `GameClockState` in `bevyout-core`
  (absolute game milliseconds, fractional timescale remainder, calendar
  projection, overflow checks);
- one ordered lifecycle scheduler whose due tasks run in pinned kind
  then owner order;
- cell reset receipts, encounter-zone lock-on-first-entry, and
  validate-then-commit fast travel;
- migrate effect ticks, merchant restock, and owned-bed restoration onto
  that clock;
- lighting `GameClock.hour` is a projection only; RPGS `TIME` is optional
  on save v9 without bumping `RPG_SAVE_REVISION`;
- console `passtime`, `fasttravel`, `resetcell`, `showgametime` stay on
  the existing world provider.

Wave 8 (V.A.T.S.) stays blocked on M5 ballistics/armor. This slice is
wave 9 only.

Tracked work:

- integer clock + `GameTimeAdvanced`
- scheduler: effects → rads/withdrawal → death → restock → reset → arrival
- 72-hour cell reset and restock boundaries
- encounter-zone lock and fast-travel evidence
- persist TIME/LIFE; viewer adapter; manual script

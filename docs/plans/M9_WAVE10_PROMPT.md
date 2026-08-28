# M9 wave 10 kickoff — shared projections, Pip-Boy, v9 freeze

Requested on 2026-08-28 (continuing remaining M9 work on `M9-Work` per
[M9_Total.md](M9_Total.md)):

- one serializable `RpgInspectionSnapshot` family in `bevyout-core`;
- Pip-Boy Stats/Items/Data format that snapshot (no second HP/rad/cripple math);
- Data World consumes integer game time from the snapshot, not a second clock;
- do not invent quests/map/radio;
- freeze save v9 / `RPG_SAVE_REVISION` 1; unknown RPGS subrecords stay skipped;
- BRP `bevyout.rpg_stats_probe`, `bevyout.vats_probe`,
  `bevyout.active_effects_probe` use the same read model;
- `vats_probe` reports unavailable because Wave 8 is blocked on M5 W4/W5;
- console `showstats` lives on the existing stats provider (no 21st provider).

Wave 8 (V.A.T.S.) stays blocked. This slice is wave 10 only.

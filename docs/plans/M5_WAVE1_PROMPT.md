# M5 wave 1 kickoff — functional 10mm pistol

Requested on 2026-07-25:

- make the Fallout 3 10mm pistol functional in first person;
- fire one shot when the captured left mouse button is pressed;
- do not count or consume ammunition yet;
- play weapon audio and animate fire/reload actions;
- cast a center-screen hitscan ray and damage/kill prepared actors;
- flash a short-lived point light at the muzzle instead of spawning particles;
- start reload with the reload key and block firing until reload completes;
- keep the weapon architecture modular so later firearms, melee weapons,
  projectiles, ammo accounting, and alternate presentation can extend it;
- verify the result through the bevyout MCP/agent bridge on real prepared data.

The approved first slice is pistol-only presentation without player hands or
arms. Actor hits mutate the canonical persisted actor state. It deliberately
does not add ammo counting, ammo consumption, aim-down-sights, spread,
particles, casing ejection, weapon condition loss, or NPC weapon combat.

Tracked work:

- #235 — prepared first-person weapon data and action audio
- #236 — pure player-weapon action and actor-damage policies
- #237 — 10mm viewmodel, input, reload, hitscan, light, and audio
- #238 — console/MCP controls and real-data acceptance

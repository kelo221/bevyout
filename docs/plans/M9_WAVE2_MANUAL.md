# M9 wave 2 manual acceptance — perk catalog, evaluator, active modifiers

Wave 2 gave the player Fallout 3's perk system on real data: all 87 perks
from `Fallout3.esm` decode into a versioned catalog (requirements, ranks,
and typed entries), a pure evaluator decides eligibility (level, rank, and
SPECIAL/skill condition gates), and owned perks actively modify gameplay —
Swift Learner multiplies awarded XP, Educated adds +3 skill points per
level. Four console commands expose all of it: `addperk`, `removeperk`,
`hasperk`, and `showperks`.

## One-time setup

1. Prepare the acceptance cell (writes the perk catalog alongside the
   wave-1 artifacts):

   ```
   cargo run -- prepare --cell 000151e3
   ```

   Expected deterministic line:

   ```
   perk catalog: 87 perks, 58 playable, 3 hidden, 18 quest, 58 ability, 42 entry-point entries, 48 unknown conditions -> catalogs/<fingerprint>/perks.ron
   ```

   (48 perks carry non-GetActorValue conditions like GetIsSex/HasPerk;
   those block eligibility with `unknown_conditions` reasons rather than
   being silently accepted.)

2. Launch the viewer with the agent bridge:

   ```
   cargo run -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
   ```

## Live acceptance

3. Eligibility gates block with reasons (fresh player is level 1):

   | Command | Expected result |
   | --- | --- |
   | `player.addperk 00031dd3` (Swift Learner, level 2) | error `perk_ineligible`: "requires level 2 (player is 1)" |
   | `player.hasperk 00031dd3` | `rank 0/3` |

4. Level up past the gate, then own and benefit from Swift Learner:

   | Command | Expected result |
   | --- | --- |
   | `player.rewardxp 1000` | level 3 (XP multiplier still ×1.00) |
   | `player.addperk 00031dd3` | `rank 1/3`, modifiers `xp ×1.10` |
   | `player.rewardxp 1000` | XP 1000 → **2100** (1000 × 1.10) |
   | `player.addperk 00031dd3` | `rank 2/3`, modifiers `xp ×1.20` |
   | `player.removeperk 00031dd3` | `rank 0/3`, modifiers back to ×1.00 |

5. Educated adds +3 skill points per level (INT 4 gate passes at INT 5):

   | Command | Expected result |
   | --- | --- |
   | `player.addperk 00031dd8` | `rank 1/1`, `+3 skill points/level` |
   | `player.advlevel` | `+18 skill points` (15 + 3) |
   | `showperks` | lists Educated rank 1/1 with active modifiers |

6. Error paths:

   | Command | Expected error code |
   | --- | --- |
   | `player.addperk 00015166` (an item FormID, not a perk) | `unknown_perk` |
   | `showperks --eligible` | lists blocked perks with reasons (e.g. level gates, `unknown_conditions`) |

## Notes

- Rank entries grant only the owned rank's entry (Swift Learner rank 2 is
  ×1.20, not 1.1×1.2), matching the engine.
- The AV condition indices (SPECIAL 5–11, skills 32–45) were derived from
  real data and are documented in plan amendment A1; `AVIF` FormIDs are
  unrelated to them.
- Measured evidence (2026-08-22) is recorded as comments on issues #312,
  #313, and #314.

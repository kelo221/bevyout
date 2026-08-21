# M9 wave 1 manual acceptance — SPECIAL, skills, derived stats, GMST, leveling

Wave 1 made the player a real Fallout 3 character sheet: the pure core
kernels compute the 13 skill bases, derived max health / action points /
carry weight / critical chance from SPECIAL, the GMST settings decoded from
`Fallout3.esm` during `prepare` drive the formula multipliers, and the XP
engine levels the player through the vanilla 200/550/1050… threshold curve.
All of it is inspectable live through five new console commands:
`getav`, `setav`, `modav`, `player.advlevel`, and `player.rewardxp`.

## One-time setup

1. Prepare the acceptance cell (reuses cached assets; also writes the GMST
   catalog):

   ```
   cargo run -- prepare --cell 000151e3
   ```

   Expected output includes (deterministic line):

   ```
   gmst catalog: 530 settings, 9 consumed, 0 undecoded, 60 actor values -> catalogs/<fingerprint>/gmst.ron
   ```

   (`9 consumed`, not 11: the base-game `Fallout3.esm` carries neither
   `iMaxPlayerLevel` — a Broken Steel setting — nor
   `iLevelUpSkillPointsInterval`, so those keep the GOTY fallback defaults
   30 and 1.)

## Live acceptance

2. Launch the viewer with the agent bridge:

   ```
   cargo run -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
   ```

3. Open a second terminal and drive the console over JSON-RPC (or type the
   commands into the in-game console — same results):

   ```
   curl -X POST http://127.0.0.1:15702/ -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":1,"method":"bevyout.console.exec","params":{"line":"player.getav health"}}'
   ```

4. Verify the fresh level-1 sheet (all SPECIAL 5):

   | Command | Expected result |
   | --- | --- |
   | `player.getav health` | `200` (= 100 + 5×20 + 0×10) |
   | `getav small_guns` | `15` (= 2 + 2×5 + ⌈5/2⌉) |
   | `getav strength` | `5` |

5. Verify clamped mutation:

   | Command | Expected result |
   | --- | --- |
   | `player.modav strength 10` | `10` (clamped from 15) |
   | `player.getav strength` | `10` |
   | `player.modav strength -30` | `1` (floor) |
   | `player.setav strength 8` | `8` |

6. Verify leveling through the vanilla XP curve
   (thresholds 200 / 550 / 1050 / … / 66 700):

   | Command | Expected result |
   | --- | --- |
   | `player.rewardxp 200` | level 2, 200 XP, +15 skill points |
   | `player.getav health` | `210` (level term is `(level-1)×10`) |
   | `player.advlevel` | level 3, 550 XP, +15 skill points |
   | `player.rewardxp 999999` | level 30, XP clamped to 66700, +405 points |
   | `player.advlevel` | error `at_level_cap` ("player is at the level cap 30") |

7. Verify error paths:

   | Command | Expected error code |
   | --- | --- |
   | `player.getav nosuchvalue` | `unknown_actor_value` |
   | `player.setav health 500` | `unsupported_actor_value` (health is derived) |

8. Regression check (optional): the full gates — `cargo fmt --check`,
   `cargo clippy --all-targets -- -D warnings`, `cargo test` (1912 tests),
   and the cucumber feature `features/rpg_stats.feature` (9 scenarios).

## Notes

- `getav health` is computed synchronously from the sheet, so values are
  correct immediately after `rewardxp` in the same console batch — no frame
  of staleness.
- NPC actor values are unchanged: they still resolve through the persisted
  actor-state surface (`actorstate`, `setactorvalue`), which predates M9.
- Measured evidence from the acceptance run (2026-08-21) is recorded as
  comments on issues #308, #309, and #310.

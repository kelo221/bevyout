# M4 doors wave — manual acceptance script

What this wave shipped, in plain language: **doors are now real to the
navigation system.** Before it, the navmesh had no idea doors existed
unless they led to another cell — so agents planned routes straight
through closed doors, walked into the solid slab, and got stuck. Three
separate "mystery wedges" tracked across four waves turned out to be
exactly that, each with a different door. Now every door blocks, an agent
walks up to an ordinary closed door and **opens it**, a locked door is a
real barrier, and a door that cannot be opened at all makes the route
fail immediately instead of walking the agent into it.

It also fixes a movement bug that made agents freeze in open corridors
with nothing touching them — the navigation backend was applying
invisible "wall avoidance" from stairs *below* the agent.

## 0. One-time setup

```
cargo run-dev -- prepare --cell 00024512
cargo run-dev -- prepare --cell 0001a273
```

Expected new line (the key field is `unreported interior polygons 0` —
nothing walkable is left inside a closed door):

```
nav doors: blockers 13, associations 422 (blocking 219) across 9 blocker(s), unreported interior polygons 0
nav doors: blockers 33, associations 350 (blocking 7) across 22 blocker(s), unreported interior polygons 0
```

## A. The headline: an agent opens an ordinary door and walks through

1. Launch Vault 101 (the `--unfocused` flag means it will not steal focus):
   ```
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00024512/scene.ron \
       --agent-bridge --agent-port 15702 --unfocused
   ```
2. Put the player between the vault door and the in-cell door, then spawn:
   `player.setpos x 154`, `player.setpos y 40.3`, `player.setpos z -70`,
   then `tna spawn` — expect `spawned at (154.18, 39.75, -70.00)`.
3. `tna goto 153.6 40.3 -60`, then poll `tna status`.

   Expected: the agent walks −70 → −68 → −66 → −64.5, the log shows
   ```
   door Door (00024657) opened (scripted, nav agent)
   nav agent door wait 00024657
   nav agent door resume 00024657
   ```
   and it continues to **`status=reached`** near (153.6, −61).

   Before this wave the agent stopped dead at z ≈ −66.2 and never opened
   anything.

## B. A locked door is a real barrier

4. `activate 00024657` (closes it again), then `setlock 00024657 25`.
5. `tna despawn`, `tna spawn`, `tna goto 153.6 40.3 -60`.
   Expected: **`status=unreachable`**, and no `door … opened` line for
   `00024657` — the agent does not walk into it and does not open it.
6. `setlock 00024657 0` and repeat step 3 — it opens and reaches again.

## C. A door that cannot be opened fails the route immediately

7. `tna despawn`, then `tna spawn` at the cell's default start (z ≈ −108),
   and `tna goto 153.6 40.3 -60`.
   Expected: **`status=unreachable`** almost immediately. This route must
   cross the closed `VaultGearDoor`, which no agent can open. Before this
   wave the agent walked straight through that closed vault door.

   Known gap (tracked in #186): the vault door animates open when
   activated but never registers as open for navigation, so it stays
   `unreachable` even then.

## D. The invisible-stall fix (#184)

8. Step 3 already proves it — the agent crosses z ≈ −66.2, which was the
   permanent stall line. If you want the diagnostic directly, watch for
   the reason label in the log:
   ```
   nav agent collision-blocked <id> contacts reason=obstructed              …blocking_planes=[…]
   nav agent collision-blocked <id> contacts reason=no_contact_no_progress  …blocking_planes=[]
   ```
   `no_contact_no_progress` means "nothing is touching the agent, it just
   isn't moving" — the distinction that took four waves to get.

## E. Console parity

9. `activate 00024657` on an ordinary door now toggles it and prints
   `door 00024657 opened` / `closed`. Before this wave it failed with
   `no_destination: door has no travel destination`, so there was no way
   for a human to open an interior door at all.

## F. Nothing else regressed

10. The wave-9 travel-door flow still works on 0001a273: `setlock 0007f7e3 0`,
    `player.setpos` to (90, 96.2, −144), `tna spawn`, `tna travel 0007f7e3`
    → `handed off to cell 0001a280`; with `setlock 0007f7e3 25` it is
    `unreachable`.

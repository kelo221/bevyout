# M9 wave 5 — manual acceptance script

What this wave shipped, in plain language: **you can repair one item with
another of the same form, craft from a prepared schematic, and buy/sell at
Fallout 3 barter prices.** Every one of those is a single canonical
`ItemLedger` mutation. Merchant restock is a 72-game-hour policy that does
not tick yet (wave 9).

- Repair uses integer condition, same `base_form_id`, and a skill cap.
  One donor unit is consumed.
- Crafting consumes ingredients atomically. Recipes with opaque GECK
  conditions refuse instead of succeeding.
- `buy` / `sell` compute a quote from player Barter skill and FO3 GMSTs,
  then commit. You cannot type a unit price.
- `repairitem` and `craftitem` are the visible runtime surface.

## 0. One-time setup

```
cargo run-dev -- prepare --cell 000151e3
```

Launch the viewer with the agent bridge (or use the in-game console `~`):

```
cargo run-dev -- view --manifest .bevyout/cache/scenes/000151e3/scene.ron --agent-bridge --agent-port 15702
```

Megaton player house. Bridge examples use `curl` against port 15702.

## A. Field repair

1. `player.getav repair` — note the skill (default sheet is 15 unless you
   changed it). For a visible cap, `player.setav repair 50`.
2. `additem 0000434f 2` (10mm Pistol; two units share one instance when
   condition matches). Then add a second stack if they merged:
   drop/re-add, or add a second damaged copy after changing condition via
   a prior repair. Practical path:
   - `additem 0000434f 1` → note instance id A
   - `additem 0000434f 1` → if it stacked onto A, use two separate
     damaged weapons already in the cell, or proceed with craft/barter
     first. For two distinct ids, add after changing one stack's
     condition is not exposed; the cucumber pins 40+40 → 50. In-game,
     two catalog-max pistols repair to the skill cap (50 at skill 50).
3. `repairitem <target> <donor>`
   Expected JSON: `condition_after` ≤ cap, donor count decreased by 1,
   `transaction_id` present. Repeating the same ids with a new
   transaction consumes another donor if one remains.
4. `repairitem <id> <id>` (same instance twice)
   Expected: rejected (`repair target and donor are the same item`).

## B. Schematic crafting

5. `craftitem 000a5fa6` (or any FormID printed by a missing-catalog
   error). If the prepared recipe catalog loaded, `craftitem` with a real
   RCPE FormID from `catalogs/<fingerprint>/recipes.ron` consumes
   ingredients and prints `crafted recipe …`.
6. A recipe whose prepared `conditions` blob is non-empty must fail with
   `recipe has an unsupported GECK condition` and leave inventory counts
   unchanged.
7. A recipe whose ingredients you do not have fails with
   `missing crafting ingredients`. `additem` afterwards must still
   allocate the next instance id that would have been used before the
   failed craft (failed crafts must not burn ids).

If `recipes.ron` is missing, startup logs
`recipes: no recipe catalog for this content set, craftitem disabled` and
`craftitem` returns `unknown_recipe`. Re-run `prepare` for this cell.

## C. Barter quote/commit

8. Mark a container merchant: `setmerchant <container-ref> 200`.
9. `player.setav barter 0`, then `buy <merchant-ref> <merchant-item-id> 1`.
   Expected: unit price is catalog value × 1.55 (skill 0 buy). A 100-cap
   item costs **155**. Log line includes that unit price, not the raw
   catalog value.
10. `player.setav barter 100`, then `sell <merchant-ref> <player-item-id> 2`.
    Expected: unit price is catalog value × 0.90; two 100-cap items
    credit **180**.
11. Caps on both holders move by `unit_price * count` in one receipt.
    Quest items and caps stacks are still rejected.

## D. Restock (policy only)

12. There is no console restock command in this wave. After 72 game hours
    the policy will fire once Wave 9 advances `GameTime`. Do not expect
    merchant inventory to refill from wall-clock time.

## E. Persistence

13. Repair an item, `save testrepair`, quit, relaunch with that slot.
    The repaired condition and any crafted instance ids must match.
    Format remains v9; ITMS snapshot fields for repair/craft receipts
    default empty on older saves.

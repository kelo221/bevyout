# M9 wave 2 kickoff — perk catalog, requirements evaluator, active modifiers

Requested on 2026-08-21:

- continue the M9 milestone (epic #19, all 10 waves per
  [M9_Start.md](M9_Start.md));
- decode Fallout 3 `PERK` records into a versioned `PreparedPerkCatalog`
  during `prepare` (requirements, ranks, and typed perk entries);
- build the pure perk policy layer: a requirements evaluator
  (`can_take_perk`), an owned-perk rank container, and an active-modifier
  projection that feeds the wave-1 stat kernels (XP award multiplier from
  Swift Learner, bonus skill points from Educated);
- expose perks at runtime: catalog loading, the player's `ActorPerks`,
  modifiers folded into XP/leveling, and the `addperk` / `removeperk` /
  `hasperk` / `showperks` console commands;
- keep the feature-first invariant and the M9 determinism contract.

The approved slice is wave 2 only: ability `SPEL` effects themselves are
decoded but not executed (that is wave 3's active-effects engine), quest
perks store their quest FormID without running scripts, and trait support
(FNV-only) is out of scope.

Tracked work:

- #312 — `PERK` record decode and `PreparedPerkCatalog`
- #313 — pure perk requirements evaluator and modifier model
- #314 — perk runtime, level-up selection, and console surface

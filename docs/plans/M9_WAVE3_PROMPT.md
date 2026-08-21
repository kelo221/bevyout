# M9 wave 3 kickoff — active effects, ingestibles, radiation, addiction

Requested on 2026-08-22 (continuing the all-10-waves goal on epic #19,
roadmap [M9_Start.md](M9_Start.md)):

- decode `ALCH` ingestibles and `MGEF` effect records during `prepare`
  into a versioned `PreparedEffectCatalog`;
- build the pure active-effects engine: a duration-ticking effect ledger
  with source tags, the radiation pool with vanilla threshold penalties,
  and the chem-addiction state machine driven by a core-owned, versioned
  PRNG (no `rand`);
- wire consumption through the existing canonical item-use seam so using
  a Stimpak heals, RadAway cures rads, chems apply temporary buffs and
  risk addiction;
- expose everything through `rads`, `addrads`, `removerads`, `addchem`,
  `cureaddiction`, and `effects` console commands;
- keep the feature-first invariant and the M9 determinism contract
  (integer milliseconds, basis points, ordered collections, inspectable
  PRNG draws).

The approved slice is wave 3 only: no limb damage (wave 4), no crafting or
merchant restock interplay (wave 5), and ENCH/SPEL ability execution stays
deferred (perk ability FormIDs remain stored, not run).

Tracked work:

- #316 — `ALCH`/`MGEF` decode and `PreparedEffectCatalog`
- #317 — pure active-effects, radiation, and addiction kernels
- #318 — chem/aid runtime, item-use integration, and console surface

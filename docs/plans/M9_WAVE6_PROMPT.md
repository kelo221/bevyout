# M9 wave 6 kickoff — stealth evidence, ownership, crime, Karma

Requested on 2026-08-28 (continuing remaining M9 work on `M9-Work` per
[M9_Total.md](M9_Total.md)):

- quantized detection evidence updates the existing `AwarenessState` /
  `ActorAwareness` authority (no second `StealthState`);
- faction membership/rank ownership replaces temporary `classify_take`
  (any non-player owner = steal);
- one crime report per `CrimeId` (witnesses sorted by FormID; bounty is
  not multiplied per witness); theft still marks canonical stolen
  provenance when unwitnessed;
- persist player bounty/Karma in RPGS `CRIM` and actor awareness on
  actor instance state (`AWRS`); save format stays v9 / RPG revision 1;
- HUD Hidden/Caution/Danger is a projection, not saved;
- console: `detectstate`, `crime`, `setownership`, `getkarma`, `modkarma`.

The approved slice is wave 6 only: lockpicking stays wave 7, V.A.T.S.
stays wave 8, sneak *toggle* writing `HudSneaking` stays a later
presentation wire.

Tracked work:

- integer millimetre/millidegree/bps/ms detection kernel
- `OwnershipClaim` + `TakerFactions` + known-faction rank policy
- `CrimeLedger` + stolen provenance
- optional CRIM/AWRS on the existing v9 save
- viewer adapters, HUD projection, crime console

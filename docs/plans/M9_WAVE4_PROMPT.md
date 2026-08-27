# M9 wave 4 kickoff — body parts, limb health, crippling, medical aid, save v9

Requested on 2026-08-28 (continuing remaining M9 work on `M9-Work` per
[M9_Total.md](M9_Total.md)):

- freeze six semantic body parts (`Head`, `Torso`, `LeftArm`, `RightArm`,
  `LeftLeg`, `RightLeg`) in core `combat/`; unknown geometry falls back to
  torso;
- apply limb damage through the existing `resolve_actor_impact` pipeline
  (no second damage listener, no Bevy limb-health components as authority);
- persist player RPG state plus player limbs as save format **v9 `RPGS`**,
  and NPC limbs on `ActorInstanceState` (`ACTR LIMB`);
- project cripple penalties into locomotion, Perception, and weapon reload;
- restore limbs through one medical policy (targeted Stimpak, doctor,
  owned-bed on explicit `GameTime`);
- expose `showlimbs` / `cripple` / `selectlimb` and live Pip-Boy meters.

The approved slice is wave 4 only: merchant restock stays wave 5, stealth
and crime stay wave 6, V.A.T.S. stays wave 8, owned-bed *activation* waits
for the wave-9 clock.

Tracked work:

- semantic anatomy and limb-state kernels
- save v9 `RPGS` + NPC `ACTR LIMB`
- hitscan `BodyPartId` evidence, Pip-Boy meters, console, Stimpak restore

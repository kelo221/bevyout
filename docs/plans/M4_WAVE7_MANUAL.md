# M4 wave 7 manual acceptance — actor assembly and fallbacks (#107, #108)

## What this wave shipped

Actors now carry one prepared assembly blueprint from resolved Fallout identity
through conversion, the scene manifest, canonical inventory, and the viewer.
Humanoids retain body, head, hair, both eye meshes, selected eye texture,
apparel, scale, and a deterministic starting weapon. Creatures retain their
authored rig and compatible visual parts without entering humanoid rules.

Nested `LVLN`/`LVLC` actor lists now resolve all the way to a concrete matching
NPC or creature. The source shell remains inspectable, while sex, race, parts,
apparel, stats, and canonical inventory all come from one deterministic leaf.

Missing assets no longer silently remove an actor. Five deterministic fallback
tiers keep source/resolved/reference identity and structured reasons; the last
tier spawns a selectable bounds proxy. `actorinspect <reference>` exposes the
prepared and live state, including the canonical actor holder and weapon bind.

Native `nifty` conversion is the production and acceptance path. The Blender
actor conversion uses PyNifly native DLLs and is therefore a Windows-only
comparison tool; it cannot satisfy the visual gate.

Native head assembly preserves the distinct Fallout frames used by hair and
the other face parts. Runtime weapon attachment also remains stable while its
deferred visual spawn becomes queryable, preventing duplicate attachment log
spam and the resulting flicker.

## One-time setup

Prepare the representative cells through native conversion. Do not pass
`--force`: the actor-catalog, pipeline, and native converter revisions must make
an older prepared scene stale and rebuild it normally.

```
cargo run-dev -- prepare SuperDuperMart
cargo run-dev -- prepare 00024511 000151e3 --converter native
```

The first Super-Duper Mart run must rebuild the stale scene and schedule the
revised actor GLBs. Run the same commands again. Expected: valid/warm prepared
scenes and no actor GLB rebuild. On Windows only, `--converter blender` may be
used afterward to compare a native discrepancy; switch back to native and
reprepare before recording acceptance.

## Steps

### A. Humanoid body, head, eyes, hair, and apparel — Vault 101 Atrium

1. Launch:
   `cargo run-dev -- render 00024511 --agent-bridge`
2. Open the console and run `actorinspect 00054432` (Vault 101 utility worker).
3. Expected: `kind=humanoid`, tier `RaceSexSpecific`, scale `1.0`, no proxy,
   and reason `missing_facegen`. The structured value reports:
   - body roles `Body(0)`, `Body(1)`, and `Body(2)`;
   - `Head(0)` plus the supplemental mouth/teeth/tongue parts;
   - visible `Hair`;
   - **two** `Eyes` geometry parts and eye texture
     `characters/eyes/eyedefault.dds`;
   - four available apparel entries (utility suit, Pip-Boy glove, glasses,
     and Pip-Boy arm);
   - canonical holder `Actor { reference_form_id: 345138 }` with four stable
     item instances.
4. Close the console and look at the worker from the head and shoulders.
   Expected: one complete rest-pose humanoid at the authored placement, not
   separate proxy/body objects, with the authored close-cropped `HairBase`
   visibly covering the scalp and the eyes/mouth aligned in the face. Hair
   metadata or GLB node names alone do not pass this step.

### B. Creature primary/secondary assembly — Megaton Player House

5. Launch with the bridge if desired:
   `cargo run-dev -- render 000151e3 --agent-bridge`
6. Run `actorinspect 0008f6ae` (Wadsworth).
7. Expected: tier `AuthoredExact`, six visual parts, skeleton
   `creatures/mistergutsy/skeleton.nif`, root `misterhandy.nif`, no proxy, and
   a canonical equipped instance for weapon `0003bc6f`. The weapon state says
   `missing_model` because that built-in buzzsaw has no separate WEAP world
   model; its authored `gutsybuzzsaw.nif` remains present in the six-part
   creature assembly.
8. Close the console. Expected: Wadsworth is visibly a complete floating robot
   with body, arms, claw, buzzsaw/flamer parts, and effects—not a bounds box.

### C. A second creature and integrated equipment — Super-Duper Mart

9. Launch the native prepared scene:
   `cargo run-dev -- render SuperDuperMart --agent-bridge`
10. Run `actorinspect 0006d921` (Protectron).
11. Expected: tier `AuthoredExact`, five creature parts, no proxy, skeleton
    `creatures/protectron/skeleton.nif`, canonical actor holder present, and
    equipped item `00018b9e`. `missing_equipment` is reported for the absent
    standalone WEAP model while the integrated right-hand laser creature part
    remains visible. This is a degraded optional attachment, not a body-tier
    downgrade.
12. Run `actorinspect 00041600` on the raider. Expected:
    - source shell `0002f6e2` and concrete resolved actor `0002f6d8`;
    - `female=true`, tier `RaceSexSpecific`, no proxy, and only the explicit
      `missing_facegen` body-tier reason;
    - nonempty canonical inventory/apparel, including worn armor `0003307c`;
    - native assembly input `armor/raiderarmor02/outfitf.nif` and the selected
      eye/hair records.
13. Close the console and inspect `00041600` from front, side, and back.
    Expected: the female raider wears the complete armor, skin appears only in
    the outfit's authored openings, and no triangle stretches, collapses,
    flashes, or flickers while the camera moves. Gender, armor, and body
    coverage must be visible in the viewport; catalog counters and GLB metadata
    do not satisfy this step. Leave the view running for several seconds:
    `actor weapon attached` must not repeat every frame, the armor must not
    flicker, and only one stable weapon visual may remain attached.

### D. Determinism and fallback surface

14. Restart any cell and repeat its `actorinspect` command. Expected: the same
    source/resolved/reference IDs, part ordering, fallback tier/reason order,
    canonical item IDs, and weapon decision.
15. The five synthetic fallback tiers are gate-tested by
    `features/actor_fallback.feature`. For a real actor that reaches the final
    tier in future content, expected viewer behavior is a visible/selectable
    bounds proxy and `actorinspect` tier `ProxyMesh`; the placement is never
    silently omitted.

## Historical measured snapshot

The original metadata-oriented acceptance recorded the following diagnostic
baseline, but it is superseded by the native visual steps above. On this
Windows dev build, Vault 101 Atrium's 600-frame bridge probe reported
average 16.670 ms, p95 18.283 ms, p99 18.985 ms, max 19.673 ms, with 8,192 ECS
entities and 1,647 mesh entities. The process snapshot was 2,863 MiB working
set / 5,529 MiB private bytes. These are local debug/dynamic-linking numbers,
not release targets; compare only on the same machine and configuration.

The final converter revision reused 1,755 assets, built 11 revised actor
assets, and completed all three cells. Actor catalogs contained 1 actor in
Megaton Player House, 17 in Vault 101 Atrium, and 11 in Super-Duper Mart, with
zero unresolved, unsupported, or skipped actor entries in all three.

## Known limits

- Exact FaceGen morph reconstruction remains #109. Authored-but-unsupported
  FaceGen uses a deterministic race/sex rest pose and reports
  `missing_facegen`.
- Some built-in creature weapons have no standalone WEAP model. Their
  integrated creature geometry is retained and the missing optional model is
  reported explicitly.
- The `GenericProjectBody` tier is policy-tested and is selected only when a
  skeleton-compatible project body is actually available. The repository does
  not currently ship one, so production data that exhausts the authored race
  tiers uses the visible/selectable bounds-proxy tier instead.
- The PyNifly actor conversion path is Windows-only because of its native DLL
  dependency and remains comparison-only. Native `nifty` actor assembly is the
  portable production path.

# M4 static NPC FaceGen reconstruction (#109)

This is the user-approved implementation expansion of #109, executed on the
single `m4-facegen` branch with one Codex agent. The branch starts at current
`master` and preserves the existing first commit, `dc865792` (`Init`), which
contains the skin/hair shader correction and its tests.

## Requested outcome

Implement static Fallout 3 NPC FaceGen identity during native actor
preparation:

- decode canonical little-endian finite-f32 `FGGS`/`FGGA`/`FGTS` coefficient
  layouts of 200/120/200 bytes;
- preserve coefficients through template inheritance, race/sex selection,
  `ActorBlueprint`, and the actor assembly descriptor;
- parse and validate the compatible `FREGM002` EGM and `FREGT003` EGT assets
  from loose data and BSA sources;
- deform only the selected head anchor, preserving topology, UVs, weights,
  normals, and tangent handedness;
- synthesize the head diffuse through the existing UASTC KTX2 path while
  preserving alpha and leaving body, hair, eyes, mouth, teeth, and apparel
  textures unchanged;
- expose policy, fingerprints, geometry/texture status, and typed diagnostics
  through `actorinspect`;
- invalidate only affected actor assets when FaceGen inputs or the algorithm
  revision changes; and
- keep missing/corrupt FaceGen data as a per-actor fidelity fallback.

## Acceptance boundary

Use the exact real-data selectors and actors from the objective:

- `SuperDuperMart` / `00017f37`: female raider `00041600`, male raider
  `00041610`;
- `MegatonCratersideSupply` / `00003a2a`: Moira `0002d2bc`, male mercenary
  `0001ff18`.

Each cell must prepare twice, reuse the actor cache on the second run, and
launch through the agent bridge. `actorinspect` and comparable close-ups must
show `facegen_policy=Authored`, no `missing_facegen`, distinct identities,
stable attachments, no new neck gaps or black normal seams, and no animation
or weapon-attachment regression.

## Explicit non-goals

Runtime morph targets, `.TRI` expressions, lip-sync, player FaceGen, James
genetics, runtime customization, SSS, Blender conversion, and a new bake format
remain deferred.

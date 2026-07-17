# M4 wave 1 — actor record preparation

Request (2026-07-16): start M4 (epic #9) with its foundation issue #103 —
"Prepare actor, creature, race, class, faction, template, inventory, and
AI-package records" — using parallel executor agents.

Scope confirmation from evaluation:

- All required record types exist in Fallout 3 (NPC_, CREA, RACE, CLAS,
  FACT, PACK, LVLN/LVLC, TPLT/ACBS templates, FaceGen subrecords). This is
  not NV-only data.
- OpenMW `components/esm4` provides prior art for NPC_, CREA, RACE, CLAS,
  and PACK layouts but has no FACT loader; FACT is decoded from the fopdoc
  Fallout 3 record reference.
- `src/vsa/openmw_esm4` already parses NPC_/CREA/LVLN/LVLC as generic
  `BaseRecord`s (EDID/FULL/MODL/CNTO/TPLT) and ACHR/ACRE references; this
  wave adds the actor-specific decode and the pure catalog on top.

The wave is internally parallel: two decode executors run first in isolated
worktrees, then one catalog executor consumes their types on the wave
branch. #104/#111/#112 are separate follow-up waves.

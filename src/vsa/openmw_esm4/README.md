# OpenMW-derived ESM4 importer

This private module is an attributed Rust adaptation of selected parsing and
record-layout behavior from the supplied `openmw-master` source snapshot. The
snapshot identifies itself as OpenMW 0.52.0. It contains no nested Git metadata,
so the exact upstream commit is unavailable and is not claimed here.

The adaptation is intentionally limited to the Fallout cell vertical slice:

- TES4/ESM4 record and group traversal, compression, extended subrecords, and
  master-relative FormID adjustment;
- Fallout `CELL`, `REFR`, `ACHR`, `ACRE`, base-object, and `NAVM` metadata;
- Fallout 3 `XCLL`/`LGTM.DATA` lighting layouts and `LNAM` field-by-field
  lighting-template inheritance;
- Fallout `SOUN`, `SNDR`, `ASPC`, `MUSC`, and `LGTM` metadata, plus base sound
  links, ownership, enable-parent state, inventory entries, and water fields;
- OpenMW's ESM4 teleport-door rule: resolve `XTEL`'s destination reference and
  use that reference's parent cell as the destination cell.
- Minimal Fallout NIF `NiVertexColorProperty` mode inspection used to
  distinguish ignored, emissive, and ambient/diffuse vertex colors before the
  attributed Blender conversion step.

It is a Rust adaptation, not a C++ binding. No OpenMW runtime, OSG, Bullet,
Detour, or VFS code is compiled into bevyout. Fallout 3's `NAVM` chunks are
retained as source metadata only because the supplied OpenMW code recognizes
but does not decode the FO3 `NVVX`, `NVTR`, `NVCA`, `NVDP`, `NVGD`, and `NVEX`
payloads.

The importer preserves CELL lighting instructions for the viewer and Blender
bake path. Original Fallout NIF/BSA lightmap extraction is intentionally not
part of this module.

See `NOTICE.md` for the exact upstream files, SHA-256 hashes, licenses, and
adaptation notes. The project remains distributed under GPL-3.0.

# Architecture wave 4 — kickoff prompt

Requested 2026-07-18 from the attached architecture reviews as the fourth
refactoring wave: decompose the viewer-side console coordinator while keeping
the already-separated pure console grammar, registry, executor, script, and UI
model intact.

Wave 4 is issue #146 under architecture epic #142. Command names, aliases,
help metadata, deterministic text/JSON output, and BRP result shapes are fixed
compatibility contracts.

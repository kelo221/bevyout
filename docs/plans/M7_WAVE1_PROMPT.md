# M7 wave 1 prompt — shared record stream and script inventory

Start M7 scripting with evidence and preparation boundaries, not a parser or
VM:

- extract the current content-index walk into a reusable internal resolved
  record stream;
- preserve compression, master FormID remapping, override/deletion semantics,
  provenance, load-order validation, and content fingerprints;
- keep `ContentIndex` as a metadata boundary rather than a payload warehouse;
- structurally catalog winning SCPT scripts, raw SCDA/SCTX, variables,
  references, unknown subrecords, and diagnostics;
- model top-level and embedded script identity from the beginning;
- index SCRI and structurally recognizable embedded-script attachments;
- extend the existing report slice with deterministic script inventory JSON and
  a concise CLI summary;
- prove behavior with synthetic plugins and run a local Fallout3.esm gate
  without committing Bethesda-derived data.

Do not decode SCDA instructions, parse SCTX, define runtime IR, add a VM, or
select a source parser in this wave. The measured report should determine those
later priorities.

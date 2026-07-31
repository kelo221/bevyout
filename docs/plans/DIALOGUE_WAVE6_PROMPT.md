# Dialogue Wave 6 prompt — Fallout dialogue inventory and catalog

Measure Fallout dialogue before generating a Yarn graph.

- Decode winning DIAL/topic and INFO/response records through the existing
  resolved record stream and script catalog boundaries.
- Build `PreparedDialogueCatalog` with conversations, lines, condition-set
  references, action-set references, provenance, and diagnostics.
- Extend the existing deterministic report slice with counts and coverage.
- Report missing localization/voice, speaker resolution failures, overrides,
  cycles, unreachable records, and unsupported conditions/actions.

Do not generate Yarn or alter Fallout semantic behavior in this wave.

# Dialogue Wave 6 plan — Fallout dialogue inventory and catalog

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute after M7 Wave 1's record
stream/script catalog and before any generated Yarn implementation.

## Fixed catalog contract

`PreparedDialogueCatalog` contains a revision, source fingerprint,
`BTreeMap`-ordered conversations, condition-set keys, action-set keys, line
records, source mappings, and structured diagnostics. It preserves canonical
FormIDs and plugin/record provenance.

## Required report fields

- DIAL/topic and INFO/response totals.
- Player-choice counts and response line counts.
- Condition and function frequencies.
- Result-script/action frequencies.
- Topic links, cycles, unreachable records, and deleted/overridden records.
- Speaker-resolution failures and missing localized strings.
- Voice/lip coverage.
- Unsupported condition/action records with source identity.

## Test-first order

1. Add `@dialogue-wave6` scenarios for synthetic DIAL/INFO graphs, overrides,
   deletion, cycles, missing speakers, and unsupported records.
2. Add deterministic catalog/report unit tests.
3. Reuse the existing report command and resolved record stream.
4. Run the report twice over the local Fallout3.esm and compare outputs.

## Acceptance gate

- Two equal inputs produce byte-identical catalog/report output.
- Every record maps back to plugin, FormID, signature, and source provenance.
- Unsupported content remains visible rather than silently omitted.
- No Yarn source is generated in this wave.
- Fallout-derived outputs remain under `.bevyout/` only.

Depends on M7 Wave 1 and Wave 5's authored runtime only for catalog shape
validation. Write `DIALOGUE_WAVE6_MANUAL.md` before the PR if a deterministic
CLI/report surface is available.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->

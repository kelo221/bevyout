# Dialogue Wave 7 plan — first generated Fallout conversation

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. Execute after the inventory report and
the M7 condition/quest/effect authorities are usable.

## Fixed generation policy

- Generate only from prepared catalog records; never compile ESM dialogue at
  ordinary viewer startup.
- Use canonical string content keys, not numeric Yarn values, for FormID
  identities.
- Preserve source plugin, FormID, signature, and record identity in node
  metadata/source indexes.
- Imported conditions call compact prepared condition-set keys; they are not
  reimplemented as literal Yarn CTDA expressions.
- Imported result actions call prepared action-set keys through `bo_run_action`.
- Authored overlay files replace generated conversations by explicit key and
  never modify generated files.

## Initial supported slice

One speaker, one opening response, two player options, basic inventory/global/
quest conditions, and one result script or quest-stage mutation. Exclude
persuasion, companion interruption, and scene choreography.

## Test-first order

1. Add `@dialogue-wave7` scenarios for source mapping, deterministic output,
   overlays, condition routing, and exactly-once result execution.
2. Add generator and source-index unit tests.
3. Generate a synthetic conversation and compare bytes across runs.
4. Run one selected Fallout conversation end to end in the viewer.

## Acceptance gate

- Generated Yarn is deterministic and every node is source-mapped.
- All imported conditions use the shared evaluator.
- All results use the shared script/host pipeline.
- Unsupported records remain visible in the Wave 6 report.
- One real Fallout conversation matches expected line, choice, condition, and
  result behavior.

Depends on Waves 3, 5, and 6 plus M7 shared authorities. Write
`DIALOGUE_WAVE7_MANUAL.md` before the PR with exact prepared FormIDs and
runtime commands.

## Shipped amendments

<!-- Record acceptance-driven changes here; do not rewrite the fixed plan. -->

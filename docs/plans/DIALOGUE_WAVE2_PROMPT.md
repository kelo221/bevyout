# Dialogue Wave 2 prompt — runner, lifecycle, UI, and input gating

Build the first complete authored Yarn runtime adapter.

- Add `ViewerSet::Dialogue` between `Interaction` and `WorldSync`.
- Create one persistent local-player runner when `YarnProject` becomes ready.
- Implement start requests, busy rejection, session lifecycle, completion
  cleanup, and runner-independent state.
- Convert Yarn line/options events into Bevyout-owned presentation state.
- Add native line/choice UI, input focus, continuation, selection, malformed
  node diagnostics, and `GameplayModal::Dialogue` gating.

Do not ship Yarn's example dialogue view or attach a runner to an NPC.

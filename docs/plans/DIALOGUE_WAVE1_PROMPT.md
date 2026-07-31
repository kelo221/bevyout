# Dialogue Wave 1 prompt — domain contracts and prepared bundle

Create the Bevyout-owned boundary and make dialogue a prepared content
artifact.

- Add Bevy-free dialogue IDs, requests, phases, presentation DTOs, and errors
  under `crates/bevyout-core/src/dialogue/`.
- Add `PreparedDialogueBundleRef` and its revision/hash to the prepared
  manifest.
- Prepare sorted explicit Yarn source paths, node/source indexes, generated
  versus authored files, and deterministic fingerprints.
- Validate node uniqueness, key resolution, source-root containment, and Yarn
  compiler diagnostics.

The viewer should be able to distinguish dialogue loading, ready, and failed
states, but this wave does not start a conversation.

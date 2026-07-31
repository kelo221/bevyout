# Dialogue Wave 8 prompt — explicit checkpoint-node save/resume

Extend the boundary save model with safe authored checkpoint resumption.

- Add explicit checkpoint metadata to authored/generated nodes.
- Restore only at checkpoint nodes, never at arbitrary Yarn instruction
  offsets.
- Persist stable dialogue/session identities and completed action idempotency
  keys.
- Suppress already committed actions on load.
- Reset or quarantine active checkpoints when the dialogue bundle hash changes.

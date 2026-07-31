# Dialogue Wave 5 prompt — authored NPC vertical slice

Prove the complete authored conversation path in the real viewer before
attempting Fallout conversion.

- Add prepared stable-actor-to-dialogue bindings and runtime `DialogueBinding`.
- Start a conversation from NPC activation.
- Focus the camera and enter the existing dialogue modal/input mode.
- Present a line and two choices, query state, apply one deferred mutation,
  present the response, and restore normal controls.
- Define busy, speaker-despawn, disable, and unload outcomes.

The runner and narrative variables remain persistent services, never NPC
components.

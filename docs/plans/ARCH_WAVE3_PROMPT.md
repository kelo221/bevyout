# Architecture wave 3 — kickoff prompt

Requested 2026-07-18 from the attached architecture reviews as the third
refactoring wave: decompose `src/viewer/interaction.rs` by capability without
changing the player-visible interaction contract.

Wave 3 is issue #145 under architecture epic #142. Existing inventory,
container, door, animation, audio, persistence, and input behavior are the
acceptance contract.

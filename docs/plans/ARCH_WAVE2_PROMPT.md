# Architecture wave 2 — kickoff prompt

Requested 2026-07-18 from the attached architecture reviews as the second
refactoring wave: replace the viewer's imperative subsystem registration with
typed Bevy plugins and explicit scheduling phases.

Wave 2 is issue #144 under architecture epic #142. The work must preserve the
current startup defaults, optional agent bridge, headless tests, and all
fixed/update schedule semantics.

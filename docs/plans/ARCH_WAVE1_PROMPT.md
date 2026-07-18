# Architecture wave 1 — kickoff prompt

Requested 2026-07-18 from the attached architecture reviews:

> Do all the waves 1 to 5 then make a PR to master, if you think you can do
> something better than the suggestion go for it.

Wave 1 is issue #143 under architecture epic #142:

- extract a small Bevy-free `bevyout-core` crate;
- move canonical item transactions into that crate;
- centralize FormID identity and master-table remapping;
- move the prepared manifest transport contract out of the Bevy runtime;
- retain compatibility at the existing `bevyout` module paths.

The review's broad recommendation is narrowed to behavior-preserving moves.
No manifest field, save encoding, command behavior, or cache revision changes
are authorized by this wave.

# M4 gate wave — package execution fixes and final gate acceptance

Request (2026-08-02): PR #271 has merged M5 Wave 3. M5 remains blocked by the
open M4 behavior gate #10. Prepare the smallest executable wave that makes the
remaining M4 package behavior honest on real Fallout data and then closes #10.

Live dependency state at kickoff:

- M2 gate #6 and M3 gate #8 are closed.
- M4A presentation gate #86 is closed by PR #260.
- M4 final gate #10 is still open and has no acceptance evidence comment.
- #222 records a reproducible real-data failure: the 1024-unit Sandbox radius
  from Vault 101 is treated as 1024 metres instead of about 14.63 metres.
- #231 and #242 have fixes on the unmerged `m4-autonomous-actors` follow-up
  branch, but those fixes are not in the current `origin/master`.

The wave is sequential because #222, #231, and #242 all affect the package
resolution/family runtime seam, and #10 must be evaluated against their merged
behavior. The wave does not start M5 Wave 4 and does not absorb unrelated M4
follow-ups.

Tracked issues: #222, #231, #242, and the final gate #10 under M4 epic #9.

# M4 wave 5 — kickoff prompt

Requested 2026-07-17, following M4 wave 4 (PR #135, #113 + #134 shipped):

> Okay cool do wave 5. If you can squeeze 138 into it, id be pleased
> (minor changes I suppose)

Wave composition agreed in the same conversation:

- **#114** — grounded actor movement, steps, slopes, collision, local
  avoidance (claimed together with the wave 4 follow-ups because #136 and
  #137 are entangled with it by their own text).
- **#137** — closed doors on the nav mesh do not gate or block nav agents
  (route/lifecycle half; the physics-blocking half belongs to #114).
- **#138** — `tnm` draws the active agent path; overlay brightness no
  longer drives auto-exposure.
- **#136** — corner hugging: *measure-first decision* after #114 lands;
  only build a fix if physics-authoritative movement plus avoidance does
  not already give acceptable clearance.

#115 + #116 were evaluated for the same PR and deliberately deferred to a
later wave (they pair with each other, and #115's runtime driver should
target the movement API #114 lands).

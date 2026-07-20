# M4 wave 10 kickoff — actor KF compatibility and animation zoo (#104)

Requested on 2026-07-19:

- continue the stopped `Animations` branch;
- do not switch or modify the caller's current branch;
- use an isolated git worktree for all continuation work; and
- determine the missing context from the repository before asking questions.

The remote branch contained one large prototype commit named `Init`: KFFZ
decoding, a prepared actor-animation catalog, external-KF clip-pack conversion,
and an isolated `animation-zoo` viewer. It had no pull request, wave plan,
manual acceptance script, or recorded real-data evidence. The continuation
worktree is `/Users/simon/projects/bevyout-worktrees/Animations` on the local
tracking branch `Animations`.

Repository and issue review identify the bounded prototype as the test surface
for [#104](https://github.com/kelo221/bevyout/issues/104), not completion of
[#106](https://github.com/kelo221/bevyout/issues/106). The latter still owns
gameplay idle/locomotion/turn/equip transitions and representative runtime
integration after the compatibility decision is measured.

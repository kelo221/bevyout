# M5 Wave 3 prompt — condition, degradation, jams, and deterministic combat RNG

Implement M5 Wave 3 on top of the landed Wave 2 combat foundation (PR #248),
covering weapon condition, per-shot degradation, deterministic combat RNG, and
fire/reload jams.

The implementation must keep the domain decisions in `bevyout-core` and route
runtime behavior through the canonical `ItemLedger`. Preserve stable
`ItemInstanceId` values for full-stack weapon moves and make partial/merge
behavior obey the existing transaction invariants. Rejected actions must not
consume RNG draws. Persist only the condition, jam, and RNG state that the
implemented Wave 3 policy actually uses, with explicit migration and revision
bumps for every changed serialized asset.

Add feature-first and dedicated unit coverage, expose enough runtime inspection
to see condition, jam state, RNG revision, draw index, and the last decision
terms, and finish with deterministic repeated-run and real-data acceptance in
the prepared Super-Duper Mart cell (`00017f37`) using the Wave 2 weapon/ammo
records (`0000434f` / `00004241`).

Wave 3 does not include armor, limbs, ballistics, VATS, AI combat, multi-weapon
balancing, or later save/profile work.

Tracked issues: #262, #263, #266, and #265 under epic #11.

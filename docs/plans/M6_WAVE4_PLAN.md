# M6 wave 4 plan

## Fixed scope

1. Define one canonical travel/location contract that distinguishes an
   exterior worldspace from an interior cell and preserves the exact authored
   arrival translation and rotation across serialization and migration.
2. Keep `SaveGame.location` as the serialized authority and
   `CurrentWorldLocation` as its live runtime projection. A legacy save without
   a WLOC record must have an explicit deterministic fallback; it must not be
   silently presented as an exact transform.
3. Keep prepared door destinations as authored data. Any runtime capsule-center
   or camera-height adjustment remains an explicit adapter owned by W4-C, not a
   second serialized anchor.
4. Specify water entry/exit, submerged depth, breath drain/recovery and the
   zero-breath consequence as pure policy decisions. Preserve the existing
   `resolve_water_contact` semantics: `swim_depth` is descriptive metadata and
   cannot make a deeply submerged player dry.
5. Specify landing/fall accumulation, jump/ceiling landings, tiny-fall
   suppression and hard-impact thresholds without making the result depend on
   fixed-update chunking.

## Lane ownership and merge order

| Lane | Owns | Must not touch |
| --- | --- | --- |
| W4-A / #279 | `crates/bevyout-core` location/door contracts, `src/save/`, `src/viewer/world/mod.rs`, focused location/migration tests | `src/viewer/world/{swap.rs,persist.rs}`, `src/viewer/interaction/activation.rs`, `src/viewer/player/movement.rs`, W4-B files, `tests/features.rs` |
| W4-B / #280 | Pure water/contact/breath/landing policy under `src/viewer/openmw_player/` plus dedicated player/openmw-player tests | W4-A files, `src/viewer/world/{swap.rs,persist.rs}`, `src/viewer/interaction/activation.rs`, runtime movement integration, `tests/features.rs` |
| W4-C / later | Sequential runtime integration in `world/{swap.rs,persist.rs}`, activation, exterior lifecycle, and `player/movement.rs`; executable feature/manual seam | Both policy lanes until their contracts are reviewed and merged |

W4-A and W4-B may run in parallel. W4-C is sequential and must not be started
as a substitute for either policy lane or before the M4 actor/runtime gate
dependency is resolved where it applies.

## Tests-first acceptance

### W4-A

- exterior and interior locations retain their distinct identity keys;
- authored destination position and rotation survive an exact round trip;
- a v7 save with WLOC decodes deterministically, including both location
  variants;
- pre-WLOC saves have an explicit, tested fallback and do not fabricate an
  exact position/rotation;
- duplicate or malformed location records are rejected consistently;
- the live location projection and serialized save use the same contract with
  no second location authority.

### W4-B

- no water, invalid water, surface contact, entry, exit, and deeper-submerged
  samples produce deterministic contact results;
- breath drains while submerged, recovers while dry at the documented rate,
  clamps at both bounds, and exposes a deterministic exhausted consequence;
- equivalent elapsed time split into different frame/update chunks produces
  the same breath and movement-policy result;
- stationary and directional jumps keep their authored arc, airborne control
  remains reduced, and ceiling stops still land audibly;
- tiny falls remain silent, normal falls emit the expected impact, and the hard
  threshold is exact and deterministic.

## Integration and evidence

- Each lane must pass its focused tests and leave a commit on its worktree.
- After merge, the integrator runs the full Rust tests, clippy, the applicable
  dev checks, and a focused feature/manual seam only when W4-C owns it.
- No W4 route, save/reload, or water acceptance claim is considered real-data
  evidence until W4-C has integrated the policies and the current prepared
  fixtures are available.
- The eventual wave manual must show one exterior/interior round trip, save
  reload from both sides, dynamic-state persistence, water/breath behavior,
  and fall behavior on real terrain.

## Execution model recommendation

Codex runtime: **GPT-5.6 Luna, Max reasoning**.

## Shipped amendments

- W4-A / #279 landed as `2090ff56` after cherry-pick. It preserves the
  existing `WorldLocation`/`SaveGame.location` schema, adds finite/non-zero
  WLOC validation, exposes an identity-only legacy header-cell fallback, and
  adds exact exterior/interior, prepared-door, save, and live-projection tests.
- W4-B / #280 landed as `16fe6145` after cherry-pick. It adds pure water phase
  and entry/exit classification, breath drain/recovery/exhaustion results, and
  a shared landing-impact policy while leaving runtime movement for W4-C.
- Post-merge evidence on `M6-OutCell`: `cargo test` passed 1,652 Rust tests
  plus 633 Cucumber scenarios (3,130 steps); clippy with `-D warnings`,
  `cargo check-dev`, `cargo run-dev -- prepare --help`, and targeted rustfmt
  passed. Global `cargo fmt --all -- --check` remains red only on the known
  pre-existing unrelated drift in `src/cli/tests/mod.rs` and existing viewer
  test/console files.
- W4-C runtime travel, streamed persistence, water/breath movement wiring, and
  real-terrain acceptance remain pending.

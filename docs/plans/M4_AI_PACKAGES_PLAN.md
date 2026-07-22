# M4 AI-packages wave — plan

## Requested
"P1 done today" — the autonomous-AI arc of M4: AI packages (#115 → #193–#198),
perception/hostility (#116), and key-aware locked doors (#185).

## Execution model (Claude runtime)
Orchestrator plans/integrates/evaluates; executors implement. Parallel isolated
worktrees for the disjoint subsystems, one coherent executor per tightly-coupled
group:
- **Foundation** #193+#194+#195 (one executor, shared types) — **Opus**.
- **Perception** #116 — **Opus**.
- **Doors** #185 — **Sonnet**.
- **Packages** #196+#197 (one executor, shared family-dispatch seam) — **Opus**,
  on a worktree off the wave branch (needs the foundation).
- **Follow/Sandbox** #198 — **Opus**, off the wave branch (needs doors + dispatch).

## Feature lists → tests → implementation
Per issue, pure std/serde policy modules with `#[cfg(test)]` + cucumber, thin
Bevy consumers. Hard invariant across all package families (verdict §2.3): the
KCC is the sole movement authority — families emit Route/Stop/Play requests and
never write `Transform.translation`.

## What landed (all green: 554 scenarios, 2731 steps)
- `src/viewer/ai/{selection,lifecycle,resolution,families,family_runtime}.rs`,
  console `showpackages`/`runpackage`.
- `crates/bevyout-core/src/{faction,disposition,perception}.rs`,
  `src/viewer/perception.rs`, console `perception`. `ACTOR_CATALOG_REVISION`
  bumped for the faction-relation table.
- `src/viewer/nav/openmw_doors/`, per-(door,actor) key resolution, `giveitem`,
  `setlock <key>`. Manifest schema 18→19 for `PreparedDoor.trapped`.

## Shipped amendments
- **#185 scope:** the executor implemented the *authoritative issue* (fetched
  from GitHub), which scopes wander-gating out (deferred) and forbids porting
  `AiAvoidDoor` — narrower than the kickoff brief. The wander-no-open-doors gate
  was then wired by **#198** (`PackageFamily::opens_doors()` / `request_door_open`).
- **Merge reconciliation:** #185's per-route lock-override rebuild
  (`apply_door_lock_overrides`) hard-required `NavArchipelagoState`, which the
  package families' shared routing helper triggered in minimal contexts —
  guarded to no-op without an archipelago (`314b76d`).
- **Deferred data (#213):** patrol-marker / editor-location subrecords are not
  decoded, so package *movement* on authored cells is gated; the engine reports
  a deterministic `unresolved_point` rather than moving. Filed as #213.

## Acceptance
Representative real-data smoke done (see `M4_AI_PACKAGES_MANUAL.md`): selection/
lifecycle/resolution and perception verified live on SuperDuperMart; doors
verified live by the #185 executor; package movement gated on #213. Exhaustive
per-family acceptance is in the manual.

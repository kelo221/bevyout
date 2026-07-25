# M5 wave 1 plan — functional 10mm pistol

## Execution model recommendation

Use **Sol X-High** in the Codex runtime. This wave crosses ESM4 preparation,
pure persistent combat state, Bevy rendering/input/audio, ray picking, console
automation, and real-data MCP acceptance; the higher reasoning setting is
warranted by the number of existing contracts that must remain aligned.

## Fixed feature list

### #235 — prepared weapon runtime data

- Decode `WEAP.WNAM` as the first-person model-object FormID.
- Decode `WEAP.SNAM` and `WEAP.XNAM` as 3D and 2D fire sounds.
- Route the WNAM-referenced model through the existing native asset pipeline.
- Add a serde-defaulted prepared weapon block to item definitions and bump
  `ITEM_CATALOG_REVISION`.
- Stage referenced fire clips through the existing prepared-audio catalog.
- Retain existing damage, clip-size, ammo FormID, and DNAM animation-type data.

### #236 — pure action and damage policy

- Add a Bevy-free `bevyout-core::weapon` module.
- Model idle, firing, and reloading states with deterministic durations.
- A fire request succeeds only from idle; reload blocks fire until complete.
- Shots carry damage/range metadata but never inspect or consume ammunition.
- Apply positive finite damage to `ActorValue::Health` by changing the
  persisted runtime mutation; set `ActorLifeState::Dead` at zero.

### #237 — first-person runtime adapter

- Add a typed `WeaponPlugin` to `ViewerPlugins`.
- Resolve the equipped weapon from `PlayerEquipment` and `PreparedItemCatalog`.
- Attach a pistol-only GLB scene to the active FPS camera, preferring the
  prepared first-person asset and falling back to the world asset.
- Use captured left-click `just_pressed` to request fire and `R`
  `just_pressed` to request reload.
- Procedurally animate recoil and reload from the pure action progress.
- Flash a camera-child point light at the muzzle for each accepted shot.
- Cast from the active camera's viewport center, excluding the viewmodel.
- Resolve hit mesh ancestry to `ActorRuntime`, then mutate canonical save state.
- Prefer the prepared 2D fire sound and fall back to the 3D sound at the muzzle.

### #238 — test and MCP surface

- Add `weaponstate`, `weaponfire`, and `weaponreload` console commands.
- Report stable state, equipped FormID, viewmodel path, action, shot count, and
  the last hit/damage result for MCP assertions.
- Prepare a real cell with the native converter, equip/add FormID `0000434F`,
  exercise fire/reload through `bevyout.console.exec`, inspect actor state, and
  capture the viewport.
- Write `M5_WAVE1_MANUAL.md` with exact commands and expected results.

## Tests-first order

1. Add `features/player_weapon.feature` with fixed action, no-ammo, and actor
   damage/death scenarios.
2. Append the feature world's fields and step definitions.
3. Add unit tests for WEAP parsing/catalog mapping and catalog revision checks.
4. Add minimal Bevy `App` tests for input/action/presentation state and console
   commands.
5. Implement until the focused feature/unit suites are green.
6. Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test`, and a representative native `prepare`.
7. Verify the real viewer through MCP/BRP and record evidence.

## Acceptance gates

- The 10mm first-person model is visible in FPS mode when equipped.
- One left click yields one accepted shot with sound, recoil, and light flash.
- A center-ray actor hit subtracts 9 health and persists in `actorstate`.
- A lethal hit marks the actor dead.
- `R` begins reload; `weaponfire` reports blocked while reload is active;
  firing succeeds after the reload duration.
- No ammo stack or count is required and no ammo state changes.
- MCP returns machine-readable command results and a non-black viewport capture.
- All repository gates pass and the wave PR targets `master`.

## Shipped amendments

### A1 — expose accepted presentation effects in weaponstate

Real-data MCP acceptance cannot sample a 50 ms light pulse reliably or listen
to the host audio device. `weaponstate` therefore also reports the prepared 2D
and 3D sound FormIDs plus the sound FormID and light-pulse duration selected
by the last accepted shot. This is observability only; the normal
`PlaySound` message and point-light path remain authoritative.

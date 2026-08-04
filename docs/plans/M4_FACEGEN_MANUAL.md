# M4 static NPC FaceGen reconstruction manual acceptance (#109)

This wave applies Fallout 3 FaceGen coefficients to the selected native head
mesh and diffuse while preserving the authored body, hair, eyes, mouth,
teeth, apparel, animation, and weapon attachment paths.

Prerequisite: the configured Fallout 3 data root must be available. All
prepared outputs and screenshots remain under the ignored `.bevyout/` cache.

1. Prepare Super-Duper Mart twice:

   ```powershell
   cargo run-dev -- prepare 00017f37
   cargo run-dev -- prepare 00017f37
   ```

   Expected: the first command completes `SuperDuperMart (00017f37)` and the
   second reports `asset cache: reused 605, missing 0` with zero scheduled
   NIF-to-GLB conversion jobs. The scene contains the authored FaceGen
   descriptors and target GLBs reference `__bevyout_facegen/*.png`.

2. Launch the prepared cell with the bridge:

   ```powershell
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00017f37/scene.ron --agent-bridge --agent-port 15702
   ```

   Expected: the viewer starts on `SuperDuperMart` and the bridge responds on
   `http://127.0.0.1:15702/` to `bevyout.session`.

3. Inspect the enabled female raider and request idle animation:

   ```text
   actorinspect 00041600
   actoranim 00041600 idle
   cam follow 00041600 4 1.4
   screenshot facegen_super_final
   ```

   Expected: `actorinspect` reports `tier=AuthoredExact`,
   `facegen.status=Applied`, geometry and texture `Applied`, empty FaceGen
   diagnostics, animation present, and a prepared/runtime right-hand weapon
   attachment. `actoranim` accepts the idle request. The screenshot shows the
   complete raider body/head/hair/apparel assembly without a neck gap or black
   normal seam and is written to `.bevyout/screenshots/`.

4. Check the requested male raider reference and record its authored data
   state:

   ```text
   actorinspect 00041610
   ```

   Expected for the vanilla cell: `reference_not_found`, because `00041610`
   is an initially disabled enable-parent reference and is not instantiated in
   the live placement set. The prepared scene still contains the enabled
   descendant assemblies and their FaceGen outputs; use the Megaton male
   mercenary in step 8 for the live male gate.

5. Stop the viewer with Ctrl+C, then prepare Megaton twice:

   ```powershell
   cargo run-dev -- prepare 00003a2a
   cargo run-dev -- prepare 00003a2a
   ```

   Expected: the first command completes `MegatonCratersideSupply
   (00003a2a)` and the second reports `asset cache: reused 507, missing 0`
   with zero scheduled NIF-to-GLB conversion jobs.

6. Launch Megaton with a different bridge port:

   ```powershell
   cargo run-dev -- view --manifest .bevyout/cache/scenes/00003a2a/scene.ron --agent-bridge --agent-port 15703
   ```

   Expected: the bridge session reports `MegatonCratersideSupply` and 269
   prepared placements.

7. Inspect Moira and request idle animation:

   ```text
   actorinspect 0002d2bc
   actoranim 0002d2bc idle
   cam follow 0002d2bc 4 1.4
   screenshot facegen_megaton_moira
   ```

   Expected: Moira reports `AuthoredExact`, FaceGen geometry/texture
   `Applied`, empty FaceGen diagnostics, animation present, and no weapon
   attachment. The screenshot shows the assembled female body, green hair,
   eyes, mouth/teeth, and apparel without a neck gap or normal seam.

8. Inspect the live male mercenary and request idle animation:

   ```text
   actorinspect 0001ff18
   actoranim 0001ff18 idle
   cam follow 0001ff18 4 1.4
   screenshot facegen_megaton_mercenary
   ```

   Expected: the mercenary reports `AuthoredExact`, FaceGen geometry/texture
   `Applied`, empty FaceGen diagnostics, animation present, and a runtime
   right-hand weapon attachment. The screenshot shows the complete male
   assembly and apparel. Stop the viewer with Ctrl+C after inspection.

9. Optional prepared-output audit:

   ```powershell
   rg -a -l "__bevyout_facegen" .bevyout/cache/assets --glob '*.glb'
   ```

   Expected: target actor GLBs are listed. Their embedded names include the
   generated FaceGen diffuse plus the unchanged body, hair, eye, mouth,
   teeth, apparel, and normal-map inputs.

Feature: PyNifly actor conversion contract
  Actor conversion keeps the authored skeleton separate from the deterministic
  visual input list so PyNifly can use it as the reference rig while importing
  every body, head, and worn apparel mesh in one batch. Inventory weapons are
  not baked into the body because runtime equipment owns those attachments.

  Scenario: The skeleton remains the reference when visual paths are sorted
    Given actor skeleton "meshes/characters/_male/skeleton.nif"
    And actor visual inputs "meshes/characters/_male/head.nif,meshes/characters/_male/upperbody.nif,meshes/characters/_male/head.nif"
    When the actor conversion inputs are canonicalized
    Then the actor reference skeleton is "meshes/characters/_male/skeleton.nif"
    And the actor visual inputs are "meshes/characters/_male/skeleton.nif,meshes/characters/_male/head.nif,meshes/characters/_male/upperbody.nif"
    And the actor converter profile is "pynifly-v28-actor-bindpose-v17"

  Scenario: Inventory weapons are not baked into the actor body
    Given actor gear record kinds "ARMO,WEAP,ARMO"
    When actor visual gear is selected
    Then the retained actor gear record kinds are "ARMO,ARMO"

  Scenario: Worn apparel uses the actor sex model rather than its world model
    Given apparel 0x100 has male worn "armor/m/outfit.nif" female worn "armor/f/outfit.nif" male world "armor/m/go.nif" female world "armor/f/go.nif" mask 0x4 rating 12 max condition 100 current condition full value 25
    When spawn apparel is selected for a female actor
    Then worn apparel models are "armor/f/outfit.nif"
    And occupied actor biped slots are 0x4

  Scenario: Effective armor rating includes current condition
    Given apparel 0x100 has male worn "armor/m/strong.nif" female worn "armor/f/strong.nif" male world "armor/m/strong-go.nif" female world "armor/f/strong-go.nif" mask 0x4 rating 20 max condition 100 current condition 40 value 50
    And apparel 0x101 has male worn "armor/m/reliable.nif" female worn "armor/f/reliable.nif" male world "armor/m/reliable-go.nif" female world "armor/f/reliable-go.nif" mask 0x4 rating 10 max condition 100 current condition full value 10
    When spawn apparel is selected for a male actor
    Then worn apparel models are "armor/m/reliable.nif"

  Scenario: Compatible apparel coexists while lower-rated conflicts are rejected
    Given apparel 0x100 has male worn "armor/m/body.nif" female worn "armor/f/body.nif" male world "armor/m/body-go.nif" female world "armor/f/body-go.nif" mask 0x4 rating 15 max condition 100 current condition full value 20
    And apparel 0x101 has male worn "armor/m/clothes.nif" female worn "armor/f/clothes.nif" male world "armor/m/clothes-go.nif" female world "armor/f/clothes-go.nif" mask 0x4 rating 5 max condition 100 current condition full value 100
    And apparel 0x102 has male worn "armor/m/helmet.nif" female worn "armor/f/helmet.nif" male world "armor/m/helmet-go.nif" female world "armor/f/helmet-go.nif" mask 0x1 rating 2 max condition 50 current condition full value 5
    When spawn apparel is selected for a male actor
    Then worn apparel models are "armor/m/body.nif,armor/m/helmet.nif"
    And occupied actor biped slots are 0x5
    And race body part 0 is hidden by the outfit
    And race body part 1 is visible under the outfit

  Scenario: Missing worn apparel leaves the underwear body visible
    Given apparel 0x100 has male worn "armor/m/missing.nif" female worn "armor/f/missing.nif" male world "armor/m/go.nif" female world "armor/f/go.nif" mask 0x4 rating 20 max condition 100 current condition full value 20
    And worn model "armor/f/missing.nif" is unavailable
    When spawn apparel is selected for a female actor
    Then worn apparel models are ""
    And race body part 0 is visible under the outfit

  Scenario: Only editor-visible dismemberment partitions survive intact conversion
    Then actor partition flags 0x1 are visible
    And actor partition flags 0x0 are hidden

  Scenario: Authored ragdoll frames preserve distinct swing and twist limits
    Given an authored spherical actor joint with cone 0.785398 plane -0.879646 to 0.523597 twist -0.174533 to 0.174533 strength 0.9
    Then the actor physics sidecar schema is 3
    And the actor joint has complete local frames
    And the actor joint keeps plane -0.879646 to 0.523597 separate from twist -0.174533 to 0.174533
    And the actor joint source is "Authored"

  Scenario: Synthetic human joints remain explicit per-edge fallbacks
    Given a synthetic fallback actor joint
    Then the actor joint source is "SyntheticFallback"

  Scenario: Authored ragdoll endpoints do not depend on Blender import order
    Then Blender ragdoll bodies and constraints use stable NIF source identities

  Scenario: Duplicate ragdoll body IDs are rejected
    Given an actor physics sidecar with duplicate body group IDs
    Then actor physics sidecar validation rejects duplicate body group IDs

  Scenario: Skin weights remain attached to the authored ragdoll
    Then non-ragdoll actor skin weights collapse to their nearest authored body ancestor

  Scenario: A model-only creature still has a reference and visual input
    Given actor skeleton ""
    And actor visual inputs "meshes/creatures/protectron/protectron.nif"
    When the actor conversion inputs are canonicalized
    Then the actor reference skeleton is "meshes/creatures/protectron/protectron.nif"
    And the actor visual inputs are "meshes/creatures/protectron/protectron.nif"

  Scenario: A creature main model is preferred over sorted attachments
    Given actor skeleton ""
    And actor visual inputs "meshes/creatures/protectron/1hprhandlaser.nif,meshes/creatures/protectron/protectron.nif,meshes/creatures/protectron/blowawaydome.nif"
    When the actor conversion inputs are canonicalized
    Then the actor reference skeleton is "meshes/creatures/protectron/protectron.nif"
    And the actor visual inputs are "meshes/creatures/protectron/protectron.nif,meshes/creatures/protectron/1hprhandlaser.nif,meshes/creatures/protectron/blowawaydome.nif"

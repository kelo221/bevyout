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
    And the actor converter profile is "pynifly-v28-actor-bindpose-v10"

  Scenario: Inventory weapons are not baked into the actor body
    Given actor gear record kinds "ARMO,WEAP,ARMO"
    When actor visual gear is selected
    Then the retained actor gear record kinds are "ARMO,ARMO"

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

Feature: Explicit actor animation compatibility conversion
  External KF clip packs are a compatibility artifact independent of the
  production scene converter. Ordinary native preparation never requires
  Blender, while an explicit clip-pack request retains native scene assets.

  Scenario: Ordinary preparation keeps both native conversion and actor animation conversion disabled
    Given no scene converter is requested for actor animation preparation
    And no actor animation converter is requested
    When the actor animation converter selections are resolved
    Then the selected scene converter is "native"
    And the selected actor animation converter is "disabled"
    And actor animation preparation does not require Blender

  Scenario: Native clip packs use Nifty without resolving Blender
    Given no scene converter is requested for actor animation preparation
    And the "native" actor animation converter is requested
    When the actor animation converter selections are resolved
    Then the selected scene converter is "native"
    And the selected actor animation converter is "native"
    And actor animation preparation does not require Blender

  Scenario: A validated warm clip pack is reused
    Given an actor animation clip pack has an output and report that both validate
    And actor animation clip-pack rebuild is not requested
    When the actor animation clip-pack cache decision is made
    Then the actor animation clip-pack cache decision is "reuse"

  Scenario: An explicit rebuild bypasses a validated warm clip pack
    Given an actor animation clip pack has an output and report that both validate
    And actor animation clip-pack rebuild is requested
    When the actor animation clip-pack cache decision is made
    Then the actor animation clip-pack cache decision is "build"

  Scenario: Existing IDLE-referenced KFs share one converted clip pack
    Given an animation set contains one KF "meshes/characters/_male/idleanims/swatting.kf"
    And an authored IDLE references the existing KF "meshes/characters/_male/idleanims/swatting.kf"
    When authored IDLE conversion is staged
    Then authored IDLE conversion uses 1 animation set and 1 clip
    And authored IDLE conversion invokes no duplicate pack job

  Scenario: Authored IDLE catalog revision is pinned to v4
    Given the authored IDLE catalog revision is inspected
    Then the authored IDLE catalog revision is "actor-animations-v4-authored-idle-definitions"

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

  Scenario: Blender clip packs do not replace native scene conversion
    Given no scene converter is requested for actor animation preparation
    And the "blender" actor animation converter is requested
    When the actor animation converter selections are resolved
    Then the selected scene converter is "native"
    And the selected actor animation converter is "blender"
    And actor animation preparation requires Blender

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

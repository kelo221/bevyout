Feature: Actor assembly planning
  Actor presentation consumes one deterministic, engine-independent blueprint
  for body parts, optional appearance, equipment, and root scale.

  Scenario: Humanoid head hair and eyes keep stable roles and head attachments
    Given actor mesh part Eyes index 0 form 0x30 model "characters/eyes/blue.nif"
    And actor mesh part Head index 0 form 0x10 model "characters/head/human.nif"
    And actor mesh part Hair index 0 form 0x20 model "characters/hair/messy.nif"
    When actor mesh parts are canonicalized
    Then actor mesh roles are "Head(0),Hair,Eyes"
    And every actor mesh part attaches to Head

  Scenario: Hair and hat apparel slots hide hair without hiding eyes
    Given occupied actor apparel slots 0x00000402
    When actor optional-part visibility is evaluated
    Then actor hair is hidden
    And actor eyes are visible

  Scenario: Starting weapon choice is deterministic and remains a right-hand decision
    Given actor weapon 0x100 model "weapons/pistol.nif" damage 12 value 30 available yes
    And actor weapon 0x101 model "weapons/rifle.nif" damage 20 value 25 available no
    And actor weapon 0x102 model "weapons/other.nif" damage 20 value 25 available yes
    When the actor starting weapon is selected
    Then actor weapon 0x101 is selected at RightHand
    And the selected actor weapon model is unavailable

  Scenario: Humanoid root scale combines reference race and actor height
    Given humanoid scale reference 1.2 race 0.95 actor 1.1
    When actor root scale is resolved
    Then actor root scale is 1.254

  Scenario: Creature root scale bypasses humanoid race and sex inputs
    Given creature scale reference 0.8 base 1.25
    When actor root scale is resolved
    Then actor root scale is 1.0

  Scenario: Invalid scale components fall back independently to one
    Given humanoid scale reference 0.0 race -1.0 actor NaN
    When actor root scale is resolved
    Then actor root scale is 1.0

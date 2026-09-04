Feature: Semantic body parts, limb health, crippling, and medical restoration
  Six anatomical pools live in core. A measured body-part hit mutates
  canonical limb state exactly once, derives penalties exactly once, and
  projects into locomotion, Perception, and weapon math. Unknown geometry
  falls back to torso. Duplicate ShotId evidence is rejected. Stimpak,
  doctor, and owned-bed restoration share one policy; owned-bed activation
  waits for the wave-9 clock.

  Scenario: All six body parts start healthy
    Given a healthy limb state
    Then the limb state has 6 parts
    And the head limb is 100000 milli and not crippled
    And the torso limb is 100000 milli and not crippled
    And the left arm limb is 100000 milli and not crippled
    And the right arm limb is 100000 milli and not crippled
    And the left leg limb is 100000 milli and not crippled
    And the right leg limb is 100000 milli and not crippled
    And the locomotion speed is 10000 basis points

  Scenario: Unknown geometry falls back to torso
    Given a healthy limb state
    When an unmarked node is mapped to a body part
    Then the mapped body part is torso
    When the unknown node "widget_mesh_02" is mapped to a body part
    Then the mapped body part is torso

  Scenario: Each named node maps onto a stable body part
    When the unknown node "Bip01 Head" is mapped to a body part
    Then the mapped body part is head
    When the unknown node "Bip01 Spine2" is mapped to a body part
    Then the mapped body part is torso
    When the unknown node "Bip01 L UpperArm" is mapped to a body part
    Then the mapped body part is left arm
    When the unknown node "Bip01 R Forearm" is mapped to a body part
    Then the mapped body part is right arm
    When the unknown node "Bip01 L Calf" is mapped to a body part
    Then the mapped body part is left leg
    When the unknown node "Bip01 R Foot" is mapped to a body part
    Then the mapped body part is right leg

  Scenario: Crossing the cripple threshold emits one transition
    Given a healthy limb state
    When limb impact shot 1 hits the left leg for 100000 milli
    Then the left leg limb is 0 milli and crippled
    And the last limb impact newly crippled
    When limb impact shot 2 hits the left leg for 50000 milli
    Then the left leg limb is 0 milli and crippled
    And the last limb impact did not newly cripple

  Scenario: One crippled leg projects 60 percent movement
    Given a healthy limb state
    When limb impact shot 1 hits the left leg for 100000 milli
    Then the locomotion speed is 6000 basis points

  Scenario: Two crippled legs project 40 percent movement
    Given a healthy limb state
    When limb impact shot 1 hits the left leg for 100000 milli
    And limb impact shot 2 hits the right leg for 100000 milli
    Then the locomotion speed is 4000 basis points

  Scenario: Arm cripple projects reload and spread penalties
    Given a healthy limb state
    Then the arm reload multiplier is 10000 basis points
    And the arm spread penalty is 0 basis points
    When limb impact shot 1 hits the right arm for 100000 milli
    Then the arm reload multiplier is 15000 basis points
    And the arm spread penalty is 2500 basis points
    When limb impact shot 2 hits the left arm for 100000 milli
    Then the arm reload multiplier is 20000 basis points
    And the arm spread penalty is 5000 basis points

  Scenario: A crippled head applies a Perception penalty
    Given a player sheet with all SPECIAL at 5 and luck at 5
    And a healthy limb state
    When effective SPECIAL is projected with limbs
    Then effective perception is 5
    When limb impact shot 1 hits the head for 100000 milli
    And effective SPECIAL is projected with limbs
    Then effective perception is 1
    And the last limb impact requested head blur

  Scenario: Duplicate ShotId does not apply damage twice
    Given a healthy actor with 80 health
    When weapon impact shot 7 hits the torso at 2 meters
    Then the actor remaining health is 70
    And the torso limb is 90000 milli and not crippled
    When weapon impact shot 7 hits the torso at 2 meters
    Then the impact was duplicate
    And the actor remaining health is 70
    And the torso limb is 90000 milli and not crippled

  Scenario: A targeted Stimpak restores the selected limb
    Given a healthy limb state
    When limb impact shot 1 hits the left arm for 80000 milli
    Then the left arm limb is 20000 milli and not crippled
    When a targeted stimpak restores the left arm at game time 0
    Then the left arm limb is 50000 milli and not crippled
    And the last restoration consumed a targeted stimpak

  Scenario: Doctor healing uses the same restoration policy
    Given a healthy limb state
    When limb impact shot 1 hits the head for 100000 milli
    And a doctor restores all limbs at game time 0
    Then the head limb is 100000 milli and not crippled
    And the last restoration used a doctor

  Scenario: Owned-bed healing is a core policy on explicit GameTime
    Given a healthy limb state
    When limb impact shot 1 hits the right leg for 100000 milli
    And an owned bed restores all limbs at game time 3600000
    Then the right leg limb is 100000 milli and not crippled
    And the last restoration used an owned bed at game time 3600000

  Scenario: Limb state round-trips through serialization
    Given a healthy limb state
    When limb impact shot 1 hits the torso for 25000 milli
    And the limb state is serialized and restored
    Then the torso limb is 75000 milli and not crippled
    And the head limb is 100000 milli and not crippled

Feature: Actor animation in normal game flow
  Prepared actor clip packs are selected by authored Fallout semantics and
  driven through a deterministic presentation state machine outside the zoo.

  Scenario Outline: FO3 weapon animation types select the authored clip prefix
    Given FO3 weapon animation type <value>
    When the actor weapon animation prefix is resolved
    Then the actor weapon animation prefix is "<prefix>"

    Examples:
      | value | prefix |
      | 0     | h2h    |
      | 1     | 1hm    |
      | 2     | 2hm    |
      | 3     | 1hp    |
      | 4     | 1hp    |
      | 5     | 2hr    |
      | 6     | 2ha    |
      | 7     | 2hr    |
      | 8     | 2hh    |
      | 9     | 2hl    |
      | 10    | 1gt    |
      | 11    | 1lm    |
      | 12    | 1md    |

  Scenario: Unknown FO3 weapon animation types stay explicit
    Given FO3 weapon animation type 99
    When the actor weapon animation prefix is resolved
    Then the actor weapon animation prefix is absent

  Scenario: Male locomotion selects the adult male source rather than a colliding child clip
    Given a male humanoid actor animation set with ready clips "mtforward@meshes/characters/_male/locomotion/child/mtforward.kf,mtforward__2@meshes/characters/_male/locomotion/female/mtforward.kf,mtforward__3@meshes/characters/_male/locomotion/male/mtforward.kf,mtidle@meshes/characters/_male/locomotion/mtidle.kf"
    And the actor requests animation state "walk"
    When the gameplay actor clip is resolved
    Then the gameplay actor clip is "mtforward__3"
    And the gameplay actor clip source is "meshes/characters/_male/locomotion/male/mtforward.kf"

  Scenario: Female locomotion selects the adult female source
    Given a female humanoid actor animation set with ready clips "mtforward@meshes/characters/_male/locomotion/child/mtforward.kf,mtforward__2@meshes/characters/_male/locomotion/female/mtforward.kf,mtforward__3@meshes/characters/_male/locomotion/male/mtforward.kf,mtidle@meshes/characters/_male/locomotion/mtidle.kf"
    And the actor requests animation state "walk"
    When the gameplay actor clip is resolved
    Then the gameplay actor clip is "mtforward__2"

  Scenario: A creature resolves locomotion only inside its mapped set
    Given a creature actor animation set with ready clips "mtidle@meshes/creatures/radroach/mtidle.kf,mtfastforward@meshes/creatures/radroach/mtfastforward.kf"
    And the actor requests animation state "run"
    When the gameplay actor clip is resolved
    Then the gameplay actor clip is "mtfastforward"

  Scenario: A missing locomotion clip falls back to idle with a diagnostic
    Given a male humanoid actor animation set with ready clips "mtidle@meshes/characters/_male/locomotion/mtidle.kf"
    And the actor requests animation state "run"
    When the gameplay actor clip is resolved
    Then the gameplay actor clip is "mtidle"
    And the gameplay actor clip resolution reports fallback from "run"

  Scenario: An authored weapon prefix selects its equip clip
    Given a male humanoid actor animation set with ready clips "1hmequip@meshes/characters/_male/1hmequip.kf,1hpequip@meshes/characters/_male/1hpequip.kf,mtidle@meshes/characters/_male/locomotion/mtidle.kf"
    And the actor uses weapon animation prefix "1hp"
    And the actor requests animation state "equip"
    When the gameplay actor clip is resolved
    Then the gameplay actor clip is "1hpequip"

  Scenario: One-shot equipment animation completion returns to idle
    Given the gameplay actor is playing animation state "equip"
    When the gameplay actor animation finishes
    Then the next gameplay actor animation state is "idle"

  Scenario: Looping locomotion completion retains the requested state
    Given the gameplay actor is playing animation state "walk"
    When the gameplay actor animation finishes
    Then the next gameplay actor animation state is "walk"

  Scenario: Hidden resident-cell actors remain paused
    Given a gameplay actor belongs to an inactive resident cell
    When gameplay actor activity is resolved
    Then gameplay actor playback is paused

  Scenario: Disabled actors in the active cell remain paused
    Given a gameplay actor belongs to the active resident cell
    And the gameplay actor is disabled
    When gameplay actor activity is resolved
    Then gameplay actor playback is paused

  Scenario: Visible actors in the active cell advance
    Given a gameplay actor belongs to the active resident cell
    And the gameplay actor is visible
    When gameplay actor activity is resolved
    Then gameplay actor playback advances

  Scenario: Special idle lifecycle gate rejects unsafe actors
    Given a stationary alive loaded actor with no equipment transition
    And the actor lifecycle is moving, dead, ragdolled, unloaded, or equipment-transitioning
    When special idle eligibility is evaluated
    Then special idle selection is rejected with reason "invalid_lifecycle"

  Scenario: Package no-idle flag disables all authored special idles
    Given an active package with No idle anims and a global authored idle
    When automatic special idle selection is evaluated
    Then special idle selection is rejected with reason "no_idle_anims"

  Scenario: A package idle collection overrides the global tree
    Given an active package idle collection "0x00000021" and a global authored idle "0x00000031"
    When automatic special idle selection is evaluated
    Then the selected special idle is "0x00000021" from source "package"

  Scenario: Sequence and do-once package idles advance deterministically
    Given a sequence do-once package idle collection "0x00000021,0x00000022"
    When the package idle collection is evaluated twice
    Then the selected special idles are "0x00000021,0x00000022"
    And the package idle collection is exhausted

  Scenario: Random package idles use actor and epoch deterministic rolls
    Given a random package idle collection "0x00000021,0x00000022,0x00000023"
    When the same package idle selection is evaluated twice with the same epoch
    Then the random package idle selections match

  Scenario: Package idle timers use seconds from stationary package entry
    Given an active package idle timer of 3 seconds
    When package idle selection is evaluated at 2 seconds
    Then special idle selection is rejected with reason "package_timer"
    When package idle selection is evaluated at 3 seconds
    Then package idle selection is eligible

  Scenario: Global idle traversal uses parent and authored sibling order
    Given a global authored idle tree with parent conditions and siblings "0x00000031,0x00000032"
    When global special idle selection is evaluated
    Then the selected special idle is "0x00000032" from source "idle_manager"

  Scenario: Unsupported idle conditions have a stable rejection
    Given a global authored idle with an unsupported CTDA condition
    When global special idle selection is evaluated
    Then special idle selection is rejected with reason "unsupported_condition"

  Scenario: Replay cooldown blocks immediate replay
    Given a global authored idle with a 5 second replay delay
    When global special idle selection is evaluated twice immediately
    Then the second special idle selection is rejected with reason "replay_cooldown"

  Scenario: Loop bounds are inclusive and deterministic
    Given an authored special idle with loop bounds 2 and 4
    When the special idle loop count is selected
    Then the loop count is between 2 and 4 inclusive

  Scenario: Only Special Idle and Whole Body groups are supported
    Given an authored idle with group section 8
    When forced special idle validation is evaluated
    Then special idle selection is rejected with reason "unsupported_group"

  Scenario: Forced play bypasses conditions and cooldown only
    Given a forced authored idle with a false condition and active cooldown
    When forced special idle selection is evaluated
    Then the selected special idle source is "forced"

  Scenario: Special completion and movement interruption return to base locomotion
    Given a special idle is playing for a stationary actor
    When special idle playback completes or movement begins
    Then base locomotion resumes without a static state

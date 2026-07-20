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

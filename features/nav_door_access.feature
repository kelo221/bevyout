Feature: Key-aware locked doors and the trapped-door barrier
  # Issue #185, following up on #177 (doors as real blockers with an
  # approach-and-open agent lifecycle). Closes the remaining
  # `MWMechanics::AiPackage::openDoors()` awareness gaps: OpenMW's actors try
  # their own inventory's key against a locked door before giving up, and
  # never open a trapped one at all. `viewer::nav::openmw_doors::
  # door_openable` is the pure port of that decision table; `nav/agent.rs`
  # feeds it the door's prepared lock/key/trap data plus whether the
  # specific routing agent's own canonical inventory holds the key, and
  # folds the verdict into the existing `locked` boolean every downstream
  # consumer (`repath::door_usable`, `door_link::crossing_gate`,
  # `door_link::effective_door_open`) already understands -- see
  # `nav/agent.rs`'s "Doors as conditional route topology" module doc
  # section for how #155's per-agent route-cost overrides and this issue's
  # per-agent key check meet.

  Scenario: An untrapped, unlocked door is always openable
    Given a door with lock level none, key none, and untrapped
    Then the door is openable

  Scenario Outline: A locked door is openable only by an actor holding its key
    Given a door with lock level 25, key 4660, and untrapped
    And the actor <possession> the door's key
    Then the door is <verdict>

    Examples:
      | possession    | verdict      |
      | holds         | openable     |
      | does not hold | not openable |

  Scenario: A locked door with no assigned key never opens for anyone
    # OpenMW: `if (keyId.empty()) return;` -- there is no key to search
    # for, so no actor, however keyed up otherwise, can force it.
    Given a door with lock level 25, key none, and untrapped
    And the actor holds the door's key
    Then the door is not openable

  Scenario: A trapped door never opens, even for the key holder
    Given a door with lock level 25, key 4660, and trapped
    And the actor holds the door's key
    Then the door is not openable

  Scenario: A trapped but unlocked door still never opens
    Given a door with lock level none, key none, and trapped
    Then the door is not openable

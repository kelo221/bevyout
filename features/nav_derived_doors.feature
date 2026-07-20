Feature: Collision-derived door to nav-polygon associations
  # Issue #177 (M4 wave 11). The authored Bethesda NAVM only associates
  # *load/travel* doors with triangles (`NVDP`): measured on the prepared
  # graph for cell 0x00024512, exactly three doors carry an association and
  # all three are travel doors. An ordinary in-cell door (`VDoor01`) and the
  # vault gear door (`VaultGearDoor`, an `ACTI`) have none at all, so they
  # get no polygon typing, no crossing gate and no door link -- the navmesh
  # runs straight through the closed slab, the agent walks in, and physics
  # wedges it.
  #
  # This feature drives the pure prepare-side derivation that recovers the
  # missing input: `vsa::prepare::nav_doors::derive_door_associations`, which
  # associates walkable polygons with a blocking placement's own collision
  # footprint, and `unreported_interior_polygons`, the deterministic
  # invariant check that no walkable polygon is left routable inside a closed
  # blocker.
  #
  # The two association classes mean different things at query time, so they
  # are reported distinctly. A *gate* polygon merely overlaps the footprint
  # (the doorway crossing itself) and stays routable, so the existing
  # crossing gate can fire on it -- pause, request open, wait, traverse,
  # resume. A *blocking* polygon lies wholly inside the solid, where there is
  # no legitimate way to stand while the blocker is shut, so the runtime
  # prices it impassable until it opens. The live query-time cost override
  # this feeds is not expressible in this suite's Bevy-App-free harness and
  # is covered by `nav/agent.rs`'s own `#[cfg(test)]` unit tests instead --
  # the same split `nav_door_topology.feature` documents.

  Scenario: A polygon crossing a door's footprint becomes a gate association
    Given a blocker 0x00000099 with footprint from -0.1, 0.2 to 0.1, 0.6 spanning height 0 to 2
    And nav mesh 0x00000010 has walkable polygon 7 with vertices -1, 0, 0 and 1, 0, 0 and 0, 0, 1
    When the derived door associations are resolved
    Then there are exactly 1 derived door associations
    And polygon 7 is a gate association for blocker 0x00000099

  Scenario: A polygon wholly inside the closed door's volume blocks when closed
    Given a blocker 0x00000099 with footprint from 0, 0 to 1, 1 spanning height 0 to 2
    And nav mesh 0x00000010 has walkable polygon 3 with vertices 0.1, 0, 0.1 and 0.9, 0, 0.1 and 0.5, 0, 0.9
    When the derived door associations are resolved
    Then there are exactly 1 derived door associations
    And polygon 3 is a blocking association for blocker 0x00000099
    And polygon 3 is reported as openable

  Scenario: A blocker with no open/close controls only claims the ground inside it
    # A kinematic activator that blocks a corridor is still solid, but it has
    # no runtime open/close FSM. Making it a crossing-gate candidate would
    # park an agent in front of it forever waiting for an open that can never
    # happen, so only the ground *inside* it is claimed.
    Given a blocker 0x00000099 with footprint from 0, 0 to 1, 1 spanning height 0 to 2
    And blocker 0x00000099 has no open and close controls
    And nav mesh 0x00000010 has walkable polygon 1 with vertices 0.1, 0, 0.1 and 0.9, 0, 0.1 and 0.5, 0, 0.9
    And nav mesh 0x00000010 has walkable polygon 2 with vertices 0.5, 0, 0.5 and 3, 0, 0.5 and 3, 0, 3
    When the derived door associations are resolved
    Then there are exactly 1 derived door associations
    And polygon 1 is a blocking association for blocker 0x00000099
    # The runtime prices a non-openable blocker's interior impassable rather
    # than merely expensive: there is no crossing to wait for.
    And polygon 1 is reported as not openable

  Scenario: A polygon on another storey above the door is never associated
    Given a blocker 0x00000099 with footprint from 0, 0 to 1, 1 spanning height 0 to 2
    And nav mesh 0x00000010 has walkable polygon 1 with vertices 0.1, 8, 0.1 and 0.9, 8, 0.1 and 0.5, 8, 0.9
    When the derived door associations are resolved
    Then there are exactly 0 derived door associations

  Scenario: A polygon merely touching the footprint edge is not associated
    # The clearance pass (issue #171) clips polygon boundaries onto collider
    # faces, so exact edge contact is the common case, not an oddity.
    Given a blocker 0x00000099 with footprint from 0, 0 to 1, 1 spanning height 0 to 2
    And nav mesh 0x00000010 has walkable polygon 1 with vertices 1, 0, 0 and 3, 0, 0 and 3, 0, 2
    When the derived door associations are resolved
    Then there are exactly 0 derived door associations

  Scenario: No walkable polygon is left unreported inside a closed blocker
    # The invariant this issue is measured by, checked deterministically at
    # prepare time and reported in the `nav doors:` summary line.
    Given a blocker 0x00000099 with footprint from 0, 0 to 1, 1 spanning height 0 to 2
    And nav mesh 0x00000010 has walkable polygon 1 with vertices 0.1, 0, 0.1 and 0.9, 0, 0.1 and 0.5, 0, 0.9
    And nav mesh 0x00000010 has walkable polygon 2 with vertices 0.2, 0, 0.2 and 0.4, 0, 0.2 and 0.3, 0, 0.4
    And nav mesh 0x00000010 has walkable polygon 3 with vertices 5, 0, 5 and 6, 0, 5 and 5.5, 0, 6
    When the derived door associations are resolved
    Then there are exactly 2 blocking door associations
    And no walkable polygon is left unreported inside a closed blocker

  Scenario: Associations are deterministic and ordered by blocker then polygon
    Given a blocker 0x00000040 with footprint from 0, 0 to 1, 1 spanning height 0 to 2
    And a blocker 0x00000020 with footprint from 0, 0 to 1, 1 spanning height 0 to 2
    And nav mesh 0x00000010 has walkable polygon 5 with vertices 0.1, 0, 0.1 and 0.9, 0, 0.1 and 0.5, 0, 0.9
    And nav mesh 0x00000010 has walkable polygon 2 with vertices 0.2, 0, 0.2 and 0.8, 0, 0.2 and 0.5, 0, 0.8
    When the derived door associations are resolved
    Then the derived association order is 0x00000020/2, 0x00000020/5, 0x00000040/2, 0x00000040/5
    And resolving the derived door associations again gives the same result

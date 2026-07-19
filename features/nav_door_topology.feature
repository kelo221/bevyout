Feature: Doors as conditional route topology - polygon typing and corridor gating
  # Issue #155 (M4 wave 8), after #154's cross-mesh portals landed on the
  # same runtime seam. Two real defects in the pre-#155 mid-route door
  # gate (issue #137): a door triangle was gated by proximity to its
  # centroid (`MID_ROUTE_DOOR_GATE_DISTANCE`), which could fire for a
  # route that merely passed *near* a doorway without its corridor ever
  # actually crossing it; and a locked door removed nothing from the
  # `landmass` search, so the solver could never discover an alternate
  # route -- a query whose only route crossed a locked door would walk the
  # agent all the way there before failing.
  #
  # This feature drives the two pure decision layers #155 adds:
  # `viewer::nav::landmass_graph::door_type_indices` (feature 1: every
  # door FormID resolves to a stable, deterministic `landmass` polygon
  # type index, shared across every mesh that references it) and
  # `viewer::nav::landmass_graph::point_in_door_triangle` (feature 3: the
  # corridor-based containment test that replaced the centroid-proximity
  # scan). The live query-time lock exclusion this typing feeds
  # (`AgentTypeIndexCostOverrides` against a real `Archipelago3d` solve,
  # feature 2) and the `resolve_status` failure-status fix (feature 4) are
  # not expressible through this suite's pure, Bevy-App-free harness (see
  # `viewer::nav::landmass_graph`'s own module doc comment for why this
  # module stays free of `bevy::app`/`Landmass3dPlugin`); they are covered
  # by `nav/agent.rs`'s own `#[cfg(test)]` unit tests instead (the
  # established split every other live-Archipelago behaviour in this nav
  # slice already uses -- e.g. `nav_overlay.rs`'s and `nav/agent.rs`'s own
  # minimal-App harnesses).

  Scenario: Each distinct door gets its own type index, ascending by FormID
    Given landmass mesh 0x00000010 has a door 0x00000099 at polygon 0
    And landmass mesh 0x00000020 has a door 0x00000050 at polygon 0
    When the door type indices are resolved
    Then door 0x00000050 has type index 1
    And door 0x00000099 has type index 2

  Scenario: The same door referenced from two meshes shares one type index
    # The two-sided `DoorLinkDescriptor` shape: the same door FormID
    # appears in both meshes' door lists. `nav/agent.rs`'s per-agent lock
    # override is keyed by type index across the whole archipelago, so
    # both triangles must resolve to the *same* index or a lock would only
    # ever exclude one side of the doorway.
    Given landmass mesh 0x00000010 has a door 0x00000099 at polygon 0
    And landmass mesh 0x00000020 has a door 0x00000099 at polygon 0
    When the door type indices are resolved
    Then door 0x00000099 has type index 1
    And there is exactly 1 resolved door type index

  Scenario: A door triangle inside the old proximity radius but outside its own footprint does not gate
    # The exact defect issue #155 fixes: a route whose corridor passes
    # within the old `MID_ROUTE_DOOR_GATE_DISTANCE` (0.75 m) of a door
    # triangle's centroid, without the corridor ever entering the
    # triangle's own footprint, must not gate.
    Given a door triangle with vertices 4, 0, -1 and 6, 0, -1 and 5, 0, 1
    And a query point at 5, 0, -1.05
    Then the query point is within 0.75 metres of the door triangle's centroid
    And the query point is outside the door triangle

  Scenario: A point inside the door triangle's footprint gates
    Given a door triangle with vertices 4, 0, -1 and 6, 0, -1 and 5, 0, 1
    And a query point at 5, 0, 0
    Then the query point is inside the door triangle

  Scenario: The door triangle's own vertical gap tolerates the agent capsule-centre offset
    # Mirrors `movement_policy::nav_point_reached`'s tolerance: a
    # capsule-centre agent above a feet-level door triangle must still be
    # contained.
    Given a door triangle with vertices 4, 0, -1 and 6, 0, -1 and 5, 0, 1
    And a query point at 5, 0.9, 0
    Then the query point is inside the door triangle

  Scenario: A point on a different floor is not contained despite matching horizontal position
    Given a door triangle with vertices 4, 0, -1 and 6, 0, -1 and 5, 0, 1
    And a query point at 5, 5, 0
    Then the query point is outside the door triangle

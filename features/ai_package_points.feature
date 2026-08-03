Feature: AI package editor-location and patrol marker-chain resolution
  # Issue #213: the #195 resolver (`viewer::ai::resolution`) already handles
  # NearEditorLocation (PLDT type 3, Sandbox) and NearLinkedReference (PLDT
  # type 6, Patrol) -- it just never received the two ResolutionContext
  # inputs those types read (`actor_editor_location`/`linked_reference`),
  # because the runtime adapter built its context purely from spawned
  # entities. This feature exercises those two paths end to end through the
  # pure resolver plus the new `linked_reference_chain` marker walk, reusing
  # `ai_package_resolution.feature`'s own step vocabulary.

  Scenario: A near-editor-location package resolves to the actor's authored point
    Given the resolving actor's editor location is 12.0 0.0 34.0
    And a package location of type 3 referencing 0x00000000 radius 512
    When the package location is resolved
    Then the location resolves to 12.0 0.0 34.0
    And the location resolves via "editor-location"

  Scenario: A patrol's linked-reference chain builds ordered waypoints from the marker chain
    Given a resolvable reference 0x00041601 of base 0x00000034 at 1.0 0.0 0.0
    And reference 0x00041601 is linked to 0x00041602
    And a resolvable reference 0x00041602 of base 0x00000034 at 2.0 0.0 0.0
    And reference 0x00041602 is linked to 0x00041603
    And a resolvable reference 0x00041603 of base 0x00000034 at 3.0 0.0 0.0
    When the linked-reference chain is walked from 0x00041601
    Then the chain has 3 markers
    And chain marker 1 resolves to 1.0 0.0 0.0
    And chain marker 2 resolves to 2.0 0.0 0.0
    And chain marker 3 resolves to 3.0 0.0 0.0

  Scenario: A current-cell package editor marker is retained as a nonvisual point
    Given actor package links "00004153"
    And package 0x00004153 has a Near Reference point 0x00076f52
    And package 0x00004153 has a Specific Reference target 0x00076f52
    And a current-cell editor marker 0x00076f52 at 12.0 0.0 34.0
    And an unrelated current-cell editor marker 0x0000beef at 90.0 0.0 90.0
    When package-linked marker points are retained
    Then package marker 0x00076f52 is retained as a nonvisual point at 12.0 0.0 34.0
    And unrelated editor marker 0x0000beef is omitted

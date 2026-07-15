Feature: Authored container animation audio
  # Animated Fallout containers can author their Open/Close sounds as NIF
  # text keys instead of CONT record fields. The prepared policy fills only
  # missing container fields and preserves record-authored sounds.

  Scenario: NIF cues fill missing container sounds case-insensitively
    Given an animated container with no record open or close sound
    And its animation sound cues are "oPeN@0.20=DRSLateOpen,OPEN@0.01=DRSEarlyOpen,cLoSe@0.01=DRSClose"
    When authored container animation audio is resolved
    Then the selected open sound is "DRSEarlyOpen"
    And the selected close sound is "DRSClose"

  Scenario: Record-authored container sounds take precedence
    Given an animated container with record open sound 0x00000011 and record close sound 0x00000012
    And its resolved animation sound ids are open 0x00000021 and close 0x00000022
    When resolved container sound ids are applied
    Then the prepared open sound id is 0x00000011
    And the prepared close sound id is 0x00000012

  Scenario: Non-container placements ignore container sound fallbacks
    Given a non-container placement with no record open or close sound
    And its resolved animation sound ids are open 0x00000021 and close 0x00000022
    When resolved container sound ids are applied
    Then the prepared open sound id is absent
    And the prepared close sound id is absent

Feature: FO3 NOTE record text decoding
  # Issue #123: FO3 authors holotape/note content in NOTE.TNAM, gated by
  # NOTE.DATA's type-enum byte (0 Sound, 1 Text, 2 Image, 3 Voice -- fopdoc's
  # Fallout3 NOTE page). The importer previously read BOOK's DESC subrecord
  # for NOTE too, which NOTE never carries, so every prepared NOTE's text
  # was always None. Only type-1 ("Text") notes carry real text; other
  # types (e.g. Voice) use TNAM for a DIAL FormID reference instead and
  # must not be decoded as text.

  Scenario: A text-type NOTE's TNAM decodes into item stats
    Given a synthetic NOTE record with DATA type 1 and TNAM text "Danger: raiders ahead."
    When the NOTE record is decoded
    Then the decoded note text is "Danger: raiders ahead."

  Scenario: A voice-type NOTE's TNAM is not decoded as text
    Given a synthetic NOTE record with DATA type 3 and TNAM formid 0x00012345
    When the NOTE record is decoded
    Then the decoded note text is absent

  Scenario: A NOTE with no DATA subrecord decodes no text
    Given a synthetic NOTE record with no DATA subrecord and TNAM text "Stray text."
    When the NOTE record is decoded
    Then the decoded note text is absent

  Scenario: A decoded NOTE's text lands in the prepared item catalog stats
    Given a synthetic NOTE record with DATA type 1 and TNAM text "Meeting at midnight."
    When the NOTE record is decoded
    And the decoded base record is prepared into item catalog stats
    Then the prepared catalog note text is "Meeting at midnight."

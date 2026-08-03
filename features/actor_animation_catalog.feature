Feature: Prepared actor animation catalog
  Animation preparation discovers external KF clips deterministically, keeps
  actor-template inheritance, and reuses one skeleton-compatible clip pack
  across actors that resolve to the same source set.

  Scenario: NPC and creature KFFZ payloads decode into animation paths
    Given an NPC KFFZ payload "Idle.kf\\0Special\\Walk.kf\\0"
    And a creature KFFZ payload "Attack.kf\\0Death.kf\\0"
    When the actor animation payloads are decoded
    Then the NPC animation paths are "Idle.kf,Special/Walk.kf"
    And the creature animation paths are "Attack.kf,Death.kf"

  Scenario: Model animation template inheritance supplies the animation list
    Given actor animation source 0x00000010 uses skeleton "meshes/characters/_male/skeleton.nif" and clips "Idle.kf,Walk.kf"
    And actor animation source 0x00000020 uses skeleton "meshes/characters/_male/skeleton.nif" and clips "LocalOnly.kf"
    And actor animation source 0x00000020 inherits model animation from 0x00000010
    When actor animation sources are resolved
    Then actor animation source 0x00000020 resolves clips "Idle.kf,Walk.kf"

  Scenario: Explicit KF filenames resolve relative to the actor model directory
    Given animation actor reference 0x00000001 base 0x00000010 model "meshes/creatures/molerat/molerat.nif" skeleton "meshes/creatures/molerat/molerat.nif" explicit clips "Idle.kf,Combat/Attack.kf"
    And available KF assets "meshes/creatures/molerat/idle.kf@idle-hash,meshes/creatures/molerat/combat/attack.kf@attack-hash"
    When the prepared actor animation catalog is built
    Then animation set for reference 0x00000001 has source paths "meshes/creatures/molerat/combat/attack.kf,meshes/creatures/molerat/idle.kf"

  Scenario: Clip discovery sorts and deduplicates paths case-insensitively
    Given animation actor reference 0x00000001 base 0x00000010 model "meshes/characters/_male/skeleton.nif" skeleton "meshes/characters/_male/skeleton.nif" explicit clips "Walk.kf,idle.kf,WALK.KF"
    And available KF assets "meshes/characters/_male/walk.kf@walk-hash,meshes/characters/_male/idle.kf@idle-hash"
    When the prepared actor animation catalog is built
    Then animation set for reference 0x00000001 has source paths "meshes/characters/_male/idle.kf,meshes/characters/_male/walk.kf"
    And animation set for reference 0x00000001 has clip names "idle,walk"

  Scenario: Missing malformed and incompatible clips remain diagnostics
    Given animation actor reference 0x00000001 base 0x00000010 model "meshes/characters/_male/skeleton.nif" skeleton "meshes/characters/_male/skeleton.nif" explicit clips "Good.kf,Missing.kf,Broken.kf,WrongRig.kf"
    And available KF assets "meshes/characters/_male/good.kf@good-hash,meshes/characters/_male/broken.kf@broken-hash!malformed,meshes/characters/_male/wrongrig.kf@wrong-hash!incompatible"
    When the prepared actor animation catalog is built
    Then animation set for reference 0x00000001 contains 1 ready clip
    And animation set for reference 0x00000001 has diagnostic codes "incompatible_kf,malformed_kf,missing_kf"

  Scenario: Compatible actors reuse one animation set
    Given animation actor reference 0x00000002 base 0x00000020 model "meshes/characters/_male/skeleton.nif" skeleton "meshes/characters/_male/skeleton.nif" explicit clips "Idle.kf,Walk.kf"
    And animation actor reference 0x00000001 base 0x00000010 model "meshes/characters/_male/skeleton.nif" skeleton "meshes/characters/_male/skeleton.nif" explicit clips "walk.kf,IDLE.KF"
    And available KF assets "meshes/characters/_male/idle.kf@idle-hash,meshes/characters/_male/walk.kf@walk-hash"
    When the prepared actor animation catalog is built
    Then the prepared actor animation catalog has 2 actor mappings and 1 animation set
    And references 0x00000001 and 0x00000002 use the same animation set

  Scenario: Authored IDLE override and deletion winners are deterministic
    Given an authored IDLE winner set with override 0x00000020 and deleted 0x00000030
    When authored IDLE definitions are prepared
    Then prepared authored IDLE FormIDs are "0x00000010,0x00000020"

  Scenario: Folder IDLE roots remain valid without a KF clip
    Given an authored IDLE folder root 0x00000010 without a KF path
    When authored IDLE definitions are prepared
    Then authored IDLE 0x00000010 has no clip

  Scenario: Authored IDLE paths normalize and match existing clips
    Given animation actor reference 0x00000001 base 0x00000010 model "meshes/characters/_male/skeleton.nif" skeleton "meshes/characters/_male/skeleton.nif" explicit clips "IdleAnims/Swatting.KF"
    And available KF assets "meshes/characters/_male/idleanims/swatting.kf@swat-hash"
    And an authored IDLE 0x00000020 references KF "Characters\\_Male\\IdleAnims\\Swatting.KF"
    When the prepared actor animation catalog is built with authored IDLE definitions
    Then authored IDLE 0x00000020 resolves clip "swatting"

  Scenario: ANAM links preserve the authored IDLE tree
    Given an authored IDLE child 0x00000020 has parent 0x00000100 and previous sibling 0x00000010
    When authored IDLE definitions are prepared
    Then authored IDLE 0x00000020 keeps parent 0x00000100 and previous sibling 0x00000010

  Scenario: Authored IDLE sibling order follows previous-sibling links
    Given authored IDLE siblings are supplied in FormID order "0x00000030,0x00000010,0x00000020"
    When the authored IDLE sibling order is reconstructed
    Then authored IDLE sibling order is "0x00000010,0x00000020,0x00000030"

  Scenario: Raw IDLE group bytes map without losing authored bits
    Given authored IDLE raw group bytes "0x47,0x87,0x54"
    When authored IDLE definitions are prepared
    Then authored IDLE canonical group sections are "7,7,20"

  Scenario: Truncated IDLE DATA is diagnosed without dropping the record
    Given an authored IDLE has truncated DATA and an unknown field
    When authored IDLE definitions are prepared
    Then authored IDLE preparation has diagnostic "DATA malformed"

  Scenario: IDLE CTDA payloads remain byte-exact and stream ordered
    Given an authored IDLE has CTDA payloads "01020304,0506"
    When authored IDLE definitions are prepared
    Then authored IDLE CTDA payloads remain "01020304,0506"

  Scenario: Authored IDLE catalog ordering and hash are deterministic
    Given authored IDLE definitions have FormIDs "0x00000020,0x00000010"
    When authored IDLE definitions are prepared twice
    Then authored IDLE catalog ordering and hash match

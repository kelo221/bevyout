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

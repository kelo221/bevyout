Feature: Fallout flat-overlay lighting policy
  Flat presentation geometry must remain visible without contaminating static lighting.

  Scenario Outline: Source-authored overlays are classified narrowly
    Given a Fallout static named "<editor_id>" using model "<model>"
    When its flat-overlay policy is evaluated
    Then its overlay kind is "<kind>"

    Examples:
      | editor_id          | model                                             | kind   |
      | Stain01            | dungeons/vaultruined/accessories/stain01.nif     | decal  |
      | VaultGraffiti01    | dungeons/vault/accessories/vaultgraffiti01.nif   | decal  |
      | AssortedPapers05   | clutter/junk/assortedpapers05.nif                 | debris |
      | ShackPaperDebris01 | clutter/junk/shackpaperdebris01.nif               | debris |
      | VaultWall01        | architecture/vault/vaultwall01.nif                | none   |
      | ChainLinkFence01   | clutter/fences/chainlinkfence01.nif               | none   |
      | WastelandTree01    | landscape/trees/wastelandtree01.nif               | none   |

  Scenario: Overlay geometry is not eligible for baked static lighting
    Given a Fallout static named "Stain03" using model "dungeons/vaultruined/accessories/stain03.nif"
    When its flat-overlay policy is evaluated
    Then the placement is excluded from static lighting inputs

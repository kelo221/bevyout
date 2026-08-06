use crate::vsa::{FalloutOverlayKind, classify_fallout_overlay};

#[test]
fn classifies_reviewed_stains_graffiti_and_flat_paper_debris() {
    for (editor_id, model, expected) in [
        (
            "Stain01",
            "dungeons/vaultruined/accessories/stain01.nif",
            FalloutOverlayKind::Decal,
        ),
        (
            "VaultGraffiti01",
            "dungeons/vault/accessories/vaultgraffiti01.nif",
            FalloutOverlayKind::Decal,
        ),
        (
            "AssortedPapers05",
            "clutter/junk/assortedpapers05.nif",
            FalloutOverlayKind::Debris,
        ),
        (
            "ShackPaperDebris01",
            "clutter/junk/shackpaperdebris01.nif",
            FalloutOverlayKind::Debris,
        ),
    ] {
        assert_eq!(
            classify_fallout_overlay(Some(editor_id), Some(model)),
            expected
        );
    }
}

#[test]
fn does_not_generalize_to_other_alpha_or_world_geometry() {
    for (editor_id, model) in [
        ("VaultWall01", "architecture/vault/vaultwall01.nif"),
        ("ChainLinkFence01", "clutter/fences/chainlinkfence01.nif"),
        ("WastelandTree01", "landscape/trees/wastelandtree01.nif"),
    ] {
        assert_eq!(
            classify_fallout_overlay(Some(editor_id), Some(model)),
            FalloutOverlayKind::None
        );
    }
}

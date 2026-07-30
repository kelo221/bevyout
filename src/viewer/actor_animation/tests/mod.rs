use super::*;

#[test]
fn shortest_angle_crosses_the_wrap_boundary() {
    assert!((shortest_angle(-std::f32::consts::TAU + 0.1) - 0.1).abs() < 0.0001);
}

#[test]
fn controller_pins_are_not_required_appearance_bones() {
    assert!(!requires_appearance_binding("##VisCtrl"));
    assert!(requires_appearance_binding("Bip01 Pelvis"));
}

#[test]
fn accumulation_root_and_nonaccum_translation_remain_gameplay_owned() {
    assert!(is_accumulation_target(Some("bip01"), "bip01"));
    assert!(is_accumulation_target(Some("bip01"), "bip01 nonaccum"));
    assert!(!is_accumulation_target(Some("bip01"), "bip01 pelvis"));
    assert!(!is_accumulation_target(None, "bip01"));
}

#[test]
fn catalog_set_ownership_ends_with_cell_eviction() {
    let mut catalogs = ActorAnimationCatalogs::default();
    catalogs.insert(
        0x10,
        PreparedActorAnimationCatalog {
            animation_sets: vec![PreparedActorAnimationSet {
                id: "shared-humanoid".into(),
                ..Default::default()
            }],
            ..Default::default()
        },
    );

    assert!(catalogs.contains_set("shared-humanoid"));
    catalogs.remove(0x10);
    assert!(!catalogs.contains_set("shared-humanoid"));
}

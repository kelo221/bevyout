use super::*;
use crate::vsa::PreparedItemCategory;

#[test]
fn prepared_weapon_prefers_first_person_asset_and_keeps_action_audio() {
    let item = PreparedItemDefinition {
        base_form_id: 0x434f,
        record_kind: "WEAP".into(),
        category: PreparedItemCategory::Weapons,
        editor_id: Some("Weap10mmPistol".into()),
        display_name: Some("10mm Pistol".into()),
        source_model_path: None,
        icon_asset_path: None,
        world_asset_path: Some("assets/world.glb".into()),
        physics_asset_path: None,
        drop_collider: Default::default(),
        value: None,
        weight: None,
        quest_item: false,
        stats: PreparedItemStats::Weapon {
            damage: Some(9),
            max_condition: Some(100),
            clip_size: Some(12),
            speed: None,
            reach: None,
            ammo_form_id: Some(0x4241),
            animation_type: Some(3),
            first_person_model_object_form_id: Some(0x100),
            first_person_asset_path: Some("assets/first.glb".into()),
            fire_sound_3d_form_id: Some(0x200),
            fire_sound_2d_form_id: Some(0x201),
        },
        audio: Default::default(),
    };
    let weapon = EquippedWeapon::from_item(&item).unwrap();
    assert_eq!(
        weapon.viewmodel_asset_path.as_deref(),
        Some("assets/first.glb")
    );
    assert_eq!(weapon.damage, 9.0);
    assert_eq!(weapon.max_condition, Some(100));
    assert_eq!(weapon.condition_policy().degradation_per_shot(), 1);
    assert_eq!(weapon.fire_sound_2d_form_id, Some(0x201));
}

#[test]
fn reload_sound_candidates_follow_fire_sound_family() {
    let fire_editor_id = "WPNPistol10mmFire2D";
    let stem = fire_editor_id.strip_suffix("Fire2D").unwrap();
    let candidates = [
        "Reload",
        "ReloadOut",
        "ReloadInOut",
        "ReloadIn",
        "ReloadChamber",
    ]
    .iter()
    .map(|suffix| format!("{stem}{suffix}"))
    .collect::<Vec<_>>();
    assert_eq!(candidates[1], "WPNPistol10mmReloadOut");
}

use super::*;
use crate::vsa::{PreparedPhysicsBody, PreparedPhysicsShape, PreparedPhysicsSource};

/// Pins the constant so a catalog-shape change that forgets the bump
/// fails here instead of shipping silently-degrading caches (issue #98
/// added serde-defaulted fields without one; v3 is the correction).
/// Bump this expectation together with `ITEM_CATALOG_REVISION`.
#[test]
fn built_catalogs_carry_the_pinned_revision() {
    let catalog = build_item_catalog(
        &HashMap::new(),
        &HashMap::new(),
        &[],
        &HashMap::new(),
        "abc",
    );
    assert_eq!(catalog.revision, "openmw-items-v10-combat-condition");
    assert_eq!(ITEM_CATALOG_REVISION, "openmw-items-v10-combat-condition");
}

#[test]
fn transfer_audio_defaults_fill_only_missing_item_descriptors() {
    let mut explicit = BaseRecord::default();
    explicit.kind = "MISC".into();
    explicit.audio.pickup_sound_form_id = Some(0x11);
    explicit.audio.drop_sound_form_id = Some(0x12);
    let mut missing = BaseRecord::default();
    missing.kind = "MISC".into();
    let bases = HashMap::from([(1, explicit), (2, missing)]);
    let mut catalog = build_item_catalog(&bases, &HashMap::new(), &[], &HashMap::new(), "abc");

    apply_item_transfer_audio_defaults(&mut catalog, Some(0x21), Some(0x22));

    let explicit = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == 1)
        .unwrap();
    assert_eq!(explicit.audio.pickup_sound_form_id, Some(0x11));
    assert_eq!(explicit.audio.drop_sound_form_id, Some(0x12));
    let defaulted = catalog
        .items
        .iter()
        .find(|item| item.base_form_id == 2)
        .unwrap();
    assert_eq!(defaulted.audio.pickup_sound_form_id, Some(0x21));
    assert_eq!(defaulted.audio.drop_sound_form_id, Some(0x22));
}

#[test]
fn catalog_is_formid_sorted_and_carries_prepared_drop_assets() {
    let mut bases = HashMap::new();
    for (form_id, kind) in [(9, "MISC"), (3, "WEAP")] {
        let mut base = BaseRecord::default();
        base.kind = kind.into();
        base.name = Some(format!("Item {form_id}"));
        base.model = Some(format!("meshes/item{form_id}.nif"));
        bases.insert(form_id, base);
    }
    let placements = vec![PreparedPlacement {
        reference_form_id: 99,
        base_form_id: 3,
        asset_path: Some("assets/item.glb".into()),
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: Some("assets/item.physics.json.gz".into()),
        physics_source: Some(PreparedPhysicsSource::GeneratedRender),
        physics_classification: PreparedPhysicsClassification::Static,
        step_support: false,
        mutability: PreparedRuntimeMutability::ScriptAddressable,
        mutability_root_form_id: None,
        reference_kind: "Object".into(),
        base_kind: "WEAP".into(),
        editor_id: None,
        display_name: Some("Item 3".into()),
        count: 1,
        semantic: PreparedSemantic::Pickup(PreparedPickup {
            category: "WEAP".into(),
            value: None,
            weight: None,
        }),
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: PreparedPlacementAudio::default(),
        ao_mode: "ao-none".into(),
    }];
    let physics = HashMap::from([(
        "assets/item.physics.json.gz".into(),
        PreparedPhysicsAsset {
            schema_version: PHYSICS_ASSET_SCHEMA_VERSION,
            source: PreparedPhysicsSource::GeneratedRender,
            bodies: vec![PreparedPhysicsBody {
                shapes: vec![PreparedPhysicsShape::TriangleMesh {
                    vertices: vec![[-1.0, 0.0, -0.5], [1.0, 0.0, -0.5], [0.0, 2.0, 0.5]],
                    indices: vec![0, 1, 2],
                }],
                ..PreparedPhysicsBody::default()
            }],
            joints: Vec::new(),
        },
    )]);
    let catalog = build_item_catalog(&bases, &HashMap::new(), &placements, &physics, "abc");
    assert_eq!(
        catalog
            .items
            .iter()
            .map(|item| item.base_form_id)
            .collect::<Vec<_>>(),
        [3, 9]
    );
    assert_eq!(
        catalog.items[0].world_asset_path.as_deref(),
        Some("assets/item.glb")
    );
    assert!(matches!(
        catalog.items[0].drop_collider,
        PreparedDropCollider::BoundsProxy { .. }
    ));
    assert!(matches!(
        catalog.items[1].drop_collider,
        PreparedDropCollider::Missing
    ));
}

// Issue #98 (F98.1): the new ammo/biped-slot fields carry through
// `prepared_stats` unchanged.
#[test]
fn weapon_ammo_and_armor_biped_slot_mask_carry_into_prepared_stats() {
    let mut weapon = BaseRecord::default();
    weapon.kind = "WEAP".into();
    weapon.item_stats = OpenMwItemStats::Weapon {
        damage: Some(10),
        max_condition: None,
        clip_size: None,
        speed: None,
        reach: None,
        ammo_form_id: Some(0x0000_00aa),
        animation_type: Some(3),
        first_person_model_object_form_id: Some(0x77),
    };
    weapon.audio.weapon_fire_3d_sound_form_id = Some(0x88);
    weapon.audio.weapon_fire_2d_sound_form_id = Some(0x99);
    let mut armor = BaseRecord::default();
    armor.kind = "ARMO".into();
    armor.item_stats = OpenMwItemStats::Apparel {
        armor_rating: None,
        max_condition: None,
        biped_slot_mask: Some(0x0000_0005),
    };
    let mut first_person = BaseRecord::default();
    first_person.kind = "STAT".into();
    first_person.model = Some("weapons/10mm-first-person.nif".into());
    let bases = HashMap::from([(1, weapon), (2, armor), (0x77, first_person)]);
    let first_person_placement = PreparedPlacement {
        reference_form_id: u32::MAX,
        base_form_id: 0x77,
        asset_path: Some("assets/10mm-first-person.glb".into()),
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: PreparedPhysicsClassification::Static,
        step_support: false,
        mutability: PreparedRuntimeMutability::ScriptAddressable,
        mutability_root_form_id: None,
        reference_kind: "Object".into(),
        base_kind: "STAT".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: PreparedSemantic::Static,
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: PreparedPlacementAudio::default(),
        ao_mode: "ao-none".into(),
    };
    let catalog = build_item_catalog(
        &bases,
        &HashMap::new(),
        &[first_person_placement],
        &HashMap::new(),
        "abc",
    );
    let weapon_stats = &catalog
        .items
        .iter()
        .find(|item| item.base_form_id == 1)
        .unwrap()
        .stats;
    assert!(matches!(
        weapon_stats,
        PreparedItemStats::Weapon {
            ammo_form_id: Some(0x0000_00aa),
            animation_type: Some(3),
            first_person_model_object_form_id: Some(0x77),
            first_person_asset_path: Some(path),
            fire_sound_3d_form_id: Some(0x88),
            fire_sound_2d_form_id: Some(0x99),
            ..
        } if path == "assets/10mm-first-person.glb"
    ));
    let armor_stats = &catalog
        .items
        .iter()
        .find(|item| item.base_form_id == 2)
        .unwrap()
        .stats;
    assert!(matches!(
        armor_stats,
        PreparedItemStats::Apparel {
            biped_slot_mask: Some(0x0000_0005),
            ..
        }
    ));
}

// Issue #123: the decoded FO3 NOTE text carries through into the same
// prepared field the Pip-Boy reader already consumes for books.
#[test]
fn note_text_carries_into_prepared_stats() {
    let mut note = BaseRecord::default();
    note.kind = "NOTE".into();
    note.item_stats = OpenMwItemStats::Note {
        text: Some("Synthetic holotape text".into()),
    };
    let bases = HashMap::from([(1, note)]);
    let catalog = build_item_catalog(&bases, &HashMap::new(), &[], &HashMap::new(), "abc");
    let note_stats = &catalog
        .items
        .iter()
        .find(|item| item.base_form_id == 1)
        .unwrap()
        .stats;
    assert!(matches!(
        note_stats,
        PreparedItemStats::Note {
            text: Some(text),
        } if text == "Synthetic holotape text"
    ));
}

#[test]
fn synthetic_catalog_references_do_not_collide_with_scene_references() {
    let mut base = BaseRecord::default();
    base.kind = "MISC".into();
    base.model = Some("meshes/item.nif".into());
    let bases = HashMap::from([(1, base)]);
    let scene = ReferenceRecord {
        form_id: u32::MAX,
        ..ReferenceRecord::default()
    };
    let (references, ids) = catalog_item_references(&bases, &[scene]);
    assert_eq!(references.len(), 1);
    assert_ne!(references[0].form_id, u32::MAX);
    assert!(ids.contains(&references[0].form_id));
}

//! Prepared item catalogue and icon staging (M3 wave 1, issue #70).

use super::*;

/// Bump whenever the catalog shape changes, even when the new fields are
/// serde-defaulted: a stale cached `items.ron` would otherwise deserialize
/// silently with those fields defaulted and downstream rules degrade
/// (issue #98 added `WEAP.ammo_form_id`/`ARMO.biped_slot_mask` that way --
/// v3 makes `view` reject pre-#98 catalogs instead).
pub(crate) const ITEM_CATALOG_REVISION: &str = "openmw-items-v3";

/// Synthetic one-per-base references route every supported item model through
/// the ordinary content-addressed GLB/physics preparation path. Their IDs are
/// reserved only for this preparation pass and are removed before the scene
/// manifest is written.
pub(crate) fn catalog_item_references(
    bases: &HashMap<u32, BaseRecord>,
    scene_references: &[ReferenceRecord],
) -> (Vec<ReferenceRecord>, HashSet<u32>) {
    let mut used = scene_references
        .iter()
        .map(|reference| reference.form_id)
        .collect::<HashSet<_>>();
    let mut next_id = u32::MAX;
    let mut ids = bases.keys().copied().collect::<Vec<_>>();
    ids.sort_unstable();
    let mut references = Vec::new();
    let mut synthetic_ids = HashSet::new();
    for base_form_id in ids {
        let base = &bases[&base_form_id];
        if !is_pickup_kind(&base.kind) || base.model.is_none() {
            continue;
        }
        while used.contains(&next_id) {
            next_id = next_id.saturating_sub(1);
        }
        let mut reference = ReferenceRecord {
            kind: ReferenceKind::Object,
            form_id: next_id,
            base_form_id,
            count: 1,
            initially_enabled: true,
            ..ReferenceRecord::default()
        };
        reference.parent_cell_form_id = u32::MAX;
        used.insert(next_id);
        synthetic_ids.insert(next_id);
        references.push(reference);
        next_id = next_id.saturating_sub(1);
    }
    (references, synthetic_ids)
}

pub(crate) fn stage_item_icons(
    bases: &HashMap<u32, BaseRecord>,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    source_fingerprint: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Result<HashMap<u32, String>> {
    let relative_root = PathBuf::from("staging")
        .join("item-icons")
        .join(source_fingerprint);
    let output_root = staging_dir.join("item-icons").join(source_fingerprint);
    fs::create_dir_all(&output_root)?;
    let mut ids = bases.keys().copied().collect::<Vec<_>>();
    ids.sort_unstable();
    let mut staged = HashMap::new();
    for form_id in ids {
        let base = &bases[&form_id];
        if !is_pickup_kind(&base.kind) {
            continue;
        }
        let Some(authored) = base.icon.as_deref().or(base.mini_icon.as_deref()) else {
            continue;
        };
        let normalized = normalize_asset_path(authored);
        let source_path = if normalized.starts_with("textures/") {
            normalized
        } else {
            format!("textures/{normalized}")
        };
        let Some(bytes) = resolve_asset(data_root, archives, &source_path)
            .with_context(|| format!("reading item icon {source_path}"))?
        else {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("missing item icon {source_path} for {form_id:08x}"),
            });
            continue;
        };
        let extension = Path::new(&source_path)
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("dds")
            .to_ascii_lowercase();
        let destination = output_root.join(format!("{form_id:08x}.{extension}"));
        if !destination.exists() {
            fs::write(&destination, bytes)?;
        }
        let final_extension = if extension == "dds" {
            "png"
        } else {
            &extension
        };
        staged.insert(
            form_id,
            relative_root
                .join(format!("{form_id:08x}.{final_extension}"))
                .to_string_lossy()
                .replace('\\', "/"),
        );
    }
    Ok(staged)
}

pub(crate) fn build_item_catalog(
    bases: &HashMap<u32, BaseRecord>,
    icons: &HashMap<u32, String>,
    item_placements: &[PreparedPlacement],
    physics_assets: &HashMap<String, PreparedPhysicsAsset>,
    source_fingerprint: &str,
) -> PreparedItemCatalog {
    let placements = item_placements
        .iter()
        .map(|placement| (placement.base_form_id, placement))
        .collect::<HashMap<_, _>>();
    let mut ids = bases.keys().copied().collect::<Vec<_>>();
    ids.sort_unstable();
    let items = ids
        .into_iter()
        .filter_map(|base_form_id| {
            let base = &bases[&base_form_id];
            let category = category(&base.kind)?;
            let placement = placements.get(&base_form_id).copied();
            let physics_asset_path =
                placement.and_then(|placement| placement.physics_asset_path.clone());
            let drop_collider = placement
                .and_then(|placement| {
                    let path = placement.physics_asset_path.as_ref()?;
                    let asset = physics_assets.get(path)?;
                    Some(
                        if placement.physics_classification
                            == PreparedPhysicsClassification::Dynamic
                        {
                            PreparedDropCollider::Authored
                        } else if let Some((center, half_extents)) = dynamic_proxy_bounds(asset) {
                            PreparedDropCollider::BoundsProxy {
                                center,
                                half_extents,
                            }
                        } else {
                            PreparedDropCollider::Missing
                        },
                    )
                })
                .unwrap_or_default();
            Some(PreparedItemDefinition {
                base_form_id,
                record_kind: base.kind.clone(),
                category,
                editor_id: base.editor_id.clone(),
                display_name: base.name.clone(),
                source_model_path: base.model.clone(),
                icon_asset_path: icons.get(&base_form_id).cloned(),
                world_asset_path: placement.and_then(|placement| placement.asset_path.clone()),
                physics_asset_path,
                drop_collider,
                value: base.value,
                weight: base.weight,
                quest_item: crate::viewer::interaction::item_rules::is_quest_item(
                    base.record_flags,
                ),
                stats: prepared_stats(&base.item_stats),
                audio: PreparedPlacementAudio {
                    loop_sound_form_id: base.audio.loop_sound_form_id,
                    activate_sound_form_id: base.audio.activation_sound_form_id,
                    open_sound_form_id: base.audio.open_sound_form_id,
                    close_sound_form_id: base.audio.close_sound_form_id,
                    pickup_sound_form_id: base.audio.pickup_sound_form_id,
                    drop_sound_form_id: base.audio.drop_sound_form_id,
                },
            })
        })
        .collect();
    PreparedItemCatalog {
        revision: ITEM_CATALOG_REVISION.into(),
        source_fingerprint: source_fingerprint.into(),
        items,
    }
}

pub(crate) fn write_item_catalog(
    cache_dir: &Path,
    catalog: &PreparedItemCatalog,
) -> Result<(String, String)> {
    let relative = PathBuf::from("catalogs")
        .join(&catalog.source_fingerprint)
        .join("items.ron");
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let serialized = to_string_pretty(catalog, PrettyConfig::default())?;
    let hash = fingerprint(serialized.as_bytes());
    fs::write(&path, serialized)?;
    Ok((relative.to_string_lossy().replace('\\', "/"), hash))
}

fn is_pickup_kind(kind: &str) -> bool {
    category(kind).is_some()
}

fn category(kind: &str) -> Option<PreparedItemCategory> {
    match kind {
        "WEAP" => Some(PreparedItemCategory::Weapons),
        "ARMO" => Some(PreparedItemCategory::Apparel),
        "ALCH" => Some(PreparedItemCategory::Aid),
        "AMMO" => Some(PreparedItemCategory::Ammo),
        "MISC" | "BOOK" | "NOTE" | "KEYM" => Some(PreparedItemCategory::Misc),
        _ => None,
    }
}

fn prepared_stats(stats: &OpenMwItemStats) -> PreparedItemStats {
    match stats {
        OpenMwItemStats::Weapon {
            damage,
            max_condition,
            clip_size,
            speed,
            reach,
            ammo_form_id,
        } => PreparedItemStats::Weapon {
            damage: *damage,
            max_condition: *max_condition,
            clip_size: *clip_size,
            speed: *speed,
            reach: *reach,
            ammo_form_id: *ammo_form_id,
        },
        OpenMwItemStats::Apparel {
            armor_rating,
            max_condition,
            biped_slot_mask,
        } => PreparedItemStats::Apparel {
            armor_rating: *armor_rating,
            max_condition: *max_condition,
            biped_slot_mask: *biped_slot_mask,
        },
        OpenMwItemStats::Ammo { damage, speed } => PreparedItemStats::Ammo {
            damage: *damage,
            speed: *speed,
        },
        OpenMwItemStats::Aid { effect_form_ids } => PreparedItemStats::Aid {
            effects: effect_form_ids
                .iter()
                .map(|form_id| PreparedItemEffect {
                    form_id: *form_id,
                    label: format!("Effect {form_id:08X}"),
                })
                .collect(),
        },
        OpenMwItemStats::Book { flags, text } => PreparedItemStats::Book {
            flags: *flags,
            text: text.clone(),
        },
        OpenMwItemStats::Note { text } => PreparedItemStats::Note { text: text.clone() },
        OpenMwItemStats::Key => PreparedItemStats::Key,
        OpenMwItemStats::Misc => PreparedItemStats::Misc,
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(catalog.revision, "openmw-items-v3");
        assert_eq!(ITEM_CATALOG_REVISION, "openmw-items-v3");
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
        };
        let mut armor = BaseRecord::default();
        armor.kind = "ARMO".into();
        armor.item_stats = OpenMwItemStats::Apparel {
            armor_rating: None,
            max_condition: None,
            biped_slot_mask: Some(0x0000_0005),
        };
        let bases = HashMap::from([(1, weapon), (2, armor)]);
        let catalog = build_item_catalog(&bases, &HashMap::new(), &[], &HashMap::new(), "abc");
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
                ..
            }
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
}

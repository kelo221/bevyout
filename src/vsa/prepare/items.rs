//! Prepared item catalogue and icon staging (M3 wave 1, issue #70).

use super::*;
use crate::vsa::openmw_esm4::BaseAudioRecord;

/// Bump whenever the catalog shape *or* the decoded meaning of an existing
/// field changes, even when the shape itself is unchanged: a stale cached
/// `items.ron` would otherwise deserialize silently with degraded data
/// (issue #98 added `WEAP.ammo_form_id`/`ARMO.biped_slot_mask` that way --
/// v3 makes `view` reject pre-#98 catalogs instead). Issue #123 fixed
/// `NOTE.text` decoding (it was always `None`); v4 forces re-`prepare` so
/// cached catalogs pick up real holotape/note text instead of a stale
/// `None` that would otherwise deserialize cleanly and hide the fix. Wave 3
/// bumps the meaning revision because weapon condition is now consumed by the
/// canonical combat policy and must not run against a pre-wave catalog.
pub(crate) const ITEM_CATALOG_REVISION: &str = "openmw-items-v10-combat-condition";
pub(crate) const DEFAULT_ITEM_TRANSFER_SOUND_EDITOR_ID: &str = "UIMenuOK";

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
    let mut ids = bases
        .iter()
        .filter_map(|(base_form_id, base)| {
            (is_pickup_kind(&base.kind) && base.model.is_some()).then_some(*base_form_id)
        })
        .collect::<HashSet<_>>();
    for base in bases.values() {
        if let OpenMwItemStats::Weapon {
            first_person_model_object_form_id: Some(form_id),
            ..
        } = &base.item_stats
            && bases
                .get(form_id)
                .is_some_and(|target| target.model.is_some())
        {
            ids.insert(*form_id);
        }
    }
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort_unstable();
    let mut references = Vec::new();
    let mut synthetic_ids = HashSet::new();
    for base_form_id in ids {
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
            "ktx2"
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
                quest_item: bevyout_core::items::is_quest_item(base.record_flags),
                stats: prepared_stats(&base.item_stats, &base.audio, &placements),
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

pub(crate) fn item_transfer_audio_defaults(parsed: &ParsedPlugin) -> (Option<u32>, Option<u32>) {
    let menu_ok = sound_form_ids_by_editor_id(parsed, DEFAULT_ITEM_TRANSFER_SOUND_EDITOR_ID)
        .into_iter()
        .next();
    (menu_ok, menu_ok)
}

pub(crate) fn apply_item_transfer_audio_defaults(
    catalog: &mut PreparedItemCatalog,
    pickup_sound_form_id: Option<u32>,
    drop_sound_form_id: Option<u32>,
) {
    for item in &mut catalog.items {
        if item.audio.pickup_sound_form_id.is_none() {
            item.audio.pickup_sound_form_id = pickup_sound_form_id;
        }
        if item.audio.drop_sound_form_id.is_none() {
            item.audio.drop_sound_form_id = drop_sound_form_id;
        }
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

fn prepared_stats(
    stats: &OpenMwItemStats,
    audio: &BaseAudioRecord,
    placements: &HashMap<u32, &PreparedPlacement>,
) -> PreparedItemStats {
    match stats {
        OpenMwItemStats::Weapon {
            damage,
            max_condition,
            clip_size,
            speed,
            reach,
            ammo_form_id,
            animation_type,
            first_person_model_object_form_id,
        } => PreparedItemStats::Weapon {
            damage: *damage,
            max_condition: *max_condition,
            clip_size: *clip_size,
            speed: *speed,
            reach: *reach,
            ammo_form_id: *ammo_form_id,
            animation_type: *animation_type,
            first_person_model_object_form_id: *first_person_model_object_form_id,
            first_person_asset_path: first_person_model_object_form_id
                .and_then(|form_id| placements.get(&form_id))
                .and_then(|placement| placement.asset_path.clone()),
            fire_sound_3d_form_id: audio.weapon_fire_3d_sound_form_id,
            fire_sound_2d_form_id: audio.weapon_fire_2d_sound_form_id,
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
#[path = "tests/items.rs"]
mod tests;

//! Prepared placement and semantic derivation.

use super::*;

pub(crate) fn prepared_placement(
    reference: &ReferenceRecord,
    base: Option<&BaseRecord>,
    asset_path: Option<String>,
    error: Option<String>,
    base_records: &HashMap<u32, BaseRecord>,
) -> PreparedPlacement {
    let transform = placement_transform(reference);
    let semantic = prepared_semantic(reference, base);
    let (mutability, mutability_root_form_id) =
        classify_runtime_mutability(reference, &semantic, error.is_some());
    PreparedPlacement {
        reference_form_id: reference.form_id,
        base_form_id: reference.base_form_id,
        asset_path,
        translation: transform.0,
        rotation_xyzw: transform.1,
        scale: transform.2,
        error,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: PreparedPhysicsClassification::Static,
        step_support: false,
        mutability,
        mutability_root_form_id,
        reference_kind: reference.kind.as_str().into(),
        base_kind: base.map_or_else(|| "MISSING".into(), |base| base.kind.clone()),
        editor_id: base.and_then(|base| base.editor_id.clone()),
        display_name: base.and_then(|base| base.name.clone()),
        count: reference.count,
        semantic,
        initially_enabled: reference.initially_enabled,
        enable_parent: reference.enable_parent.map(|parent| PreparedEnableParent {
            reference_form_id: parent.parent_reference_form_id,
            inverted: parent.is_inverted(),
            pop_in: parent.is_pop_in(),
        }),
        owner_form_id: reference.owner_form_id,
        owner_faction_rank: reference.owner_faction_rank,
        inventory: base
            .map(|base| {
                base.inventory
                    .iter()
                    .map(|item| prepared_inventory_entry(item, base_records))
                    .collect()
            })
            .unwrap_or_default(),
        audio: base
            .map(|base| PreparedPlacementAudio {
                loop_sound_form_id: base.audio.loop_sound_form_id,
                activate_sound_form_id: base.audio.activation_sound_form_id,
                open_sound_form_id: base.audio.open_sound_form_id,
                close_sound_form_id: base.audio.close_sound_form_id,
                pickup_sound_form_id: base.audio.pickup_sound_form_id,
                drop_sound_form_id: base.audio.drop_sound_form_id,
            })
            .unwrap_or_default(),
        ao_mode: "ao-none".into(),
    }
}

pub(crate) fn prepared_inventory_entry(
    item: &crate::vsa::openmw_esm4::InventoryItemRecord,
    base_records: &HashMap<u32, BaseRecord>,
) -> PreparedInventoryEntry {
    let item_base = base_records.get(&item.item_form_id);
    PreparedInventoryEntry {
        base_form_id: item.item_form_id,
        count: item.count.min(i32::MAX as u32) as i32,
        record_kind: item_base.map_or_else(|| "UNKNOWN".into(), |base| base.kind.clone()),
        editor_id: item_base.and_then(|base| base.editor_id.clone()),
        display_name: item_base.and_then(|base| base.name.clone()),
        leveled: item_base
            .is_some_and(|base| matches!(base.kind.as_str(), "LVLI" | "LVLN" | "LVLC")),
    }
}

pub(crate) fn model_static_usage(
    references: &[ReferenceRecord],
    bases: &HashMap<u32, BaseRecord>,
) -> HashMap<String, bool> {
    let mut usage = HashMap::new();
    for reference in references {
        let Some(base) = bases.get(&reference.base_form_id) else {
            continue;
        };
        let Some(model) = base.model.as_deref() else {
            continue;
        };
        let model = normalize_asset_path(model);
        if is_editor_marker(&model) || is_non_rendering_effect(&model) {
            continue;
        }
        let static_asset = reference.kind == ReferenceKind::Object
            && matches!(base.kind.as_str(), "STAT" | "MSTT");
        usage
            .entry(model)
            .and_modify(|value| *value &= static_asset)
            .or_insert(static_asset);
    }
    usage
}

pub(crate) fn prepared_semantic(
    reference: &ReferenceRecord,
    base: Option<&BaseRecord>,
) -> PreparedSemantic {
    let actor = PreparedActor {
        base_template_form_id: base.and_then(|base| base.base_template_form_id),
    };
    match reference.kind {
        ReferenceKind::Npc => return PreparedSemantic::Npc(actor),
        ReferenceKind::Creature => return PreparedSemantic::Creature(actor),
        ReferenceKind::Object => {}
    }
    let Some(base) = base else {
        return PreparedSemantic::Unsupported;
    };
    match base.kind.as_str() {
        "WEAP" | "AMMO" | "ARMO" | "ALCH" | "MISC" | "BOOK" | "NOTE" | "KEYM" => {
            PreparedSemantic::Pickup(PreparedPickup {
                category: base.kind.clone(),
                value: base.value,
                weight: base.weight,
            })
        }
        "CONT" => PreparedSemantic::Container,
        "DOOR" => {
            let door = reference.door.as_ref();
            PreparedSemantic::Door(PreparedDoor {
                lock_level: door.and_then(|door| door.lock_level),
                key_form_id: door.and_then(|door| door.key_form_id),
                destination: door
                    .and_then(|door| door.destination.as_ref())
                    .map(|destination| {
                        let transform = placement_transform_parts(
                            destination.position,
                            destination.rotation,
                            1.0,
                        );
                        PreparedDoorDestination {
                            door_reference_form_id: destination.door_reference_form_id,
                            cell_form_id: destination.cell_form_id,
                            translation: transform.0,
                            rotation_xyzw: transform.1,
                        }
                    }),
            })
        }
        "ACTI" | "TACT" | "TERM" => PreparedSemantic::Activator,
        "FURN" => PreparedSemantic::Furniture,
        "STAT" | "MSTT" => PreparedSemantic::Static,
        _ => PreparedSemantic::Unsupported,
    }
}

/// Classifies a placement's `PreparedRuntimeMutability` (F38.1, F38.3).
///
/// Precedence, most to least certain:
/// 1. `error` present (missing base/model/etc.) → `Unknown`; we don't know
///    enough about the record to make any other claim.
/// 2. The reference is part of an XESP enable-parent chain → `EnableGroup`,
///    carrying the resolved chain root FormID. If the chain root could not
///    be resolved (cycle/unresolved parent), classification stays
///    `Unknown` rather than guessing a root.
/// 3. A record kind known to be commonly reachable/mutated by scripts
///    (doors, activators, containers, furniture, actors, pickups) →
///    `ScriptAddressable`.
/// 4. Plain static scenery (`PreparedSemantic::Static`) → `Immutable`.
/// 5. Anything else, including unrecognized record kinds
///    (`PreparedSemantic::Unsupported`) → `Unknown`.
///
/// This match is exhaustive over `PreparedSemantic` on purpose: a future
/// semantic variant added without updating this function fails to compile
/// instead of silently falling through to `Immutable`.
pub(crate) fn classify_runtime_mutability(
    reference: &ReferenceRecord,
    semantic: &PreparedSemantic,
    has_error: bool,
) -> (PreparedRuntimeMutability, Option<u32>) {
    if has_error {
        return (PreparedRuntimeMutability::Unknown, None);
    }
    if reference.enable_parent.is_some() {
        return match reference.enable_root_form_id {
            Some(root) => (PreparedRuntimeMutability::EnableGroup, Some(root)),
            None => (PreparedRuntimeMutability::Unknown, None),
        };
    }
    let mutability = match semantic {
        PreparedSemantic::Door(_)
        | PreparedSemantic::Activator
        | PreparedSemantic::Container
        | PreparedSemantic::Furniture
        | PreparedSemantic::Npc(_)
        | PreparedSemantic::Creature(_)
        | PreparedSemantic::Pickup(_) => PreparedRuntimeMutability::ScriptAddressable,
        PreparedSemantic::Static => PreparedRuntimeMutability::Immutable,
        PreparedSemantic::Unsupported => PreparedRuntimeMutability::Unknown,
    };
    (mutability, None)
}

/// Deterministic QA counts of `PreparedRuntimeMutability` across a prepared
/// scene's placements (F38.4).
pub(crate) fn summarize_mutability(placements: &[PreparedPlacement]) -> PreparedMutabilitySummary {
    let mut summary = PreparedMutabilitySummary::default();
    for placement in placements {
        match placement.mutability {
            PreparedRuntimeMutability::Immutable => summary.immutable += 1,
            PreparedRuntimeMutability::EnableGroup => summary.enable_group += 1,
            PreparedRuntimeMutability::ScriptAddressable => summary.script_addressable += 1,
            PreparedRuntimeMutability::Unknown => summary.unknown += 1,
        }
    }
    summary
}

pub(crate) fn is_structural_step_support(
    semantic: &PreparedSemantic,
    normalized_model: &str,
) -> bool {
    matches!(semantic, PreparedSemantic::Static)
        && ["architecture/", "dungeons/", "landscape/"]
            .iter()
            .any(|prefix| normalized_model.starts_with(prefix))
}

pub(crate) fn retain_static_step_support(
    candidate: bool,
    classification: PreparedPhysicsClassification,
) -> bool {
    candidate && classification == PreparedPhysicsClassification::Static
}

#[derive(Debug)]
pub(crate) struct PlacementStage {
    pub(crate) jobs: Vec<BlenderAssetJob>,
    pub(crate) placements: Vec<PreparedPlacement>,
    pub(crate) lights: Vec<PreparedLight>,
    pub(crate) cache_hits: usize,
    pub(crate) cache_missing: usize,
    pub(crate) cache_invalid: usize,
    pub(crate) cache_explicit_rebuilds: usize,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn stage_placements(
    references: Vec<ReferenceRecord>,
    bases: &HashMap<u32, BaseRecord>,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    assets_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
    rebuild_assets: bool,
) -> Result<PlacementStage> {
    let model_static_usage = model_static_usage(&references, bases);
    let mut jobs: Vec<BlenderAssetJob> = Vec::new();
    let mut placements = Vec::new();
    let mut lights = Vec::new();
    let mut seen_models = HashMap::<String, String>::new();
    let mut cache_hits = 0usize;
    let mut cache_missing = 0usize;
    let mut cache_invalid = 0usize;
    let mut cache_explicit_rebuilds = 0usize;

    for reference in references {
        if reference.flags & (RECORD_DELETED | RECORD_DISABLED) != 0 {
            continue;
        }
        let transform = placement_transform(&reference);
        let Some(base) = bases.get(&reference.base_form_id) else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "missing base record {:08x} for reference {:08x}",
                    reference.base_form_id, reference.form_id
                ),
            });
            placements.push(prepared_placement(
                &reference,
                None,
                None,
                Some("missing base record".into()),
                bases,
            ));
            continue;
        };
        if base.kind == "LIGH" {
            let light = base.light.as_ref();
            let radius = light.map_or(5.0, |light| light.radius * FO3_SCALE);
            lights.push(PreparedLight {
                reference_form_id: reference.form_id,
                base_form_id: reference.base_form_id,
                translation: transform.0,
                rotation_xyzw: transform.1,
                color_rgba: light.map_or([1.0, 0.78, 0.55, 1.0], |light| light.color_rgba),
                radius: radius.max(0.1),
                intensity_lumens: radius.max(0.1) * radius.max(0.1) * 2.0 * 8192.0,
                kind: "point".into(),
                flags: 0,
                initially_enabled: reference.initially_enabled,
            });
        }
        let model = (reference.kind != ReferenceKind::Npc)
            .then_some(base.model.as_ref())
            .flatten();
        let Some(model) = model else {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "reference {:08x} ({}) has no standalone world model",
                    reference.form_id, base.kind
                ),
            });
            placements.push(prepared_placement(
                &reference,
                Some(base),
                None,
                None,
                bases,
            ));
            continue;
        };
        let normalized_model = normalize_asset_path(model);
        if is_editor_marker(&normalized_model) {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("skipping non-rendering editor marker {normalized_model}"),
            });
            if reference.kind != ReferenceKind::Object || base.kind == "DOOR" {
                placements.push(prepared_placement(
                    &reference,
                    Some(base),
                    None,
                    None,
                    bases,
                ));
            }
            continue;
        }
        if is_non_rendering_effect(&normalized_model) {
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!("skipping non-rendering effect {normalized_model}"),
            });
            continue;
        }
        let Some(nif_bytes) = resolve_asset(data_root, archives, &normalized_model)? else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "missing model {normalized_model} for reference {:08x}",
                    reference.form_id
                ),
            });
            placements.push(prepared_placement(
                &reference,
                Some(base),
                None,
                Some(format!("missing model {normalized_model}")),
                bases,
            ));
            continue;
        };
        let static_asset = model_static_usage
            .get(&normalized_model)
            .copied()
            .unwrap_or(false);
        let conversion = asset_conversion(static_asset);
        let conversion_profile = conversion.profile_tag().to_owned();
        let converter_profile = format!("{NIF_CONVERTER_REVISION}-{conversion_profile}");
        let asset_name = content_addressed_glb_name(&converter_profile, &nif_bytes);
        let asset_path = format!("assets/{asset_name}");
        let physics_name = physics_sidecar_name(&asset_name);
        let physics_asset_path = format!("assets/{physics_name}");
        if !seen_models.contains_key(&normalized_model) {
            let staging_nif = staging_dir.join(&normalized_model);
            if let Some(parent) = staging_nif.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&staging_nif, &nif_bytes)?;
            stage_textures(&nif_bytes, data_root, archives, staging_dir, diagnostics)?;
            let output = assets_dir.join(&asset_name);
            let physics_output = assets_dir.join(&physics_name);
            let outputs_exist = output.exists() || physics_output.exists();
            let cache_valid = output.exists()
                && physics_output.exists()
                && validate_asset_cache_pair(&output, &physics_output).is_ok();
            match asset_cache_decision(outputs_exist, cache_valid, rebuild_assets) {
                AssetCacheDecision::Reuse => cache_hits += 1,
                AssetCacheDecision::BuildMissing => {
                    cache_missing += 1;
                    jobs.push(BlenderAssetJob {
                        input: staging_nif,
                        output,
                        physics_output,
                        model: normalized_model.clone(),
                        conversion,
                    });
                }
                AssetCacheDecision::RebuildInvalid => {
                    cache_invalid += 1;
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message: format!(
                            "cached GLB/physics pair for {} is missing or invalid; scheduling NIF reconversion",
                            output.display()
                        ),
                    });
                    jobs.push(BlenderAssetJob {
                        input: staging_nif,
                        output,
                        physics_output,
                        model: normalized_model.clone(),
                        conversion,
                    });
                }
                AssetCacheDecision::RebuildRequested => {
                    cache_explicit_rebuilds += 1;
                    jobs.push(BlenderAssetJob {
                        input: staging_nif,
                        output,
                        physics_output,
                        model: normalized_model.clone(),
                        conversion,
                    });
                }
            }
            seen_models.insert(normalized_model.clone(), asset_path.clone());
        }
        let mut placement =
            prepared_placement(&reference, Some(base), Some(asset_path), None, bases);
        placement.ao_mode = conversion_profile;
        placement.physics_asset_path = Some(physics_asset_path);
        placement.step_support = is_structural_step_support(&placement.semantic, &normalized_model);
        placements.push(placement);
    }

    Ok(PlacementStage {
        jobs,
        placements,
        lights,
        cache_hits,
        cache_missing,
        cache_invalid,
        cache_explicit_rebuilds,
    })
}

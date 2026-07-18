//! Prepared placement and semantic derivation.

use super::*;
use crate::vsa::assets::AssetConversion;

use std::collections::{BTreeMap, VecDeque};

use crate::vsa::manifest::{PreparedLeveledEntry, PreparedLeveledList};

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

/// Collects every leveled list transitively reachable from prepared
/// container inventory entries flagged `leveled: true` (issue #74, F74.2):
/// starts a breadth-first walk from those top-level entries' base FormIDs
/// and, for each list found, follows any of its own `LVLO` entries that
/// themselves name another leveled list base. A FormID missing from `bases`
/// (unresolved override, bad content) is simply absent from the result --
/// the resolver treats an unknown list FormID as empty rather than erroring.
pub(crate) fn collect_leveled_lists(
    placements: &[PreparedPlacement],
    bases: &HashMap<u32, BaseRecord>,
) -> BTreeMap<u32, PreparedLeveledList> {
    let mut lists = BTreeMap::new();
    let mut visited: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<u32> = VecDeque::new();
    for placement in placements {
        for entry in &placement.inventory {
            if entry.leveled && visited.insert(entry.base_form_id) {
                queue.push_back(entry.base_form_id);
            }
        }
    }
    while let Some(form_id) = queue.pop_front() {
        let Some(base) = bases.get(&form_id) else {
            continue;
        };
        let Some(data) = base.leveled.as_ref() else {
            continue;
        };
        let entries = data
            .entries
            .iter()
            .map(|entry| PreparedLeveledEntry {
                level: entry.level,
                base_form_id: entry.item_form_id,
                count: entry.count,
            })
            .collect::<Vec<_>>();
        for entry in &entries {
            let is_nested_list = bases
                .get(&entry.base_form_id)
                .is_some_and(|base| base.leveled.is_some());
            if is_nested_list && visited.insert(entry.base_form_id) {
                queue.push_back(entry.base_form_id);
            }
        }
        lists.insert(
            form_id,
            PreparedLeveledList {
                chance_none: data.chance_none,
                flags: data.flags,
                entries,
            },
        );
    }
    lists
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
        | PreparedSemantic::Corpse
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
    pub(crate) visual_assets: Vec<PreparedVisualAsset>,
    pub(crate) placements: Vec<PreparedPlacement>,
    pub(crate) lights: Vec<PreparedLight>,
    pub(crate) cache_hits: usize,
    pub(crate) cache_missing: usize,
    pub(crate) cache_invalid: usize,
    pub(crate) cache_explicit_rebuilds: usize,
    /// Every leveled list transitively reachable from this cell's prepared
    /// container inventory entries (issue #74, F74.2). See
    /// `collect_leveled_lists`.
    pub(crate) leveled_lists: BTreeMap<u32, PreparedLeveledList>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn stage_placements(
    references: Vec<ReferenceRecord>,
    bases: &HashMap<u32, BaseRecord>,
    actor_models: &HashMap<u32, ActorAssemblyDescriptor>,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    assets_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
    rebuild_assets: bool,
) -> Result<PlacementStage> {
    let model_static_usage = model_static_usage(&references, bases);
    let mut jobs: Vec<BlenderAssetJob> = Vec::new();
    let mut visual_assets = Vec::new();
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
        let actor_assembly =
            if reference.kind == ReferenceKind::Npc || reference.kind == ReferenceKind::Creature {
                actor_models.get(&reference.form_id).cloned()
            } else {
                None
            };
        let model = if let Some(actor) = actor_assembly.as_ref() {
            Some(actor.skeleton.as_str())
        } else {
            (reference.kind != ReferenceKind::Npc)
                .then_some(base.model.as_ref())
                .flatten()
                .map(String::as_str)
        };
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
        let mut model_paths = if let Some(actor) = actor_assembly.as_ref() {
            actor.visual_inputs.clone()
        } else {
            vec![normalize_asset_path(model)]
        };
        if actor_assembly.is_none() {
            model_paths.sort();
            model_paths.dedup();
        }
        let normalized_model = model_paths[0].clone();
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
        let mut model_bytes = Vec::with_capacity(model_paths.len());
        for model_path in &model_paths {
            let Some(nif_bytes) = resolve_asset(data_root, archives, model_path)? else {
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!(
                        "missing model {model_path} for reference {:08x}",
                        reference.form_id
                    ),
                });
                placements.push(prepared_placement(
                    &reference,
                    Some(base),
                    None,
                    Some(format!("missing model {model_path}")),
                    bases,
                ));
                model_bytes.clear();
                break;
            };
            model_bytes.push(nif_bytes);
        }
        if model_bytes.len() != model_paths.len() {
            continue;
        }
        let nif_bytes = &model_bytes[0];
        let assembly = actor_assembly.is_some();
        let conversion = asset_conversion(
            model_static_usage
                .get(&normalized_model)
                .copied()
                .unwrap_or(false),
        );
        let root_transform_policy = root_transform_policy(&normalized_model);
        let conversion_profile = conversion.profile_tag().to_owned();
        let mut cache_bytes = Vec::new();
        for bytes in &model_bytes {
            cache_bytes.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
            cache_bytes.extend_from_slice(bytes);
        }
        let cache_profile = if assembly {
            ACTOR_CONVERTER_REVISION.to_owned()
        } else {
            format!(
                "{NIF_CONVERTER_REVISION}-{conversion_profile}-{}",
                root_transform_policy.tag()
            )
        };
        let asset_name = content_addressed_glb_name(&cache_profile, &cache_bytes);
        let asset_path = format!("assets/{asset_name}");
        let physics_name = physics_sidecar_name(&asset_name);
        let physics_asset_path = format!("assets/{physics_name}");
        let model_key = if assembly {
            format!("actor:{}", model_paths.join("|"))
        } else {
            normalized_model.clone()
        };
        if !seen_models.contains_key(&model_key) {
            let staging_nif = staging_dir.join(&normalized_model);
            if let Some(parent) = staging_nif.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&staging_nif, nif_bytes)?;
            stage_textures(nif_bytes, data_root, archives, staging_dir, diagnostics)?;
            let input = if assembly {
                let assembly_key = fingerprint(asset_name.as_bytes());
                let assembly_path = staging_dir
                    .join("actors")
                    .join(format!("{assembly_key}.actor.json"));
                if let Some(parent) = assembly_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                let mut inputs = Vec::with_capacity(model_paths.len());
                let mut staged_paths = HashMap::new();
                for (index, (model_path, bytes)) in model_paths.iter().zip(&model_bytes).enumerate()
                {
                    let path = if index == 0 {
                        staging_nif.clone()
                    } else {
                        let path = staging_dir.join(model_path);
                        if let Some(parent) = path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::write(&path, bytes)?;
                        stage_textures(bytes, data_root, archives, staging_dir, diagnostics)?;
                        path
                    };
                    let staged_path = path.to_string_lossy().to_string();
                    staged_paths.insert(normalize_asset_path(model_path), staged_path.clone());
                    inputs.push(staged_path);
                }
                let actor = actor_assembly
                    .as_ref()
                    .expect("assembly staging requires an actor descriptor");
                let staged = ActorAssemblyDescriptor {
                    skeleton: inputs[0].clone(),
                    visual_inputs: inputs,
                    body_parts: actor
                        .body_parts
                        .iter()
                        .filter_map(|part| {
                            staged_paths
                                .get(&normalize_asset_path(&part.path))
                                .map(|path| ActorBodyPartInput {
                                    path: path.clone(),
                                    index: part.index,
                                })
                        })
                        .collect(),
                    apparel: actor
                        .apparel
                        .iter()
                        .filter_map(|item| {
                            staged_paths
                                .get(&normalize_asset_path(&item.path))
                                .map(|path| ActorApparelInput {
                                    path: path.clone(),
                                    form_id: item.form_id,
                                    biped_slot_mask: item.biped_slot_mask,
                                })
                        })
                        .collect(),
                };
                fs::write(&assembly_path, serde_json::to_string(&staged)?)?;
                assembly_path
            } else {
                staging_nif.clone()
            };
            let output = assets_dir.join(&asset_name);
            let physics_output = assets_dir.join(&physics_name);
            let outputs_exist = output.exists() || physics_output.exists();
            let cache_valid = output.exists()
                && physics_output.exists()
                && validate_asset_cache_pair(&output, &physics_output).is_ok()
                && (!assembly || validate_actor_glb(&output).is_ok());
            match asset_cache_decision(outputs_exist, cache_valid, rebuild_assets) {
                AssetCacheDecision::Reuse => cache_hits += 1,
                AssetCacheDecision::BuildMissing => {
                    cache_missing += 1;
                    jobs.push(BlenderAssetJob {
                        input,
                        output,
                        physics_output,
                        model: if assembly {
                            "actors/assembled".into()
                        } else {
                            normalized_model.clone()
                        },
                        conversion: if assembly {
                            AssetConversion::Preserve
                        } else {
                            conversion
                        },
                        root_transform_policy: if assembly {
                            RootTransformPolicy::PreserveVerified
                        } else {
                            root_transform_policy
                        },
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
                        input,
                        output,
                        physics_output,
                        model: if assembly {
                            "actors/assembled".into()
                        } else {
                            normalized_model.clone()
                        },
                        conversion: if assembly {
                            AssetConversion::Preserve
                        } else {
                            conversion
                        },
                        root_transform_policy: if assembly {
                            RootTransformPolicy::PreserveVerified
                        } else {
                            root_transform_policy
                        },
                    });
                }
                AssetCacheDecision::RebuildRequested => {
                    cache_explicit_rebuilds += 1;
                    jobs.push(BlenderAssetJob {
                        input,
                        output,
                        physics_output,
                        model: if assembly {
                            "actors/assembled".into()
                        } else {
                            normalized_model.clone()
                        },
                        conversion: if assembly {
                            AssetConversion::Preserve
                        } else {
                            conversion
                        },
                        root_transform_policy: if assembly {
                            RootTransformPolicy::PreserveVerified
                        } else {
                            root_transform_policy
                        },
                    });
                }
            }
            visual_assets.push(PreparedVisualAsset {
                model_path: if assembly {
                    "actors/assembled".into()
                } else {
                    normalized_model.clone()
                },
                asset_path: asset_path.clone(),
                root_transform_policy: if assembly {
                    RootTransformPolicy::PreserveVerified
                } else {
                    root_transform_policy
                },
            });
            seen_models.insert(model_key.clone(), asset_path.clone());
        }
        let mut placement =
            prepared_placement(&reference, Some(base), Some(asset_path), None, bases);
        placement.ao_mode = if assembly {
            "ao-none".into()
        } else {
            conversion_profile
        };
        placement.physics_asset_path = Some(physics_asset_path);
        placement.step_support = is_structural_step_support(&placement.semantic, &normalized_model);
        placements.push(placement);
        continue;
    }

    let leveled_lists = collect_leveled_lists(&placements, bases);

    Ok(PlacementStage {
        jobs,
        visual_assets,
        placements,
        lights,
        cache_hits,
        cache_missing,
        cache_invalid,
        cache_explicit_rebuilds,
        leveled_lists,
    })
}

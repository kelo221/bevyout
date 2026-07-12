use anyhow::{Context, Result, bail};
use ron::ser::{PrettyConfig, to_string_pretty};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use super::assets::{
    BlenderAssetJob, NIF_CONVERTER_REVISION, asset_conversion, content_addressed_glb_name,
    convert_staged_textures, find_blender, load_archives, resolve_asset, run_blender_batch,
    stage_textures, validate_glb_images,
};
use super::audio_assets::{load_audio_archives, resolve_audio_asset, stage_audio_asset};
use super::manifest::{
    Diagnostic, PreparedActor, PreparedAudioClip, PreparedCellAudio, PreparedCellLighting,
    PreparedDoor, PreparedDoorDestination, PreparedEnableParent, PreparedInventoryEntry,
    PreparedLight, PreparedLightingTemplate, PreparedNavMeshChunk, PreparedNavMeshSource,
    PreparedPickup, PreparedPlacement, PreparedPlacementAudio, PreparedPluginSource,
    PreparedSceneManifest, PreparedSemantic,
};
use super::openmw_esm4::LightingData;
use super::paths::{
    FO3_SCALE, absolutize, fingerprint, is_editor_marker, is_non_rendering_effect,
    normalize_asset_path, parse_form_id, placement_transform, placement_transform_parts,
};
use super::plugin::{
    BaseRecord, ParsedPlugin, PluginSource, RECORD_DELETED, RECORD_DISABLED, ReferenceKind,
    ReferenceRecord, SoundRecord, SoundReferenceRecord, parse_content_set, read_master_names,
};
use crate::cli::PrepareArgs;

fn prepared_lighting(lighting: LightingData) -> PreparedCellLighting {
    PreparedCellLighting {
        ambient_rgba: lighting.ambient_rgba,
        directional_rgba: lighting.directional_rgba,
        fog_rgba: lighting.fog_rgba,
        fog_near: lighting.fog_near,
        fog_far: lighting.fog_far,
        directional_rotation_xy: lighting.rotation_xy,
        directional_rotation_z: lighting.rotation_z,
        directional_fade: lighting.fog_directional_fade,
        fog_clip_distance: lighting.fog_clip_distance,
        fog_power: lighting.fog_power,
    }
}

fn legacy_lighting(cell: &super::manifest::CellInfo) -> LightingData {
    LightingData {
        ambient_rgba: cell.ambient_rgba,
        directional_rgba: cell.directional_rgba,
        fog_rgba: [0.0; 4],
        fog_near: 0.0,
        fog_far: 0.0,
        rotation_xy: 0,
        rotation_z: 0,
        fog_directional_fade: 0.0,
        fog_clip_distance: 0.0,
        fog_power: 1.0,
    }
}

pub(crate) fn prepare(args: PrepareArgs) -> Result<()> {
    let game_root = args
        .game_root
        .context("Fallout 3 is not configured; pass --game-root or create .bevyout/config.toml")?;
    let root = fs::canonicalize(game_root).context("game root does not exist")?;
    let plugin = args.plugin.unwrap_or_else(|| PathBuf::from("Fallout3.esm"));
    let plugin_path = if plugin.is_absolute() {
        plugin.clone()
    } else {
        root.join("Data").join(&plugin)
    };
    let plugin_path = fs::canonicalize(&plugin_path).context("plugin does not exist")?;
    let cell_id = parse_form_id(&args.cell)?;
    let data_root = root.join("Data");
    let cache_dir = absolutize(
        &args
            .cache_dir
            .unwrap_or_else(|| PathBuf::from(".bevyout/cache")),
    )?;
    let staging_dir = cache_dir.join("staging");
    let assets_dir = cache_dir.join("assets");
    let scene_dir = cache_dir.join("scenes").join(format!("{cell_id:08x}"));
    fs::create_dir_all(&staging_dir)?;
    fs::create_dir_all(&assets_dir)?;
    fs::create_dir_all(&scene_dir)?;

    let loaded_plugins = load_plugin_chain(&plugin_path, &data_root)?;
    let source_fingerprint = content_set_fingerprint(&loaded_plugins);
    let source_plugins = loaded_plugins
        .iter()
        .map(|plugin| PreparedPluginSource {
            name: plugin.name.clone(),
            path: plugin.path.to_string_lossy().to_string(),
            fingerprint: fingerprint(&plugin.bytes),
        })
        .collect::<Vec<_>>();
    let mut diagnostics = Vec::new();
    let mut validator = esplugin::Plugin::new(esplugin::GameId::Fallout3, &plugin_path);
    if let Err(error) = validator.parse_file(esplugin::ParseOptions::header_only()) {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!("esplugin validation failed: {error}"),
        });
    }
    let plugin_sources = loaded_plugins
        .iter()
        .map(|plugin| PluginSource {
            name: &plugin.name,
            bytes: &plugin.bytes,
        })
        .collect::<Vec<_>>();
    let mut parsed = parse_content_set(&plugin_sources, cell_id)
        .context("failed to parse Fallout content set")?;
    diagnostics.extend(parsed.diagnostics.drain(..).map(|message| Diagnostic {
        severity: "info".into(),
        message,
    }));
    let mut cell = parsed.cell.take().context("requested cell was not found")?;
    if let Some(image_space_form_id) = cell.image_space_form_id {
        if let Some(image_space) = parsed.image_spaces.get(&image_space_form_id).cloned() {
            info_image_space(&mut diagnostics, &image_space);
            cell.image_space = Some(image_space);
        } else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "cell {:08x} references unresolved ImageSpace {:08x}",
                    cell_id, image_space_form_id
                ),
            });
        }
    }
    if !cell.interior {
        bail!("cell {cell_id:08x} is not an interior cell; LAND support is not part of this slice")
    }

    let archives = load_archives(&data_root, &mut diagnostics)?;
    let audio_plugin_names = loaded_plugins
        .iter()
        .rev()
        .map(|plugin| plugin.name.clone())
        .collect::<Vec<_>>();
    let audio_archive_load = load_audio_archives(&data_root, &audio_plugin_names);
    diagnostics.extend(
        audio_archive_load
            .diagnostics
            .into_iter()
            .map(|message| Diagnostic {
                severity: "info".into(),
                message,
            }),
    );
    let (cell_audio, audio_clips) = stage_audio(
        &data_root,
        &audio_archive_load.archives,
        &parsed,
        &mut diagnostics,
        &cache_dir.join("audio"),
    )?;
    let (footstep_sets, hard_landing_clips) = stage_footsteps(
        &data_root,
        &audio_archive_load.archives,
        &mut diagnostics,
        &cache_dir.join("audio"),
    )?;
    if let Some(metadata) = parsed.cell_metadata.as_ref() {
        cell.lighting_template_form_id = metadata.lighting_template_form_id;
        cell.lighting_template_flags = metadata.lighting_template_flags;
        cell.water_form_id = metadata.water_form_id;
        cell.water_height = metadata.water_height;
        let raw_lighting = metadata.lighting;
        let mut effective_lighting = raw_lighting.unwrap_or_else(|| legacy_lighting(&cell));
        cell.raw_lighting = raw_lighting.map(prepared_lighting);
        if let Some(template_form_id) = metadata.lighting_template_form_id {
            if let Some(template) = parsed.lighting_templates.get(&template_form_id) {
                if let Some(lighting) = template.lighting {
                    effective_lighting.apply_template(lighting, metadata.lighting_template_flags);
                    cell.lighting_template = Some(PreparedLightingTemplate {
                        form_id: template.form_id,
                        editor_id: template.editor_id.clone(),
                        ambient_rgba: lighting.ambient_rgba,
                        directional_rgba: lighting.directional_rgba,
                        fog_rgba: lighting.fog_rgba,
                        fog_near: lighting.fog_near,
                        fog_far: lighting.fog_far,
                        fog_directional_fade: lighting.fog_directional_fade,
                        fog_clip_distance: lighting.fog_clip_distance,
                        fog_power: lighting.fog_power,
                        directional_rotation_xy: lighting.rotation_xy,
                        directional_rotation_z: lighting.rotation_z,
                    });
                } else {
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message: format!(
                            "cell {:08x} references LightingTemplate {:08x} without usable DATA; retained CELL lighting",
                            cell_id, template_form_id
                        ),
                    });
                }
            } else {
                diagnostics.push(Diagnostic {
                    severity: "warning".into(),
                    message: format!(
                        "cell {:08x} references unresolved LightingTemplate {:08x}",
                        cell_id, template_form_id
                    ),
                });
            }
        }
        cell.ambient_rgba = effective_lighting.ambient_rgba;
        cell.directional_rgba = effective_lighting.directional_rgba;
        cell.effective_lighting = Some(prepared_lighting(effective_lighting));
    } else {
        cell.effective_lighting = Some(prepared_lighting(legacy_lighting(&cell)));
    }
    let blender = find_blender(args.blender)?;
    let mut jobs: Vec<BlenderAssetJob> = Vec::new();
    let mut placements = Vec::new();
    let mut lights = Vec::new();
    let mut seen_models = HashMap::<String, String>::new();
    let model_static_usage = model_static_usage(&parsed.references, &parsed.bases);
    let mut cache_hits = 0usize;
    let mut cache_missing = 0usize;
    let mut cache_invalid = 0usize;
    let mut cache_explicit_rebuilds = 0usize;
    let navmeshes = stage_navmeshes(&scene_dir, &mut diagnostics, &parsed.navmeshes)?;

    for reference in parsed.references.drain(..) {
        if reference.flags & (RECORD_DELETED | RECORD_DISABLED) != 0 {
            continue;
        }
        let transform = placement_transform(&reference);
        let Some(base) = parsed.bases.get(&reference.base_form_id) else {
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
                &parsed.bases,
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
                &parsed.bases,
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
                    &parsed.bases,
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
        let Some(nif_bytes) = resolve_asset(&data_root, &archives, &normalized_model)? else {
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
                &parsed.bases,
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
        if !seen_models.contains_key(&normalized_model) {
            let staging_nif = staging_dir.join(&normalized_model);
            if let Some(parent) = staging_nif.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&staging_nif, &nif_bytes)?;
            stage_textures(
                &nif_bytes,
                data_root.as_path(),
                &archives,
                staging_dir.as_path(),
                &mut diagnostics,
            )?;
            let output = assets_dir.join(&asset_name);
            let cache_valid = output.exists() && validate_glb_images(&output).is_ok();
            match asset_cache_decision(output.exists(), cache_valid, args.rebuild_assets) {
                AssetCacheDecision::Reuse => cache_hits += 1,
                AssetCacheDecision::BuildMissing => {
                    cache_missing += 1;
                    jobs.push(BlenderAssetJob {
                        input: staging_nif,
                        output,
                        model: normalized_model.clone(),
                        conversion,
                    });
                }
                AssetCacheDecision::RebuildInvalid => {
                    cache_invalid += 1;
                    diagnostics.push(Diagnostic {
                        severity: "warning".into(),
                        message: format!(
                            "cached GLB {} failed validation; scheduling NIF reconversion",
                            output.display()
                        ),
                    });
                    jobs.push(BlenderAssetJob {
                        input: staging_nif,
                        output,
                        model: normalized_model.clone(),
                        conversion,
                    });
                }
                AssetCacheDecision::RebuildRequested => {
                    cache_explicit_rebuilds += 1;
                    jobs.push(BlenderAssetJob {
                        input: staging_nif,
                        output,
                        model: normalized_model.clone(),
                        conversion,
                    });
                }
            }
            seen_models.insert(normalized_model.clone(), asset_path.clone());
        }
        let mut placement = prepared_placement(
            &reference,
            Some(base),
            Some(asset_path),
            None,
            &parsed.bases,
        );
        placement.ao_mode = conversion_profile;
        placements.push(placement);
    }

    convert_staged_textures(&staging_dir, &mut diagnostics)?;
    let cache_summary = format!(
        "asset cache: reused {cache_hits}, missing {cache_missing}, invalid {cache_invalid}, explicitly rebuilt {cache_explicit_rebuilds}; scheduled {} NIF-to-GLB conversion(s)",
        jobs.len()
    );
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: cache_summary.clone(),
    });
    println!("{cache_summary}");
    if !jobs.is_empty() {
        run_blender_batch(&blender, &jobs, &data_root, &staging_dir)
            .context("headless Blender conversion failed")?;
    }
    let failures = placements.iter().filter(|p| p.error.is_some()).count();
    if args.strict && failures > 0 {
        bail!("strict preparation failed with {failures} unresolved placements")
    }
    if placements.iter().all(|p| p.asset_path.is_none()) && lights.is_empty() {
        bail!("no renderable assets were found in cell {cell_id:08x}")
    }

    let manifest = PreparedSceneManifest {
        schema_version: 7,
        asset_root: cache_dir.to_string_lossy().to_string(),
        source_plugin: plugin_path.to_string_lossy().to_string(),
        source_fingerprint,
        source_plugins,
        cell,
        placements,
        lights,
        diagnostics,
        navmeshes,
        cell_audio,
        audio_clips,
        footstep_sets,
        hard_landing_clips,
        bake: None,
    };
    let manifest_path = scene_dir.join("scene.ron");
    fs::write(
        &manifest_path,
        to_string_pretty(&manifest, PrettyConfig::default())?,
    )?;
    println!(
        "prepared {} placements ({} unresolved) -> {}",
        manifest.placements.len(),
        failures,
        manifest_path.display()
    );
    Ok(())
}

#[derive(Debug)]
struct LoadedPlugin {
    name: String,
    path: PathBuf,
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AssetCacheDecision {
    Reuse,
    BuildMissing,
    RebuildInvalid,
    RebuildRequested,
}

fn asset_cache_decision(
    output_exists: bool,
    cache_valid: bool,
    rebuild_assets: bool,
) -> AssetCacheDecision {
    if !output_exists {
        AssetCacheDecision::BuildMissing
    } else if !cache_valid {
        AssetCacheDecision::RebuildInvalid
    } else if rebuild_assets {
        AssetCacheDecision::RebuildRequested
    } else {
        AssetCacheDecision::Reuse
    }
}

fn load_plugin_chain(selected: &Path, data_root: &Path) -> Result<Vec<LoadedPlugin>> {
    fn visit(
        path: &Path,
        data_root: &Path,
        loaded: &mut Vec<LoadedPlugin>,
        loaded_names: &mut HashSet<String>,
        visiting: &mut HashSet<String>,
    ) -> Result<()> {
        let path = fs::canonicalize(path)
            .with_context(|| format!("plugin dependency does not exist: {}", path.display()))?;
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .context("plugin path has no UTF-8 filename")?
            .to_string();
        let key = name.to_ascii_lowercase();
        if loaded_names.contains(&key) {
            return Ok(());
        }
        if !visiting.insert(key.clone()) {
            bail!("cyclic Fallout plugin dependency involving {name}")
        }
        let bytes = fs::read(&path)
            .with_context(|| format!("failed to read plugin dependency {}", path.display()))?;
        for master in read_master_names(&bytes)? {
            visit(
                &data_root.join(master),
                data_root,
                loaded,
                loaded_names,
                visiting,
            )?;
        }
        visiting.remove(&key);
        loaded_names.insert(key);
        loaded.push(LoadedPlugin { name, path, bytes });
        Ok(())
    }

    let mut loaded = Vec::new();
    visit(
        selected,
        data_root,
        &mut loaded,
        &mut HashSet::new(),
        &mut HashSet::new(),
    )?;
    Ok(loaded)
}

fn content_set_fingerprint(plugins: &[LoadedPlugin]) -> String {
    let mut hasher = Sha256::new();
    for plugin in plugins {
        hasher.update(plugin.name.to_ascii_lowercase().as_bytes());
        hasher.update([0]);
        hasher.update((plugin.bytes.len() as u64).to_le_bytes());
        hasher.update(&plugin.bytes);
    }
    format!("{:x}", hasher.finalize())
}

#[derive(Debug, Clone)]
struct AudioDescriptor {
    source_path: String,
    editor_id: Option<String>,
    flags: u16,
    min_attenuation: u8,
    max_attenuation: u8,
    frequency_adjustment: i8,
    static_attenuation_hundredths_db: u16,
    looping: bool,
    is_2d: bool,
}

fn stage_audio(
    data_root: &Path,
    archives: &[super::audio_assets::AudioArchive],
    parsed: &ParsedPlugin,
    diagnostics: &mut Vec<Diagnostic>,
    audio_dir: &Path,
) -> Result<(PreparedCellAudio, Vec<PreparedAudioClip>)> {
    let metadata = parsed.cell_metadata.as_ref();
    let acoustic_space = metadata
        .and_then(|metadata| metadata.acoustic_space_form_id)
        .and_then(|form_id| parsed.acoustic_spaces.get(&form_id));
    let cell_audio = PreparedCellAudio {
        acoustic_space_form_id: metadata.and_then(|metadata| metadata.acoustic_space_form_id),
        acoustic_environment_type: acoustic_space.and_then(|space| space.environment_type),
        ambient_loop_sound_form_ids: acoustic_space
            .map(|space| space.ambient_loop_sound_form_ids.clone())
            .unwrap_or_default(),
        sound_region_form_id: acoustic_space.and_then(|space| space.sound_region_form_id),
        music_form_id: metadata.and_then(|metadata| metadata.music_form_id),
        music_path: metadata
            .and_then(|metadata| metadata.music_form_id)
            .and_then(|form_id| parsed.music.get(&form_id))
            .and_then(|music| music.file.clone()),
    };
    if let Some(form_id) = cell_audio.acoustic_space_form_id
        && acoustic_space.is_none()
    {
        diagnostics.push(Diagnostic {
            severity: "warning".into(),
            message: format!("cell acoustic space {form_id:08x} was not found"),
        });
    }
    if let Some(form_id) = cell_audio.music_form_id
        && cell_audio.music_path.is_none()
    {
        diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!("music record {form_id:08x} has no file path; playback deferred"),
        });
    }

    let mut form_ids = HashSet::new();
    form_ids.extend(cell_audio.ambient_loop_sound_form_ids.iter().copied());
    for reference in &parsed.references {
        let Some(base) = parsed.bases.get(&reference.base_form_id) else {
            continue;
        };
        let audio = &base.audio;
        form_ids.extend(
            [
                audio.loop_sound_form_id,
                audio.activation_sound_form_id,
                audio.open_sound_form_id,
                audio.close_sound_form_id,
                audio.pickup_sound_form_id,
                audio.drop_sound_form_id,
            ]
            .into_iter()
            .flatten(),
        );
    }

    let mut clips = Vec::new();
    let mut sorted_ids = form_ids.into_iter().collect::<Vec<_>>();
    sorted_ids.sort_unstable();
    for form_id in sorted_ids {
        let Some(descriptor) = resolve_audio_descriptor(parsed, form_id) else {
            diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!("sound record {form_id:08x} could not be resolved"),
            });
            continue;
        };
        let mut clip = PreparedAudioClip {
            form_id,
            editor_id: descriptor.editor_id,
            source_path: normalize_asset_path(&descriptor.source_path),
            asset_path: None,
            flags: descriptor.flags,
            min_attenuation: descriptor.min_attenuation,
            max_attenuation: descriptor.max_attenuation,
            frequency_adjustment: descriptor.frequency_adjustment,
            static_attenuation_hundredths_db: descriptor.static_attenuation_hundredths_db,
            looping: descriptor.looping,
            is_2d: descriptor.is_2d,
        };
        match resolve_audio_asset(data_root, archives, &clip.source_path) {
            Ok(Some(asset)) => {
                let staged = stage_audio_asset(&asset, audio_dir)?;
                clip.source_path = asset.source_path;
                clip.asset_path = Some(relative_cache_path(audio_dir, &staged.path));
            }
            Ok(None) => diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "missing audio {} for sound {:08x}",
                    clip.source_path, form_id
                ),
            }),
            Err(error) => diagnostics.push(Diagnostic {
                severity: "warning".into(),
                message: format!(
                    "could not read audio {} for sound {:08x}: {error}",
                    clip.source_path, form_id
                ),
            }),
        }
        clips.push(clip);
    }
    Ok((cell_audio, clips))
}

fn stage_footsteps(
    data_root: &Path,
    archives: &[super::audio_assets::AudioArchive],
    diagnostics: &mut Vec<Diagnostic>,
    audio_dir: &Path,
) -> Result<(Vec<super::manifest::PreparedFootstepSet>, Vec<String>)> {
    const FAMILIES: &[(&str, &str)] = &[
        ("concrete", "conc_solid"),
        ("concrete_broken", "conc_broken"),
        ("dirt", "dirt"),
        ("grass", "grass"),
        ("gravel", "gravel"),
        ("wood", "wood"),
        ("water", "water"),
        ("metal_solid", "metal_solid"),
        ("metal_hollow", "metal_hollow"),
        ("metal_sheet", "metal_sheet"),
    ];
    const LAND_PATTERNS: &[(&str, &str, &str, u8)] = &[
        ("concrete", "conc_solid", "fst_conc_solid_land", 2),
        ("concrete_broken", "conc_broken", "fst_conc_broken_land", 3),
        ("dirt", "dirt", "fst_dirt_land", 2),
        ("grass", "grass", "fst_grass_land", 2),
        ("gravel", "gravel", "fst_gravel_land", 2),
        ("wood", "wood", "fst_wood_land", 2),
        ("water", "water", "fst_waterland", 2),
        ("metal_solid", "metal_solid", "fst_metalsolid_land", 3),
        ("metal_hollow", "metal_hollow", "fst_metalhollow_land", 3),
        ("metal_sheet", "metal_sheet", "fst_metal_sheet_land", 3),
    ];
    let mut sets = Vec::with_capacity(FAMILIES.len());
    for &(surface, archive_family) in FAMILIES {
        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut land = Vec::new();
        for (side, output, indices) in [("left", &mut left, 1..=3), ("right", &mut right, 4..=6)] {
            for index in indices {
                let path = format!(
                    "sound/fx/fst/{archive_family}/walk/{side}/fst_{archive_family}_walk_{index:02}.wav"
                );
                match super::audio_assets::resolve_audio_asset(data_root, archives, &path)? {
                    Some(asset) => {
                        let staged = super::audio_assets::stage_audio_asset(&asset, audio_dir)?;
                        output.push(relative_cache_path(audio_dir, &staged.path));
                    }
                    None => diagnostics.push(Diagnostic {
                        severity: "info".into(),
                        message: format!("missing native footstep clip {path}"),
                    }),
                }
            }
        }
        let (_, land_archive_family, land_file_stem, land_count) = LAND_PATTERNS
            .iter()
            .find(|(land_surface, _, _, _)| *land_surface == surface)
            .expect("every footstep family has a landing pattern");
        for index in 1..=*land_count {
            let path =
                format!("sound/fx/fst/{land_archive_family}/land/{land_file_stem}_{index:02}.wav");
            match super::audio_assets::resolve_audio_asset(data_root, archives, &path)? {
                Some(asset) => {
                    let staged = super::audio_assets::stage_audio_asset(&asset, audio_dir)?;
                    land.push(relative_cache_path(audio_dir, &staged.path));
                }
                None => diagnostics.push(Diagnostic {
                    severity: "info".into(),
                    message: format!("missing native landing clip {path}"),
                }),
            }
        }
        if !left.is_empty() || !right.is_empty() || !land.is_empty() {
            sets.push(super::manifest::PreparedFootstepSet {
                surface: surface.into(),
                left,
                right,
                land,
            });
        }
    }
    let hard_path = "sound/fx/fst/landhard/fst_landhard_01.wav";
    let mut hard_landing_clips = Vec::new();
    match super::audio_assets::resolve_audio_asset(data_root, archives, hard_path)? {
        Some(asset) => {
            let staged = super::audio_assets::stage_audio_asset(&asset, audio_dir)?;
            hard_landing_clips.push(relative_cache_path(audio_dir, &staged.path));
        }
        None => diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!("missing native hard-landing clip {hard_path}"),
        }),
    }
    Ok((sets, hard_landing_clips))
}

fn relative_cache_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root.parent().unwrap_or(root))
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn resolve_audio_descriptor(parsed: &ParsedPlugin, form_id: u32) -> Option<AudioDescriptor> {
    fn visit(
        parsed: &ParsedPlugin,
        form_id: u32,
        visiting: &mut HashSet<u32>,
    ) -> Option<AudioDescriptor> {
        if !visiting.insert(form_id) {
            return None;
        }
        let result = if let Some(sound) = parsed.sounds.get(&form_id) {
            sound_descriptor(sound)
        } else if let Some(reference) = parsed.sound_references.get(&form_id) {
            sound_reference_descriptor(parsed, reference, visiting)
        } else {
            None
        };
        visiting.remove(&form_id);
        result
    }
    visit(parsed, form_id, &mut HashSet::new())
}

fn sound_descriptor(sound: &SoundRecord) -> Option<AudioDescriptor> {
    let params = sound.parameters.unwrap_or_default();
    Some(AudioDescriptor {
        source_path: sound.file.clone()?,
        editor_id: sound.editor_id.clone(),
        flags: params.flags,
        min_attenuation: params.min_attenuation,
        max_attenuation: params.max_attenuation,
        frequency_adjustment: params.frequency_adjustment,
        static_attenuation_hundredths_db: params.static_attenuation.unwrap_or_default(),
        looping: params.is_looping(),
        is_2d: params.is_two_dimensional(),
    })
}

fn sound_reference_descriptor(
    parsed: &ParsedPlugin,
    reference: &SoundReferenceRecord,
    visiting: &mut HashSet<u32>,
) -> Option<AudioDescriptor> {
    if let Some(source_path) = reference.file.clone() {
        let sound_info = reference
            .sound_info
            .unwrap_or(super::openmw_esm4::SoundInfo {
                frequency_adjustment: 0,
                frequency_variance: 0,
                priority: 0,
                decibel_variance: 0,
                static_attenuation: 0,
            });
        return Some(AudioDescriptor {
            source_path,
            editor_id: reference.editor_id.clone(),
            flags: reference.loop_info.map_or(0, |loop_info| loop_info.flags),
            min_attenuation: 0,
            max_attenuation: 0,
            frequency_adjustment: sound_info.frequency_adjustment,
            static_attenuation_hundredths_db: sound_info.static_attenuation,
            looping: reference
                .loop_info
                .is_some_and(|loop_info| loop_info.flags & 1 != 0),
            is_2d: false,
        });
    }
    reference.sound_reference_form_id.and_then(|form_id| {
        if !visiting.insert(form_id) {
            return None;
        }
        let resolved = parsed
            .sounds
            .get(&form_id)
            .and_then(sound_descriptor)
            .or_else(|| {
                parsed
                    .sound_references
                    .get(&form_id)
                    .and_then(|reference| sound_reference_descriptor(parsed, reference, visiting))
            });
        visiting.remove(&form_id);
        resolved
    })
}

fn stage_navmeshes(
    scene_dir: &Path,
    diagnostics: &mut Vec<Diagnostic>,
    navmeshes: &[super::openmw_esm4::NavMeshRecord],
) -> Result<Vec<PreparedNavMeshSource>> {
    if navmeshes.is_empty() {
        return Ok(Vec::new());
    }
    let navmesh_dir = scene_dir.join("navmesh");
    fs::create_dir_all(&navmesh_dir)?;
    navmeshes
        .iter()
        .map(|navmesh| {
            let filename = format!("{:08x}.navm.bin", navmesh.form_id);
            fs::write(navmesh_dir.join(&filename), &navmesh.payload)?;
            diagnostics.push(Diagnostic {
                severity: "info".into(),
                message: format!(
                    "retained FO3 NAVM {:08x} source metadata; runtime navigation is not decoded",
                    navmesh.form_id
                ),
            });
            Ok(PreparedNavMeshSource {
                form_id: navmesh.form_id,
                record_flags: navmesh.flags,
                version: navmesh.version,
                asset_path: format!(
                    "scenes/{}/navmesh/{filename}",
                    scene_dir
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or_default()
                ),
                chunks: navmesh
                    .chunks
                    .iter()
                    .map(|chunk| PreparedNavMeshChunk {
                        signature: chunk.signature.clone(),
                        byte_len: chunk.byte_len,
                    })
                    .collect(),
            })
        })
        .collect()
}

fn prepared_placement(
    reference: &ReferenceRecord,
    base: Option<&BaseRecord>,
    asset_path: Option<String>,
    error: Option<String>,
    base_records: &HashMap<u32, BaseRecord>,
) -> PreparedPlacement {
    let transform = placement_transform(reference);
    PreparedPlacement {
        reference_form_id: reference.form_id,
        base_form_id: reference.base_form_id,
        asset_path,
        translation: transform.0,
        rotation_xyzw: transform.1,
        scale: transform.2,
        error,
        reference_kind: reference.kind.as_str().into(),
        base_kind: base.map_or_else(|| "MISSING".into(), |base| base.kind.clone()),
        editor_id: base.and_then(|base| base.editor_id.clone()),
        display_name: base.and_then(|base| base.name.clone()),
        count: reference.count,
        semantic: prepared_semantic(reference, base),
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

fn prepared_inventory_entry(
    item: &super::openmw_esm4::InventoryItemRecord,
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

fn model_static_usage(
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

fn prepared_semantic(reference: &ReferenceRecord, base: Option<&BaseRecord>) -> PreparedSemantic {
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

fn info_image_space(
    diagnostics: &mut Vec<Diagnostic>,
    image_space: &super::manifest::ImageSpaceInfo,
) {
    diagnostics.push(Diagnostic {
        severity: "info".into(),
        message: format!(
            "resolved ImageSpace {:08x} ({}) eye_adapt_speed={:.3} target_lum={:.3}",
            image_space.form_id,
            image_space.editor_id.as_deref().unwrap_or("<unnamed>"),
            image_space.eye_adapt_speed,
            image_space.hdr_target_lum,
        ),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_cached_assets_are_reused_even_when_manifest_is_forced() {
        assert_eq!(
            asset_cache_decision(true, true, false),
            AssetCacheDecision::Reuse
        );
        assert_eq!(
            asset_cache_decision(true, true, true),
            AssetCacheDecision::RebuildRequested
        );
    }

    #[test]
    fn missing_and_invalid_cached_assets_are_rebuilt() {
        assert_eq!(
            asset_cache_decision(false, false, false),
            AssetCacheDecision::BuildMissing
        );
        assert_eq!(
            asset_cache_decision(true, false, false),
            AssetCacheDecision::RebuildInvalid
        );
        assert_eq!(
            asset_cache_decision(true, false, true),
            AssetCacheDecision::RebuildInvalid
        );
    }
}

//! Prepared audio and footstep staging.

use super::*;

pub(crate) fn stage_audio(
    data_root: &Path,
    archives: &[crate::vsa::audio_assets::AudioArchive],
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
    // M5/#235: the item catalog can equip a weapon that is not placed in the
    // startup cell. Stage its authored attack sounds alongside other prepared
    // clips so firing never depends on an incidental world placement.
    for base in parsed.bases.values().filter(|base| base.kind == "WEAP") {
        form_ids.extend(
            [
                base.audio.weapon_fire_3d_sound_form_id,
                base.audio.weapon_fire_2d_sound_form_id,
                weapon_reload_sound_form_id(parsed, base),
            ]
            .into_iter()
            .flatten(),
        );
    }

    let clips = stage_audio_clips(
        data_root,
        archives,
        parsed,
        diagnostics,
        audio_dir,
        form_ids,
    )?;
    Ok((cell_audio, clips))
}

/// Resolve the authored Fallout weapon reload cue from the weapon's fire
/// sound family. WEAP records carry fire sounds but no reload FormID; the
/// stock SOUN editor IDs provide the stable family relationship (for example
/// WPNPistol10mmFire2D -> WPNPistol10mmReloadOut).
fn weapon_reload_sound_form_id(
    parsed: &ParsedPlugin,
    base: &crate::vsa::openmw_esm4::BaseRecord,
) -> Option<u32> {
    let fire_form_id = base
        .audio
        .weapon_fire_2d_sound_form_id
        .or(base.audio.weapon_fire_3d_sound_form_id)?;
    let fire_editor_id = resolve_audio_descriptor(parsed, fire_form_id)?.editor_id?;
    let stem = fire_editor_id
        .strip_suffix("Fire2D")
        .or_else(|| fire_editor_id.strip_suffix("Fire3D"))?;
    [
        "Reload",
        "ReloadOut",
        "ReloadInOut",
        "ReloadIn",
        "ReloadChamber",
    ]
    .iter()
    .find_map(|suffix| {
        let candidate = format!("{stem}{suffix}");
        sound_form_ids_by_editor_id(parsed, &candidate)
            .into_iter()
            .next()
    })
}

pub(crate) fn stage_audio_clips(
    data_root: &Path,
    archives: &[crate::vsa::audio_assets::AudioArchive],
    parsed: &ParsedPlugin,
    diagnostics: &mut Vec<Diagnostic>,
    audio_dir: &Path,
    form_ids: impl IntoIterator<Item = u32>,
) -> Result<Vec<PreparedAudioClip>> {
    let mut clips = Vec::new();
    let mut sorted_ids = form_ids
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
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
    Ok(clips)
}

pub(crate) fn stage_footsteps(
    data_root: &Path,
    archives: &[crate::vsa::audio_assets::AudioArchive],
    diagnostics: &mut Vec<Diagnostic>,
    audio_dir: &Path,
) -> Result<(Vec<crate::vsa::manifest::PreparedFootstepSet>, Vec<String>)> {
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
                match crate::vsa::audio_assets::resolve_audio_asset(data_root, archives, &path)? {
                    Some(asset) => {
                        let staged =
                            crate::vsa::audio_assets::stage_audio_asset(&asset, audio_dir)?;
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
            match crate::vsa::audio_assets::resolve_audio_asset(data_root, archives, &path)? {
                Some(asset) => {
                    let staged = crate::vsa::audio_assets::stage_audio_asset(&asset, audio_dir)?;
                    land.push(relative_cache_path(audio_dir, &staged.path));
                }
                None => diagnostics.push(Diagnostic {
                    severity: "info".into(),
                    message: format!("missing native landing clip {path}"),
                }),
            }
        }
        if !left.is_empty() || !right.is_empty() || !land.is_empty() {
            sets.push(crate::vsa::manifest::PreparedFootstepSet {
                surface: surface.into(),
                left,
                right,
                land,
            });
        }
    }
    let hard_path = "sound/fx/fst/landhard/fst_landhard_01.wav";
    let mut hard_landing_clips = Vec::new();
    match crate::vsa::audio_assets::resolve_audio_asset(data_root, archives, hard_path)? {
        Some(asset) => {
            let staged = crate::vsa::audio_assets::stage_audio_asset(&asset, audio_dir)?;
            hard_landing_clips.push(relative_cache_path(audio_dir, &staged.path));
        }
        None => diagnostics.push(Diagnostic {
            severity: "info".into(),
            message: format!("missing native hard-landing clip {hard_path}"),
        }),
    }
    Ok((sets, hard_landing_clips))
}

pub(crate) fn relative_cache_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root.parent().unwrap_or(root))
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

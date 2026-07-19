//! Actor external-KF discovery, clip-pack conversion, and catalog caching.

use super::*;
use bevyout_core::actor_animation::{
    ActorAnimationAsset, ActorAnimationAssetState, ActorAnimationDiscoveryInput,
    PreparedActorAnimationCatalog, PreparedActorAnimationClip, PreparedActorAnimationClipStatus,
    PreparedActorAnimationDiagnostic, PreparedActorAnimationKind, PreparedActorAnimationSet,
    build_actor_animation_catalog, canonical_mesh_path,
};

pub(crate) const ACTOR_ANIMATION_CATALOG_REVISION: &str =
    "actor-animations-v3-normalized-runtime-contract";
pub(crate) const ACTOR_ANIMATION_CONVERTER_REVISION: &str =
    "niftools-external-kf-clip-pack-v6-source-metadata";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ActorAnimationCatalogArtifact {
    pub(crate) relative_path: String,
    pub(crate) hash: String,
    pub(crate) reused: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct ActorAnimationConversionSummary {
    pub(crate) built_packs: usize,
    pub(crate) reused_packs: usize,
    pub(crate) failed_clips: usize,
}

pub(crate) struct ActorAnimationConversionContext<'a> {
    pub(crate) converter: crate::converter_policy::ActorAnimationBackend,
    pub(crate) blender: Option<&'a Path>,
    pub(crate) data_root: &'a Path,
    pub(crate) archives: &'a [crate::vsa::bsa::BsaArchive],
    pub(crate) staging_dir: &'a Path,
    pub(crate) assets_dir: &'a Path,
    pub(crate) rebuild: bool,
}

fn collect_loose_kf_paths(data_root: &Path) -> Result<Vec<String>> {
    let meshes = data_root.join("Meshes");
    if !meshes.is_dir() {
        return Ok(Vec::new());
    }
    let mut pending = vec![meshes];
    let mut paths = Vec::new();
    while let Some(directory) = pending.pop() {
        let mut entries = fs::read_dir(&directory)?.collect::<Result<Vec<_>, _>>()?;
        entries.sort_by_key(std::fs::DirEntry::file_name);
        for entry in entries {
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                pending.push(entry.path());
                continue;
            }
            if !file_type.is_file()
                || !entry
                    .path()
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("kf"))
            {
                continue;
            }
            let entry_path = entry.path();
            let relative = entry_path.strip_prefix(data_root).with_context(|| {
                format!("loose KF path escaped Data root: {}", entry_path.display())
            })?;
            paths.push(canonical_mesh_path(&relative.to_string_lossy()));
        }
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn available_kf_paths(
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
) -> Result<Vec<String>> {
    let mut paths = collect_loose_kf_paths(data_root)?;
    paths.extend(
        archives
            .iter()
            .flat_map(|archive| archive.paths_with_extension("kf")),
    );
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn parent_directory(path: &str) -> String {
    canonical_mesh_path(path)
        .rsplit_once('/')
        .map_or_else(String::new, |(parent, _)| parent.to_owned())
}

fn discovery_inputs(
    actor_catalog: &PreparedActorCatalog,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
) -> Result<Vec<ActorAnimationDiscoveryInput>> {
    let mut output = Vec::new();
    for entry in &actor_catalog.entries {
        let ActorCatalogEntry::Prepared(actor) = entry else {
            continue;
        };
        let skeleton_path = actor
            .assembly
            .as_ref()
            .and_then(|assembly| assembly.skeleton_path.clone())
            .or_else(|| actor.model_path.clone())
            .unwrap_or_default();
        let skeleton_fingerprint = if skeleton_path.is_empty() {
            String::new()
        } else {
            resolve_asset(data_root, archives, &canonical_mesh_path(&skeleton_path))?
                .map_or_else(String::new, |bytes| fingerprint(&bytes))
        };
        let model_path = actor
            .model_path
            .clone()
            .filter(|path| !path.is_empty())
            .unwrap_or_else(|| skeleton_path.clone());
        output.push(ActorAnimationDiscoveryInput {
            reference_form_id: actor.reference_form_id,
            base_form_id: actor.base_form_id,
            kind: if actor.record_kind.eq_ignore_ascii_case("CREA") {
                PreparedActorAnimationKind::Creature
            } else {
                PreparedActorAnimationKind::Npc
            },
            model_path,
            skeleton_path: skeleton_path.clone(),
            skeleton_fingerprint,
            explicit_kf_paths: actor.animation_candidates.clone(),
            default_directories: (!skeleton_path.is_empty())
                .then(|| parent_directory(&skeleton_path))
                .into_iter()
                .filter(|directory| !directory.is_empty())
                .collect(),
        });
    }
    output.sort_by_key(|actor| actor.reference_form_id);
    Ok(output)
}

pub(crate) fn discover_actor_animation_catalog(
    actor_catalog: &PreparedActorCatalog,
    source_fingerprint: &str,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
) -> Result<PreparedActorAnimationCatalog> {
    let actors = discovery_inputs(actor_catalog, data_root, archives)?;
    let paths = available_kf_paths(data_root, archives)?;
    let path_index = paths
        .iter()
        .map(|path| ActorAnimationAsset {
            path: path.clone(),
            fingerprint: String::new(),
            state: ActorAnimationAssetState::Compatible,
        })
        .collect::<Vec<_>>();
    let draft = build_actor_animation_catalog(
        ACTOR_ANIMATION_CATALOG_REVISION,
        source_fingerprint,
        &actors,
        &path_index,
    );
    let needed = draft
        .animation_sets
        .iter()
        .flat_map(|set| set.clips.iter())
        .map(|clip| clip.source_kf_path.clone())
        .collect::<HashSet<_>>();
    let mut assets = Vec::new();
    for path in paths {
        if !needed.contains(&path) {
            continue;
        }
        let Some(bytes) = resolve_asset(data_root, archives, &path)? else {
            continue;
        };
        assets.push(ActorAnimationAsset {
            path,
            fingerprint: fingerprint(&bytes),
            state: ActorAnimationAssetState::Compatible,
        });
    }
    Ok(build_actor_animation_catalog(
        ACTOR_ANIMATION_CATALOG_REVISION,
        source_fingerprint,
        &actors,
        &assets,
    ))
}

fn animation_diagnostic(
    severity: &str,
    code: &str,
    source_path: Option<&str>,
    message: impl Into<String>,
) -> PreparedActorAnimationDiagnostic {
    PreparedActorAnimationDiagnostic {
        severity: severity.to_owned(),
        code: code.to_owned(),
        source_path: source_path.map(str::to_owned),
        message: message.into(),
    }
}

fn fail_clip(clip: &mut PreparedActorAnimationClip, code: &str, message: impl Into<String>) {
    let message = message.into();
    clip.status = PreparedActorAnimationClipStatus::ConversionFailed;
    clip.duration_seconds = None;
    clip.animated_channel_count = 0;
    clip.animated_target_count = 0;
    clip.missing_targets.clear();
    clip.diagnostics.push(animation_diagnostic(
        "warning",
        code,
        Some(&clip.source_kf_path),
        message,
    ));
}

fn mark_conversion_not_requested(catalog: &mut PreparedActorAnimationCatalog) {
    let message = "external KF clip-pack conversion was not requested; rerun prepare with --actor-animation-converter blender";
    catalog.diagnostics.push(animation_diagnostic(
        "info",
        "conversion_not_requested",
        None,
        message,
    ));
    for set in &mut catalog.animation_sets {
        set.clip_pack_asset_path = None;
        set.clip_pack_hash = None;
        set.diagnostics.push(animation_diagnostic(
            "info",
            "conversion_not_requested",
            Some(&set.skeleton_path),
            message,
        ));
        for clip in &mut set.clips {
            if clip.status == PreparedActorAnimationClipStatus::Ready {
                clip.status = PreparedActorAnimationClipStatus::NotConverted;
            }
        }
    }
}

fn stage_pack_job(
    set: &mut PreparedActorAnimationSet,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    staging_dir: &Path,
    assets_dir: &Path,
) -> Result<Option<(ActorAnimationPackJob, String)>> {
    let Some(skeleton_bytes) = resolve_asset(data_root, archives, &set.skeleton_path)? else {
        let message = format!("prepared skeleton asset is missing: {}", set.skeleton_path);
        set.diagnostics.push(animation_diagnostic(
            "warning",
            "missing_skeleton",
            Some(&set.skeleton_path),
            &message,
        ));
        for clip in &mut set.clips {
            if clip.status == PreparedActorAnimationClipStatus::Ready {
                fail_clip(clip, "missing_skeleton", &message);
            }
        }
        return Ok(None);
    };
    let mut source_clips = Vec::new();
    for (index, clip) in set.clips.iter_mut().enumerate() {
        if clip.status != PreparedActorAnimationClipStatus::Ready {
            continue;
        }
        let Some(bytes) = resolve_asset(data_root, archives, &clip.source_kf_path)? else {
            fail_clip(
                clip,
                "missing_kf",
                "KF disappeared after discovery and could not be staged",
            );
            continue;
        };
        source_clips.push((index, clip.name.clone(), clip.source_kf_path.clone(), bytes));
    }
    if source_clips.is_empty() {
        return Ok(None);
    }
    let identity_inputs = source_clips
        .iter()
        .map(|(_, name, path, bytes)| (name.as_str(), path.as_str(), bytes.as_slice()))
        .collect::<Vec<_>>();
    let identity = actor_animation_pack_fingerprint(
        ACTOR_ANIMATION_CONVERTER_REVISION,
        &set.skeleton_path,
        &skeleton_bytes,
        &identity_inputs,
    );
    let workspace = staging_dir.join("actor_animations").join(&identity);
    fs::create_dir_all(&workspace)?;
    let skeleton = workspace.join("skeleton.nif");
    if fs::read(&skeleton).ok().as_deref() != Some(skeleton_bytes.as_slice()) {
        fs::write(&skeleton, &skeleton_bytes)?;
    }
    let mut clips = Vec::new();
    for (index, name, source_path, bytes) in source_clips {
        let path = workspace.join(format!("{index:04}.kf"));
        if fs::read(&path).ok().as_deref() != Some(bytes.as_slice()) {
            fs::write(&path, bytes)?;
        }
        clips.push(ActorAnimationClipJob {
            name,
            source_path,
            path,
        });
    }
    let output = assets_dir.join(format!("{identity}.animations.glb"));
    let report = assets_dir.join(format!("{identity}.animations.json"));
    Ok(Some((
        ActorAnimationPackJob {
            revision: ACTOR_ANIMATION_CONVERTER_REVISION.to_owned(),
            skeleton_path: set.skeleton_path.clone(),
            skeleton,
            clips,
            output,
            report,
        },
        identity,
    )))
}

fn apply_pack_report(
    set: &mut PreparedActorAnimationSet,
    job: &ActorAnimationPackJob,
    identity: &str,
    report: &ActorAnimationPackReport,
) -> Result<()> {
    if report.revision != ACTOR_ANIMATION_CONVERTER_REVISION
        || report.skeleton_path != set.skeleton_path
    {
        bail!("actor animation report identity does not match its catalog set");
    }
    if let Some(error) = &report.pack_error {
        for clip in &mut set.clips {
            if clip.status == PreparedActorAnimationClipStatus::Ready {
                fail_clip(clip, "clip_pack_failed", error);
            }
        }
        set.diagnostics.push(animation_diagnostic(
            "warning",
            "clip_pack_failed",
            Some(&set.skeleton_path),
            error,
        ));
        return Ok(());
    }
    let reports = report
        .clips
        .iter()
        .map(|clip| ((clip.name.as_str(), clip.source_path.as_str()), clip))
        .collect::<HashMap<_, _>>();
    let successful_names = report
        .clips
        .iter()
        .filter(|clip| clip.success)
        .map(|clip| clip.name.clone())
        .collect::<HashSet<_>>();
    if successful_names.is_empty() {
        for clip in &mut set.clips {
            if clip.status == PreparedActorAnimationClipStatus::Ready {
                fail_clip(
                    clip,
                    "conversion_failed",
                    "clip pack report contains no compatible animation clips",
                );
            }
        }
        return Ok(());
    }
    if let Err(error) = validate_actor_animation_glb(&job.output, &successful_names) {
        let message = format!("animation-only GLB validation failed: {error}");
        for clip in &mut set.clips {
            if clip.status == PreparedActorAnimationClipStatus::Ready {
                fail_clip(clip, "invalid_clip_pack", &message);
            }
        }
        set.diagnostics.push(animation_diagnostic(
            "warning",
            "invalid_clip_pack",
            Some(&set.skeleton_path),
            message,
        ));
        return Ok(());
    }
    for clip in &mut set.clips {
        if clip.status != PreparedActorAnimationClipStatus::Ready {
            continue;
        }
        let Some(item) = reports.get(&(clip.name.as_str(), clip.source_kf_path.as_str())) else {
            fail_clip(
                clip,
                "missing_conversion_report",
                "Blender report omitted this discovered KF clip",
            );
            continue;
        };
        clip.duration_seconds = item.duration_seconds.filter(|value| value.is_finite());
        clip.source_sequence_name
            .clone_from(&item.source_sequence_name);
        clip.source_start_seconds = item.source_start_seconds.filter(|value| value.is_finite());
        clip.source_end_seconds = item.source_end_seconds.filter(|value| value.is_finite());
        clip.source_frequency = item.source_frequency.filter(|value| value.is_finite());
        clip.source_phase = item.source_phase.filter(|value| value.is_finite());
        clip.loop_mode = item.loop_mode;
        clip.root_motion_policy = item.root_motion_policy;
        clip.accumulation_root.clone_from(&item.accumulation_root);
        clip.animated_channel_count = item.animated_channel_count;
        clip.animated_target_count = item.animated_target_count;
        clip.required_targets.clone_from(&item.required_targets);
        clip.animated_targets.clone_from(&item.animated_targets);
        clip.missing_targets.clone_from(&item.missing_targets);
        clip.controller_types.clone_from(&item.controller_types);
        clip.interpolator_types.clone_from(&item.interpolator_types);
        clip.text_keys.clone_from(&item.text_keys);
        if !item.success {
            fail_clip(
                clip,
                "conversion_failed",
                item.error
                    .as_deref()
                    .unwrap_or("NIFTools could not apply this KF to the prepared skeleton"),
            );
            continue;
        }
        if !item.missing_targets.is_empty() {
            clip.diagnostics.push(animation_diagnostic(
                "warning",
                "missing_targets",
                Some(&clip.source_kf_path),
                format!(
                    "{} authored target(s) were absent from the prepared skeleton: {}",
                    item.missing_targets.len(),
                    item.missing_targets.join(", ")
                ),
            ));
        }
    }
    let bytes = fs::read(&job.output)?;
    set.clip_pack_asset_path = Some(format!("assets/{identity}.animations.glb"));
    set.clip_pack_hash = Some(fingerprint(&bytes));
    Ok(())
}

pub(crate) fn convert_actor_animation_catalog(
    catalog: &mut PreparedActorAnimationCatalog,
    context: &ActorAnimationConversionContext<'_>,
) -> Result<ActorAnimationConversionSummary> {
    if context.converter == crate::converter_policy::ActorAnimationBackend::Disabled {
        mark_conversion_not_requested(catalog);
        return Ok(ActorAnimationConversionSummary::default());
    }
    fs::create_dir_all(context.assets_dir)?;
    let mut pending = Vec::<(usize, ActorAnimationPackJob, String)>::new();
    let mut summary = ActorAnimationConversionSummary::default();
    for index in 0..catalog.animation_sets.len() {
        let Some((job, identity)) = stage_pack_job(
            &mut catalog.animation_sets[index],
            context.data_root,
            context.archives,
            context.staging_dir,
            context.assets_dir,
        )?
        else {
            continue;
        };
        let output_present = job.output.is_file();
        let report_present = job.report.is_file();
        let validation_passed = output_present
            && report_present
            && read_actor_animation_report(&job.report)
                .ok()
                .is_some_and(|report| {
                    let expected_reports = job
                        .clips
                        .iter()
                        .map(|clip| (clip.name.as_str(), clip.source_path.as_str()))
                        .collect::<HashSet<_>>();
                    let actual_reports = report
                        .clips
                        .iter()
                        .map(|clip| (clip.name.as_str(), clip.source_path.as_str()))
                        .collect::<HashSet<_>>();
                    let expected = report
                        .clips
                        .iter()
                        .filter(|clip| clip.success)
                        .map(|clip| clip.name.clone())
                        .collect::<HashSet<_>>();
                    report.revision == job.revision
                        && report.skeleton_path == job.skeleton_path
                        && expected_reports == actual_reports
                        && report.clips.len() == job.clips.len()
                        && report.pack_error.is_none()
                        && !expected.is_empty()
                        && validate_actor_animation_glb(&job.output, &expected).is_ok()
                });
        let cache_decision = actor_animation_pack_cache_decision(ActorAnimationPackCacheState {
            rebuild_requested: context.rebuild,
            output_present,
            report_present,
            validation_passed,
        });
        if cache_decision == ActorAnimationPackCacheDecision::Reuse {
            let report = read_actor_animation_report(&job.report)?;
            apply_pack_report(&mut catalog.animation_sets[index], &job, &identity, &report)?;
            summary.reused_packs += 1;
        } else {
            pending.push((index, job, identity));
        }
    }
    if !pending.is_empty() {
        let jobs = pending
            .iter()
            .map(|(_, job, _)| job.clone())
            .collect::<Vec<_>>();
        run_actor_animation_batch(
            context
                .blender
                .context("Blender backend was selected but no executable was resolved")?,
            &jobs,
            context.staging_dir,
        )?;
        for (index, job, identity) in pending {
            let report = read_actor_animation_report(&job.report)?;
            apply_pack_report(&mut catalog.animation_sets[index], &job, &identity, &report)?;
            summary.built_packs += 1;
        }
    }
    summary.failed_clips = catalog
        .animation_sets
        .iter()
        .flat_map(|set| &set.clips)
        .filter(|clip| clip.status == PreparedActorAnimationClipStatus::ConversionFailed)
        .count();
    Ok(summary)
}

pub(crate) fn write_actor_animation_catalog(
    cache_dir: &Path,
    cell_form_id: u32,
    catalog: &PreparedActorAnimationCatalog,
) -> Result<ActorAnimationCatalogArtifact> {
    let relative = PathBuf::from("scenes")
        .join(format!("{cell_form_id:08x}"))
        .join("actor_animations.ron");
    let path = cache_dir.join(&relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let serialized = to_string_pretty(catalog, PrettyConfig::default())?;
    let hash = fingerprint(serialized.as_bytes());
    let reused = fs::read(&path)
        .map(|existing| existing == serialized.as_bytes())
        .unwrap_or(false);
    if !reused {
        fs::write(&path, serialized)?;
    }
    Ok(ActorAnimationCatalogArtifact {
        relative_path: relative.to_string_lossy().replace('\\', "/"),
        hash,
        reused,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parent_directory_keeps_the_mesh_root() {
        assert_eq!(
            parent_directory("characters/_female/skeleton.nif"),
            "meshes/characters/_female"
        );
    }

    #[test]
    fn disabled_conversion_is_diagnostic_not_failure() {
        let mut catalog = build_actor_animation_catalog(
            ACTOR_ANIMATION_CATALOG_REVISION,
            "source",
            &[ActorAnimationDiscoveryInput {
                reference_form_id: 1,
                base_form_id: 2,
                model_path: "meshes/characters/_male/skeleton.nif".into(),
                skeleton_path: "meshes/characters/_male/skeleton.nif".into(),
                skeleton_fingerprint: "skeleton".into(),
                explicit_kf_paths: vec!["idle.kf".into()],
                ..Default::default()
            }],
            &[ActorAnimationAsset {
                path: "meshes/characters/_male/idle.kf".into(),
                fingerprint: "idle".into(),
                state: ActorAnimationAssetState::Compatible,
            }],
        );
        let summary = convert_actor_animation_catalog(
            &mut catalog,
            &ActorAnimationConversionContext {
                converter: crate::converter_policy::ActorAnimationBackend::Disabled,
                blender: None,
                data_root: Path::new("unused-data"),
                archives: &[],
                staging_dir: Path::new("unused-staging"),
                assets_dir: Path::new("unused-assets"),
                rebuild: false,
            },
        )
        .expect("disabled conversion must not touch tools or files");

        assert_eq!(summary, ActorAnimationConversionSummary::default());
        assert_eq!(
            catalog.animation_sets[0].clips[0].status,
            PreparedActorAnimationClipStatus::NotConverted
        );
        assert!(
            catalog.animation_sets[0]
                .diagnostics
                .iter()
                .any(|diagnostic| {
                    diagnostic.code == "conversion_not_requested" && diagnostic.severity == "info"
                })
        );
    }
}

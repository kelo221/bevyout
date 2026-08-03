//! Bounded in-process native NIF conversion for prepared static assets.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

use bevyout_core::facegen::{FaceGenAssetKind, FaceGenDiagnostic};

use crate::cli::NifConversionMode;
use crate::vsa::nif_convert::{
    ActorSceneConversionRequest, NifConversionRequest, NifConversionResult, convert_actor_scene,
    convert_nif,
};

use super::*;

#[derive(Debug)]
pub(crate) struct NativeBatchResult {
    pub(crate) outcomes: Vec<NativeJobOutcome>,
}

impl NativeBatchResult {
    pub(crate) fn summary(&self) -> NativeBatchSummary {
        summarize_native_jobs(&self.outcomes)
    }

    pub(crate) fn failed_outputs(&self, jobs: &[AssetJob]) -> HashMap<PathBuf, String> {
        self.outcomes
            .iter()
            .filter(|outcome| outcome.status != NativeJobStatus::Converted)
            .filter_map(|outcome| {
                jobs.get(outcome.index).map(|job| {
                    (
                        job.output.clone(),
                        outcome
                            .error
                            .clone()
                            .unwrap_or_else(|| "native conversion failed".into()),
                    )
                })
            })
            .collect()
    }

    pub(crate) fn enforce_strict(&self, strict: bool) -> Result<()> {
        let summary = self.summary();
        if strict && summary.has_failures() {
            bail!(
                "native conversion strict mode rejected {} failed and {} unsupported job(s)",
                summary.failed,
                summary.unsupported
            );
        }
        Ok(())
    }
}

pub(crate) fn run_native_batch(
    jobs: &[AssetJob],
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    requested_workers: Option<usize>,
    strict: bool,
) -> Result<NativeBatchResult> {
    reject_duplicate_native_outputs(jobs)?;
    let worker_count = native_worker_count(requested_workers, jobs.len());
    println!(
        "native conversion: workers={} assets={}",
        worker_count,
        jobs.len()
    );
    let outcomes = run_bounded(jobs, worker_count, |index, job| {
        convert_native_job(index, job, data_root, archives, strict)
    });
    let outcomes = sorted_native_outcomes(&outcomes);
    Ok(NativeBatchResult { outcomes })
}

fn convert_native_job(
    index: usize,
    job: &AssetJob,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    strict: bool,
) -> NativeJobOutcome {
    let result = match job.kind {
        AssetJobKind::StaticNif => fs::read(&job.input)
            .with_context(|| format!("reading staged NIF {}", job.input.display()))
            .and_then(|nif_bytes| {
                convert_nif(NifConversionRequest {
                    source_name: &job.model,
                    nif_bytes: &nif_bytes,
                    output: &job.output,
                    physics_output: Some(&job.physics_output),
                    report: None,
                    conversion: match job.conversion {
                        super::super::assets::AssetConversion::Preserve => {
                            NifConversionMode::Preserve
                        }
                        super::super::assets::AssetConversion::QuickAo => {
                            NifConversionMode::QuickAo
                        }
                        super::super::assets::AssetConversion::WorldspaceLod => {
                            NifConversionMode::WorldspaceLod
                        }
                    },
                    root_transform_policy: job.root_transform_policy,
                    allow_lossy: !strict,
                    force: true,
                    data_root: Some(data_root),
                    archives,
                })
                .context("native NIF conversion")
            }),
        AssetJobKind::ActorAssembly => convert_native_actor(job, data_root, archives, strict),
    }
    .and_then(|conversion| {
        validate_asset_cache_pair(&job.output, &job.physics_output)
            .context("validating native GLB/physics output")?;
        validate_glb_images(&job.output).context("validating native GLB textures")?;
        Ok(conversion)
    });

    match result {
        Ok(conversion) => NativeJobOutcome {
            index,
            model: job.model.clone(),
            status: NativeJobStatus::Converted,
            stage: "complete".into(),
            error: (!conversion.missing_textures.is_empty() || conversion.lossy_scene_issues != 0)
                .then(|| {
                    format!(
                        "lossy issues={} missing textures={}",
                        conversion.lossy_scene_issues,
                        conversion.missing_textures.len()
                    )
                }),
        },
        Err(error) => {
            remove_failed_native_outputs(job);
            let message = format!("{error:#}");
            NativeJobOutcome {
                index,
                model: job.model.clone(),
                status: NativeJobStatus::Failed,
                stage: native_error_stage(&message).into(),
                error: Some(message),
            }
        }
    }
}

fn convert_native_actor(
    job: &AssetJob,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    strict: bool,
) -> Result<NifConversionResult> {
    let descriptor_bytes = fs::read(&job.input)
        .with_context(|| format!("reading actor assembly {}", job.input.display()))?;
    let descriptor: ActorAssemblyDescriptor = serde_json::from_slice(&descriptor_bytes)
        .with_context(|| format!("parsing actor assembly {}", job.input.display()))?;
    let skeleton_bytes = fs::read(&descriptor.skeleton)
        .with_context(|| format!("reading actor skeleton {}", descriptor.skeleton))?;
    let skeleton_document = nif::fo3::parse(&skeleton_bytes)
        .with_context(|| format!("parsing actor skeleton {}", descriptor.skeleton))?;
    let mut actor = nif::fo3::extract_scene(&skeleton_document)
        .with_context(|| format!("extracting actor skeleton scene {}", descriptor.skeleton))?;
    super::super::nif_convert::apply_native_material_policy(&skeleton_document, &mut actor)?;

    let apparel = descriptor
        .apparel
        .iter()
        .map(|item| (actor_path_key(&item.path), item.biped_slot_mask))
        .collect::<HashMap<_, _>>();
    let body_parts = descriptor
        .body_parts
        .iter()
        .map(|part| (actor_path_key(&part.path), part.index))
        .collect::<HashMap<_, _>>();
    let head_part_keys = descriptor
        .head_parts
        .iter()
        .map(|path| actor_path_key(path))
        .collect::<Vec<_>>();
    let head_parts = head_part_keys.iter().cloned().collect::<HashSet<_>>();
    let head_anim_parts = descriptor
        .head_anim_parts
        .iter()
        .map(|path| actor_path_key(path))
        .collect::<HashSet<_>>();
    let head_anchor = head_part_keys.first();
    let mut decoded = Vec::new();
    let mut warnings = Vec::new();
    let skeleton_key = actor_path_key(&descriptor.skeleton);
    for path in &descriptor.visual_inputs {
        let key = actor_path_key(path);
        if key == skeleton_key || actor_helper_geometry(path) {
            continue;
        }
        match extract_actor_part(path) {
            Ok(scene) if scene.has_visible_geometry() => decoded.push((path.clone(), key, scene)),
            Ok(_) => warnings.push(format!("{} contributed no visible geometry", path)),
            Err(error) => warnings.push(format!("{} could not be decoded: {error:#}", path)),
        }
    }

    let mut extra_textures = BTreeMap::new();
    let mut facegen_texture_replacement = None;
    if let Some(facegen) = descriptor.facegen.as_ref()
        && let Some(anchor_key) = head_anchor
        && let Some((path, _, scene)) = decoded.iter_mut().find(|(_, key, _)| key == anchor_key)
    {
        match apply_native_facegen(scene, facegen, data_root, archives, &mut extra_textures) {
            Ok(replacement) => facegen_texture_replacement = Some(replacement),
            Err(error) => warnings.push(format!(
                "actor head FaceGen {} could not be applied; retaining authored rest pose: {error}",
                path
            )),
        }
    }

    let mut occupied_slots = 0u32;
    for (path, key, scene) in decoded
        .iter()
        .filter(|(_, key, _)| apparel.contains_key(key))
    {
        if !scene.has_visible_weighted_geometry() {
            warnings.push(format!(
                "apparel {} contributed no visible weighted geometry; retaining body fallback",
                path
            ));
            continue;
        }
        match nif::fo3::merge_actor_scene(&mut actor, scene) {
            Ok(()) => occupied_slots |= apparel[key],
            Err(error) => warnings.push(format!(
                "apparel {} could not bind to the shared skeleton: {}; retaining body fallback",
                path, error
            )),
        }
    }

    for (path, _key, scene) in decoded.iter().filter(|(_, key, _)| {
        !apparel.contains_key(key) && !body_parts.contains_key(key) && !head_parts.contains(key)
    }) {
        if let Err(error) = nif::fo3::merge_actor_scene(&mut actor, scene) {
            warnings.push(format!(
                "actor visual {} could not bind to the shared skeleton: {}",
                path, error
            ));
        }
    }

    if let Some(anchor_key) = head_anchor
        && let Some((path, _, scene)) = decoded.iter().find(|(_, key, _)| key == anchor_key)
        && let Err(error) = nif::fo3::merge_actor_scene(&mut actor, scene)
    {
        warnings.push(format!(
            "actor head anchor {} could not bind to the shared skeleton: {}",
            path, error
        ));
    }
    for (path, key, scene) in decoded.iter().filter(|(_, key, _)| {
        head_parts.contains(key) && head_anchor.is_none_or(|anchor| key != anchor)
    }) {
        let attachment = head_part_attachment(head_anim_parts.contains(key));
        if let Err(error) = nif::fo3::merge_actor_scene_attached(&mut actor, scene, attachment) {
            warnings.push(format!(
                "actor head visual {} could not bind to the shared skeleton: {}",
                path, error
            ));
        }
    }

    for (path, key, scene) in decoded
        .iter()
        .filter(|(_, key, _)| body_parts.contains_key(key))
    {
        let slot = match body_parts[key] {
            0 => 0x0000_0004,
            1 => 0x0000_0008,
            2 => 0x0000_0010,
            _ => 0,
        };
        if slot != 0 && occupied_slots & slot != 0 {
            continue;
        }
        if let Err(error) = nif::fo3::merge_actor_scene(&mut actor, scene) {
            warnings.push(format!(
                "body fallback {} could not bind to the shared skeleton: {}",
                path, error
            ));
        }
    }
    if let Some(replacement) = facegen_texture_replacement {
        for material in &mut actor.materials {
            if material.diffuse_texture.as_deref() == Some(replacement.source_path.as_str()) {
                material.diffuse_texture = Some(replacement.generated_path.clone());
            }
        }
    }
    actor.animations.clear();
    let mut result = convert_actor_scene(ActorSceneConversionRequest {
        source_name: &job.model,
        scene: actor,
        skeleton_document: &skeleton_document,
        output: &job.output,
        physics_output: &job.physics_output,
        allow_lossy: !strict,
        data_root: Some(data_root),
        archives,
        extra_textures,
    })?;
    result.lossy_scene_issues += warnings.len();
    result.lines.extend(
        warnings
            .into_iter()
            .map(|warning| format!("native actor warning: {warning}")),
    );
    Ok(result)
}

struct FaceGenTextureReplacement {
    source_path: String,
    generated_path: String,
}

fn apply_native_facegen(
    scene: &mut nif::fo3::Scene,
    descriptor: &ActorFaceGenDescriptor,
    data_root: &Path,
    archives: &[crate::vsa::bsa::BsaArchive],
    extra_textures: &mut BTreeMap<String, Vec<u8>>,
) -> Result<FaceGenTextureReplacement, bevyout_core::facegen::FaceGenDiagnostic> {
    let geometry_bytes = resolve_asset(data_root, archives, &descriptor.geometry_morph_path)
        .map_err(|error| FaceGenDiagnostic::UnsupportedAsset {
            asset: FaceGenAssetKind::GeometryMorph,
            reason: format!("reading {}: {error:#}", descriptor.geometry_morph_path),
        })?
        .ok_or_else(|| FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::GeometryMorph,
            path: descriptor.geometry_morph_path.clone(),
        })?;
    let texture_bytes = resolve_asset(data_root, archives, &descriptor.texture_morph_path)
        .map_err(|error| FaceGenDiagnostic::UnsupportedAsset {
            asset: FaceGenAssetKind::TextureMorph,
            reason: format!("reading {}: {error:#}", descriptor.texture_morph_path),
        })?
        .ok_or_else(|| FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::TextureMorph,
            path: descriptor.texture_morph_path.clone(),
        })?;
    let geometry = parse_geometry_morph(&geometry_bytes)?;
    let texture = parse_texture_morph(&texture_bytes)?;
    let mut candidate = scene.clone();
    apply_geometry_morph(
        &mut candidate,
        &geometry,
        &descriptor.resolved.coefficients,
        descriptor.base_vertex_count,
    )?;

    let diffuse_path = candidate
        .materials
        .iter()
        .find_map(|material| material.diffuse_texture.clone())
        .ok_or_else(|| FaceGenDiagnostic::UnsupportedAsset {
            asset: FaceGenAssetKind::TextureMorph,
            reason: "selected head has no diffuse texture".into(),
        })?;
    let base_bytes = resolve_asset(data_root, archives, &diffuse_path)
        .map_err(|error| FaceGenDiagnostic::UnsupportedAsset {
            asset: FaceGenAssetKind::TextureMorph,
            reason: format!("reading {diffuse_path}: {error:#}"),
        })?
        .ok_or_else(|| FaceGenDiagnostic::MissingAsset {
            asset: FaceGenAssetKind::TextureMorph,
            path: diffuse_path.clone(),
        })?;
    let synthesized =
        synthesize_head_diffuse(&base_bytes, &texture, &descriptor.resolved.coefficients)?;
    let generated_path = facegen_texture_key(&diffuse_path, &descriptor.resolved.coefficients);
    for material in &mut candidate.materials {
        if material.diffuse_texture.as_deref() == Some(diffuse_path.as_str()) {
            material.diffuse_texture = Some(generated_path.clone());
        }
    }
    // Keep the generated identity as the preferred material key, but also
    // alias the original diffuse path.  The native scene merge can retain an
    // equivalent material slot from the shared skeleton; aliasing that key
    // guarantees the authored head diffuse is replaced by the synthesized
    // pixels without changing any body, hair, eye, mouth, tooth, or apparel
    // texture references.
    extra_textures.insert(generated_path.clone(), synthesized.clone());
    extra_textures.insert(diffuse_path.clone(), synthesized);
    *scene = candidate;
    Ok(FaceGenTextureReplacement {
        source_path: diffuse_path,
        generated_path,
    })
}

fn extract_actor_part(path: &str) -> Result<nif::fo3::Scene> {
    let bytes = fs::read(path).with_context(|| format!("reading actor visual {path}"))?;
    let document =
        nif::fo3::parse(&bytes).with_context(|| format!("parsing actor visual {path}"))?;
    let mut scene = nif::fo3::extract_scene(&document)
        .with_context(|| format!("extracting actor visual {path}"))?;
    super::super::nif_convert::apply_native_material_policy(&document, &mut scene)?;
    Ok(scene)
}

fn actor_path_key(path: &str) -> String {
    path.replace('\\', "/").to_ascii_lowercase()
}

fn actor_helper_geometry(path: &str) -> bool {
    let key = actor_path_key(path);
    ["blowaway", "gore", "gib", "meatcap", "meat_cap"]
        .iter()
        .any(|marker| key.contains(marker))
}

/// Skeleton node a head accessory parents under during actor assembly (#160,
/// #206). FO3 hair (a `head_anim_part`) is authored directly in the animated
/// `HeadAnims` frame, so it must ride the `HeadAnims` skeleton node — that is
/// the node the idle KF's `HeadAnims` target binds against at runtime. Every
/// other head part (eyes, mouth, teeth, tongue) rides the static `Bip01 Head`
/// bone. Attaching hair to `Bip01 Head` instead would leave the assembled
/// actor without a `HeadAnims`-anchored accessory and the animation binding
/// reporting a missing required target.
fn head_part_attachment(is_head_anim_part: bool) -> &'static str {
    if is_head_anim_part {
        "HeadAnims"
    } else {
        "Bip01 Head"
    }
}

fn native_error_stage(message: &str) -> &'static str {
    if message.contains("reading staged NIF") {
        "input"
    } else if message.contains("parsing FO3/FNV NIF") {
        "parse"
    } else if message.contains("extracting NIF scene") {
        "extract_scene"
    } else if message.contains("encoding self-contained GLB") {
        "encode_glb"
    } else if message.contains("extracting authored Havok collision") {
        "extract_physics"
    } else if message.contains("validating native") {
        "validate_output"
    } else {
        "convert"
    }
}

fn remove_failed_native_outputs(job: &AssetJob) {
    for path in [&job.output, &job.physics_output] {
        if path.is_file() {
            let _ = fs::remove_file(path);
        }
        if let Some(parent) = path.parent()
            && let Some(file_name) = path.file_name().and_then(|name| name.to_str())
            && let Ok(entries) = fs::read_dir(parent)
        {
            for entry in entries.flatten() {
                if entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with(file_name) && name.contains(".tmp-"))
                {
                    let _ = fs::remove_file(entry.path());
                }
            }
        }
    }
}

fn reject_duplicate_native_outputs(jobs: &[AssetJob]) -> Result<()> {
    let mut outputs = HashSet::new();
    for job in jobs {
        for path in [&job.output, &job.physics_output] {
            if !outputs.insert(path.clone()) {
                bail!(
                    "native conversion jobs contain duplicate output path {}",
                    path.display()
                );
            }
        }
    }
    Ok(())
}

fn run_bounded<T, R, F>(items: &[T], worker_count: usize, work: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(usize, &T) -> R + Sync,
{
    let next_index = AtomicUsize::new(0);
    let results = Mutex::new(Vec::with_capacity(items.len()));
    std::thread::scope(|scope| {
        for _ in 0..worker_count.max(1).min(items.len().max(1)) {
            scope.spawn(|| {
                loop {
                    let index = next_index.fetch_add(1, Ordering::SeqCst);
                    let Some(item) = items.get(index) else {
                        break;
                    };
                    let result = work(index, item);
                    results.lock().unwrap().push((index, result));
                }
            });
        }
    });
    let mut results = results.into_inner().expect("native worker mutex poisoned");
    results.sort_by_key(|(index, _)| *index);
    results.into_iter().map(|(_, result)| result).collect()
}

#[cfg(test)]
#[path = "tests/native.rs"]
mod tests;

//! External-KF animation-only GLB conversion.

use std::collections::{BTreeMap, HashMap, HashSet};

use glam::{EulerRot, Quat};

use super::*;
use bevyout_core::actor_animation::{
    PreparedActorAnimationLoopMode, PreparedActorAnimationRootMotionPolicy,
    PreparedActorAnimationTextKey,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ActorAnimationClipJob {
    pub(crate) name: String,
    pub(crate) source_path: String,
    pub(crate) path: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct ActorAnimationPackJob {
    pub(crate) revision: String,
    pub(crate) skeleton_path: String,
    pub(crate) skeleton: PathBuf,
    pub(crate) clips: Vec<ActorAnimationClipJob>,
    pub(crate) output: PathBuf,
    pub(crate) report: PathBuf,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ActorAnimationClipReport {
    pub(crate) name: String,
    pub(crate) source_path: String,
    pub(crate) success: bool,
    pub(crate) duration_seconds: Option<f32>,
    #[serde(default)]
    pub(crate) source_sequence_name: Option<String>,
    #[serde(default)]
    pub(crate) source_start_seconds: Option<f32>,
    #[serde(default)]
    pub(crate) source_end_seconds: Option<f32>,
    #[serde(default)]
    pub(crate) source_frequency: Option<f32>,
    #[serde(default)]
    pub(crate) source_phase: Option<f32>,
    #[serde(default)]
    pub(crate) loop_mode: PreparedActorAnimationLoopMode,
    #[serde(default)]
    pub(crate) root_motion_policy: PreparedActorAnimationRootMotionPolicy,
    #[serde(default)]
    pub(crate) accumulation_root: Option<String>,
    pub(crate) animated_channel_count: usize,
    pub(crate) animated_target_count: usize,
    #[serde(default)]
    pub(crate) required_targets: Vec<String>,
    #[serde(default)]
    pub(crate) animated_targets: Vec<String>,
    #[serde(default)]
    pub(crate) missing_targets: Vec<String>,
    #[serde(default)]
    pub(crate) controller_types: Vec<String>,
    #[serde(default)]
    pub(crate) interpolator_types: Vec<String>,
    #[serde(default)]
    pub(crate) text_keys: Vec<PreparedActorAnimationTextKey>,
    pub(crate) error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct ActorAnimationPackReport {
    pub(crate) revision: String,
    pub(crate) skeleton_path: String,
    pub(crate) clips: Vec<ActorAnimationClipReport>,
    pub(crate) pack_error: Option<String>,
}

pub(crate) fn actor_animation_conversion_script() -> &'static str {
    include_str!("actor_animation_script.py")
}

/// Content identity for one reusable clip pack. The key deliberately includes
/// the exact skeleton and KF bytes, normalized source paths, stable clip names,
/// and converter policy revision.
pub(crate) fn actor_animation_pack_fingerprint(
    revision: &str,
    skeleton_path: &str,
    skeleton_bytes: &[u8],
    clips: &[(&str, &str, &[u8])],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(revision.as_bytes());
    hasher.update([0]);
    hasher.update(skeleton_path.as_bytes());
    hasher.update([0]);
    hasher.update((skeleton_bytes.len() as u64).to_le_bytes());
    hasher.update(skeleton_bytes);
    for (name, source_path, bytes) in clips {
        hasher.update(name.as_bytes());
        hasher.update([0]);
        hasher.update(source_path.as_bytes());
        hasher.update([0]);
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    format!("{:x}", hasher.finalize())
}

pub(crate) fn run_actor_animation_batch(
    blender: &Path,
    jobs: &[ActorAnimationPackJob],
    staging_dir: &Path,
) -> Result<()> {
    if jobs.is_empty() {
        return Ok(());
    }
    let job_identity = fingerprint(&serde_json::to_vec(jobs)?);
    let workspace = staging_dir.join("actor_animations");
    fs::create_dir_all(&workspace)?;
    let job_file = workspace.join(format!("jobs-{job_identity}.json"));
    let script_file = workspace.join(format!("script-{job_identity}.py"));
    fs::write(&job_file, serde_json::to_vec(jobs)?)?;
    fs::write(&script_file, actor_animation_conversion_script())?;
    let result = Command::new(blender)
        .arg("--background")
        .arg("--factory-startup")
        .arg("--python")
        .arg(&script_file)
        .arg("--")
        .arg(&job_file)
        .output();
    let _ = fs::remove_file(&script_file);
    let _ = fs::remove_file(&job_file);
    let result = result?;
    if !result.status.success() {
        let stdout_tail = output_tail(&result.stdout, 120);
        let stderr_tail = output_tail(&result.stderr, 120);
        bail!(
            "actor animation Blender conversion exited with {}\nstdout tail:\n{}\nstderr tail:\n{}",
            result.status,
            stdout_tail,
            stderr_tail
        );
    }
    for job in jobs {
        if !job.report.is_file() {
            bail!(
                "actor animation Blender conversion did not create report {}",
                job.report.display()
            );
        }
    }
    Ok(())
}

/// Builds an animation-only GLB directly from the native FO3 parser. External
/// KF files contain controller sequences but no scene roots, so the shared
/// skeleton supplies the target hierarchy while Nifty supplies the authored
/// channels and text keys.
pub(crate) fn run_native_actor_animation_batch(jobs: &[ActorAnimationPackJob]) -> Result<()> {
    for job in jobs {
        let report = native_actor_animation_pack(job)?;
        let mut bytes = serde_json::to_vec(&report)?;
        bytes.push(b'\n');
        fs::write(&job.report, bytes)?;
    }
    Ok(())
}

fn native_actor_animation_pack(job: &ActorAnimationPackJob) -> Result<ActorAnimationPackReport> {
    let skeleton_bytes = fs::read(&job.skeleton).with_context(|| {
        format!(
            "reading native animation skeleton {}",
            job.skeleton.display()
        )
    })?;
    let skeleton_document =
        nif::fo3::parse(&skeleton_bytes).context("parsing native animation skeleton")?;
    let mut skeleton = nif::fo3::extract_scene(&skeleton_document)
        .context("extracting native animation skeleton")?;
    for node in &mut skeleton.nodes {
        node.mesh = None;
        node.skin = None;
    }
    skeleton.materials.clear();
    skeleton.skins.clear();
    skeleton.statistics = nif::fo3::SceneStatistics::default();
    skeleton.animations.clear();
    skeleton.animation_sound_cues.clear();

    let mut report = ActorAnimationPackReport {
        revision: job.revision.clone(),
        skeleton_path: job.skeleton_path.clone(),
        clips: Vec::with_capacity(job.clips.len()),
        pack_error: None,
    };
    let mut animations = Vec::new();
    for clip in &job.clips {
        match native_actor_animation_clip(clip, &skeleton) {
            Ok(converted) => {
                report.clips.push(converted.report);
                animations.push(converted.animation);
            }
            Err(error) => report.clips.push(ActorAnimationClipReport {
                name: clip.name.clone(),
                source_path: clip.source_path.clone(),
                success: false,
                duration_seconds: None,
                source_sequence_name: None,
                source_start_seconds: None,
                source_end_seconds: None,
                source_frequency: None,
                source_phase: None,
                loop_mode: PreparedActorAnimationLoopMode::Unknown,
                root_motion_policy: PreparedActorAnimationRootMotionPolicy::Unknown,
                accumulation_root: None,
                animated_channel_count: 0,
                animated_target_count: 0,
                required_targets: Vec::new(),
                animated_targets: Vec::new(),
                missing_targets: Vec::new(),
                controller_types: Vec::new(),
                interpolator_types: Vec::new(),
                text_keys: Vec::new(),
                error: Some(format!("{error:#}")),
            }),
        }
    }
    if animations.is_empty() {
        report.pack_error = Some("no compatible KF clips were imported".into());
        let _ = fs::remove_file(&job.output);
        return Ok(report);
    }
    skeleton.animations = animations;
    let output = nif::fo3::encode_glb(
        &skeleton,
        &BTreeMap::new(),
        &nif::fo3::GlbOptions {
            source_name: job.skeleton_path.clone(),
            allow_missing_textures: true,
        },
    )
    .context("encoding native animation GLB")?;
    fs::write(&job.output, output.bytes)?;
    Ok(report)
}

struct NativeActorAnimationClip {
    animation: nif::fo3::SceneAnimation,
    report: ActorAnimationClipReport,
}

fn native_actor_animation_clip(
    clip: &ActorAnimationClipJob,
    skeleton: &nif::fo3::Scene,
) -> Result<NativeActorAnimationClip> {
    let bytes = fs::read(&clip.path)
        .with_context(|| format!("reading native KF {}", clip.path.display()))?;
    let document = nif::fo3::parse(&bytes)
        .with_context(|| format!("parsing native KF {}", clip.source_path))?;
    let nodes = skeleton
        .nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (animation_node_key(&node.name), index))
        .collect::<HashMap<_, _>>();
    let mut required_targets = HashSet::new();
    let mut animated_targets = HashSet::new();
    let mut missing_targets = HashSet::new();
    let mut controller_types = HashSet::new();
    let mut interpolator_types = HashSet::new();
    let mut unsupported_interpolator_types = HashSet::new();
    let mut text_keys = Vec::new();
    let mut start = f32::INFINITY;
    let mut end = f32::NEG_INFINITY;
    let mut sequence_names = HashSet::new();
    let mut frequencies = Vec::new();
    let mut phases = Vec::new();
    let mut loop_modes = Vec::new();
    let mut accumulation_roots = HashSet::new();
    let mut channels = HashMap::<usize, nif::fo3::SceneAnimationChannel>::new();

    for (block_index, block) in document.blocks.iter().enumerate() {
        if block.type_name != "NiControllerSequence" {
            continue;
        }
        let sequence = native_controller_sequence(&document, block_index)
            .with_context(|| format!("decoding KF controller sequence {block_index}"))?;
        sequence_names.insert(sequence.name.clone());
        start = start.min(sequence.start_time);
        end = end.max(sequence.stop_time);
        frequencies.push(sequence.frequency);
        if let Some(phase) = sequence.phase {
            phases.push(phase);
        }
        loop_modes.push(sequence.loop_mode);
        if let Some(root) = sequence.accumulation_root {
            accumulation_roots.insert(root);
        }
        if sequence.text_keys >= 0
            && let Ok(nif::fo3::TypedBlock::TextKeyExtraData(extra)) =
                document.decode_block(sequence.text_keys as usize)
        {
            text_keys.extend(extra.keys.into_iter().filter_map(|key| {
                key.time
                    .is_finite()
                    .then_some(PreparedActorAnimationTextKey {
                        time_seconds: key.time,
                        value: key.value,
                    })
            }));
        }
        for controlled in sequence.controlled_blocks {
            let target_name = controlled
                .node_name
                .split_once(':')
                .map_or(controlled.node_name.as_str(), |(name, _)| name)
                .trim();
            if target_name.is_empty() {
                continue;
            }
            required_targets.insert(target_name.to_owned());
            if controlled.controller >= 0
                && let Some(index) = document.blocks.get(controlled.controller as usize)
            {
                controller_types.insert(index.type_name.clone());
            }
            if !controlled.controller_type.is_empty() {
                controller_types.insert(controlled.controller_type.clone());
            }
            let interpolator_type = controlled
                .interpolator
                .try_into()
                .ok()
                .and_then(|index: usize| document.blocks.get(index));
            if let Some(index) = interpolator_type {
                interpolator_types.insert(index.type_name.clone());
            }
            if !controlled.interpolator_id.is_empty() {
                interpolator_types.insert(controlled.interpolator_id.clone());
            }
            let node = nodes.get(&animation_node_key(target_name)).copied();
            let Some(node) = node else {
                missing_targets.insert(target_name.to_owned());
                continue;
            };
            let Some(data) = native_transform_data(&document, controlled.interpolator)? else {
                if let Some(index) = interpolator_type {
                    unsupported_interpolator_types.insert(index.type_name.clone());
                }
                continue;
            };
            let channel = channels
                .entry(node)
                .or_insert_with(|| nif::fo3::SceneAnimationChannel {
                    node,
                    translations: Vec::new(),
                    rotations: Vec::new(),
                    scales: Vec::new(),
                });
            native_merge_animation_data(channel, data);
            animated_targets.insert(target_name.to_owned());
        }
    }
    if channels.is_empty() {
        let unsupported = sorted_strings(unsupported_interpolator_types);
        if unsupported.is_empty() {
            bail!("KF produced no supported native transform channels");
        }
        bail!(
            "KF produced no supported native transform channels (interpolators: {})",
            unsupported.join(", ")
        );
    }
    for channel in channels.values() {
        end = end.max(
            channel
                .translations
                .iter()
                .map(|key| key.time)
                .chain(channel.rotations.iter().map(|key| key.time))
                .chain(channel.scales.iter().map(|key| key.time))
                .fold(f32::NEG_INFINITY, f32::max),
        );
    }
    let start = if start.is_finite() { start } else { 0.0 };
    let end = if end.is_finite() {
        end.max(start)
    } else {
        start
    };
    let mut channels = channels.into_values().collect::<Vec<_>>();
    channels.sort_by_key(|channel| channel.node);
    let required_targets = sorted_strings(required_targets);
    let animated_targets = sorted_strings(animated_targets);
    let missing_targets = sorted_strings(missing_targets);
    text_keys.sort_by(|left, right| {
        left.time_seconds
            .total_cmp(&right.time_seconds)
            .then_with(|| left.value.cmp(&right.value))
    });
    let report = ActorAnimationClipReport {
        name: clip.name.clone(),
        source_path: clip.source_path.clone(),
        success: true,
        duration_seconds: Some(end - start),
        source_sequence_name: (sequence_names.len() == 1)
            .then(|| sequence_names.into_iter().next().unwrap_or_default()),
        source_start_seconds: Some(start),
        source_end_seconds: Some(end),
        source_frequency: unique_finite(frequencies),
        source_phase: unique_finite(phases),
        loop_mode: native_loop_mode(&loop_modes),
        root_motion_policy: PreparedActorAnimationRootMotionPolicy::PreserveAuthored,
        accumulation_root: (accumulation_roots.len() == 1)
            .then(|| accumulation_roots.into_iter().next().unwrap_or_default()),
        animated_channel_count: channels
            .iter()
            .map(|channel| {
                usize::from(!channel.translations.is_empty())
                    + usize::from(!channel.rotations.is_empty())
                    + usize::from(!channel.scales.is_empty())
            })
            .sum(),
        animated_target_count: animated_targets.len(),
        required_targets,
        animated_targets,
        missing_targets,
        controller_types: sorted_strings(controller_types),
        interpolator_types: sorted_strings(interpolator_types),
        text_keys,
        error: None,
    };
    Ok(NativeActorAnimationClip {
        animation: nif::fo3::SceneAnimation {
            name: clip.name.clone(),
            start_time: start,
            stop_time: end,
            channels,
        },
        report,
    })
}

struct NativeControllerSequence {
    name: String,
    start_time: f32,
    stop_time: f32,
    text_keys: i32,
    controlled_blocks: Vec<nif::fo3::ControlledBlock>,
    frequency: f32,
    phase: Option<f32>,
    loop_mode: PreparedActorAnimationLoopMode,
    accumulation_root: Option<String>,
}

fn native_controller_sequence(
    document: &nif::fo3::Document,
    block_index: usize,
) -> Result<NativeControllerSequence> {
    let block = document
        .blocks
        .get(block_index)
        .with_context(|| format!("missing KF sequence block {block_index}"))?;
    let mut reader = NativeByteReader::new(&block.bytes);
    let name = string_ref(document, reader.i32("sequence name")?)
        .unwrap_or_else(|| format!("sequence#{block_index}"));
    let count = reader.u32("controlled block count")? as usize;
    let _array_grow_by = reader.u32("controlled block array grow by")?;
    let mut controlled_blocks = Vec::with_capacity(count);
    for _ in 0..count {
        let interpolator = reader.i32("controlled block interpolator")?;
        let controller = reader.i32("controlled block controller")?;
        let _priority = reader.u8("controlled block priority")?;
        let node_name =
            string_ref(document, reader.i32("controlled block node name")?).unwrap_or_default();
        let property_type =
            string_ref(document, reader.i32("controlled block property type")?).unwrap_or_default();
        let controller_type = string_ref(document, reader.i32("controlled block controller type")?)
            .unwrap_or_default();
        let controller_id =
            string_ref(document, reader.i32("controlled block controller id")?).unwrap_or_default();
        let interpolator_id = string_ref(document, reader.i32("controlled block interpolator id")?)
            .unwrap_or_default();
        controlled_blocks.push(nif::fo3::ControlledBlock {
            interpolator,
            controller,
            node_name,
            property_type,
            controller_type,
            controller_id,
            interpolator_id,
        });
    }
    let _weight = reader.f32("sequence weight")?;
    let text_keys = reader.i32("sequence text keys")?;
    let cycle_type = reader.u32("sequence cycle type")?;
    let frequency = reader.f32("sequence frequency")?;
    let start_time = reader.f32("sequence start time")?;
    let stop_time = reader.f32("sequence stop time")?;
    let _manager = reader.i32("sequence manager")?;
    let accumulation_root = string_ref(document, reader.i32("sequence accumulation root")?);
    // Nifty's typed decoder rejects some otherwise valid FO3 sequences when
    // Bethesda appends an opaque four-byte tail. The animation adapter only
    // needs the authored sequence fields, so consume the optional note array
    // when it is structurally present and deliberately ignore any tail.
    if document.header.bethesda.version > 28 && reader.remaining() >= 2 {
        let note_count = reader.u16("animation note array count")? as usize;
        let note_bytes = note_count.saturating_mul(4);
        if note_bytes <= reader.remaining() {
            reader.skip(note_bytes, "animation note array")?;
        }
    }
    Ok(NativeControllerSequence {
        name,
        start_time,
        stop_time,
        text_keys,
        controlled_blocks,
        frequency,
        phase: None,
        loop_mode: match cycle_type {
            0 => PreparedActorAnimationLoopMode::Loop,
            1 => PreparedActorAnimationLoopMode::Reverse,
            2 => PreparedActorAnimationLoopMode::Clamp,
            _ => PreparedActorAnimationLoopMode::Unknown,
        },
        accumulation_root,
    })
}

fn native_transform_data(
    document: &nif::fo3::Document,
    interpolator: i32,
) -> Result<Option<nif::fo3::TransformData>> {
    if interpolator < 0 {
        return Ok(None);
    }
    let block = document
        .blocks
        .get(interpolator as usize)
        .with_context(|| format!("missing transform interpolator {interpolator}"))?;
    if matches!(
        block.type_name.as_str(),
        "NiBSplineCompTransformInterpolator" | "NiBSplineTransformInterpolator"
    ) {
        return native_bspline_transform_data(document, interpolator as usize);
    }
    let nif::fo3::TypedBlock::TransformInterpolator(interpolator) = document
        .decode_block(interpolator as usize)
        .with_context(|| format!("decoding transform interpolator {interpolator}"))?
    else {
        return Ok(None);
    };
    if interpolator.data < 0 {
        return Ok(None);
    }
    let nif::fo3::TypedBlock::TransformData(data) = document
        .decode_block(interpolator.data as usize)
        .with_context(|| format!("decoding transform data {}", interpolator.data))?
    else {
        return Ok(None);
    };
    Ok(Some(data))
}

fn native_bspline_transform_data(
    document: &nif::fo3::Document,
    interpolator_index: usize,
) -> Result<Option<nif::fo3::TransformData>> {
    let block = document
        .blocks
        .get(interpolator_index)
        .with_context(|| format!("missing B-spline interpolator {interpolator_index}"))?;
    let compressed = block.type_name == "NiBSplineCompTransformInterpolator";
    let mut reader = NativeByteReader::new(&block.bytes);
    let start = reader.f32("B-spline start time")?;
    let stop = reader.f32("B-spline stop time")?;
    let spline_data = reader.i32("B-spline spline data")?;
    let basis_data = reader.i32("B-spline basis data")?;
    let _translation = [
        reader.f32("B-spline base translation x")?,
        reader.f32("B-spline base translation y")?,
        reader.f32("B-spline base translation z")?,
    ];
    let _rotation = [
        reader.f32("B-spline base rotation w")?,
        reader.f32("B-spline base rotation x")?,
        reader.f32("B-spline base rotation y")?,
        reader.f32("B-spline base rotation z")?,
    ];
    let _scale = reader.f32("B-spline base scale")?;
    let translation_handle = reader.u32("B-spline translation handle")?;
    let rotation_handle = reader.u32("B-spline rotation handle")?;
    let scale_handle = reader.u32("B-spline scale handle")?;
    let (
        translation_offset,
        translation_half_range,
        rotation_offset,
        rotation_half_range,
        scale_offset,
        scale_half_range,
    ) = if compressed {
        (
            reader.f32("B-spline translation offset")?,
            reader.f32("B-spline translation half range")?,
            reader.f32("B-spline rotation offset")?,
            reader.f32("B-spline rotation half range")?,
            reader.f32("B-spline scale offset")?,
            reader.f32("B-spline scale half range")?,
        )
    } else {
        (0.0, 1.0, 0.0, 1.0, 0.0, 1.0)
    };
    if spline_data < 0 || basis_data < 0 {
        return Ok(None);
    }
    let spline = document
        .blocks
        .get(spline_data as usize)
        .with_context(|| format!("missing B-spline data {spline_data}"))?;
    if spline.type_name != "NiBSplineData" {
        return Ok(None);
    }
    let mut spline_reader = NativeByteReader::new(&spline.bytes);
    let float_count = spline_reader.u32("B-spline float control point count")? as usize;
    let mut floats = Vec::with_capacity(float_count);
    for _ in 0..float_count {
        floats.push(spline_reader.f32("B-spline float control point")?);
    }
    let compact_count = spline_reader.u32("B-spline compact control point count")? as usize;
    let mut compact = Vec::with_capacity(compact_count);
    for _ in 0..compact_count {
        compact.push(spline_reader.i16("B-spline compact control point")?);
    }
    let basis = document
        .blocks
        .get(basis_data as usize)
        .with_context(|| format!("missing B-spline basis data {basis_data}"))?;
    if basis.type_name != "NiBSplineBasisData" {
        return Ok(None);
    }
    let mut basis_reader = NativeByteReader::new(&basis.bytes);
    let control_points = basis_reader.u32("B-spline control point count")? as usize;
    if control_points == 0 || control_points > 100_000 {
        return Ok(None);
    }
    let data = NativeSplineData { floats, compact };
    let translations = native_spline_vec3_keys(
        &data,
        NativeSplineSpec {
            handle: translation_handle,
            control_points,
            compressed,
            offset: translation_offset,
            half_range: translation_half_range,
            start,
            stop,
        },
    )?;
    let rotations = native_spline_quat_keys(
        &data,
        NativeSplineSpec {
            handle: rotation_handle,
            control_points,
            compressed,
            offset: rotation_offset,
            half_range: rotation_half_range,
            start,
            stop,
        },
    )?;
    let scales = native_spline_scalar_group(
        &data,
        NativeSplineSpec {
            handle: scale_handle,
            control_points,
            compressed,
            offset: scale_offset,
            half_range: scale_half_range,
            start,
            stop,
        },
    )?;
    Ok(Some(nif::fo3::TransformData {
        rotations,
        xyz_rotations: None,
        translations,
        scales,
    }))
}

struct NativeSplineData {
    floats: Vec<f32>,
    compact: Vec<i16>,
}

#[derive(Clone, Copy)]
struct NativeSplineSpec {
    handle: u32,
    control_points: usize,
    compressed: bool,
    offset: f32,
    half_range: f32,
    start: f32,
    stop: f32,
}

const NATIVE_BSPLINE_DEGREE: usize = 3;

fn native_spline_times(spec: NativeSplineSpec) -> impl Iterator<Item = f32> {
    // NiBSplineBasisData stores frames + degree - 1 control points. The
    // authored animation therefore has control_points - degree + 1 samples.
    // Sampling those frame times preserves the source timing while evaluating
    // the open, uniform basis instead of exposing control points as keys.
    let sample_count = spec
        .control_points
        .saturating_sub(NATIVE_BSPLINE_DEGREE)
        .saturating_add(1)
        .max(2);
    (0..sample_count).map(move |index| {
        if sample_count <= 1 {
            spec.start
        } else {
            spec.start + (spec.stop - spec.start) * index as f32 / (sample_count - 1) as f32
        }
    })
}

fn native_spline_values(
    data: &NativeSplineData,
    components: usize,
    spec: NativeSplineSpec,
) -> Option<Vec<Vec<f32>>> {
    if spec.handle == u16::MAX as u32 || !spec.offset.is_finite() || !spec.half_range.is_finite() {
        return None;
    }
    let handle = spec.handle as usize;
    let count = spec.control_points.checked_mul(components)?;
    if spec.compressed {
        let values = data.compact.get(handle..handle.checked_add(count)?)?;
        Some(
            values
                .chunks_exact(components)
                .map(|values| {
                    values
                        .iter()
                        // Keep compact values normalized until after the
                        // weighted spline sum. Applying the offset to every
                        // control point before evaluation is incorrect.
                        .map(|value| f32::from(*value) / 32767.0)
                        .collect()
                })
                .collect(),
        )
    } else {
        Some(
            data.floats
                .get(handle..handle.checked_add(count)?)?
                .chunks_exact(components)
                .map(|values| values.to_vec())
                .collect(),
        )
    }
}

fn native_spline_knot(index: usize, control_points: usize) -> f32 {
    let n = control_points.saturating_sub(1);
    let order = NATIVE_BSPLINE_DEGREE + 1;
    if index < order {
        0.0
    } else if index <= n {
        (index - order + 1) as f32
    } else {
        control_points.saturating_sub(NATIVE_BSPLINE_DEGREE) as f32
    }
}

fn native_spline_basis(index: usize, order: usize, value: f32, control_points: usize) -> f32 {
    if order == 1 {
        let left = native_spline_knot(index, control_points);
        let right = native_spline_knot(index + 1, control_points);
        let end = native_spline_knot(control_points + NATIVE_BSPLINE_DEGREE, control_points);
        return if (left <= value && value < right)
            || (value >= end && index + 1 == control_points + NATIVE_BSPLINE_DEGREE)
        {
            1.0
        } else {
            0.0
        };
    }
    let left_denominator = native_spline_knot(index + order - 1, control_points)
        - native_spline_knot(index, control_points);
    let right_denominator = native_spline_knot(index + order, control_points)
        - native_spline_knot(index + 1, control_points);
    let left = if left_denominator == 0.0 {
        0.0
    } else {
        (value - native_spline_knot(index, control_points)) / left_denominator
            * native_spline_basis(index, order - 1, value, control_points)
    };
    let right = if right_denominator == 0.0 {
        0.0
    } else {
        (native_spline_knot(index + order, control_points) - value) / right_denominator
            * native_spline_basis(index + 1, order - 1, value, control_points)
    };
    left + right
}

#[cfg(test)]
fn native_spline_sample(
    data: &NativeSplineData,
    components: usize,
    spec: NativeSplineSpec,
    time: f32,
) -> Option<Vec<f32>> {
    let controls = native_spline_values(data, components, spec)?;
    native_spline_sample_controls(&controls, spec, time)
}

fn native_spline_sample_controls(
    controls: &[Vec<f32>],
    spec: NativeSplineSpec,
    time: f32,
) -> Option<Vec<f32>> {
    if controls.is_empty() {
        return None;
    }
    let span_count = spec.control_points.saturating_sub(NATIVE_BSPLINE_DEGREE);
    let interval = if spec.stop > spec.start {
        ((time - spec.start) / (spec.stop - spec.start) * span_count as f32)
            .clamp(0.0, span_count as f32)
    } else {
        0.0
    };
    let mut output = vec![0.0; controls.first()?.len()];
    if interval >= span_count as f32 {
        output.copy_from_slice(controls.last()?);
    } else {
        let span = interval.floor() as usize + NATIVE_BSPLINE_DEGREE;
        let first = span.saturating_sub(NATIVE_BSPLINE_DEGREE);
        for (index, control) in controls
            .iter()
            .enumerate()
            .skip(first)
            .take(NATIVE_BSPLINE_DEGREE + 1)
        {
            let weight = native_spline_basis(
                index,
                NATIVE_BSPLINE_DEGREE + 1,
                interval,
                spec.control_points,
            );
            for (output, value) in output.iter_mut().zip(control) {
                *output += value * weight;
            }
        }
    }
    if spec.compressed {
        for value in &mut output {
            *value = *value * spec.half_range + spec.offset;
        }
    }
    output
        .iter()
        .all(|value| value.is_finite())
        .then_some(output)
}

fn native_spline_vec3_keys(
    data: &NativeSplineData,
    spec: NativeSplineSpec,
) -> Result<nif::fo3::AnimationKeyGroup<[f32; 3]>> {
    let Some(controls) = native_spline_values(data, 3, spec) else {
        return Ok(nif::fo3::AnimationKeyGroup {
            interpolation: Some(nif::fo3::KeyType::Linear),
            keys: Vec::new(),
        });
    };
    let keys = native_spline_times(spec)
        .filter_map(|time| {
            native_spline_sample_controls(&controls, spec, time).map(|value| (time, value))
        })
        .filter_map(|(time, value)| {
            (value.iter().all(|value| value.is_finite())).then_some(nif::fo3::AnimationKey {
                time,
                value: [value[0], value[1], value[2]],
            })
        })
        .collect();
    Ok(nif::fo3::AnimationKeyGroup {
        interpolation: Some(nif::fo3::KeyType::Linear),
        keys,
    })
}

fn native_spline_quat_keys(
    data: &NativeSplineData,
    spec: NativeSplineSpec,
) -> Result<Vec<nif::fo3::AnimationKey<[f32; 4]>>> {
    let Some(controls) = native_spline_values(data, 4, spec) else {
        return Ok(Vec::new());
    };
    let keys = native_spline_times(spec)
        .filter_map(|time| {
            native_spline_sample_controls(&controls, spec, time).map(|value| (time, value))
        })
        .filter_map(|(time, value)| {
            if !value.iter().all(|value| value.is_finite()) {
                return None;
            }
            let [w, x, y, z] = value.as_slice() else {
                return None;
            };
            let quaternion = Quat::from_xyzw(*x, *y, *z, *w);
            let quaternion = if quaternion.length_squared() > f32::EPSILON {
                quaternion.normalize()
            } else {
                Quat::IDENTITY
            };
            let [x, y, z, w] = quaternion.to_array();
            Some(nif::fo3::AnimationKey {
                time,
                value: [w, x, y, z],
            })
        })
        .collect();
    Ok(keys)
}

fn native_spline_scalar_group(
    data: &NativeSplineData,
    spec: NativeSplineSpec,
) -> Result<nif::fo3::AnimationKeyGroup<f32>> {
    let Some(controls) = native_spline_values(data, 1, spec) else {
        return Ok(nif::fo3::AnimationKeyGroup {
            interpolation: Some(nif::fo3::KeyType::Linear),
            keys: Vec::new(),
        });
    };
    let keys = native_spline_times(spec)
        .filter_map(|time| {
            native_spline_sample_controls(&controls, spec, time).map(|value| (time, value))
        })
        .filter_map(|(time, value)| {
            value
                .first()
                .copied()
                .filter(|value| value.is_finite())
                .map(|value| nif::fo3::AnimationKey { time, value })
        })
        .collect();
    Ok(nif::fo3::AnimationKeyGroup {
        interpolation: Some(nif::fo3::KeyType::Linear),
        keys,
    })
}

fn native_merge_animation_data(
    channel: &mut nif::fo3::SceneAnimationChannel,
    data: nif::fo3::TransformData,
) {
    let nif::fo3::TransformData {
        rotations,
        xyz_rotations,
        translations,
        scales,
    } = data;
    channel.rotations.extend(rotations.into_iter().map(|key| {
        let [w, x, y, z] = key.value;
        nif::fo3::AnimationKey {
            time: key.time,
            value: [x, y, z, w],
        }
    }));
    if let Some(xyz_rotations) = xyz_rotations {
        channel
            .rotations
            .extend(native_xyz_rotation_keys(&xyz_rotations));
    }
    channel.translations.extend(translations.keys);
    channel.scales.extend(scales.keys);
}

fn native_xyz_rotation_keys(
    groups: &[nif::fo3::AnimationKeyGroup<f32>; 3],
) -> Vec<nif::fo3::AnimationKey<[f32; 4]>> {
    let mut times = groups
        .iter()
        .flat_map(|group| group.keys.iter().map(|key| key.time))
        .collect::<Vec<_>>();
    times.sort_by(f32::total_cmp);
    times.dedup_by(|left, right| (*left - *right).abs() <= 1.0e-6);
    times
        .into_iter()
        .map(|time| nif::fo3::AnimationKey {
            time,
            value: Quat::from_euler(
                EulerRot::XYZ,
                native_sample_scalar_group(&groups[0], time),
                native_sample_scalar_group(&groups[1], time),
                native_sample_scalar_group(&groups[2], time),
            )
            .to_array(),
        })
        .collect()
}

fn native_sample_scalar_group(group: &nif::fo3::AnimationKeyGroup<f32>, time: f32) -> f32 {
    let Some(first) = group.keys.first() else {
        return 0.0;
    };
    if time <= first.time {
        return first.value;
    }
    let Some(last) = group.keys.last() else {
        return first.value;
    };
    if time >= last.time {
        return last.value;
    }
    for pair in group.keys.windows(2) {
        let [left, right] = pair else { continue };
        if time <= right.time {
            let span = right.time - left.time;
            let weight = if span.abs() <= f32::EPSILON {
                0.0
            } else {
                (time - left.time) / span
            };
            return left.value + (right.value - left.value) * weight;
        }
    }
    last.value
}

fn animation_node_key(value: &str) -> String {
    let value = value
        .split_once(':')
        .map_or(value, |(name, _)| name)
        .trim()
        .to_ascii_lowercase();
    let Some((prefix, side)) = value.rsplit_once('.') else {
        return value;
    };
    if !matches!(side, "l" | "r") {
        return value;
    }
    let Some((root, bone)) = prefix.split_once(' ') else {
        return value;
    };
    if root != "bip01" {
        return value;
    }
    format!("{root} {side} {bone}")
}

fn string_ref(document: &nif::fo3::Document, index: i32) -> Option<String> {
    (index >= 0)
        .then(|| document.header.strings.get(index as usize))
        .flatten()
        .cloned()
        .filter(|value| !value.trim().is_empty())
}

fn sorted_strings(values: HashSet<String>) -> Vec<String> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort_by(|left, right| {
        left.to_ascii_lowercase()
            .cmp(&right.to_ascii_lowercase())
            .then_with(|| left.cmp(right))
    });
    values
}

fn unique_finite(values: Vec<f32>) -> Option<f32> {
    let mut values = values.into_iter().filter(|value| value.is_finite());
    let first = values.next()?;
    values.all(|value| value == first).then_some(first)
}

fn native_loop_mode(values: &[PreparedActorAnimationLoopMode]) -> PreparedActorAnimationLoopMode {
    let unique = values.iter().copied().collect::<HashSet<_>>();
    if unique.len() == 1 {
        unique
            .into_iter()
            .next()
            .unwrap_or(PreparedActorAnimationLoopMode::Unknown)
    } else if unique.is_empty() {
        PreparedActorAnimationLoopMode::Unknown
    } else {
        PreparedActorAnimationLoopMode::Mixed
    }
}

struct NativeByteReader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> NativeByteReader<'a> {
    const fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, length: usize, field: &str) -> Result<&'a [u8]> {
        let end = self
            .offset
            .checked_add(length)
            .with_context(|| format!("overflow reading {field}"))?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .with_context(|| format!("truncated {field}"))?;
        self.offset = end;
        Ok(bytes)
    }

    fn u8(&mut self, field: &str) -> Result<u8> {
        Ok(self.take(1, field)?[0])
    }

    fn u32(&mut self, field: &str) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4, field)?.try_into()?))
    }

    fn i32(&mut self, field: &str) -> Result<i32> {
        Ok(i32::from_le_bytes(self.take(4, field)?.try_into()?))
    }

    fn f32(&mut self, field: &str) -> Result<f32> {
        Ok(f32::from_le_bytes(self.take(4, field)?.try_into()?))
    }

    fn u16(&mut self, field: &str) -> Result<u16> {
        Ok(u16::from_le_bytes(self.take(2, field)?.try_into()?))
    }

    fn i16(&mut self, field: &str) -> Result<i16> {
        Ok(i16::from_le_bytes(self.take(2, field)?.try_into()?))
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.offset)
    }

    fn skip(&mut self, length: usize, field: &str) -> Result<()> {
        self.take(length, field).map(|_| ())
    }
}

fn output_tail(bytes: &[u8], count: usize) -> String {
    String::from_utf8_lossy(bytes)
        .lines()
        .rev()
        .take(count)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn read_actor_animation_report(path: &Path) -> Result<ActorAnimationPackReport> {
    serde_json::from_slice(&fs::read(path)?)
        .with_context(|| format!("invalid actor animation report {}", path.display()))
}

/// Validate the cached output at the contract Bevy consumes: unique named
/// clips, at least one channel per clip, valid target nodes, and finite input
/// timestamps.
pub(crate) fn validate_actor_animation_glb(
    path: &Path,
    expected_names: &HashSet<String>,
) -> Result<()> {
    let bytes = fs::read(path)?;
    let gltf = gltf::Gltf::from_slice(&bytes)
        .with_context(|| format!("invalid actor animation GLB {}", path.display()))?;
    let blob = gltf
        .blob
        .as_deref()
        .context("animation GLB has no binary buffer")?;
    let mut names = HashSet::new();
    for animation in gltf.document.animations() {
        let name = animation
            .name()
            .filter(|name| !name.is_empty())
            .context("animation clip has no stable name")?;
        if !names.insert(name.to_owned()) {
            bail!("duplicate animation clip name '{name}'");
        }
        let mut channels = 0_usize;
        for channel in animation.channels() {
            channels += 1;
            let _target = channel.target().node();
            let reader = channel.reader(|buffer| match buffer.source() {
                gltf::buffer::Source::Bin => Some(blob),
                gltf::buffer::Source::Uri(_) => None,
            });
            let mut inputs = reader
                .read_inputs()
                .context("animation channel has no input timestamps")?;
            if !inputs.all(f32::is_finite) {
                bail!("animation '{name}' has non-finite input timestamps");
            }
        }
        if channels == 0 {
            bail!("animation '{name}' has no channels");
        }
    }
    if names != *expected_names {
        let mut missing = expected_names
            .difference(&names)
            .cloned()
            .collect::<Vec<_>>();
        let mut extra = names
            .difference(expected_names)
            .cloned()
            .collect::<Vec<_>>();
        missing.sort();
        extra.sort();
        bail!(
            "animation names do not match report (missing: {}; extra: {})",
            missing.join(", "),
            extra.join(", ")
        );
    }
    Ok(())
}

#[cfg(test)]
#[path = "tests/actor_animation.rs"]
mod tests;

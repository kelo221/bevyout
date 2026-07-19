//! External-KF animation-only GLB conversion.

use super::*;
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
    pub(crate) animated_channel_count: usize,
    pub(crate) animated_target_count: usize,
    #[serde(default)]
    pub(crate) animated_targets: Vec<String>,
    #[serde(default)]
    pub(crate) missing_targets: Vec<String>,
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
mod tests {
    use super::*;

    fn synthetic_animation_glb(input_end: f32) -> Vec<u8> {
        let document = serde_json::json!({
            "asset": {"version": "2.0"},
            "scene": 0,
            "scenes": [{"nodes": [0]}],
            "nodes": [
                {"name": "Skeleton", "children": [1]},
                {"name": "Bip01 Spine"}
            ],
            "buffers": [{"byteLength": 32}],
            "bufferViews": [
                {"buffer": 0, "byteOffset": 0, "byteLength": 8},
                {"buffer": 0, "byteOffset": 8, "byteLength": 24}
            ],
            "accessors": [
                {"bufferView": 0, "componentType": 5126, "count": 2, "type": "SCALAR", "min": [0.0], "max": [1.0]},
                {"bufferView": 1, "componentType": 5126, "count": 2, "type": "VEC3"}
            ],
            "animations": [{
                "name": "idle",
                "samplers": [{"input": 0, "output": 1, "interpolation": "LINEAR"}],
                "channels": [{"sampler": 0, "target": {"node": 1, "path": "translation"}}]
            }]
        });
        let mut json = serde_json::to_vec(&document).unwrap();
        while json.len() % 4 != 0 {
            json.push(b' ');
        }
        let mut binary = Vec::new();
        for value in [0.0_f32, input_end, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0] {
            binary.extend_from_slice(&value.to_le_bytes());
        }
        let total_length = 12 + 8 + json.len() + 8 + binary.len();
        let mut glb = Vec::new();
        glb.extend_from_slice(b"glTF");
        glb.extend_from_slice(&2_u32.to_le_bytes());
        glb.extend_from_slice(&(total_length as u32).to_le_bytes());
        glb.extend_from_slice(&(json.len() as u32).to_le_bytes());
        glb.extend_from_slice(b"JSON");
        glb.extend_from_slice(&json);
        glb.extend_from_slice(&(binary.len() as u32).to_le_bytes());
        glb.extend_from_slice(b"BIN\0");
        glb.extend_from_slice(&binary);
        glb
    }

    #[test]
    fn fingerprint_changes_with_skeleton_clip_bytes_and_policy() {
        let clips = [("idle", "meshes/actors/idle.kf", b"clip".as_slice())];
        let baseline = actor_animation_pack_fingerprint(
            "v1",
            "meshes/actors/skeleton.nif",
            b"skeleton",
            &clips,
        );
        assert_ne!(
            baseline,
            actor_animation_pack_fingerprint(
                "v2",
                "meshes/actors/skeleton.nif",
                b"skeleton",
                &clips,
            )
        );
        assert_ne!(
            baseline,
            actor_animation_pack_fingerprint(
                "v1",
                "meshes/actors/skeleton.nif",
                b"changed",
                &clips,
            )
        );
        let changed = [("idle", "meshes/actors/idle.kf", b"changed".as_slice())];
        assert_ne!(
            baseline,
            actor_animation_pack_fingerprint(
                "v1",
                "meshes/actors/skeleton.nif",
                b"skeleton",
                &changed,
            )
        );
    }

    #[test]
    fn synthetic_skeleton_clip_pack_validates_channels_targets_and_times() {
        let path =
            std::env::temp_dir().join(format!("bevyout-animation-pack-{}.glb", std::process::id()));
        fs::write(&path, synthetic_animation_glb(1.0)).unwrap();
        validate_actor_animation_glb(&path, &HashSet::from(["idle".to_owned()])).unwrap();
        fs::write(&path, synthetic_animation_glb(f32::NAN)).unwrap();
        assert!(
            validate_actor_animation_glb(&path, &HashSet::from(["idle".to_owned()]))
                .unwrap_err()
                .to_string()
                .contains("non-finite")
        );
        let _ = fs::remove_file(path);
    }
}

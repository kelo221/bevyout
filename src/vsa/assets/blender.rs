//! Blender batch conversion and job serialization.

use super::*;

pub(crate) fn blender_conversion_script() -> &'static str {
    include_str!("blender_script.py")
}

pub(crate) fn run_blender_batch(
    blender: &Path,
    jobs: &[BlenderAssetJob],
    data_root: &Path,
    staging_dir: &Path,
) -> Result<()> {
    let job_file = staging_dir.join("blender_jobs.ron");
    let job_text = blender_jobs_json(jobs);
    fs::write(&job_file, job_text)?;
    let script_file = staging_dir.join("blender_script.py");
    fs::write(&script_file, blender_conversion_script())?;
    let result = Command::new(blender)
        .arg("--background")
        .arg("--factory-startup")
        .arg("--python")
        .arg(&script_file)
        .arg("--")
        .arg(&job_file)
        .arg(staging_dir)
        .current_dir(data_root)
        .output();
    let _ = fs::remove_file(&script_file);
    let result = result?;
    if !result.status.success() {
        bail!(
            "Blender exited with {}:\n{}",
            result.status,
            String::from_utf8_lossy(&result.stderr)
        );
    }
    for job in jobs {
        if !job.output.exists() {
            let stdout_tail = String::from_utf8_lossy(&result.stdout)
                .lines()
                .rev()
                .take(80)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
            let stderr_tail = String::from_utf8_lossy(&result.stderr)
                .lines()
                .rev()
                .take(80)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\n");
            bail!(
                "Blender reported success but did not create {}\nstdout tail:\n{}\nstderr tail:\n{}",
                job.output.display(),
                stdout_tail,
                stderr_tail
            );
        }
        apply_material_policy_to_glb_file(&job.output).with_context(|| {
            format!(
                "applying authored material policy to {}",
                job.output.display()
            )
        })?;
        validate_glb_images(&job.output).with_context(|| {
            format!(
                "converted GLB failed texture validation: {}",
                job.output.display()
            )
        })?;
        if job
            .input
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
        {
            validate_actor_glb(&job.output).with_context(|| {
                format!(
                    "converted actor GLB failed skin/material validation: {}",
                    job.output.display()
                )
            })?;
        }
        read_physics_asset(&job.physics_output).with_context(|| {
            format!(
                "converted physics sidecar failed validation: {}",
                job.physics_output.display()
            )
        })?;
    }
    Ok(())
}

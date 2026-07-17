use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn capture(path: &Path, custom_enabled: bool, orthographic: bool) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_bevyout"));
    command
        .arg("lighting-test")
        .args(["--shadow-resolution", "128"])
        .args(["--trace-seconds", "3"])
        .arg("--gpu-acceptance-capture")
        .arg(path)
        .arg("--gpu-acceptance-custom-only")
        .env("WGPU_BACKEND", "dx12");
    if !custom_enabled {
        command.arg("--gpu-acceptance-disable-custom");
    }
    if orthographic {
        command.arg("--gpu-acceptance-orthographic");
    }
    let output = command
        .output()
        .expect("launch lighting-test GPU acceptance");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "lighting-test failed\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr,
    );
    assert!(
        !stderr.contains("ERROR"),
        "lighting-test logged a render error\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        path.is_file(),
        "acceptance capture was not written: {}",
        path.display()
    );
}

fn temp_capture_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("bevyout-dynamic-lighting-gpu-{nonce}"))
}

fn assert_projection_changes_with_custom_lighting(directory: &Path, orthographic: bool) {
    let projection = if orthographic {
        "orthographic"
    } else {
        "perspective"
    };
    let enabled_path = directory.join(format!("{projection}-custom-enabled.png"));
    let disabled_path = directory.join(format!("{projection}-custom-disabled.png"));
    capture(&enabled_path, true, orthographic);
    capture(&disabled_path, false, orthographic);

    let enabled = image::open(&enabled_path).unwrap().to_rgb8();
    let disabled = image::open(&disabled_path).unwrap().to_rgb8();
    assert_eq!(enabled.dimensions(), disabled.dimensions());

    let mut changed_pixels = 0usize;
    let mut enabled_energy = 0u64;
    let mut disabled_energy = 0u64;
    for (on, off) in enabled.pixels().zip(disabled.pixels()) {
        let difference =
            on.0.into_iter()
                .zip(off.0)
                .map(|(left, right)| left.abs_diff(right) as u32)
                .sum::<u32>();
        changed_pixels += usize::from(difference >= 12);
        enabled_energy += on.0.into_iter().map(u64::from).sum::<u64>();
        disabled_energy += off.0.into_iter().map(u64::from).sum::<u64>();
    }

    assert!(
        changed_pixels >= 2_000,
        "production WGSL changed only {changed_pixels} {projection} pixels"
    );
    assert!(
        enabled_energy > disabled_energy,
        "{projection} custom-light image energy {enabled_energy} did not exceed control {disabled_energy}"
    );
}

#[test]
fn production_wgsl_changes_perspective_and_orthographic_targets_with_zero_bevy_lights() {
    let directory = temp_capture_dir();
    fs::create_dir_all(&directory).unwrap();
    assert_projection_changes_with_custom_lighting(&directory, false);
    assert_projection_changes_with_custom_lighting(&directory, true);

    let _ = fs::remove_dir_all(directory);
}

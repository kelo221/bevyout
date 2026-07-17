#![cfg(target_os = "windows")]

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn capture(enabled_path: &Path, control_path: &Path, orthographic: bool) {
    let mut command = Command::new(env!("CARGO_BIN_EXE_bevyout"));
    command
        .arg("lighting-test")
        .args(["--shadow-resolution", "128"])
        .args(["--trace-seconds", "6"])
        .arg("--gpu-acceptance-capture")
        .arg(enabled_path)
        .arg("--gpu-acceptance-control-capture")
        .arg(control_path)
        .arg("--gpu-acceptance-custom-only")
        .env("WGPU_BACKEND", "dx12");
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
        enabled_path.is_file() && control_path.is_file(),
        "same-process acceptance captures were not written: {} / {}\nstdout:\n{}\nstderr:\n{}",
        enabled_path.display(),
        control_path.display(),
        stdout,
        stderr,
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
    capture(&enabled_path, &disabled_path, orthographic);

    let enabled = image::open(&enabled_path).unwrap().to_rgb8();
    let disabled = image::open(&disabled_path).unwrap().to_rgb8();
    assert_eq!(enabled.dimensions(), disabled.dimensions());

    let column_centers = if orthographic {
        [0.207, 0.402, 0.598, 0.793]
    } else {
        [0.118, 0.373, 0.627, 0.882]
    };
    let row_centers = if orthographic {
        [0.593, 0.766]
    } else {
        [0.621, 0.847]
    };
    let light_types = [
        "Point",
        "Spot",
        "Discoball",
        "Wave",
        "Interference",
        "Rotor",
        "Shock",
        "Disco",
    ];
    for (index, light_type) in light_types.into_iter().enumerate() {
        let changed = changed_pixels_in_region(
            &enabled,
            &disabled,
            column_centers[index % 4] - 0.04,
            row_centers[index / 4] - 0.045,
            column_centers[index % 4] + 0.04,
            row_centers[index / 4] + 0.045,
        );
        assert!(
            changed >= 120,
            "{projection} {light_type} target changed only {changed} pixels"
        );
    }

    for (label, bounds) in [
        ("clear-background sphere fog", (0.24, 0.10, 0.44, 0.38)),
        ("clear-background ConeZ fog", (0.63, 0.10, 0.84, 0.38)),
    ] {
        let changed =
            changed_pixels_in_region(&enabled, &disabled, bounds.0, bounds.1, bounds.2, bounds.3);
        assert!(
            changed >= 500,
            "{projection} {label} changed only {changed} background pixels"
        );
    }
}

fn changed_pixels_in_region(
    enabled: &image::RgbImage,
    disabled: &image::RgbImage,
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
) -> usize {
    let (width, height) = enabled.dimensions();
    let x0 = (left * width as f32).round() as u32;
    let x1 = (right * width as f32).round() as u32;
    let y0 = (top * height as f32).round() as u32;
    let y1 = (bottom * height as f32).round() as u32;
    let mut changed = 0;
    for y in y0..y1 {
        for x in x0..x1 {
            let on = enabled.get_pixel(x, y);
            let off = disabled.get_pixel(x, y);
            let difference =
                on.0.into_iter()
                    .zip(off.0)
                    .map(|(left, right)| left.abs_diff(right) as u32)
                    .sum::<u32>();
            changed += usize::from(difference >= 12);
        }
    }
    changed
}

#[test]
fn dx12_production_wgsl_covers_each_spatial_mode_fog_background_and_both_projections() {
    let directory = temp_capture_dir();
    fs::create_dir_all(&directory).unwrap();
    assert_projection_changes_with_custom_lighting(&directory, false);
    assert_projection_changes_with_custom_lighting(&directory, true);

    let _ = fs::remove_dir_all(directory);
}

use super::*;
use bevy::render::settings::RenderCreation;

#[test]
fn validation_stays_off_without_cli_or_env() {
    assert!(!gpu_validation_enabled(false, None));
}

#[test]
fn cli_flag_enables_validation() {
    assert!(gpu_validation_enabled(true, None));
}

#[test]
fn env_one_enables_validation_without_the_cli_flag() {
    assert!(gpu_validation_enabled(false, Some("1")));
}

#[test]
fn env_zero_does_not_enable_validation() {
    assert!(!gpu_validation_enabled(false, Some("0")));
}

#[test]
fn cli_flag_wins_over_env_zero() {
    assert!(gpu_validation_enabled(true, Some("0")));
}

#[test]
fn any_non_zero_env_value_enables_validation() {
    assert!(gpu_validation_enabled(false, Some("")));
    assert!(gpu_validation_enabled(false, Some("true")));
}

#[test]
fn render_plugin_clears_validation_when_disabled() {
    let plugin = viewer_render_plugin(false);
    let RenderCreation::Automatic(settings) = plugin.render_creation else {
        panic!("viewer render plugin should use automatic wgpu settings");
    };
    assert!(!settings.instance_flags.contains(InstanceFlags::VALIDATION));
    assert!(
        !settings
            .instance_flags
            .contains(InstanceFlags::GPU_BASED_VALIDATION)
    );
}

#[test]
fn render_plugin_sets_validation_when_enabled() {
    let plugin = viewer_render_plugin(true);
    let RenderCreation::Automatic(settings) = plugin.render_creation else {
        panic!("viewer render plugin should use automatic wgpu settings");
    };
    assert!(settings.instance_flags.contains(InstanceFlags::VALIDATION));
}

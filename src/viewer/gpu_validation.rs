//! GPU API validation policy for Bevy viewer windows.
//!
//! wgpu turns on Vulkan/D3D12 validation whenever `debug_assertions` is set
//! (`dev` and `dev-opt`). On SuperDuperMart that layer owned ~23% of process
//! CPU and ~3× frame time. Viewer launches keep it off unless the operator
//! opts in with `--wgpu-validation` or `WGPU_VALIDATION=1`.

use bevy::render::RenderPlugin;
use bevy::render::settings::{InstanceFlags, RenderCreation, WgpuSettings};
use tracing::info;

/// wgpu's `InstanceFlags::with_env` rule: a present value enables the flag
/// unless it is exactly `"0"`.
pub(crate) fn wgpu_env_flag_enabled(value: Option<&str>) -> bool {
    matches!(value, Some(value) if value != "0")
}

/// CLI `--wgpu-validation` wins; otherwise a non-`"0"` `WGPU_VALIDATION` env
/// value enables the layers. Unset env stays off.
pub(crate) fn gpu_validation_enabled(
    cli_requested: bool,
    wgpu_validation_env: Option<&str>,
) -> bool {
    cli_requested || wgpu_env_flag_enabled(wgpu_validation_env)
}

pub(crate) fn gpu_validation_enabled_from_env(cli_requested: bool) -> bool {
    gpu_validation_enabled(
        cli_requested,
        std::env::var("WGPU_VALIDATION").ok().as_deref(),
    )
}

pub(crate) fn viewer_render_plugin(enable_validation: bool) -> RenderPlugin {
    let mut settings = WgpuSettings::default();
    if enable_validation {
        settings.instance_flags.insert(InstanceFlags::VALIDATION);
        info!("wgpu validation on");
    } else {
        settings.instance_flags.remove(InstanceFlags::VALIDATION);
        settings
            .instance_flags
            .remove(InstanceFlags::GPU_BASED_VALIDATION);
        info!("wgpu validation off");
    }
    RenderPlugin {
        render_creation: RenderCreation::from(settings),
        ..Default::default()
    }
}

#[cfg(test)]
#[path = "tests/gpu_validation.rs"]
mod tests;

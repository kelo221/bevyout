//! Rendering, timing, and capture console commands.

use super::*;
use crate::viewer::{ImageSpaceBloomOverrides, LoadedSceneManifest, image_space_bloom_values};

pub(super) struct RenderCommandProvider;

impl ConsoleCommandProvider for RenderCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        for command in [
            ConsoleCommand::new(
                "getrender",
                "getrender [setting]",
                "Get one render setting, the active ImageSpace, or all render settings.",
                get_render,
            ),
            ConsoleCommand::new(
                "setrender",
                "setrender <setting> <value>",
                "Set a validated render setting.",
                set_render,
            )
            .mutating(),
            ConsoleCommand::new(
                "tonemap",
                "tonemap [mode]",
                "Get or set the active camera tonemapper.",
                tonemap,
            )
            .mutating(),
            ConsoleCommand::new(
                "renderreport",
                "renderreport",
                "Write the configured render timing and diagnostic reports immediately.",
                render_report,
            )
            .mutating(),
            ConsoleCommand::new(
                "shadowcache",
                "shadowcache <status|rebuild>",
                "Inspect the prepared point-shadow artifact or show rebuild instructions.",
                shadow_cache,
            )
            .mutating(),
            ConsoleCommand::new(
                "sgtm",
                "sgtm <0.01..100>",
                "Set Time<Virtual> relative speed without changing pause state.",
                set_global_time_multiplier,
            )
            .mutating(),
            ConsoleCommand::new(
                "screenshot",
                "screenshot [name]",
                "Save the primary window under .bevyout/screenshots using a sanitized name.",
                screenshot,
            )
            .mutating(),
        ] {
            registry.register(command)?;
        }
        Ok(())
    }
}

pub(super) const RENDER_SETTINGS: [&str; 12] = [
    "lighting",
    "irradiance",
    "ambient",
    "bloom_intensity",
    "bloom_threshold",
    "bloom_softness",
    "fog",
    "volumetric_fog",
    "ao",
    "emission",
    "shadow_samples",
    "realtime_shadows",
];

const IMAGE_SPACE_DIAGNOSTIC: &str = "imagespace";

pub(super) const TONEMAPPER_NAMES: [&str; 9] = [
    "none",
    "reinhard",
    "reinhard_luminance",
    "aces_fitted",
    "agx",
    "somewhat_boring_display_transform",
    "tony_mc_mapface",
    "blender_filmic",
    "khronos_pbr_neutral",
];

pub(super) fn tonemapper_name(tonemapper: Tonemapping) -> &'static str {
    match tonemapper {
        Tonemapping::None => "none",
        Tonemapping::Reinhard => "reinhard",
        Tonemapping::ReinhardLuminance => "reinhard_luminance",
        Tonemapping::AcesFitted => "aces_fitted",
        Tonemapping::AgX => "agx",
        Tonemapping::SomewhatBoringDisplayTransform => "somewhat_boring_display_transform",
        Tonemapping::TonyMcMapface => "tony_mc_mapface",
        Tonemapping::BlenderFilmic => "blender_filmic",
        Tonemapping::KhronosPbrNeutral => "khronos_pbr_neutral",
    }
}

pub(super) fn tonemapper_display_name(tonemapper: Tonemapping) -> &'static str {
    match tonemapper {
        Tonemapping::None => "None",
        Tonemapping::Reinhard => "Reinhard",
        Tonemapping::ReinhardLuminance => "Reinhard Luminance",
        Tonemapping::AcesFitted => "ACES Fitted",
        Tonemapping::AgX => "AgX",
        Tonemapping::SomewhatBoringDisplayTransform => "Somewhat Boring Display Transform",
        Tonemapping::TonyMcMapface => "Tony McMapface",
        Tonemapping::BlenderFilmic => "Blender Filmic",
        Tonemapping::KhronosPbrNeutral => "Khronos PBR Neutral",
    }
}

pub(super) fn parse_tonemapper(value: &str) -> Option<Tonemapping> {
    match value.to_ascii_lowercase().as_str() {
        "none" => Some(Tonemapping::None),
        "reinhard" => Some(Tonemapping::Reinhard),
        "reinhard_luminance" => Some(Tonemapping::ReinhardLuminance),
        "aces_fitted" => Some(Tonemapping::AcesFitted),
        "agx" => Some(Tonemapping::AgX),
        "somewhat_boring_display_transform" => Some(Tonemapping::SomewhatBoringDisplayTransform),
        "tony_mc_mapface" => Some(Tonemapping::TonyMcMapface),
        "blender_filmic" => Some(Tonemapping::BlenderFilmic),
        "khronos_pbr_neutral" => Some(Tonemapping::KhronosPbrNeutral),
        _ => None,
    }
}

pub(super) fn tonemapper_camera(world: &mut World) -> Result<Entity, ConsoleError> {
    let mut query = world.query_filtered::<Entity, (With<Camera3d>, With<Tonemapping>)>();
    let mut cameras = query.iter(world);
    let Some(camera) = cameras.next() else {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "expected exactly one active 3D camera with tonemapping",
        ));
    };
    if cameras.next().is_some() {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "expected exactly one active 3D camera with tonemapping",
        ));
    }
    Ok(camera)
}

pub(super) fn tonemap(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "tonemap accepts at most one mode",
        ));
    }
    let camera = tonemapper_camera(world)?;
    let requested = invocation
        .args
        .first()
        .map(|value| {
            parse_tonemapper(value).ok_or_else(|| {
                ConsoleError::new(
                    "unknown_tonemapper",
                    format!(
                        "unknown tonemapper '{value}'; expected one of: {}",
                        TONEMAPPER_NAMES.join(", ")
                    ),
                )
            })
        })
        .transpose()?;
    if let Some(requested) = requested {
        *world
            .get_mut::<Tonemapping>(camera)
            .expect("tonemapper camera query required Tonemapping") = requested;
    }
    let current = *world
        .get::<Tonemapping>(camera)
        .expect("tonemapper camera query required Tonemapping");
    let value = tonemapper_name(current);
    let display = tonemapper_display_name(current);
    let message = if requested.is_some() {
        format!("Tonemapper set to {display}.")
    } else {
        format!("Tonemapper is {display}.")
    };
    Ok(ConsoleCommandResult::new(
        json!({ "tonemapper": value }),
        vec![message],
    ))
}

pub(super) fn bloom_values(world: &mut World) -> Result<(f32, f32, f32), ConsoleError> {
    let mut query = world.query_filtered::<&Bloom, With<Camera3d>>();
    let mut blooms = query.iter(world);
    let Some(bloom) = blooms.next() else {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "the active camera does not have bloom settings",
        ));
    };
    let values = (
        bloom.intensity,
        bloom.prefilter.threshold,
        bloom.prefilter.threshold_softness,
    );
    if blooms.next().is_some() {
        return Err(ConsoleError::new(
            "camera_unavailable",
            "expected exactly one camera with bloom settings",
        ));
    }
    Ok(values)
}

pub(super) fn render_values(world: &mut World) -> Result<Map<String, Value>, ConsoleError> {
    let (bloom_intensity, bloom_threshold, bloom_softness) = bloom_values(world)?;
    let mut values = Map::new();
    values.insert(
        "lighting".into(),
        json!(world.resource::<LightingScale>().0),
    );
    values.insert(
        "irradiance".into(),
        json!(world.resource::<IrradianceIntensity>().0),
    );
    values.insert("ambient".into(), json!(world.resource::<AmbientScale>().0));
    values.insert("bloom_intensity".into(), json!(bloom_intensity));
    values.insert("bloom_threshold".into(), json!(bloom_threshold));
    values.insert("bloom_softness".into(), json!(bloom_softness));
    values.insert("fog".into(), json!(world.resource::<FogStrength>().0));
    values.insert(
        "volumetric_fog".into(),
        json!(world.resource::<VolumetricFogMultiplier>().0),
    );
    values.insert("ao".into(), json!(world.resource::<AoStrength>().0));
    values.insert(
        "emission".into(),
        json!(world.resource::<EmissionScale>().0),
    );
    values.insert(
        "shadow_samples".into(),
        json!(world.resource::<PointLightShadowSamples>().0),
    );
    values.insert(
        "realtime_shadows".into(),
        json!(world.resource::<RealtimeShadowSettings>().enabled as u8),
    );
    Ok(values)
}

fn image_space_values(world: &World) -> Value {
    let manifest = world.resource::<LoadedSceneManifest>();
    let Some(image_space) = manifest.cell.image_space.as_ref() else {
        return json!({
            "form_id": null,
            "editor_id": null,
            "interior": manifest.cell.interior,
            "flags": 0,
            "resolved": false,
        });
    };
    let (bloom_intensity, bloom_threshold, bloom_softness) =
        image_space_bloom_values(Some(image_space), manifest.cell.interior);
    let overrides = world
        .get_resource::<ImageSpaceBloomOverrides>()
        .copied()
        .unwrap_or_default();
    json!({
        "form_id": image_space.form_id,
        "editor_id": image_space.editor_id,
        "interior": manifest.cell.interior,
        "resolved": true,
        "flags": image_space.flags,
        "brightness": image_space.brightness,
        "saturation": image_space.cinematic_saturation,
        "contrast": image_space.cinematic_contrast,
        "contrast_avg_lum": image_space.cinematic_contrast_avg_lum,
        "tint_rgb": image_space.cinematic_brightness_tint_rgb,
        "tint_value": image_space.cinematic_brightness_tint_value,
        "hdr_target_lum": image_space.hdr_target_lum,
        "eye_adapt_speed": image_space.eye_adapt_speed,
        "hdr_emissive_multiplier": image_space.hdr_emissive_multiplier,
        "hdr_bright_scale": image_space.hdr_bright_scale,
        "hdr_bright_clamp": image_space.hdr_bright_clamp,
        "bloom_blur_radius": image_space.bloom_blur_radius,
        "bloom_alpha_mult": if manifest.cell.interior {
            image_space.bloom_alpha_mult_interior
        } else {
            image_space.bloom_alpha_mult_exterior
        },
        "bloom_defaults": {
            "intensity": bloom_intensity,
            "threshold": bloom_threshold,
            "softness": bloom_softness,
        },
        "bloom_overrides": {
            "intensity": overrides.intensity,
            "threshold": overrides.threshold,
            "softness": overrides.softness,
        },
    })
}

pub(super) fn validate_render_setting(setting: &str) -> Result<(), ConsoleError> {
    if RENDER_SETTINGS.contains(&setting) {
        Ok(())
    } else {
        Err(ConsoleError::new(
            "unknown_setting",
            format!("unknown render setting '{setting}'"),
        ))
    }
}

pub(super) fn render_setting_label(setting: &str) -> &'static str {
    match setting {
        "lighting" => "Lighting",
        "irradiance" => "Irradiance",
        "ambient" => "Ambient light",
        "bloom_intensity" => "Bloom intensity",
        "bloom_threshold" => "Bloom threshold",
        "bloom_softness" => "Bloom softness",
        "fog" => "Fog",
        "volumetric_fog" => "Volumetric fog",
        "ao" => "Ambient occlusion",
        "emission" => "Material emission",
        "shadow_samples" => "Point-shadow samples per pixel",
        "realtime_shadows" => "Realtime point shadows",
        _ => "Render setting",
    }
}

pub(super) fn get_render(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "getrender accepts at most one setting",
        ));
    }
    let values = render_values(world)?;
    if let Some(setting) = invocation.args.first() {
        let setting = setting.to_ascii_lowercase();
        if setting == IMAGE_SPACE_DIAGNOSTIC {
            return Ok(ConsoleCommandResult::new(
                image_space_values(world),
                vec!["Active ImageSpace reported.".into()],
            ));
        }
        validate_render_setting(&setting)?;
        let value = values[&setting].clone();
        let message = format!("{}: {value}.", render_setting_label(&setting));
        Ok(ConsoleCommandResult::new(
            json!({
                "setting": setting,
                "value": value
            }),
            vec![message],
        ))
    } else {
        let log = RENDER_SETTINGS
            .iter()
            .map(|setting| format!("{}: {}.", render_setting_label(setting), values[*setting]))
            .collect();
        Ok(ConsoleCommandResult::new(Value::Object(values), log))
    }
}

pub(super) fn set_render(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 2 {
        return Err(ConsoleError::new(
            "bad_arity",
            "setrender expects a setting and value",
        ));
    }
    let setting = invocation.args[0].to_ascii_lowercase();
    validate_render_setting(&setting)?;
    let value = invocation.args[1]
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", "render value must be a number"))?;
    if !value.is_finite() {
        return Err(ConsoleError::new(
            "non_finite",
            "render value must be finite",
        ));
    }
    let valid = match setting.as_str() {
        "lighting" => (0.0001..=262_144.0).contains(&value),
        "irradiance" => (0.0..=4096.0).contains(&value),
        "ambient" => (0.0001..=4096.0).contains(&value),
        "bloom_intensity" | "bloom_softness" | "fog" | "ao" | "emission" => {
            (0.0..=1.0).contains(&value)
        }
        "volumetric_fog" => (0.0..=100.0).contains(&value),
        "bloom_threshold" => value >= 0.0,
        "shadow_samples" => value == 0.0 || value == 1.0,
        "realtime_shadows" => value == 0.0 || value == 1.0,
        _ => unreachable!(),
    };
    if !valid {
        return Err(ConsoleError::new(
            "out_of_range",
            format!("value {value} is outside the supported range for {setting}"),
        ));
    }

    match setting.as_str() {
        "lighting" => world.resource_mut::<LightingScale>().0 = value,
        "irradiance" => world.resource_mut::<IrradianceIntensity>().0 = value,
        "ambient" => world.resource_mut::<AmbientScale>().0 = value,
        "fog" => world.resource_mut::<FogStrength>().0 = value,
        "volumetric_fog" => world.resource_mut::<VolumetricFogMultiplier>().0 = value,
        "ao" => world.resource_mut::<AoStrength>().0 = value,
        "emission" => world.resource_mut::<EmissionScale>().0 = value,
        "shadow_samples" => world.resource_mut::<PointLightShadowSamples>().0 = value as u32,
        "realtime_shadows" => {
            world.resource_mut::<RealtimeShadowSettings>().enabled = value == 1.0;
        }
        "bloom_intensity" | "bloom_threshold" | "bloom_softness" => {
            let camera = {
                let mut query = world.query_filtered::<Entity, (With<Camera3d>, With<Bloom>)>();
                let mut cameras = query.iter(world);
                let Some(camera) = cameras.next() else {
                    return Err(ConsoleError::new(
                        "camera_unavailable",
                        "the active camera does not have bloom settings",
                    ));
                };
                if cameras.next().is_some() {
                    return Err(ConsoleError::new(
                        "camera_unavailable",
                        "expected exactly one camera with bloom settings",
                    ));
                }
                camera
            };
            let override_kind = {
                let mut bloom = world.get_mut::<Bloom>(camera).ok_or_else(|| {
                    ConsoleError::new("camera_unavailable", "bloom is unavailable")
                })?;
                match setting.as_str() {
                    "bloom_intensity" => {
                        bloom.intensity = value;
                        0
                    }
                    "bloom_threshold" => {
                        bloom.prefilter.threshold = value;
                        1
                    }
                    "bloom_softness" => {
                        bloom.prefilter.threshold_softness = value;
                        2
                    }
                    _ => unreachable!(),
                }
            };
            let mut overrides = world.resource_mut::<ImageSpaceBloomOverrides>();
            match override_kind {
                0 => overrides.intensity = Some(value),
                1 => overrides.threshold = Some(value),
                2 => overrides.softness = Some(value),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
    let message = format!("{} set to {value}.", render_setting_label(&setting));
    Ok(ConsoleCommandResult::new(
        json!({
            "setting": setting,
            "value": value
        }),
        vec![message],
    ))
}

pub(super) fn shadow_cache(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "shadowcache accepts status or rebuild",
        ));
    }
    match invocation
        .args
        .first()
        .map(String::as_str)
        .unwrap_or("status")
    {
        "status" => Ok(ConsoleCommandResult::new(
            shadow_cache_status(world),
            vec!["Shadow cache status reported.".into()],
        )),
        "rebuild" => Err(ConsoleError::new(
            "prepare_required",
            "prepared shadows cannot be rebuilt in the viewer; run `prepare --rebuild-shadows` for this cell, then restart render",
        )),
        _ => Err(ConsoleError::new(
            "bad_value",
            "shadowcache expects status or rebuild",
        )),
    }
}

pub(super) fn render_report(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    no_args(invocation)?;
    let path = diagnostics::save_render_report_now(world).map_err(ConsoleError::internal)?;
    Ok(ConsoleCommandResult::new(
        json!({ "path": path }),
        vec![format!("Render report written to {}.", path.display())],
    ))
}

pub(super) fn set_global_time_multiplier(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() != 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "sgtm expects exactly one multiplier",
        ));
    }
    let multiplier = invocation.args[0]
        .parse::<f32>()
        .map_err(|_| ConsoleError::new("bad_type", "time multiplier must be a number"))?;
    if !multiplier.is_finite() || !(0.01..=100.0).contains(&multiplier) {
        return Err(ConsoleError::new(
            "out_of_range",
            "time multiplier must be between 0.01 and 100",
        ));
    }
    world
        .resource_mut::<Time<Virtual>>()
        .set_relative_speed(multiplier);
    Ok(ConsoleCommandResult::new(
        json!({ "relative_speed": multiplier }),
        vec![format!("Time multiplier set to {multiplier}.")],
    ))
}

pub(super) fn screenshot(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if invocation.args.len() > 1 {
        return Err(ConsoleError::new(
            "bad_arity",
            "screenshot accepts at most one name",
        ));
    }
    let default_name = format!("frame-{:08}", invocation.frame);
    let supplied = invocation
        .args
        .first()
        .map_or(default_name.as_str(), String::as_str);
    let name = supplied.strip_suffix(".png").unwrap_or(supplied);
    if name.is_empty()
        || name.len() > 96
        || !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(ConsoleError::new(
            "invalid_path",
            "screenshot name may contain only letters, numbers, '-' and '_'",
        ));
    }
    let has_window = {
        let mut windows = world.query_filtered::<Entity, With<PrimaryWindow>>();
        windows.iter(world).next().is_some()
    };
    if !has_window {
        return Err(ConsoleError::new(
            "unsupported",
            "screenshots require a primary window",
        ));
    }
    let directory = PathBuf::from(".bevyout/screenshots");
    std::fs::create_dir_all(&directory).map_err(ConsoleError::internal)?;
    let path = directory.join(format!("{name}.png"));
    world
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path.clone()));
    Ok(ConsoleCommandResult::new(
        json!({ "path": path, "persistence": "runtime_capture" }),
        vec![format!("Screenshot queued at {}.", path.display())],
    ))
}

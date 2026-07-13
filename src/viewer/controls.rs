//! Viewer cursor, adjustment, and diagnostic controls.

use super::scene::CellDirectionalLight;
use super::*;

pub(crate) fn capture_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub(crate) fn capture_cursor_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    } else if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
}

#[derive(Resource)]
pub(crate) struct UnlitMode(pub(crate) bool);

#[derive(Resource)]
pub(crate) struct LightingScale(pub(crate) f32);

#[derive(Resource)]
pub(crate) struct IrradianceIntensity(pub(crate) f32);

#[derive(Resource)]
pub(crate) struct AmbientScale(pub(crate) f32);

#[derive(Resource)]
pub(crate) struct FogStrength(pub(crate) f32);

#[derive(Resource)]
pub(crate) struct AoStrength(pub(crate) f32);

#[derive(Resource, Default)]
pub(crate) struct AoMeshBases {
    values: HashMap<AssetId<Mesh>, VertexAttributeValues>,
}

#[derive(Default)]
pub(crate) struct AoScanState {
    last_mesh_entity_count: usize,
    last_mesh_asset_count: usize,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum AdjustmentTarget {
    #[default]
    LightingScale,
    IrradianceIntensity,
    AmbientScale,
    BloomIntensity,
    BloomThreshold,
    BloomSoftness,
    FogStrength,
    AoStrength,
}

impl AdjustmentTarget {
    const ALL: [Self; 8] = [
        Self::LightingScale,
        Self::IrradianceIntensity,
        Self::AmbientScale,
        Self::BloomIntensity,
        Self::BloomThreshold,
        Self::BloomSoftness,
        Self::FogStrength,
        Self::AoStrength,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::LightingScale => "Lighting scale",
            Self::IrradianceIntensity => "Irradiance intensity",
            Self::AmbientScale => "Ambient scale",
            Self::BloomIntensity => "Bloom intensity",
            Self::BloomThreshold => "Bloom threshold",
            Self::BloomSoftness => "Bloom softness",
            Self::FogStrength => "Fog strength",
            Self::AoStrength => "AO strength",
        }
    }

    pub(crate) fn cycle(self, delta: i32) -> Self {
        let index = Self::ALL
            .iter()
            .position(|target| *target == self)
            .unwrap_or(0);
        let next = (index as i32 + delta).rem_euclid(Self::ALL.len() as i32) as usize;
        Self::ALL[next]
    }
}

#[derive(Component)]
pub(crate) struct AdjustmentHud;

#[derive(Resource)]
pub(crate) struct LightsDisabled(pub(crate) bool);

#[allow(clippy::too_many_arguments)]
pub(crate) fn adjust_selected_value(
    keys: Res<ButtonInput<KeyCode>>,
    mut target: ResMut<AdjustmentTarget>,
    mut lighting: ResMut<LightingScale>,
    mut irradiance: ResMut<IrradianceIntensity>,
    mut ambient: ResMut<AmbientScale>,
    mut fog_strength: ResMut<FogStrength>,
    mut ao_strength: ResMut<AoStrength>,
    mut cameras: Query<&mut Bloom, With<Camera3d>>,
) {
    if keys.just_pressed(KeyCode::PageUp) {
        *target = (*target).cycle(1);
        info!("adjustment target: {}", target.label());
    } else if keys.just_pressed(KeyCode::PageDown) {
        *target = (*target).cycle(-1);
        info!("adjustment target: {}", target.label());
    }

    let direction = if keys.just_pressed(KeyCode::F1) {
        Some(-1)
    } else if keys.just_pressed(KeyCode::F2) {
        Some(1)
    } else {
        None
    };
    let Some(direction) = direction else {
        return;
    };

    match *target {
        AdjustmentTarget::LightingScale => {
            lighting.0 = if direction < 0 {
                (lighting.0 * 0.5).max(0.0001)
            } else {
                (lighting.0 * 2.0).min(262_144.0)
            };
            info!("lighting scale: {:.4}", lighting.0);
        }
        AdjustmentTarget::IrradianceIntensity => {
            irradiance.0 = if direction < 0 {
                (irradiance.0 * 0.5).max(0.0)
            } else {
                (irradiance.0 * 2.0).min(4096.0)
            };
            info!("irradiance intensity: {:.4}", irradiance.0);
        }
        AdjustmentTarget::AmbientScale => {
            ambient.0 = if direction < 0 {
                (ambient.0 * 0.5).max(0.0001)
            } else {
                (ambient.0 * 2.0).min(4096.0)
            };
            info!("ambient scale: {:.4}", ambient.0);
        }
        AdjustmentTarget::BloomIntensity
        | AdjustmentTarget::BloomThreshold
        | AdjustmentTarget::BloomSoftness => {
            let Ok(mut bloom) = cameras.single_mut() else {
                return;
            };
            match *target {
                AdjustmentTarget::BloomIntensity => {
                    bloom.intensity = if direction < 0 {
                        (bloom.intensity * 0.5).max(0.0)
                    } else {
                        (bloom.intensity * 2.0).min(1.0)
                    };
                }
                AdjustmentTarget::BloomThreshold => {
                    bloom.prefilter.threshold = if direction < 0 {
                        (bloom.prefilter.threshold - 0.1).max(0.0)
                    } else {
                        bloom.prefilter.threshold + 0.1
                    };
                }
                AdjustmentTarget::BloomSoftness => {
                    bloom.prefilter.threshold_softness = (bloom.prefilter.threshold_softness
                        + if direction < 0 { -0.1 } else { 0.1 })
                    .clamp(0.0, 1.0);
                }
                _ => unreachable!(),
            }
            info!(
                "bloom: intensity {:.2}, threshold {:.2}, softness {:.2}",
                bloom.intensity, bloom.prefilter.threshold, bloom.prefilter.threshold_softness
            );
        }
        AdjustmentTarget::FogStrength => {
            fog_strength.0 = if direction < 0 {
                (fog_strength.0 * 0.5).max(0.0)
            } else {
                (fog_strength.0 * 2.0).min(1.0)
            };
            info!("fog strength: {:.2}", fog_strength.0);
        }
        AdjustmentTarget::AoStrength => {
            ao_strength.0 =
                (ao_strength.0 + if direction < 0 { -0.1 } else { 0.1 }).clamp(0.0, 1.0);
            info!("AO strength: {:.2}", ao_strength.0);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn update_adjustment_hud(
    target: Res<AdjustmentTarget>,
    lighting: Res<LightingScale>,
    irradiance: Res<IrradianceIntensity>,
    ambient: Res<AmbientScale>,
    fog_strength: Res<FogStrength>,
    ao_strength: Res<AoStrength>,
    cameras: Query<&Bloom, With<Camera3d>>,
    mut text: Single<&mut Text, With<AdjustmentHud>>,
) {
    let value = match *target {
        AdjustmentTarget::LightingScale => format!("{:.4}", lighting.0),
        AdjustmentTarget::IrradianceIntensity => format!("{:.4}", irradiance.0),
        AdjustmentTarget::AmbientScale => format!("{:.4}", ambient.0),
        AdjustmentTarget::BloomIntensity => cameras
            .single()
            .map(|bloom| format!("{:.2}", bloom.intensity))
            .unwrap_or_else(|_| "--".into()),
        AdjustmentTarget::BloomThreshold => cameras
            .single()
            .map(|bloom| format!("{:.2}", bloom.prefilter.threshold))
            .unwrap_or_else(|_| "--".into()),
        AdjustmentTarget::BloomSoftness => cameras
            .single()
            .map(|bloom| format!("{:.2}", bloom.prefilter.threshold_softness))
            .unwrap_or_else(|_| "--".into()),
        AdjustmentTarget::FogStrength => format!("{:.2}", fog_strength.0),
        AdjustmentTarget::AoStrength => format!("{:.2}", ao_strength.0),
    };
    text.0 = format!(
        "Adjusting: {} = {}\nPage Up/Down: select   F1/F2: change",
        target.label(),
        value
    );
}

pub(crate) fn apply_irradiance_intensity(
    intensity: Res<IrradianceIntensity>,
    mut volumes: Query<&mut IrradianceVolume>,
) {
    if !intensity.is_changed() {
        return;
    }
    for mut volume in &mut volumes {
        volume.intensity = intensity.0;
    }
}

pub(crate) fn apply_ao_strength(
    strength: Res<AoStrength>,
    mut bases: ResMut<AoMeshBases>,
    mut meshes: ResMut<Assets<Mesh>>,
    mesh_entities: Query<(
        &Mesh3d,
        Option<&ChildOf>,
        Option<&interaction::PlacementRoot>,
    )>,
    parents: Query<&ChildOf>,
    roots: Query<&interaction::PlacementRoot>,
    mut scan_state: Local<AoScanState>,
) {
    let mesh_entity_count = mesh_entities.iter().count();
    let mesh_asset_count = meshes.len();
    if !strength.is_changed()
        && scan_state.last_mesh_entity_count == mesh_entity_count
        && scan_state.last_mesh_asset_count == mesh_asset_count
    {
        return;
    }
    scan_state.last_mesh_entity_count = mesh_entity_count;
    scan_state.last_mesh_asset_count = mesh_asset_count;

    let mut seen = HashSet::new();
    for (mesh_handle, child_of, own_root) in &mesh_entities {
        let Some(child_of) = child_of else {
            if !own_root.is_some_and(interaction::PlacementRoot::uses_quick_ao) {
                continue;
            }
            let id = mesh_handle.0.id();
            if !seen.insert(id) {
                continue;
            }
            if !strength.is_changed() && bases.values.contains_key(&id) {
                continue;
            }
            let Some(mut mesh) = meshes.get_mut(id) else {
                continue;
            };
            let Ok(colors) = mesh.try_attribute(Mesh::ATTRIBUTE_COLOR) else {
                continue;
            };
            let baseline = bases.values.entry(id).or_insert_with(|| colors.clone());
            let Ok(colors) = mesh.try_attribute_mut(Mesh::ATTRIBUTE_COLOR) else {
                continue;
            };
            scale_ao_colors(colors, baseline, strength.0);
            continue;
        };
        let mut entity = child_of.0;
        let mut quick_ao = false;
        for _ in 0..64 {
            if roots
                .get(entity)
                .is_ok_and(interaction::PlacementRoot::uses_quick_ao)
            {
                quick_ao = true;
                break;
            }
            let Ok(parent) = parents.get(entity) else {
                break;
            };
            entity = parent.0;
        }
        if !quick_ao {
            continue;
        }
        let id = mesh_handle.0.id();
        if !seen.insert(id) {
            continue;
        }
        if !strength.is_changed() && bases.values.contains_key(&id) {
            continue;
        }
        let Some(mut mesh) = meshes.get_mut(id) else {
            continue;
        };
        let Ok(colors) = mesh.try_attribute(Mesh::ATTRIBUTE_COLOR) else {
            continue;
        };
        let baseline = bases.values.entry(id).or_insert_with(|| colors.clone());
        let Ok(colors) = mesh.try_attribute_mut(Mesh::ATTRIBUTE_COLOR) else {
            continue;
        };
        scale_ao_colors(colors, baseline, strength.0);
    }
}

pub(crate) fn scale_ao_colors(
    values: &mut VertexAttributeValues,
    baseline: &VertexAttributeValues,
    strength: f32,
) {
    let strength = strength.clamp(0.0, 1.0);
    match (values, baseline) {
        (VertexAttributeValues::Float32x3(values), VertexAttributeValues::Float32x3(base)) => {
            for (value, base) in values.iter_mut().zip(base) {
                value[0] = scale_ao_channel(base[0], strength);
                value[1] = scale_ao_channel(base[1], strength);
                value[2] = scale_ao_channel(base[2], strength);
            }
        }
        (VertexAttributeValues::Float32x4(values), VertexAttributeValues::Float32x4(base)) => {
            for (value, base) in values.iter_mut().zip(base) {
                value[0] = scale_ao_channel(base[0], strength);
                value[1] = scale_ao_channel(base[1], strength);
                value[2] = scale_ao_channel(base[2], strength);
                value[3] = base[3];
            }
        }
        (VertexAttributeValues::Unorm8x4(values), VertexAttributeValues::Unorm8x4(base)) => {
            for (value, base) in values.iter_mut().zip(base) {
                value[0] = scale_ao_byte(base[0], strength);
                value[1] = scale_ao_byte(base[1], strength);
                value[2] = scale_ao_byte(base[2], strength);
                value[3] = base[3];
            }
        }
        (VertexAttributeValues::Unorm16x4(values), VertexAttributeValues::Unorm16x4(base)) => {
            for (value, base) in values.iter_mut().zip(base) {
                value[0] = scale_ao_u16(base[0], strength);
                value[1] = scale_ao_u16(base[1], strength);
                value[2] = scale_ao_u16(base[2], strength);
                value[3] = base[3];
            }
        }
        (
            VertexAttributeValues::Unorm8x4Bgra(values),
            VertexAttributeValues::Unorm8x4Bgra(base),
        ) => {
            for (value, base) in values.iter_mut().zip(base) {
                value[0] = scale_ao_byte(base[0], strength);
                value[1] = scale_ao_byte(base[1], strength);
                value[2] = scale_ao_byte(base[2], strength);
                value[3] = base[3];
            }
        }
        _ => {}
    }
}

pub(crate) fn scale_ao_channel(value: f32, strength: f32) -> f32 {
    (1.0 - (1.0 - value.clamp(0.0, 1.0)) * strength).clamp(0.0, 1.0)
}

pub(crate) fn scale_ao_byte(value: u8, strength: f32) -> u8 {
    (scale_ao_channel(f32::from(value) / 255.0, strength) * 255.0).round() as u8
}

pub(crate) fn scale_ao_u16(value: u16, strength: f32) -> u16 {
    (scale_ao_channel(f32::from(value) / 65_535.0, strength) * 65_535.0).round() as u16
}

pub(crate) fn toggle_lights_disabled(
    keys: Res<ButtonInput<KeyCode>>,
    mut disabled: ResMut<LightsDisabled>,
) {
    if keys.just_pressed(KeyCode::F3) {
        disabled.0 = !disabled.0;
        info!(
            "all runtime lights: {}",
            if disabled.0 { "off" } else { "on" }
        );
    }
}

pub(crate) fn apply_lighting_scale(
    lighting: Res<LightingScale>,
    ambient_scale: Res<AmbientScale>,
    disabled: Res<LightsDisabled>,
    mut ambient: ResMut<GlobalAmbientLight>,
    mut points: Query<&mut PointLight>,
    mut directionals: Query<(&CellDirectionalLight, &mut DirectionalLight)>,
) {
    if !lighting.is_changed() && !ambient_scale.is_changed() && !disabled.is_changed() {
        return;
    }
    ambient.brightness = if disabled.0 {
        0.0
    } else {
        25.0 * lighting.0 * ambient_scale.0
    };
    for mut light in &mut points {
        light.intensity = if disabled.0 {
            0.0
        } else {
            light.range * light.range * 2.0 * lighting.0
        };
    }
    for (cell_light, mut light) in &mut directionals {
        light.illuminance =
            scaled_directional_illuminance(cell_light.base_illuminance, lighting.0, disabled.0);
    }
}

pub(crate) fn toggle_unlit_mode(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<UnlitMode>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        mode.0 = !mode.0;
        info!(
            "unlit diagnostic mode: {}",
            if mode.0 { "on" } else { "off" }
        );
    }
}

pub(crate) fn apply_unlit_mode(
    mode: Res<UnlitMode>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !mode.is_changed() {
        return;
    }

    for material in materials.iter_mut().map(|(_, material)| material) {
        material.unlit = mode.0;
    }
}

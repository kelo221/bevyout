//! Viewer cursor, adjustment, and diagnostic controls.

use bevy::input::keyboard::{Key, KeyboardFocusLost};

use super::scene::CellDirectionalLight;
use super::*;

/// Issue #131: Bevy 0.19.0's `bevy_input::keyboard::keyboard_input_system`
/// releases `ButtonInput<KeyCode>` on `KeyboardFocusLost` (e.g. Cmd+Tab away
/// from the window) but never releases the logical `ButtonInput<Key>`
/// resource. A stuck `Key::Super` then makes `bevy_ui_widgets`' text input
/// treat every subsequent keystroke as a Cmd-chord, breaking console typing.
/// Mirror Bevy's own `release_all()` call for the logical resource here; this
/// system (and its registration) can be deleted once upstream fixes the
/// asymmetry. Runs in `PreUpdate` after `bevy::input::InputSystems` so it is
/// not immediately clobbered by that frame's `ButtonInput::clear()`.
pub(crate) fn release_stuck_logical_modifiers_on_focus_lost(
    mut key_input: ResMut<ButtonInput<Key>>,
    mut focus_lost: MessageReader<KeyboardFocusLost>,
) {
    if focus_lost.read().next().is_some() {
        key_input.release_all();
    }
}

pub(crate) fn capture_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub(crate) fn capture_cursor_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    modal: Res<State<GameplayModal>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if modal.get() != &GameplayModal::None {
        return;
    }
    if keys.just_pressed(KeyCode::Escape) {
        cursor_options.visible = true;
        cursor_options.grab_mode = CursorGrabMode::None;
    } else if mouse_buttons.just_pressed(MouseButton::Left) {
        cursor_options.visible = false;
        cursor_options.grab_mode = CursorGrabMode::Locked;
    }
}

/// Mouse motion generated while a modal owns the cursor, plus the synthetic
/// warp emitted when pointer lock resumes, must never reach a camera.
pub(crate) fn mouse_look_is_safe(
    was_captured: &mut bool,
    captured: bool,
    gameplay_active: bool,
) -> bool {
    if !gameplay_active {
        *was_captured = false;
        return false;
    }
    let newly_captured = captured && !*was_captured;
    *was_captured = captured;
    captured && !newly_captured
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

#[derive(Resource)]
pub(crate) struct LightsDisabled(pub(crate) bool);

pub(crate) const MIN_HORIZONTAL_FOV_DEGREES: f32 = 10.0;
pub(crate) const MAX_HORIZONTAL_FOV_DEGREES: f32 = 170.0;

#[derive(Component, Clone, Copy, Debug)]
pub(crate) struct HorizontalFov(pub(crate) f32);

impl Default for HorizontalFov {
    fn default() -> Self {
        Self(DEFAULT_HORIZONTAL_FOV_DEGREES)
    }
}

pub(crate) fn horizontal_to_vertical_fov(horizontal_degrees: f32, aspect_ratio: f32) -> f32 {
    let aspect_ratio = aspect_ratio.max(f32::EPSILON);
    2.0 * ((horizontal_degrees.to_radians() * 0.5).tan() / aspect_ratio).atan()
}

pub(crate) fn default_perspective_projection() -> PerspectiveProjection {
    let aspect_ratio = DEFAULT_WINDOW_WIDTH as f32 / DEFAULT_WINDOW_HEIGHT as f32;
    PerspectiveProjection {
        fov: horizontal_to_vertical_fov(DEFAULT_HORIZONTAL_FOV_DEGREES, aspect_ratio),
        aspect_ratio,
        ..default()
    }
}

pub(crate) fn apply_horizontal_fov(
    mut cameras: Query<(&HorizontalFov, &mut Projection), With<Camera3d>>,
) {
    for (horizontal, mut projection) in &mut cameras {
        let Projection::Perspective(perspective) = &*projection else {
            continue;
        };
        let target = horizontal_to_vertical_fov(horizontal.0, perspective.aspect_ratio);
        if (perspective.fov - target).abs() <= f32::EPSILON {
            continue;
        }
        let Projection::Perspective(perspective) = &mut *projection else {
            unreachable!("projection variant was checked above");
        };
        perspective.fov = target;
    }
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

#[cfg(test)]
mod tests {
    use bevy::app::PreUpdate;
    use bevy::ecs::entity::Entity;
    use bevy::ecs::message::Messages;
    use bevy::input::ButtonState;
    use bevy::input::keyboard::{Key, KeyCode, KeyboardFocusLost, KeyboardInput};
    use bevy::input::{ButtonInput, InputPlugin, InputSystems};
    use bevy::prelude::{App, IntoScheduleConfigs, MinimalPlugins};

    use super::{
        horizontal_to_vertical_fov, mouse_look_is_safe,
        release_stuck_logical_modifiers_on_focus_lost,
    };

    /// Issue #131 regression test: a real `bevy_input::InputPlugin` (so
    /// `keyboard_input_system` runs exactly as it does in the viewer) drives
    /// `Key::Super` pressed, then a `KeyboardFocusLost` message arrives.
    /// Without `release_stuck_logical_modifiers_on_focus_lost` registered
    /// after `InputSystems`, `ButtonInput<Key>` stays stuck on `Key::Super`
    /// even though `ButtonInput<KeyCode>` correctly clears -- this is the
    /// asymmetry that broke console typing after Cmd+Tab.
    #[test]
    fn focus_lost_releases_stuck_logical_super_after_input_systems() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin)).add_systems(
            PreUpdate,
            release_stuck_logical_modifiers_on_focus_lost.after(InputSystems),
        );

        // Frame 1: press Super through the real keyboard_input_system, the
        // same path winit uses.
        app.world_mut()
            .resource_mut::<Messages<KeyboardInput>>()
            .write(KeyboardInput {
                key_code: KeyCode::SuperLeft,
                logical_key: Key::Super,
                state: ButtonState::Pressed,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        assert!(
            app.world()
                .resource::<ButtonInput<Key>>()
                .pressed(Key::Super),
            "logical Super should be pressed after the KeyboardInput event"
        );
        assert!(
            app.world()
                .resource::<ButtonInput<KeyCode>>()
                .pressed(KeyCode::SuperLeft),
            "physical SuperLeft should be pressed after the KeyboardInput event"
        );

        // Frame 2: the window loses focus (Cmd+Tab away) with no matching
        // key-up event -- the scenario that leaves Key::Super stuck.
        app.world_mut()
            .resource_mut::<Messages<KeyboardFocusLost>>()
            .write(KeyboardFocusLost);
        app.update();

        assert!(
            !app.world()
                .resource::<ButtonInput<Key>>()
                .pressed(Key::Super),
            "our system must release the logical Key::Super that Bevy's own \
             keyboard_input_system leaves stuck on KeyboardFocusLost"
        );
        assert!(
            app.world()
                .resource::<ButtonInput<Key>>()
                .just_released(Key::Super),
            "release should land as just_released in the same frame the \
             focus-lost message arrives, which requires running after \
             InputSystems rather than before it"
        );
        assert!(
            !app.world()
                .resource::<ButtonInput<KeyCode>>()
                .pressed(KeyCode::SuperLeft),
            "physical SuperLeft release (Bevy's own release_all) must still work"
        );
    }

    #[test]
    fn horizontal_fov_conversion_matches_a_16_by_9_camera() {
        let vertical = horizontal_to_vertical_fov(90.0, 16.0 / 9.0);
        assert!((vertical.to_degrees() - 58.715_508).abs() < 1e-5);
    }

    #[test]
    fn mouse_look_discards_modal_motion_and_first_recaptured_frame() {
        let mut captured = true;
        assert!(!mouse_look_is_safe(&mut captured, true, false));
        assert!(!captured);
        assert!(!mouse_look_is_safe(&mut captured, true, true));
        assert!(captured);
        assert!(mouse_look_is_safe(&mut captured, true, true));
    }
}

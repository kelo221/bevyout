//! Viewer cursor, adjustment, and diagnostic controls.

use bevy::ecs::system::NonSendMarker;
use bevy::input::keyboard::{Key, KeyboardFocusLost};
use bevy::window::{PrimaryWindow, WindowFocused};

use super::scene::CellDirectionalLight;
use super::*;

/// Issue #131: Bevy 0.19.0's `bevy_input::keyboard::keyboard_input_system`
/// releases `ButtonInput<KeyCode>` on `KeyboardFocusLost` (e.g. Cmd+Tab away
/// from the window) but never releases the logical `ButtonInput<Key>`
/// resource. A stuck `Key::Super` then makes `bevy_ui_widgets`' text input
/// treat every subsequent keystroke as a Cmd-chord, breaking console typing.
///
/// `KeyboardFocusLost` alone is not enough. `bevy_winit`'s
/// `check_keyboard_focus_lost` (bevy_winit 0.19.0 `src/system.rs`, ~line 74)
/// only emits synthetic key-up events and `KeyboardFocusLost` when it drains
/// a frame's `WindowFocused` messages and finds `focus_gained == false` --
/// i.e. no `WindowFocused { focused: true }` was read that frame. macOS's
/// Cmd+Shift+5 screen-recording overlay stalls the winit event loop long
/// enough that the `Focused(false)`/`Focused(true)` pair for that window
/// coalesces into a single frame's message batch: `focus_gained` ends up
/// `true`, so neither the synthetic releases nor `KeyboardFocusLost` fire,
/// and Bevy's own `ButtonInput<KeyCode>` release_all() is suppressed too --
/// both the physical and logical modifier state stay stuck.
///
/// So this system triggers on either signal: any `WindowFocused` message at
/// all (covers the same-frame bounce, in both directions) or a
/// `KeyboardFocusLost` message (belt and braces, covering whatever winit
/// path emits it without a paired `WindowFocused`). On trigger it releases
/// both `ButtonInput<Key>` and `ButtonInput<KeyCode>`, mirroring and
/// extending Bevy's own `release_all()`. Trade-off accepted: a modifier
/// genuinely held across a real refocus now reads as released until
/// re-pressed, which is benign compared to a permanently stuck modifier.
/// Runs in `PreUpdate` after `bevy::input::InputSystems` so it is not
/// immediately clobbered by that frame's `ButtonInput::clear()`. This system
/// (and its registration) can be deleted once upstream fixes the asymmetry.
pub(crate) fn release_stuck_keys_on_focus_change(
    mut key_input: ResMut<ButtonInput<Key>>,
    mut key_code_input: ResMut<ButtonInput<KeyCode>>,
    mut window_focused: MessageReader<WindowFocused>,
    mut focus_lost: MessageReader<KeyboardFocusLost>,
) {
    let mut window_focus_changed = 0usize;
    for event in window_focused.read() {
        window_focus_changed += 1;
        info!(
            "focus event focused={} window={:?}",
            event.focused, event.window
        );
    }
    let keyboard_focus_lost = focus_lost.read().count();
    if window_focus_changed > 0 || keyboard_focus_lost > 0 {
        let released_keys = key_input.get_pressed().count();
        info!(
            "focus release window_focused={window_focus_changed} \
             keyboard_focus_lost={keyboard_focus_lost} released_keys={released_keys}"
        );
        key_input.release_all();
        key_code_input.release_all();
    }
}

/// Issue #131 follow-up: macOS's Cmd+Shift+5 screen-recording overlay can
/// leave the window unfocused *forever*, not just stuck-key-until-next-focus
/// as `release_stuck_keys_on_focus_change` above assumes. Once the overlay's
/// activation dance confuses winit, macOS keeps routing mouse events to the
/// window (clicks, selection) but stops delivering `Focused(true)` even when
/// the user clicks back into it -- no `WindowFocused` message ever arrives,
/// so the console stays dead for the rest of the recording.
///
/// Mitigation: when the window is unfocused and the user just clicked inside
/// it, ask winit to focus it. Gated on a fresh click (`just_pressed`), not
/// merely "unfocused this frame", so this never fights legitimate app
/// switching (e.g. Cmd+Tab away) by re-stealing focus every frame the window
/// happens to be unfocused.
///
/// Two ways to request focus were considered:
/// 1. Write `Window::focused = true` and let `bevy_winit`'s
///    `changed_windows` system (bevy_winit 0.19.0 `src/system.rs` ~line
///    517) call `winit_window.focus_window()` when it sees the component
///    flip false->true.
/// 2. Call `winit_window.focus_window()` directly from here.
///
/// (1) is tempting but fragile: `Window::focused` is otherwise
/// event-driven, so writing it directly desyncs it from reality if the OS
/// silently drops the focus request (which is exactly the failure mode
/// here) -- the component would read stuck `true` with no confirming
/// `WindowFocused { focused: true }` message ever arriving, and every
/// future click would then see `focused` already `true` and never retry.
/// Making that safe needs its own confirm/expire bookkeeping.
///
/// (2) sidesteps all of that: a direct call retries naturally on every
/// click and never touches ECS-visible state, so there is nothing to get
/// stuck. The catch is that `bevy_winit` 0.19 doesn't expose `WinitWindows`
/// as a queryable `NonSend` resource -- it lives behind the `WINIT_WINDOWS`
/// thread-local (see `bevy_winit::lib.rs`, "Temporary storage of
/// WinitWindows data to replace usage of `!Send` resources", pending
/// upstream issue #17667) and is only guaranteed populated on the main
/// thread. `NonSendMarker` (bevy_ecs 0.19.0
/// `system/system_param.rs::NonSendMarker`) is the same trick
/// `bevy_winit`'s own `changed_windows` system uses to pin itself to the
/// main thread for exactly this reason; taking it here does the same.
/// Chose (2).
pub(crate) fn should_request_focus(focused: bool, clicked: bool) -> bool {
    !focused && clicked
}

pub(crate) fn request_focus_on_click_while_unfocused(
    primary_window: Single<(Entity, &Window), With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    // Forces this system onto the main thread, matching how the
    // `WINIT_WINDOWS` thread-local is populated. See the doc comment above.
    _non_send_marker: NonSendMarker,
) {
    let (entity, window) = *primary_window;
    let clicked = mouse_buttons.get_just_pressed().next().is_some();
    if !should_request_focus(window.focused, clicked) {
        return;
    }
    info!("focus request: click while unfocused");
    bevy::winit::WINIT_WINDOWS.with_borrow(|winit_windows| {
        if let Some(winit_window) = winit_windows.get_window(entity) {
            winit_window.focus_window();
        }
    });
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

/// Live multiplier for the cell-driven volumetric fog density. This is kept
/// separate from `FogStrength` so the existing distance-fog tuning remains
/// unchanged. The 0.05 baseline preserves authored cell values without the
/// over-dense result observed in SuperDuperMart.
pub(crate) const DEFAULT_VOLUMETRIC_FOG_MULTIPLIER: f32 = 0.05;

#[derive(Resource)]
pub(crate) struct VolumetricFogMultiplier(pub(crate) f32);

#[derive(Resource)]
pub(crate) struct AoStrength(pub(crate) f32);

/// Runtime multiplier for shader-authorized StandardMaterial emissive colors.
/// This keeps authored Fallout lights visible without returning to the
/// original whiteout-prone source intensity; it remains live-tunable with
/// `setrender emission`.
pub(crate) const DEFAULT_EMISSION_SCALE: f32 = 0.15;

#[derive(Resource)]
pub(crate) struct EmissionScale(pub(crate) f32);

/// Explicit runtime overrides for bloom values that would otherwise be
/// derived from the active Fallout ImageSpace. The override remains active
/// across cell swaps until the viewer is restarted, preserving the existing
/// live-tuning behavior of `setrender bloom_*`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Resource)]
pub(crate) struct ImageSpaceBloomOverrides {
    pub(crate) intensity: Option<f32>,
    pub(crate) threshold: Option<f32>,
    pub(crate) softness: Option<f32>,
}

/// Material handles whose GLB metadata explicitly authorizes Fallout
/// emission. The live emission console control must not turn arbitrary
/// StandardMaterials (including legacy or synthetic props) into emitters.
#[derive(Resource, Default)]
pub(crate) struct AuthorizedEmissionMaterials {
    pub(crate) ids: HashSet<AssetId<StandardMaterial>>,
    pub(crate) revision: u64,
}

impl AuthorizedEmissionMaterials {
    pub(crate) fn set(&mut self, id: AssetId<StandardMaterial>, authorized: bool) {
        let changed = if authorized {
            self.ids.insert(id)
        } else {
            self.ids.remove(&id)
        };
        if changed {
            self.revision = self.revision.wrapping_add(1);
        }
    }
}

#[derive(Default)]
pub(crate) struct EmissionMaterialState {
    baselines: HashMap<AssetId<StandardMaterial>, LinearRgba>,
    last_asset_count: usize,
    last_authorized_revision: u64,
}

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

pub(crate) fn apply_emission_scale(
    scale: Res<EmissionScale>,
    authorized: Res<AuthorizedEmissionMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: Local<EmissionMaterialState>,
) {
    if !scale.is_changed()
        && state.last_asset_count == materials.len()
        && state.last_authorized_revision == authorized.revision
    {
        return;
    }
    state.last_asset_count = materials.len();
    state.last_authorized_revision = authorized.revision;

    let scale = scale.0.clamp(0.0, 1.0);
    for (id, material) in materials.iter_mut() {
        if !authorized.ids.contains(&id) {
            continue;
        }
        let baseline = *state.baselines.entry(id).or_insert(material.emissive);
        let [red, green, blue, alpha] = baseline.to_f32_array();
        material.emissive = LinearRgba::new(red * scale, green * scale, blue * scale, alpha);
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::{PreUpdate, Update};
    use bevy::asset::Assets;
    use bevy::color::LinearRgba;
    use bevy::ecs::entity::Entity;
    use bevy::ecs::message::Messages;
    use bevy::input::ButtonState;
    use bevy::input::keyboard::{Key, KeyCode, KeyboardFocusLost, KeyboardInput};
    use bevy::input::mouse::MouseButtonInput;
    use bevy::input::{ButtonInput, InputPlugin, InputSystems};
    use bevy::pbr::StandardMaterial;
    use bevy::prelude::{
        App, ColorToComponents, IntoScheduleConfigs, MinimalPlugins, MouseButton, Window, default,
    };
    use bevy::window::{PrimaryWindow, WindowFocused};

    use super::{
        AuthorizedEmissionMaterials, EmissionScale, apply_emission_scale,
        horizontal_to_vertical_fov, mouse_look_is_safe, release_stuck_keys_on_focus_change,
        request_focus_on_click_while_unfocused, should_request_focus,
    };

    #[test]
    fn emission_scale_reuses_baseline_without_scaling_exposure_alpha() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<AuthorizedEmissionMaterials>()
            .insert_resource(EmissionScale(0.0))
            .add_systems(Update, apply_emission_scale);
        let material = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                emissive: LinearRgba::new(2.0, 1.0, 0.5, 0.75),
                ..default()
            });
        app.world_mut()
            .resource_mut::<AuthorizedEmissionMaterials>()
            .set(material.id(), true);

        app.update();
        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(&material)
                .unwrap()
                .emissive
                .to_f32_array(),
            [0.0, 0.0, 0.0, 0.75]
        );

        app.world_mut().resource_mut::<EmissionScale>().0 = 0.25;
        app.update();
        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(&material)
                .unwrap()
                .emissive
                .to_f32_array(),
            [0.5, 0.25, 0.125, 0.75]
        );

        app.world_mut().resource_mut::<EmissionScale>().0 = 0.1;
        app.update();
        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(&material)
                .unwrap()
                .emissive
                .to_f32_array(),
            [0.2, 0.1, 0.05, 0.75]
        );
    }

    #[test]
    fn emission_scale_ignores_materials_without_shader_authorization() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Assets<StandardMaterial>>()
            .init_resource::<AuthorizedEmissionMaterials>()
            .insert_resource(EmissionScale(1.0))
            .add_systems(Update, apply_emission_scale);
        let material = app
            .world_mut()
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                emissive: LinearRgba::new(2.0, 1.0, 0.5, 0.75),
                ..default()
            });

        app.update();

        assert_eq!(
            app.world()
                .resource::<Assets<StandardMaterial>>()
                .get(&material)
                .unwrap()
                .emissive
                .to_f32_array(),
            [2.0, 1.0, 0.5, 0.75]
        );
    }

    fn app_with_system() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin))
            .add_message::<WindowFocused>()
            .add_systems(
                PreUpdate,
                release_stuck_keys_on_focus_change.after(InputSystems),
            );
        app
    }

    fn press_super(app: &mut App) {
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
    }

    /// Issue #131 regression test: a real `bevy_input::InputPlugin` (so
    /// `keyboard_input_system` runs exactly as it does in the viewer) drives
    /// `Key::Super` pressed, then a `KeyboardFocusLost` message arrives.
    /// Without `release_stuck_keys_on_focus_change` registered after
    /// `InputSystems`, `ButtonInput<Key>` stays stuck on `Key::Super` even
    /// though `ButtonInput<KeyCode>` correctly clears -- this is the
    /// asymmetry that broke console typing after Cmd+Tab.
    #[test]
    fn focus_lost_releases_stuck_logical_super_after_input_systems() {
        let mut app = app_with_system();

        // Frame 1: press Super through the real keyboard_input_system, the
        // same path winit uses.
        press_super(&mut app);
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

    /// Issue #131 root-cause test: macOS's Cmd+Shift+5 screen-recording
    /// overlay stalls the winit event loop long enough that the window's
    /// `Focused(false)`/`Focused(true)` pair coalesces into a single frame.
    /// `bevy_winit::system::check_keyboard_focus_lost` only emits synthetic
    /// key releases and `KeyboardFocusLost` when it sees no
    /// `WindowFocused { focused: true }` in that frame's batch -- with the
    /// bounce, it sees one, so neither fires and both `ButtonInput<Key>` and
    /// `ButtonInput<KeyCode>` stay stuck. Simulate that: press Super/Shift,
    /// then in one frame write a false/true `WindowFocused` pair with no
    /// `KeyboardFocusLost` and no release `KeyboardInput` -- our system must
    /// still clear both resources because it triggers on `WindowFocused`
    /// alone, not just `KeyboardFocusLost`.
    #[test]
    fn window_focus_bounce_releases_stuck_keys_without_focus_lost_message() {
        let mut app = app_with_system();
        let window = app.world_mut().spawn_empty().id();

        // Frame 1: press Super and Shift through the real
        // keyboard_input_system.
        press_super(&mut app);
        app.world_mut()
            .resource_mut::<Messages<KeyboardInput>>()
            .write(KeyboardInput {
                key_code: KeyCode::ShiftLeft,
                logical_key: Key::Shift,
                state: ButtonState::Pressed,
                text: None,
                repeat: false,
                window: Entity::PLACEHOLDER,
            });
        app.update();
        assert!(
            app.world()
                .resource::<ButtonInput<Key>>()
                .pressed(Key::Super)
        );
        assert!(
            app.world()
                .resource::<ButtonInput<Key>>()
                .pressed(Key::Shift)
        );
        assert!(
            app.world()
                .resource::<ButtonInput<KeyCode>>()
                .pressed(KeyCode::SuperLeft)
        );
        assert!(
            app.world()
                .resource::<ButtonInput<KeyCode>>()
                .pressed(KeyCode::ShiftLeft)
        );

        // Frame 2: a same-frame focus false->true bounce, with no
        // KeyboardFocusLost and no release KeyboardInput -- the suppressed
        // bounce that check_keyboard_focus_lost's `!focus_gained` gate
        // produces.
        {
            let mut focused = app.world_mut().resource_mut::<Messages<WindowFocused>>();
            focused.write(WindowFocused {
                window,
                focused: false,
            });
            focused.write(WindowFocused {
                window,
                focused: true,
            });
        }
        app.update();

        assert!(
            !app.world()
                .resource::<ButtonInput<Key>>()
                .pressed(Key::Super),
            "logical Super must be released on a same-frame focus bounce \
             even without a KeyboardFocusLost message"
        );
        assert!(
            !app.world()
                .resource::<ButtonInput<Key>>()
                .pressed(Key::Shift),
            "logical Shift must be released on a same-frame focus bounce"
        );
        assert!(
            !app.world()
                .resource::<ButtonInput<KeyCode>>()
                .pressed(KeyCode::SuperLeft),
            "physical SuperLeft must be released on a same-frame focus bounce \
             because Bevy's own keyboard_input_system release_all() is \
             suppressed by the same !focus_gained gate"
        );
        assert!(
            !app.world()
                .resource::<ButtonInput<KeyCode>>()
                .pressed(KeyCode::ShiftLeft),
            "physical ShiftLeft must be released on a same-frame focus bounce"
        );
    }

    /// Startup case: a lone `WindowFocused { focused: true }` (the window
    /// gaining focus for the first time) with nothing pressed must not
    /// panic, and must leave key state empty.
    #[test]
    fn lone_initial_focus_gained_with_nothing_pressed_is_a_no_op() {
        let mut app = app_with_system();
        let window = app.world_mut().spawn_empty().id();

        app.world_mut()
            .resource_mut::<Messages<WindowFocused>>()
            .write(WindowFocused {
                window,
                focused: true,
            });
        app.update();

        assert!(
            app.world()
                .resource::<ButtonInput<Key>>()
                .get_pressed()
                .next()
                .is_none(),
            "no logical keys should be pressed after a startup focus-gained \
             message with nothing pressed"
        );
        assert!(
            app.world()
                .resource::<ButtonInput<KeyCode>>()
                .get_pressed()
                .next()
                .is_none(),
            "no physical keys should be pressed after a startup focus-gained \
             message with nothing pressed"
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

    // Issue #131 follow-up: `should_request_focus` is the pure decision
    // logic behind `request_focus_on_click_while_unfocused`. The system
    // itself calls out to the `WINIT_WINDOWS` thread-local (a real winit
    // backend, absent in these headless tests), so the gating decision is
    // exercised directly here rather than through the system's side effect.

    #[test]
    fn should_request_focus_when_unfocused_and_just_clicked() {
        assert!(should_request_focus(false, true));
    }

    #[test]
    fn should_not_request_focus_when_already_focused() {
        assert!(!should_request_focus(true, true));
    }

    #[test]
    fn should_not_request_focus_without_a_click() {
        assert!(!should_request_focus(false, false));
    }

    fn app_with_focus_request_system() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin))
            .add_systems(Update, request_focus_on_click_while_unfocused);
        app
    }

    fn spawn_primary_window(app: &mut App, focused: bool) -> Entity {
        app.world_mut()
            .spawn((
                Window {
                    focused,
                    ..Default::default()
                },
                PrimaryWindow,
            ))
            .id()
    }

    fn click_left_mouse_button(app: &mut App) {
        app.world_mut()
            .resource_mut::<Messages<MouseButtonInput>>()
            .write(MouseButtonInput {
                button: MouseButton::Left,
                state: ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
    }

    /// System-level sanity check backing the "direct winit call, not a
    /// `Window.focused` write" choice documented on
    /// `request_focus_on_click_while_unfocused`: even with an unfocused
    /// window and a fresh click -- the case that requests focus -- the
    /// system must run without a real winit backend registered (headless
    /// `MinimalPlugins`, so `WINIT_WINDOWS` finds no matching window and the
    /// call is a no-op) and it must leave the `Window` component itself
    /// untouched. A component-write mitigation would instead force
    /// `focused` to `true` here, which is exactly the state corruption this
    /// approach avoids.
    #[test]
    fn request_focus_system_runs_headless_without_touching_window_component() {
        let mut app = app_with_focus_request_system();
        let window = spawn_primary_window(&mut app, false);
        click_left_mouse_button(&mut app);

        app.update();

        assert!(
            !app.world().get::<Window>(window).unwrap().focused,
            "the direct winit call must not write the `Window` component; \
             only a confirmed `WindowFocused` message may flip it"
        );
    }

    /// Focused window + click: nothing to request, and (as above) the
    /// component must stay untouched either way.
    #[test]
    fn request_focus_system_is_a_no_op_when_already_focused() {
        let mut app = app_with_focus_request_system();
        let window = spawn_primary_window(&mut app, true);
        click_left_mouse_button(&mut app);

        app.update();

        assert!(app.world().get::<Window>(window).unwrap().focused);
    }

    /// Unfocused window, no click: `should_request_focus` gates this out,
    /// so the system must not even attempt the winit call.
    #[test]
    fn request_focus_system_is_a_no_op_without_a_click() {
        let mut app = app_with_focus_request_system();
        let window = spawn_primary_window(&mut app, false);

        app.update();

        assert!(!app.world().get::<Window>(window).unwrap().focused);
    }
}

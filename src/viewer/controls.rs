//! Viewer cursor, adjustment, and diagnostic controls.

use bevy::asset::AssetEvent;
use bevy::ecs::system::NonSendMarker;
use bevy::input::keyboard::{Key, KeyboardFocusLost};
use bevy::light::EnvironmentMapLight;
use bevy::window::{PrimaryWindow, WindowFocused};

use super::ao_policy::AoEligibilityTracker;

use super::scene::{
    CellDirectionalLight, DEFAULT_REFLECTION_PROBE_STRENGTH, PREPARED_REFLECTION_PROBE_INTENSITY,
    PreparedReflectionProbe,
};
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

/// Reversible runtime gate for StandardMaterial metalness.
///
/// Disabling the gate snapshots every loaded material's metallic factor and
/// forces it to zero. Materials that finish loading while disabled are
/// captured on the next update. Re-enabling restores the exact snapshots.
#[derive(Resource)]
pub(crate) struct MetallicGate {
    enabled: bool,
    baselines: HashMap<AssetId<StandardMaterial>, f32>,
}

impl Default for MetallicGate {
    fn default() -> Self {
        Self {
            enabled: true,
            baselines: HashMap::new(),
        }
    }
}

impl MetallicGate {
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// Reversible runtime gate for the dielectric reflectance term.
///
/// `StandardMaterial::reflectance` contributes to dielectric F0 but not the
/// base-color F0 of pure metals. Disabling this gate therefore isolates
/// non-metal specular response without destroying authored metallic shading.
#[derive(Resource)]
pub(crate) struct DielectricSpecularGate {
    enabled: bool,
    baselines: HashMap<AssetId<StandardMaterial>, f32>,
}

impl Default for DielectricSpecularGate {
    fn default() -> Self {
        Self {
            enabled: true,
            baselines: HashMap::new(),
        }
    }
}

impl DielectricSpecularGate {
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

pub(crate) const MIN_ROUGHNESS_SCALE: f32 = 0.5;
pub(crate) const MAX_ROUGHNESS_SCALE: f32 = 2.0;
pub(crate) const MIN_REFLECTION_PROBE_STRENGTH: f32 = 0.0;
pub(crate) const MAX_REFLECTION_PROBE_STRENGTH: f32 = 4096.0;

/// Reversible multiplier over each material's original perceptual roughness.
///
/// Baselines remain stable while the scale changes, so repeated console
/// adjustments never compound. Returning to 1 restores exact authored values.
#[derive(Resource)]
pub(crate) struct RoughnessScale {
    scale: f32,
    baselines: HashMap<AssetId<StandardMaterial>, f32>,
}

impl Default for RoughnessScale {
    fn default() -> Self {
        Self {
            scale: 1.0,
            baselines: HashMap::new(),
        }
    }
}

impl RoughnessScale {
    pub(crate) fn scale(&self) -> f32 {
        self.scale
    }

    pub(crate) fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }
}

/// Runtime multiplier over the prepared reflection probe intensity.
///
/// Enablement remains independent so `setrender reflection_probes 0|1` can
/// temporarily gate probes without discarding a tuned strength.
#[derive(Clone, Copy, Debug, Resource)]
pub(crate) struct ReflectionProbeSettings {
    enabled: bool,
    strength: f32,
}

impl Default for ReflectionProbeSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            strength: DEFAULT_REFLECTION_PROBE_STRENGTH,
        }
    }
}

impl ReflectionProbeSettings {
    pub(crate) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub(crate) fn strength(&self) -> f32 {
        self.strength
    }

    pub(crate) fn set_strength(&mut self, strength: f32) {
        self.strength = strength;
    }

    fn effective_intensity(&self) -> f32 {
        if self.enabled {
            PREPARED_REFLECTION_PROBE_INTENSITY * self.strength
        } else {
            0.0
        }
    }
}

/// Active Fallout ImageSpace multiplier layered over the viewer's user-facing
/// emission control. Keeping this separate means `setrender emission` remains
/// a stable manual multiplier while cell swaps can still apply each cell's
/// authored HDR emissive strength.
#[derive(Clone, Copy, Debug, PartialEq, Resource)]
pub(crate) struct ImageSpaceEmissionMultiplier(pub(crate) f32);

impl Default for ImageSpaceEmissionMultiplier {
    fn default() -> Self {
        Self(1.0)
    }
}

pub(crate) fn image_space_emission_multiplier(
    image_space: Option<&ImageSpaceInfo>,
) -> ImageSpaceEmissionMultiplier {
    let Some(value) = image_space.map(|image_space| image_space.hdr_emissive_multiplier) else {
        return ImageSpaceEmissionMultiplier::default();
    };
    if value.is_finite() {
        ImageSpaceEmissionMultiplier(value.clamp(0.0, 8.0))
    } else {
        ImageSpaceEmissionMultiplier::default()
    }
}

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

/// Cached AO-mesh eligibility state (issue #270, PERF wave 1):
/// incremented by `track_ao_mesh_eligibility` from entity
/// added/changed/removed signals and consumed by `apply_ao_strength`, so
/// quiet frames never scan the mesh query to count it (the old
/// `AoScanState` count sentinel also missed remove+add pairs with equal
/// totals). The policy decisions live in the Bevy-free
/// `viewer::ao_policy` module.
#[derive(Resource, Default)]
pub(crate) struct AoEligibility {
    pub(crate) tracker: AoEligibilityTracker<Entity, AssetId<Mesh>>,
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
    mut cameras: Query<
        (&HorizontalFov, &mut Projection),
        (
            With<Camera3d>,
            Or<(Changed<HorizontalFov>, Changed<Projection>)>,
        ),
    >,
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

pub(crate) fn apply_reflection_probe_settings(
    settings: Res<ReflectionProbeSettings>,
    new_probes: Query<(), Added<PreparedReflectionProbe>>,
    mut probes: Query<&mut EnvironmentMapLight, With<PreparedReflectionProbe>>,
) {
    if !settings.is_changed() && new_probes.is_empty() {
        return;
    }
    let target = settings.effective_intensity();
    for mut probe in &mut probes {
        if (probe.intensity - target).abs() > f32::EPSILON {
            probe.intensity = target;
        }
    }
}

/// The exact `apply_ao_strength` eligibility test, shared by the event
/// loop below: meshes under a `PlacementRoot::uses_quick_ao` ancestor
/// (up to 64 hops along `ChildOf`, root-only meshes included).
fn entity_uses_quick_ao(
    child_of: Option<&ChildOf>,
    own_root: Option<&interaction::PlacementRoot>,
    parents: &Query<&ChildOf>,
    roots: &Query<&interaction::PlacementRoot>,
) -> bool {
    let Some(child_of) = child_of else {
        return own_root.is_some_and(interaction::PlacementRoot::uses_quick_ao);
    };
    let mut entity = child_of.0;
    for _ in 0..64 {
        if roots
            .get(entity)
            .is_ok_and(interaction::PlacementRoot::uses_quick_ao)
        {
            return true;
        }
        let Ok(parent) = parents.get(entity) else {
            break;
        };
        entity = parent.0;
    }
    false
}

/// Maintains `AoEligibility` from mesh-entity lifecycle signals instead of
/// a per-frame count: handle creation/swaps surface through
/// `Changed<Mesh3d>` discovery, despawn/component-removal through
/// `RemovedComponents<Mesh3d>`. Quiet frames read zero signals and touch
/// nothing.
pub(crate) fn track_ao_mesh_eligibility(
    mut eligibility: ResMut<AoEligibility>,
    discovered: Query<
        (
            Entity,
            &Mesh3d,
            Option<&ChildOf>,
            Option<&interaction::PlacementRoot>,
        ),
        Changed<Mesh3d>,
    >,
    parents: Query<&ChildOf>,
    roots: Query<&interaction::PlacementRoot>,
    mut removed_meshes: RemovedComponents<Mesh3d>,
) {
    for entity in removed_meshes.read() {
        eligibility.tracker.release(entity);
    }
    for (entity, mesh, child_of, own_root) in &discovered {
        eligibility.tracker.discover(
            entity,
            mesh.0.id(),
            entity_uses_quick_ao(child_of, own_root, &parents, &roots),
        );
    }
}

pub(crate) fn apply_ao_strength(
    strength: Res<AoStrength>,
    mut bases: ResMut<AoMeshBases>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut eligibility: ResMut<AoEligibility>,
    mut mesh_events: MessageReader<AssetEvent<Mesh>>,
) {
    // Asset-store signals keep the cache honest without a scan: a reload
    // re-queues still-referenced meshes (their old baseline was captured
    // from already-scaled colors), a removal drops its stale baseline.
    for event in mesh_events.read() {
        match event {
            AssetEvent::Added { id } => eligibility.tracker.asset_added(*id),
            AssetEvent::Removed { id } => {
                bases.values.remove(id);
            }
            _ => {}
        }
    }

    let strength_changed = strength.is_changed();
    if !strength_changed && !eligibility.tracker.has_pending() {
        return;
    }

    let mut targets: HashSet<AssetId<Mesh>> = eligibility.tracker.pending_meshes().collect();
    if strength_changed {
        targets.extend(eligibility.tracker.eligible_meshes());
    }
    for id in targets {
        if !strength_changed && bases.values.contains_key(&id) {
            eligibility.tracker.resolve_pending(id);
            continue;
        }
        let Some(mut mesh) = meshes.get_mut(id) else {
            // Keep queued until the asset load lands.
            continue;
        };
        eligibility.tracker.resolve_pending(id);
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

pub(crate) fn apply_metallic_gate(
    mut gate: ResMut<MetallicGate>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if gate.enabled {
        if gate.baselines.is_empty() {
            return;
        }
        let baselines = std::mem::take(&mut gate.baselines);
        for (id, baseline) in baselines {
            let Some(material) = materials.get(id) else {
                continue;
            };
            if material.metallic != baseline
                && let Some(mut material) = materials.get_mut(id)
            {
                material.metallic = baseline;
            }
        }
        return;
    }

    let newly_loaded = materials
        .iter()
        .filter(|(id, _)| !gate.baselines.contains_key(id))
        .map(|(id, material)| (id, material.metallic))
        .collect::<Vec<_>>();
    gate.baselines.extend(newly_loaded);

    let metallic_ids = materials
        .iter()
        .filter(|(_, material)| material.metallic != 0.0)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    for id in metallic_ids {
        if let Some(mut material) = materials.get_mut(id) {
            material.metallic = 0.0;
        }
    }
}

pub(crate) fn apply_dielectric_specular_gate(
    mut gate: ResMut<DielectricSpecularGate>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if gate.enabled {
        if gate.baselines.is_empty() {
            return;
        }
        let baselines = std::mem::take(&mut gate.baselines);
        for (id, baseline) in baselines {
            let Some(material) = materials.get(id) else {
                continue;
            };
            if material.reflectance != baseline
                && let Some(mut material) = materials.get_mut(id)
            {
                material.reflectance = baseline;
            }
        }
        return;
    }

    let newly_loaded = materials
        .iter()
        .filter(|(id, _)| !gate.baselines.contains_key(id))
        .map(|(id, material)| (id, material.reflectance))
        .collect::<Vec<_>>();
    gate.baselines.extend(newly_loaded);

    let reflective_ids = materials
        .iter()
        .filter(|(_, material)| material.reflectance != 0.0)
        .map(|(id, _)| id)
        .collect::<Vec<_>>();
    for id in reflective_ids {
        if let Some(mut material) = materials.get_mut(id) {
            material.reflectance = 0.0;
        }
    }
}

pub(crate) fn apply_roughness_scale(
    mut control: ResMut<RoughnessScale>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if control.scale == 1.0 {
        if control.baselines.is_empty() {
            return;
        }
        let baselines = std::mem::take(&mut control.baselines);
        for (id, baseline) in baselines {
            let Some(material) = materials.get(id) else {
                continue;
            };
            if material.perceptual_roughness != baseline
                && let Some(mut material) = materials.get_mut(id)
            {
                material.perceptual_roughness = baseline;
            }
        }
        return;
    }

    let newly_loaded = materials
        .iter()
        .filter(|(id, _)| !control.baselines.contains_key(id))
        .map(|(id, material)| (id, material.perceptual_roughness))
        .collect::<Vec<_>>();
    control.baselines.extend(newly_loaded);

    let scale = control.scale;
    let updates = control
        .baselines
        .iter()
        .filter_map(|(id, baseline)| {
            let target = (baseline * scale).clamp(0.0, 1.0);
            materials
                .get(*id)
                .filter(|material| material.perceptual_roughness != target)
                .map(|_| (*id, target))
        })
        .collect::<Vec<_>>();
    for (id, target) in updates {
        if let Some(mut material) = materials.get_mut(id) {
            material.perceptual_roughness = target;
        }
    }
}

pub(crate) fn apply_emission_scale(
    scale: Res<EmissionScale>,
    image_space_multiplier: Res<ImageSpaceEmissionMultiplier>,
    authorized: Res<AuthorizedEmissionMaterials>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut state: Local<EmissionMaterialState>,
) {
    if !scale.is_changed()
        && !image_space_multiplier.is_changed()
        && state.last_asset_count == materials.len()
        && state.last_authorized_revision == authorized.revision
    {
        return;
    }
    state.last_asset_count = materials.len();
    state.last_authorized_revision = authorized.revision;

    let scale = scale.0.clamp(0.0, 1.0) * image_space_multiplier.0;
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
#[path = "tests/controls.rs"]
mod tests;

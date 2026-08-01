use super::*;
use bevy::app::{PreUpdate, Update};
use bevy::asset::{AssetEvent, Assets, RenderAssetUsages};
use bevy::color::LinearRgba;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::Messages;
use bevy::input::keyboard::{Key, KeyCode, KeyboardFocusLost, KeyboardInput};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::{ButtonInput, ButtonState, InputPlugin, InputSystems};
use bevy::light::EnvironmentMapLight;
use bevy::mesh::PrimitiveTopology;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    default, App, ColorToComponents, IntoScheduleConfigs, MinimalPlugins, MouseButton, Window,
};
use bevy::window::{PrimaryWindow, WindowFocused};

use crate::viewer::scene::{PreparedReflectionProbe, PREPARED_REFLECTION_PROBE_INTENSITY};

fn material_clamp_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<StandardMaterial>>()
        .add_message::<AssetEvent<StandardMaterial>>()
        .insert_resource(ClampWriteProbe::default())
        .insert_resource(MaterialClampSettings::default())
        .insert_resource(MaterialClampBaselines::default())
        .add_systems(
            Update,
            (apply_material_clamps, probe_material_clamp_writes).chain(),
        );
    app
}

/// Change-detection probe chained after the clamp system: a frame with zero
/// hits on both resources is provably free of baseline and material-store
/// writes (Bevy change detection fires on deref-mut only).
#[derive(Resource, Default)]
struct ClampWriteProbe {
    baseline_writes: usize,
    material_store_writes: usize,
}

fn probe_material_clamp_writes(
    baselines: Res<MaterialClampBaselines>,
    materials: Res<Assets<StandardMaterial>>,
    mut probe: ResMut<ClampWriteProbe>,
) {
    probe.baseline_writes += usize::from(baselines.is_changed());
    probe.material_store_writes += usize::from(materials.is_changed());
}

fn clamp_probe_frame(app: &mut App) -> (usize, usize) {
    {
        let mut probe = app.world_mut().resource_mut::<ClampWriteProbe>();
        probe.baseline_writes = 0;
        probe.material_store_writes = 0;
    }
    app.update();
    let probe = app.world().resource::<ClampWriteProbe>();
    (probe.baseline_writes, probe.material_store_writes)
}

fn clamp_settings(app: &mut App) -> bevy::ecs::world::Mut<'_, MaterialClampSettings> {
    app.world_mut().resource_mut::<MaterialClampSettings>()
}

fn write_material_event(app: &mut App, event: AssetEvent<StandardMaterial>) {
    app.world_mut()
        .resource_mut::<Messages<AssetEvent<StandardMaterial>>>()
        .write(event);
}

#[test]
fn metallic_gate_restores_baselines_and_catches_late_loaded_materials() {
    let mut app = material_clamp_test_app();
    let metal = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 1.0,
            ..default()
        });
    let mixed = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 0.35,
            ..default()
        });

    clamp_settings(&mut app).set_metallic_enabled(false);
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&metal)
            .unwrap()
            .metallic,
        0.0
    );
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&mixed)
            .unwrap()
            .metallic,
        0.0
    );

    let late = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 0.8,
            ..default()
        });
    write_material_event(&mut app, AssetEvent::Added { id: late.id() });
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&late)
            .unwrap()
            .metallic,
        0.0
    );

    clamp_settings(&mut app).set_metallic_enabled(true);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&metal).unwrap().metallic, 1.0);
    assert_eq!(materials.get(&mixed).unwrap().metallic, 0.35);
    assert_eq!(materials.get(&late).unwrap().metallic, 0.8);
}

#[test]
fn dielectric_specular_gate_restores_baselines_and_catches_late_loaded_materials() {
    let mut app = material_clamp_test_app();
    let default_reflectance = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            reflectance: 0.5,
            ..default()
        });
    let custom_reflectance = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            reflectance: 0.2,
            ..default()
        });

    clamp_settings(&mut app).set_dielectric_enabled(false);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(
        materials.get(&default_reflectance).unwrap().reflectance,
        0.0
    );
    assert_eq!(materials.get(&custom_reflectance).unwrap().reflectance, 0.0);

    let late = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            reflectance: 0.8,
            ..default()
        });
    write_material_event(&mut app, AssetEvent::Added { id: late.id() });
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&late)
            .unwrap()
            .reflectance,
        0.0
    );

    clamp_settings(&mut app).set_dielectric_enabled(true);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(
        materials.get(&default_reflectance).unwrap().reflectance,
        0.5
    );
    assert_eq!(materials.get(&custom_reflectance).unwrap().reflectance, 0.2);
    assert_eq!(materials.get(&late).unwrap().reflectance, 0.8);
}

#[test]
fn roughness_scale_uses_original_baselines_and_catches_late_loaded_materials() {
    let mut app = material_clamp_test_app();
    let rough = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            perceptual_roughness: 0.4,
            ..default()
        });
    let saturated = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            perceptual_roughness: 0.8,
            ..default()
        });

    clamp_settings(&mut app).set_roughness_scale(1.5);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert!((materials.get(&rough).unwrap().perceptual_roughness - 0.6).abs() < 1e-6);
    assert_eq!(materials.get(&saturated).unwrap().perceptual_roughness, 1.0);

    let late = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            perceptual_roughness: 0.2,
            ..default()
        });
    write_material_event(&mut app, AssetEvent::Added { id: late.id() });
    app.update();
    assert!(
        (app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&late)
            .unwrap()
            .perceptual_roughness
            - 0.3)
            .abs()
            < 1e-6
    );

    clamp_settings(&mut app).set_roughness_scale(0.5);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&rough).unwrap().perceptual_roughness, 0.2);
    assert_eq!(materials.get(&saturated).unwrap().perceptual_roughness, 0.4);
    assert_eq!(materials.get(&late).unwrap().perceptual_roughness, 0.1);

    clamp_settings(&mut app).set_roughness_scale(1.0);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&rough).unwrap().perceptual_roughness, 0.4);
    assert_eq!(materials.get(&saturated).unwrap().perceptual_roughness, 0.8);
    assert_eq!(materials.get(&late).unwrap().perceptual_roughness, 0.2);
}

#[test]
fn material_clamp_baseline_is_dropped_when_its_asset_is_removed() {
    let mut app = material_clamp_test_app();
    let metal = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 1.0,
            ..default()
        });
    let other = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 0.5,
            ..default()
        });

    clamp_settings(&mut app).set_metallic_enabled(false);
    app.update();
    assert_eq!(
        app.world()
            .resource::<MaterialClampBaselines>()
            .store
            .baseline_count(),
        2
    );

    app.world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .remove(metal.id());
    write_material_event(&mut app, AssetEvent::Removed { id: metal.id() });
    app.update();
    assert_eq!(
        app.world()
            .resource::<MaterialClampBaselines>()
            .store
            .baseline_count(),
        1,
        "the removed material's baseline must be dropped on AssetEvent::Removed"
    );

    // Disengagement restores only surviving materials and empties the store.
    clamp_settings(&mut app).set_metallic_enabled(true);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&other).unwrap().metallic, 0.5);
    assert_eq!(
        app.world()
            .resource::<MaterialClampBaselines>()
            .store
            .baseline_count(),
        0
    );
}

#[test]
fn engaged_steady_frames_perform_no_material_store_writes() {
    let mut app = material_clamp_test_app();
    let _metal = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 0.75,
            reflectance: 0.5,
            perceptual_roughness: 0.9,
            ..default()
        });

    {
        let mut settings = clamp_settings(&mut app);
        settings.set_metallic_enabled(false);
        settings.set_dielectric_enabled(false);
        settings.set_roughness_scale(0.5);
    }
    // Absorb the engage pass; the steady-state guarantee starts after it.
    app.update();
    app.update();

    for frame in 0..3 {
        assert_eq!(
            clamp_probe_frame(&mut app),
            (0, 0),
            "engaged steady frame {frame} touched materials or baselines"
        );
    }
}

#[test]
fn modified_events_fed_back_into_an_engaged_gate_reclamp() {
    let mut app = material_clamp_test_app();
    let metal = app
        .world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .add(StandardMaterial {
            metallic: 1.0,
            ..default()
        });
    clamp_settings(&mut app).set_metallic_enabled(false);
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&metal)
            .unwrap()
            .metallic,
        0.0
    );

    // An external write sneaks metallic back in; the engaged gate re-clamps
    // it from the unchanged baseline on its Modified event.
    app.world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .get_mut(&metal)
        .unwrap()
        .metallic = 0.9;
    write_material_event(&mut app, AssetEvent::Modified { id: metal.id() });
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&metal)
            .unwrap()
            .metallic,
        0.0
    );

    clamp_settings(&mut app).set_metallic_enabled(true);
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&metal)
            .unwrap()
            .metallic,
        1.0,
        "the baseline must survive intermediate re-clamps"
    );
}

#[test]
fn reflection_probe_settings_preserve_strength_across_the_gate_and_catch_late_probes() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ReflectionProbeSettings>()
        .add_systems(Update, apply_reflection_probe_settings);
    let first = app
        .world_mut()
        .spawn((
            PreparedReflectionProbe,
            EnvironmentMapLight {
                intensity: 99.0,
                ..default()
            },
        ))
        .id();
    app.update();
    assert_eq!(
        app.world()
            .entity(first)
            .get::<EnvironmentMapLight>()
            .unwrap()
            .intensity,
        PREPARED_REFLECTION_PROBE_INTENSITY * 100.0
    );

    app.world_mut()
        .resource_mut::<ReflectionProbeSettings>()
        .set_strength(2.5);
    app.update();
    assert_eq!(
        app.world()
            .entity(first)
            .get::<EnvironmentMapLight>()
            .unwrap()
            .intensity,
        PREPARED_REFLECTION_PROBE_INTENSITY * 2.5
    );

    app.world_mut()
        .resource_mut::<ReflectionProbeSettings>()
        .set_enabled(false);
    app.update();
    assert_eq!(
        app.world()
            .entity(first)
            .get::<EnvironmentMapLight>()
            .unwrap()
            .intensity,
        0.0
    );
    let late = app
        .world_mut()
        .spawn((
            PreparedReflectionProbe,
            EnvironmentMapLight {
                intensity: 99.0,
                ..default()
            },
        ))
        .id();
    app.update();
    assert_eq!(
        app.world()
            .entity(late)
            .get::<EnvironmentMapLight>()
            .unwrap()
            .intensity,
        0.0
    );

    app.world_mut()
        .resource_mut::<ReflectionProbeSettings>()
        .set_enabled(true);
    app.update();
    assert_eq!(
        app.world()
            .entity(first)
            .get::<EnvironmentMapLight>()
            .unwrap()
            .intensity,
        PREPARED_REFLECTION_PROBE_INTENSITY * 2.5
    );
    assert_eq!(
        app.world()
            .entity(late)
            .get::<EnvironmentMapLight>()
            .unwrap()
            .intensity,
        PREPARED_REFLECTION_PROBE_INTENSITY * 2.5
    );
}

#[test]
fn emission_scale_reuses_baseline_without_scaling_exposure_alpha() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<AuthorizedEmissionMaterials>()
        .insert_resource(EmissionScale(0.0))
        .insert_resource(ImageSpaceEmissionMultiplier(1.0))
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

    app.world_mut()
        .insert_resource(ImageSpaceEmissionMultiplier(3.0));
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&material)
            .unwrap()
            .emissive
            .to_f32_array(),
        [0.6, 0.3, 0.15, 0.75]
    );
}

#[test]
fn emission_scale_ignores_materials_without_shader_authorization() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<AuthorizedEmissionMaterials>()
        .insert_resource(EmissionScale(1.0))
        .insert_resource(ImageSpaceEmissionMultiplier(1.0))
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
    assert!(app
        .world()
        .resource::<ButtonInput<Key>>()
        .pressed(Key::Super));
    assert!(app
        .world()
        .resource::<ButtonInput<Key>>()
        .pressed(Key::Shift));
    assert!(app
        .world()
        .resource::<ButtonInput<KeyCode>>()
        .pressed(KeyCode::SuperLeft));
    assert!(app
        .world()
        .resource::<ButtonInput<KeyCode>>()
        .pressed(KeyCode::ShiftLeft));

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

// ---------------------------------------------------------------------
// Issue #270 (PERF wave 1): scene classification is revision/event-driven.
// The AO and camera/probe gate tests below pin the semantics the
// event-driven implementations must preserve, including the remove+add
// count-coincidence blind spot of the old `AoScanState` sentinel.
// ---------------------------------------------------------------------

fn vertex_color_mesh(colors: &[[f32; 4]]) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD,
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(colors.to_vec()),
    );
    mesh
}

fn ao_placement(quick_ao: bool) -> crate::vsa::PreparedPlacement {
    crate::vsa::PreparedPlacement {
        reference_form_id: 0x0002_96D1,
        base_form_id: 1,
        asset_path: None,
        translation: [0.0; 3],
        rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
        scale: 1.0,
        error: None,
        physics_asset_path: None,
        physics_source: None,
        physics_classification: Default::default(),
        step_support: false,
        mutability: Default::default(),
        mutability_root_form_id: None,
        reference_kind: "REFR".into(),
        base_kind: "STAT".into(),
        editor_id: None,
        display_name: None,
        count: 1,
        semantic: Default::default(),
        initially_enabled: true,
        enable_parent: None,
        owner_form_id: None,
        owner_faction_rank: None,
        linked_reference_form_id: None,
        inventory: Vec::new(),
        audio: Default::default(),
        ao_mode: if quick_ao {
            "ao-quick-v1".into()
        } else {
            "ao-none".into()
        },
    }
}

fn spawn_quick_ao_mesh(app: &mut App, mesh: &Handle<Mesh>) -> Entity {
    let root = app
        .world_mut()
        .spawn(interaction::PlacementRoot::new(ao_placement(true)))
        .id();
    app.world_mut()
        .spawn((Transform::default(), Mesh3d(mesh.clone()), ChildOf(root)))
        .id()
}

fn spawn_plain_mesh(app: &mut App, mesh: &Handle<Mesh>) -> Entity {
    app.world_mut()
        .spawn((Transform::default(), Mesh3d(mesh.clone())))
        .id()
}

fn mesh_colors(app: &App, handle: &Handle<Mesh>) -> [f32; 4] {
    let meshes = app.world().resource::<Assets<Mesh>>();
    let colors = meshes
        .get(handle)
        .expect("test mesh asset")
        .attribute(Mesh::ATTRIBUTE_COLOR)
        .expect("test mesh colors");
    match colors {
        VertexAttributeValues::Float32x4(values) => values[0],
        other => panic!("expected float colors, got {other:?}"),
    }
}

fn assert_colors_near(actual: [f32; 4], expected: [f32; 4]) {
    for (actual, expected) in actual.iter().zip(expected) {
        assert!(
            (actual - expected).abs() < 1e-6,
            "color channel drifted: {actual} vs {expected}"
        );
    }
}

/// Change-detection probe placed after the AO systems: a frame with zero
/// hits on both resources is provably free of baseline and mesh-store
/// writes (Bevy change detection fires on deref-mut only).
#[derive(Resource, Default)]
struct AoWriteProbe {
    bases_writes: usize,
    mesh_store_writes: usize,
    eligibility_writes: usize,
}

fn probe_ao_writes(
    bases: Res<AoMeshBases>,
    meshes: Res<Assets<Mesh>>,
    eligibility: Res<AoEligibility>,
    mut probe: ResMut<AoWriteProbe>,
) {
    probe.bases_writes += usize::from(bases.is_changed());
    probe.mesh_store_writes += usize::from(meshes.is_changed());
    probe.eligibility_writes += usize::from(eligibility.is_changed());
}

fn ao_probe_frame(app: &mut App) -> (usize, usize, usize) {
    {
        let mut probe = app.world_mut().resource_mut::<AoWriteProbe>();
        probe.bases_writes = 0;
        probe.mesh_store_writes = 0;
        probe.eligibility_writes = 0;
    }
    app.update();
    let probe = app.world().resource::<AoWriteProbe>();
    (
        probe.bases_writes,
        probe.mesh_store_writes,
        probe.eligibility_writes,
    )
}

fn ao_test_app(strength: f32) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .insert_resource(AoStrength(strength))
        .init_resource::<AoMeshBases>()
        .init_resource::<AoEligibility>()
        .init_resource::<AoWriteProbe>()
        .init_resource::<Assets<Mesh>>()
        .add_message::<AssetEvent<Mesh>>()
        .add_systems(
            Update,
            (
                track_ao_mesh_eligibility,
                apply_ao_strength,
                probe_ao_writes,
            )
                .chain(),
        );
    app
}

#[test]
fn ao_remove_and_add_in_one_tick_still_scales_the_new_mesh() {
    let mut app = ao_test_app(0.5);
    let mesh_a = app
        .world_mut()
        .resource_mut::<Assets<Mesh>>()
        .add(vertex_color_mesh(&[[0.2, 0.4, 0.6, 0.8]]));
    let entity_a = spawn_quick_ao_mesh(&mut app, &mesh_a);
    app.update();
    assert_colors_near(mesh_colors(&app, &mesh_a), [0.6, 0.7, 0.8, 0.8]);

    // The #270 blind spot: one mesh entity despawns and its asset is dropped
    // while a new mesh entity and asset appear in the SAME tick -- entity
    // and asset counts are identical across the frame boundary, so the old
    // count sentinel never fired and the new mesh kept raw colors.
    app.world_mut().despawn(entity_a);
    app.world_mut()
        .resource_mut::<Assets<Mesh>>()
        .remove(mesh_a.id());
    app.world_mut()
        .resource_mut::<Messages<AssetEvent<Mesh>>>()
        .write(AssetEvent::Removed { id: mesh_a.id() });
    let mesh_b = app
        .world_mut()
        .resource_mut::<Assets<Mesh>>()
        .add(vertex_color_mesh(&[[0.4, 0.2, 0.0, 1.0]]));
    let _entity_b = spawn_quick_ao_mesh(&mut app, &mesh_b);
    app.update();

    assert_colors_near(mesh_colors(&app, &mesh_b), [0.7, 0.6, 0.5, 1.0]);
    let bases = &app.world().resource::<AoMeshBases>().values;
    assert!(
        !bases.contains_key(&mesh_a.id()),
        "the removed mesh's baseline must be dropped on AssetEvent::Removed"
    );
    assert_eq!(
        bases.len(),
        1,
        "only the newly discovered mesh keeps a baseline"
    );
}

#[test]
fn ao_strength_change_rescales_value_exactly_from_baselines() {
    let mut app = ao_test_app(0.5);
    let authored = [0.25, 0.5, 0.75, 0.33];
    let eligible = app
        .world_mut()
        .resource_mut::<Assets<Mesh>>()
        .add(vertex_color_mesh(&[authored]));
    spawn_quick_ao_mesh(&mut app, &eligible);
    let plain = app
        .world_mut()
        .resource_mut::<Assets<Mesh>>()
        .add(vertex_color_mesh(&[authored]));
    spawn_plain_mesh(&mut app, &plain);
    app.update();
    app.update();

    let half = |channel: f32| scale_ao_channel(channel, 0.5);
    assert_colors_near(
        mesh_colors(&app, &eligible),
        [half(0.25), half(0.5), half(0.75), 0.33],
    );
    // Ineligible meshes are never regenerated -- authored values intact.
    assert_eq!(mesh_colors(&app, &plain), authored);

    // Strength 0 lifts every baked-darkness channel exactly to 1.0 from
    // the baseline (alpha passthrough included); a transform compounded on
    // the half-strength colors could not produce these exact values.
    app.world_mut().resource_mut::<AoStrength>().0 = 0.0;
    app.update();
    assert_eq!(mesh_colors(&app, &eligible), [1.0, 1.0, 1.0, 0.33]);

    app.world_mut().resource_mut::<AoStrength>().0 = 0.9;
    app.update();
    let near = |channel: f32| scale_ao_channel(channel, 0.9);
    assert_colors_near(
        mesh_colors(&app, &eligible),
        [near(0.25), near(0.5), near(0.75), 0.33],
    );
    assert_eq!(mesh_colors(&app, &plain), authored);
}

#[test]
fn ao_baseline_is_dropped_when_its_mesh_asset_is_removed() {
    let mut app = ao_test_app(0.5);
    let mesh = app
        .world_mut()
        .resource_mut::<Assets<Mesh>>()
        .add(vertex_color_mesh(&[[0.2, 0.4, 0.6, 0.8]]));
    let _entity = spawn_quick_ao_mesh(&mut app, &mesh);
    app.update();
    assert_eq!(app.world().resource::<AoMeshBases>().values.len(), 1);

    app.world_mut()
        .resource_mut::<Assets<Mesh>>()
        .remove(mesh.id());
    app.world_mut()
        .resource_mut::<Messages<AssetEvent<Mesh>>>()
        .write(AssetEvent::Removed { id: mesh.id() });
    app.update();

    assert_eq!(app.world().resource::<AoMeshBases>().values.len(), 0);
}

#[test]
fn settled_ao_frames_perform_no_mesh_store_writes() {
    let mut app = ao_test_app(0.5);
    let mesh = app
        .world_mut()
        .resource_mut::<Assets<Mesh>>()
        .add(vertex_color_mesh(&[[0.2, 0.4, 0.6, 0.8]]));
    let _entity = spawn_quick_ao_mesh(&mut app, &mesh);
    // Absorb creation ticks; discovery/processing frames are outside the
    // steady-state guarantee.
    app.update();
    app.update();

    for frame in 0..3 {
        assert_eq!(
            ao_probe_frame(&mut app),
            (0, 0, 0),
            "settled frame {frame} touched meshes or baselines"
        );
    }
}

#[derive(Resource, Default)]
struct ProjectionWriteProbe {
    projection_writes: usize,
}

fn probe_projection_writes(
    projections: Query<Ref<Projection>, With<Camera3d>>,
    mut probe: ResMut<ProjectionWriteProbe>,
) {
    for projection in &projections {
        probe.projection_writes += usize::from(projection.is_changed());
    }
}

fn fov_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<ProjectionWriteProbe>()
        .add_systems(
            Update,
            (apply_horizontal_fov, probe_projection_writes).chain(),
        );
    app
}

fn spawn_fov_camera(app: &mut App, horizontal_degrees: f32) -> Entity {
    app.world_mut()
        .spawn((
            Camera3d::default(),
            HorizontalFov(horizontal_degrees),
            Projection::Perspective(default_perspective_projection()),
        ))
        .id()
}

fn camera_fov(app: &App, camera: Entity) -> f32 {
    let Projection::Perspective(perspective) = app.world().get::<Projection>(camera).unwrap()
    else {
        panic!("test camera uses a perspective projection");
    };
    perspective.fov
}

#[test]
fn horizontal_fov_tracks_marker_changes_and_writes_nothing_when_settled() {
    let mut app = fov_test_app();
    let camera = spawn_fov_camera(&mut app, 90.0);
    app.update();
    let aspect = 16.0 / 9.0;
    assert!(camera_fov(&app, camera) > 0.0);

    app.world_mut().get_mut::<HorizontalFov>(camera).unwrap().0 = 120.0;
    app.update();
    assert!(
        (camera_fov(&app, camera) - horizontal_to_vertical_fov(120.0, aspect)).abs() < 1e-4,
        "HorizontalFov changes must still drive the projection"
    );

    // A mutated projection (window resize bumps its change tick) must also
    // re-trigger the gate even though the marker is unchanged.
    {
        let mut projection = app.world_mut().get_mut::<Projection>(camera).unwrap();
        let Projection::Perspective(perspective) = &mut *projection else {
            unreachable!();
        };
        perspective.fov = 0.01;
    }
    app.update();
    assert!(
        (camera_fov(&app, camera) - horizontal_to_vertical_fov(120.0, aspect)).abs() < 1e-4,
        "projection changes must recompute the vertical FOV"
    );

    // Settled frames write nothing.
    {
        let mut probe = app.world_mut().resource_mut::<ProjectionWriteProbe>();
        probe.projection_writes = 0;
    }
    app.update();
    app.update();
    assert_eq!(
        app.world()
            .resource::<ProjectionWriteProbe>()
            .projection_writes,
        0
    );
}

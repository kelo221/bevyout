use super::*;
use bevy::app::{PreUpdate, Update};
use bevy::asset::Assets;
use bevy::color::LinearRgba;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::Messages;
use bevy::input::keyboard::{Key, KeyCode, KeyboardFocusLost, KeyboardInput};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::{ButtonInput, ButtonState, InputPlugin, InputSystems};
use bevy::light::EnvironmentMapLight;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{
    App, ColorToComponents, IntoScheduleConfigs, MinimalPlugins, MouseButton, Window, default,
};
use bevy::window::{PrimaryWindow, WindowFocused};

use crate::viewer::scene::{PREPARED_REFLECTION_PROBE_INTENSITY, PreparedReflectionProbe};

#[test]
fn metallic_gate_restores_baselines_and_catches_late_loaded_materials() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<MetallicGate>()
        .add_systems(Update, apply_metallic_gate);
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

    app.world_mut()
        .resource_mut::<MetallicGate>()
        .set_enabled(false);
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
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&late)
            .unwrap()
            .metallic,
        0.0
    );

    app.world_mut()
        .resource_mut::<MetallicGate>()
        .set_enabled(true);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&metal).unwrap().metallic, 1.0);
    assert_eq!(materials.get(&mixed).unwrap().metallic, 0.35);
    assert_eq!(materials.get(&late).unwrap().metallic, 0.8);
}

#[test]
fn dielectric_specular_gate_restores_baselines_and_catches_late_loaded_materials() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<DielectricSpecularGate>()
        .add_systems(Update, apply_dielectric_specular_gate);
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

    app.world_mut()
        .resource_mut::<DielectricSpecularGate>()
        .set_enabled(false);
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
    app.update();
    assert_eq!(
        app.world()
            .resource::<Assets<StandardMaterial>>()
            .get(&late)
            .unwrap()
            .reflectance,
        0.0
    );

    app.world_mut()
        .resource_mut::<DielectricSpecularGate>()
        .set_enabled(true);
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
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<Assets<StandardMaterial>>()
        .init_resource::<RoughnessScale>()
        .add_systems(Update, apply_roughness_scale);
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

    app.world_mut()
        .resource_mut::<RoughnessScale>()
        .set_scale(1.5);
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

    app.world_mut()
        .resource_mut::<RoughnessScale>()
        .set_scale(0.5);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&rough).unwrap().perceptual_roughness, 0.2);
    assert_eq!(materials.get(&saturated).unwrap().perceptual_roughness, 0.4);
    assert_eq!(materials.get(&late).unwrap().perceptual_roughness, 0.1);

    app.world_mut()
        .resource_mut::<RoughnessScale>()
        .set_scale(1.0);
    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    assert_eq!(materials.get(&rough).unwrap().perceptual_roughness, 0.4);
    assert_eq!(materials.get(&saturated).unwrap().perceptual_roughness, 0.8);
    assert_eq!(materials.get(&late).unwrap().perceptual_roughness, 0.2);
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

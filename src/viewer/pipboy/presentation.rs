//! Physical first-person Pip-Boy presentation.
//!
//! The menu remains ordinary Bevy UI, but targets an offscreen image which is
//! bound to the prepared model's dedicated screen mesh.

use std::f32::consts::PI;

use bevy::camera::{ClearColorConfig, RenderTarget};
use bevy::gltf::{GltfAssetLabel, GltfMeshName};
use bevy::math::Affine2;
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::app_state::{GameplayModal, RequestStateTransition};
use crate::vsa::PreparedItemCatalog;

use super::super::controls::HorizontalFov;
use super::super::interaction::InteractionNotice;
use super::super::player::{CameraMode, CameraModeState};
use super::super::{FlyCamera, WorldAssetRoot};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
// Keep the experimental physical presentation compiled and ready for the next
// iteration, while the known-good 2D Pip-Boy remains the default.
const PHYSICAL_PIPBOY_ENABLED: bool = false;
const PIPBOY_FOV_DEGREES: f32 = 47.0;
const RAISE_SECONDS: f32 = 0.45;
const LOWER_SECONDS: f32 = 0.30;
const SCREEN_UV_MIN: Vec2 = Vec2::new(-0.010_453_255, 0.001_969_397);
const SCREEN_UV_MAX: Vec2 = Vec2::new(0.752_925_75, 0.762_479_3);

// The prepared model's origin is the forearm attachment, not the display.
// Its screen centre resolves to approximately (+0.161, +0.058, -0.015) m
// after the model's own conversion transform, and its outward normal points
// along local +Y. The shown pose compensates that offset and turns +Y toward
// camera-space +Z so the display, rather than the wrist origin, is centred.
const HIDDEN_TRANSLATION: Vec3 = Vec3::new(-0.05, -0.92, -0.95);
const SHOWN_TRANSLATION: Vec3 = Vec3::new(-0.161, -0.015, -0.62);
const HIDDEN_ROTATION: Vec3 = Vec3::new(0.78, 0.10, -0.18);
const SHOWN_ROTATION: Vec3 = Vec3::new(PI * 0.5, 0.0, 0.0);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum PresentationPhase {
    #[default]
    Hidden,
    Raising,
    Interactive,
    Lowering,
}

#[derive(Resource, Default)]
pub(super) struct PipBoyPresentation {
    phase: PresentationPhase,
    elapsed: f32,
    saved_horizontal_fov: Option<f32>,
    root: Option<Entity>,
    ui_camera: Option<Entity>,
    screen_material: Option<Handle<StandardMaterial>>,
    physical_available: bool,
}

#[derive(Component)]
struct PipBoyPhysicalRoot;

#[derive(Component)]
struct PipBoyUiCamera;

#[derive(Component)]
struct PipBoyScreenMesh;

pub(super) struct PipBoyPresentationPlugin;

impl Plugin for PipBoyPresentationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PipBoyPresentation>()
            .add_message::<RequestStateTransition>()
            .add_systems(
                Update,
                (advance_presentation, bind_live_screen_material)
                    .chain()
                    .in_set(super::super::plugins::ViewerSet::Ui)
                    .run_if(in_state(GameplayModal::PipBoy)),
            )
            .add_systems(
                Update,
                project_pointer_to_screen
                    .in_set(super::super::plugins::ViewerSet::Input)
                    .run_if(in_state(GameplayModal::PipBoy))
                    .run_if(resource_exists::<Assets<Mesh>>),
            );
    }
}

impl PipBoyPresentation {
    pub(super) fn ui_camera(&self) -> Option<Entity> {
        self.ui_camera
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(super) fn begin_open(
    mut commands: Commands,
    camera_mode: Option<Res<CameraModeState>>,
    catalog: Res<PreparedItemCatalog>,
    asset_server: Res<AssetServer>,
    images: Option<ResMut<Assets<Image>>>,
    materials: Option<ResMut<Assets<StandardMaterial>>>,
    mut presentation: ResMut<PipBoyPresentation>,
    mut main_cameras: Query<(Entity, &mut HorizontalFov), (With<Camera3d>, With<FlyCamera>)>,
    mut notice: ResMut<InteractionNotice>,
    mut transitions: MessageWriter<RequestStateTransition>,
) {
    if !PHYSICAL_PIPBOY_ENABLED {
        presentation.phase = PresentationPhase::Interactive;
        presentation.elapsed = 0.0;
        presentation.physical_available = false;
        return;
    }

    if camera_mode
        .as_deref()
        .is_some_and(|state| state.mode != CameraMode::Fps)
    {
        notice.show("Pip-Boy requires FPS camera mode");
        transitions.write(RequestStateTransition::Modal(GameplayModal::None));
        return;
    }

    presentation.phase = PresentationPhase::Raising;
    presentation.elapsed = 0.0;

    let Ok((main_camera, mut horizontal_fov)) = main_cameras.single_mut() else {
        warn!("pipboy physical fallback: main FPS camera unavailable");
        presentation.physical_available = false;
        return;
    };
    presentation.saved_horizontal_fov = Some(horizontal_fov.0);
    horizontal_fov.0 = PIPBOY_FOV_DEGREES;

    let Some(asset_path) = physical_asset_path(&catalog) else {
        warn!("pipboy physical fallback: prepared Pip-Boy arm asset unavailable");
        presentation.physical_available = false;
        return;
    };
    let (Some(mut images), Some(mut materials)) = (images, materials) else {
        warn!("pipboy physical fallback: render asset stores unavailable");
        presentation.physical_available = false;
        return;
    };

    let image = Image::new_target_texture(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        TextureFormat::Bgra8UnormSrgb,
        None,
    );
    let screen_image = images.add(image);
    let ui_camera = commands
        .spawn((
            PipBoyUiCamera,
            Camera2d,
            Camera {
                order: -20,
                clear_color: ClearColorConfig::Custom(Color::srgb(0.002, 0.012, 0.006)),
                ..default()
            },
            RenderTarget::Image(screen_image.clone().into()),
        ))
        .id();

    let screen_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        base_color_texture: Some(screen_image.clone()),
        emissive: LinearRgba::new(0.015, 0.22, 0.05, 1.0),
        emissive_texture: Some(screen_image),
        uv_transform: screen_uv_transform(),
        unlit: true,
        ..default()
    });

    let root = commands
        .spawn((
            PipBoyPhysicalRoot,
            WorldAssetRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(asset_path.to_owned())),
            ),
            pose_transform(0.0),
            ChildOf(main_camera),
        ))
        .id();

    presentation.root = Some(root);
    presentation.ui_camera = Some(ui_camera);
    presentation.screen_material = Some(screen_material);
    presentation.physical_available = true;
}

pub(super) fn finish_close(
    mut commands: Commands,
    mut presentation: ResMut<PipBoyPresentation>,
    mut cameras: Query<&mut HorizontalFov, (With<Camera3d>, With<FlyCamera>)>,
) {
    if let Some(saved) = presentation.saved_horizontal_fov.take()
        && let Ok(mut fov) = cameras.single_mut()
    {
        fov.0 = saved;
    }
    for entity in [presentation.root.take(), presentation.ui_camera.take()]
        .into_iter()
        .flatten()
    {
        commands.entity(entity).despawn();
    }
    presentation.screen_material = None;
    presentation.phase = PresentationPhase::Hidden;
    presentation.elapsed = 0.0;
    presentation.physical_available = false;
}

fn advance_presentation(
    time: Res<Time<Real>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut presentation: ResMut<PipBoyPresentation>,
    mut roots: Query<&mut Transform, With<PipBoyPhysicalRoot>>,
    mut transitions: MessageWriter<RequestStateTransition>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if !PHYSICAL_PIPBOY_ENABLED {
        return;
    }

    if keys.just_pressed(KeyCode::Tab) {
        let progress = phase_progress(presentation.phase, presentation.elapsed);
        match presentation.phase {
            PresentationPhase::Raising | PresentationPhase::Interactive => {
                presentation.phase = PresentationPhase::Lowering;
                presentation.elapsed = (1.0 - progress) * LOWER_SECONDS;
            }
            PresentationPhase::Lowering => {
                presentation.phase = PresentationPhase::Raising;
                presentation.elapsed = progress * RAISE_SECONDS;
            }
            PresentationPhase::Hidden => {}
        }
    }

    presentation.elapsed += time.delta_secs();
    let progress = phase_progress(presentation.phase, presentation.elapsed);
    if let Some(root) = presentation.root
        && let Ok(mut transform) = roots.get_mut(root)
    {
        *transform = pose_transform(smoothstep(progress));
    }
    if presentation.phase == PresentationPhase::Raising && presentation.elapsed >= RAISE_SECONDS {
        presentation.phase = PresentationPhase::Interactive;
        presentation.elapsed = 0.0;
    } else if presentation.phase == PresentationPhase::Lowering
        && presentation.elapsed >= LOWER_SECONDS
    {
        transitions.write(RequestStateTransition::Modal(GameplayModal::None));
    }

    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = presentation.phase != PresentationPhase::Hidden;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn physical_asset_path(catalog: &PreparedItemCatalog) -> Option<&str> {
    catalog.items.iter().find_map(|item| {
        let is_pipboy = item
            .editor_id
            .as_deref()
            .is_some_and(|id| id.eq_ignore_ascii_case("PipBoy"))
            || item.source_model_path.as_deref().is_some_and(|path| {
                path.replace('\\', "/")
                    .eq_ignore_ascii_case("PipBoy3000/PipBoyArm.NIF")
            });
        is_pipboy
            .then_some(item.world_asset_path.as_deref())
            .flatten()
    })
}

fn bind_live_screen_material(
    mut commands: Commands,
    presentation: Res<PipBoyPresentation>,
    names: Query<(Entity, &GltfMeshName), Added<GltfMeshName>>,
    parents: Query<&ChildOf>,
    roots: Query<Entity, With<PipBoyPhysicalRoot>>,
) {
    let Some(material) = presentation.screen_material.as_ref() else {
        return;
    };
    for (entity, name) in &names {
        if !has_ancestor(entity, &roots, &parents) {
            continue;
        }
        if name.0.eq_ignore_ascii_case("pipboyscreen:0") {
            commands
                .entity(entity)
                .insert((MeshMaterial3d(material.clone()), PipBoyScreenMesh));
        } else if name.0.eq_ignore_ascii_case("ScreenLit:8")
            || name.0.eq_ignore_ascii_case("PipboyLightEffect:0")
        {
            // These are duplicate full-screen emissive shells from the
            // source material stack. Rendering them over the live surface
            // causes both overexposure and coplanar flicker.
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn project_pointer_to_screen(
    presentation: Res<PipBoyPresentation>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), (With<Camera3d>, With<FlyCamera>)>,
    screens: Query<Entity, With<PipBoyScreenMesh>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut raycast: MeshRayCast,
    mut interactables: Query<(&ComputedNode, &UiGlobalTransform, &mut Interaction), With<Button>>,
) {
    // The normal 2D Pip-Boy owns pointer interaction while the experimental
    // physical presentation is disabled. Do not clear its button states when
    // there is no screen mesh to raycast against.
    if !PHYSICAL_PIPBOY_ENABLED {
        return;
    }

    let target_point = if presentation.phase == PresentationPhase::Interactive {
        windows
            .single()
            .ok()
            .and_then(Window::cursor_position)
            .and_then(|cursor| {
                cameras.single().ok().and_then(|(camera, transform)| {
                    camera.viewport_to_world(transform, cursor).ok()
                })
            })
            .and_then(|ray| {
                let settings = MeshRayCastSettings::default().never_early_exit();
                raycast
                    .cast_ray(ray, &settings)
                    .iter()
                    .find_map(|(entity, hit)| screens.contains(*entity).then_some(hit.uv).flatten())
            })
            .map(render_target_point)
    } else {
        None
    };

    for (node, transform, mut interaction) in &mut interactables {
        let hovered = target_point.is_some_and(|point| node.contains_point(*transform, point));
        let next = if hovered && mouse.pressed(MouseButton::Left) {
            Interaction::Pressed
        } else if hovered {
            Interaction::Hovered
        } else {
            Interaction::None
        };
        if *interaction != next {
            *interaction = next;
        }
    }
}

fn render_target_point(uv: Vec2) -> Vec2 {
    let normalized = (uv - SCREEN_UV_MIN) / (SCREEN_UV_MAX - SCREEN_UV_MIN);
    Vec2::new(
        normalized.x.clamp(0.0, 1.0) * SCREEN_WIDTH as f32,
        normalized.y.clamp(0.0, 1.0) * SCREEN_HEIGHT as f32,
    )
}

fn screen_uv_transform() -> Affine2 {
    let scale = Vec2::ONE / (SCREEN_UV_MAX - SCREEN_UV_MIN);
    Affine2::from_scale_angle_translation(scale, 0.0, -SCREEN_UV_MIN * scale)
}

fn has_ancestor(
    mut entity: Entity,
    roots: &Query<Entity, With<PipBoyPhysicalRoot>>,
    parents: &Query<&ChildOf>,
) -> bool {
    loop {
        if roots.get(entity).is_ok() {
            return true;
        }
        let Ok(parent) = parents.get(entity) else {
            return false;
        };
        entity = parent.parent();
    }
}

fn pose_transform(progress: f32) -> Transform {
    Transform::from_translation(HIDDEN_TRANSLATION.lerp(SHOWN_TRANSLATION, progress)).with_rotation(
        Quat::from_euler(
            EulerRot::XYZ,
            HIDDEN_ROTATION.x + (SHOWN_ROTATION.x - HIDDEN_ROTATION.x) * progress,
            HIDDEN_ROTATION.y + (SHOWN_ROTATION.y - HIDDEN_ROTATION.y) * progress,
            HIDDEN_ROTATION.z + (SHOWN_ROTATION.z - HIDDEN_ROTATION.z) * progress,
        ),
    )
}

fn phase_progress(phase: PresentationPhase, elapsed: f32) -> f32 {
    match phase {
        PresentationPhase::Hidden => 0.0,
        PresentationPhase::Raising => (elapsed / RAISE_SECONDS).clamp(0.0, 1.0),
        PresentationPhase::Interactive => 1.0,
        PresentationPhase::Lowering => 1.0 - (elapsed / LOWER_SECONDS).clamp(0.0, 1.0),
    }
}

fn smoothstep(value: f32) -> f32 {
    value * value * (3.0 - 2.0 * value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pose_endpoints_are_exact() {
        assert_eq!(pose_transform(0.0).translation, HIDDEN_TRANSLATION);
        assert_eq!(pose_transform(1.0).translation, SHOWN_TRANSLATION);
    }

    #[test]
    fn smoothstep_is_bounded_and_hits_endpoints() {
        assert_eq!(smoothstep(0.0), 0.0);
        assert_eq!(smoothstep(1.0), 1.0);
        assert!((smoothstep(0.5) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn screen_uv_maps_to_render_target_edges() {
        assert_eq!(render_target_point(SCREEN_UV_MIN), Vec2::ZERO);
        assert_eq!(
            render_target_point(SCREEN_UV_MAX),
            Vec2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32)
        );
        let quarter = SCREEN_UV_MIN + (SCREEN_UV_MAX - SCREEN_UV_MIN) * Vec2::new(0.25, 0.75);
        assert_eq!(render_target_point(quarter), Vec2::new(256.0, 576.0));
    }

    #[test]
    fn phase_progress_is_continuous_in_both_directions() {
        let raising = phase_progress(PresentationPhase::Raising, RAISE_SECONDS * 0.4);
        let lowering = phase_progress(PresentationPhase::Lowering, LOWER_SECONDS * 0.6);
        assert!((raising - 0.4).abs() < f32::EPSILON);
        assert!((lowering - 0.4).abs() < f32::EPSILON);
    }
}

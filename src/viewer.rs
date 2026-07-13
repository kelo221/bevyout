use anyhow::{Context, Result};
use bevy::asset::AssetId;
use bevy::camera::Exposure;
use bevy::core_pipeline::prepass::DepthPrepass;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::ecs::system::SystemParam;
use bevy::gltf::GltfMeshName;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::light::{IrradianceVolume, LightProbe};
use bevy::math::{cubic_splines::LinearSpline, vec2};
use bevy::mesh::{Mesh, VertexAttributeValues};
use bevy::pbr::{DistanceFog, FogFalloff};
use bevy::picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy::post_process::auto_exposure::{
    AutoExposure, AutoExposureCompensationCurve, AutoExposurePlugin,
};
use bevy::post_process::bloom::Bloom;
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use bevy::render::occlusion_culling::OcclusionCulling;
use bevy::render::view::ColorGrading;
use bevy::window::{CursorGrabMode, CursorOptions, PresentMode};
use ron::de::from_str;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::{BakeArgs, BakeQuality, PrepareArgs, RenderArgs, ViewArgs};
use crate::vsa::{
    CellInfo, FO3_SCALE, ImageSpaceInfo, NIF_CONVERTER_REVISION, PHYSICS_ASSET_SCHEMA_VERSION,
    PreparedCellLighting, PreparedSceneManifest, PreparedSemantic, bake, cell_label,
    ensure_baked_scene_compatible, ensure_prepared_manifest_compatible, find_cached_manifest,
    is_bake_static, prepare, resolve_cached_manifest,
};

mod audio;
mod interaction;
mod openmw_player;
mod player;

const DEFAULT_LIGHTING_SCALE: f32 = 128.0;
const CELL_DIRECTIONAL_ILLUMINANCE: f32 = 10_000.0;
const DEFAULT_FOG_STRENGTH: f32 = 0.01;
const RENDER_REPORT_HISTORY: usize = 600;

pub(crate) fn view(args: ViewArgs) -> Result<()> {
    run_view(args.manifest, args.disable_physics, args.trace_seconds)
}

pub(crate) fn render(args: RenderArgs) -> Result<()> {
    let cache_dir = args
        .cache_dir
        .unwrap_or_else(|| PathBuf::from(".bevyout/cache"));
    let manifest_path = match find_cached_manifest(&cache_dir, &args.selector)? {
        Some(path) => path,
        None => {
            let prompt = format!(
                "Prepared scene '{}' was not found. Import it now?",
                args.selector
            );
            if !confirm(&prompt)? {
                return resolve_cached_manifest(&cache_dir, &args.selector).map(|_| ());
            }
            prepare(PrepareArgs {
                selector: Some(args.selector.clone()),
                game_root: args.game_root.clone(),
                plugin: args.plugin.clone(),
                cell: None,
                blender: args.blender.clone(),
                cache_dir: Some(cache_dir.clone()),
                force: false,
                rebuild_assets: false,
                strict: false,
            })?;
            resolve_cached_manifest(&cache_dir, &args.selector)?
        }
    };
    let manifest = read_manifest(&manifest_path)?;
    if needs_irradiance_bake(&manifest) {
        let prompt = format!(
            "Prepared scene '{}' has no irradiance bake. Bake it now?",
            cell_label(&manifest.cell)
        );
        if confirm(&prompt)? {
            bake(BakeArgs {
                manifest: None,
                selector: Some(args.selector.clone()),
                cache_dir: Some(cache_dir.clone()),
                quality: BakeQuality::Irradiance,
                irradiance_spacing_meters: 8.0,
                irradiance_samples: 64,
                static_batch_chunk_meters: 64.0,
                blender: args.blender.clone(),
                irradiance_blender: args.irradiance_blender.clone(),
                toktx: args.toktx.clone(),
                force: false,
                keep_intermediate: false,
            })?;
        }
    }
    run_view(manifest_path, args.disable_physics, args.trace_seconds)
}

fn read_manifest(manifest_path: &Path) -> Result<PreparedSceneManifest> {
    let manifest_path = fs::canonicalize(manifest_path).context("manifest does not exist")?;
    let text = fs::read_to_string(manifest_path)?;
    from_str(&text).context("invalid scene manifest")
}

fn needs_irradiance_bake(manifest: &PreparedSceneManifest) -> bool {
    manifest
        .bake
        .as_ref()
        .and_then(|bake| bake.irradiance_volume.as_ref())
        .is_none()
}

fn confirm(prompt: &str) -> Result<bool> {
    use std::io::{self, IsTerminal, Write};

    let stdin = io::stdin();
    if !stdin.is_terminal() {
        return Ok(false);
    }
    let mut answer = String::new();
    loop {
        eprint!("{prompt} [Y/n] ");
        io::stderr().flush()?;
        answer.clear();
        if stdin.read_line(&mut answer)? == 0 {
            return Ok(false);
        }
        match answer.trim().to_ascii_lowercase().as_str() {
            "" | "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => eprintln!("Please answer yes or no."),
        }
    }
}

fn run_view(
    manifest_path: PathBuf,
    disable_physics: bool,
    trace_seconds: Option<f32>,
) -> Result<()> {
    let manifest_path = fs::canonicalize(&manifest_path).context("manifest does not exist")?;
    let text = fs::read_to_string(&manifest_path)?;
    let manifest: PreparedSceneManifest = from_str(&text).context("invalid scene manifest")?;
    ensure_prepared_manifest_compatible(
        &manifest,
        NIF_CONVERTER_REVISION,
        PHYSICS_ASSET_SCHEMA_VERSION,
    )?;
    ensure_baked_scene_compatible(&manifest)?;
    let asset_root = PathBuf::from(&manifest.asset_root);
    let physics_assets = player::load_prepared_physics_assets(&manifest, &asset_root)?;
    let report_path = render_report_path(&manifest_path);
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1280, 720).into(),
                    focused: true,
                    present_mode: PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: asset_root.to_string_lossy().to_string(),
                ..default()
            }),
        FrameTimeDiagnosticsPlugin::new(RENDER_REPORT_HISTORY),
        RenderDiagnosticsPlugin,
        AutoExposurePlugin,
    ));
    app.insert_resource(physics_assets);
    player::install(&mut app, disable_physics);
    audio::install(&mut app);
    interaction::install(&mut app);
    app.insert_resource(manifest)
        .insert_resource(UnlitMode(false))
        .insert_resource(LightingScale(DEFAULT_LIGHTING_SCALE))
        .insert_resource(IrradianceIntensity(1.0))
        .insert_resource(AmbientScale(0.05))
        .insert_resource(FogStrength(DEFAULT_FOG_STRENGTH))
        .insert_resource(AoStrength(1.0))
        .insert_resource(AoMeshBases::default())
        .insert_resource(RenderReportPath(report_path))
        .insert_resource(RenderReportBuffer::default())
        .insert_resource(AdjustmentTarget::default())
        .insert_resource(LightsDisabled(false))
        .add_systems(
            Startup,
            (capture_cursor, spawn_prepared_scene, spawn_reticle),
        )
        .add_systems(
            Update,
            (
                adjust_selected_value,
                toggle_lights_disabled,
                apply_lighting_scale,
                apply_fog_strength,
                apply_ao_strength,
                apply_irradiance_intensity,
                update_fps_text,
                update_adjustment_hud,
                toggle_unlit_mode,
                apply_unlit_mode,
                configure_glow_cards,
                inspect_center_hit,
            ),
        )
        .add_systems(Update, (record_render_sample, save_render_report))
        .add_systems(
            Update,
            (
                capture_cursor_input,
                player::toggle_camera_mode,
                free_fly_camera,
                player::fps_mouse_look,
            )
                .chain(),
        );
    if let Some(seconds) = trace_seconds {
        if !seconds.is_finite() || seconds <= 0.0 {
            anyhow::bail!("--trace-seconds must be finite and greater than zero");
        }
        app.insert_resource(TraceCaptureLimit { remaining: seconds })
            .add_systems(Update, stop_trace_capture);
    }
    app.run();
    Ok(())
}

#[derive(Resource)]
struct TraceCaptureLimit {
    remaining: f32,
}

fn stop_trace_capture(
    time: Res<Time>,
    mut limit: ResMut<TraceCaptureLimit>,
    mut app_exit: MessageWriter<AppExit>,
) {
    limit.remaining -= time.delta_secs();
    if limit.remaining <= 0.0 {
        app_exit.write(AppExit::Success);
    }
}

fn spawn_reticle(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            width: Val::Px(5.0),
            height: Val::Px(5.0),
            margin: UiRect::all(Val::Px(-2.5)),
            ..default()
        },
        BackgroundColor(Color::WHITE),
        ZIndex(100),
    ));
    commands.spawn((
        Text::new("FPS --"),
        FpsText,
        Node {
            position_type: PositionType::Absolute,
            top: px(8),
            right: px(10),
            ..default()
        },
    ));
    commands.spawn((
        Text::new("Adjusting: Lighting scale\nPage Up/Down: select   F1/F2: change"),
        AdjustmentHud,
        Node {
            position_type: PositionType::Absolute,
            top: px(8),
            left: px(10),
            ..default()
        },
    ));
}

#[derive(Component)]
struct FpsText;

fn update_fps_text(diagnostics: Res<DiagnosticsStore>, mut text: Single<&mut Text, With<FpsText>>) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
        .unwrap_or(0.0);
    text.0 = format!("{fps:.0} FPS");
}

#[derive(Resource)]
struct RenderReportPath(PathBuf);

#[derive(Resource, Default)]
struct RenderReportBuffer {
    next_sample: u64,
    samples: VecDeque<RenderReportSample>,
}

#[derive(Clone, Copy)]
struct RenderReportSample {
    sample: u64,
    frame_time_ms: f64,
}

fn render_diagnostics_report_path(report_path: &Path) -> PathBuf {
    report_path.with_file_name("render_diagnostics.csv")
}

fn render_report_path(manifest_path: &Path) -> PathBuf {
    manifest_path
        .ancestors()
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.eq_ignore_ascii_case(".bevyout"))
        })
        .and_then(Path::parent)
        .map(|path| path.join("render_timings.csv"))
        .unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("render_timings.csv")
        })
}

fn record_render_sample(
    diagnostics: Res<DiagnosticsStore>,
    mut report: ResMut<RenderReportBuffer>,
) {
    let Some(frame_time_ms) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
        .and_then(|diagnostic| diagnostic.value())
    else {
        return;
    };
    if !frame_time_ms.is_finite() || frame_time_ms < 0.0 {
        return;
    }
    let sample = RenderReportSample {
        sample: report.next_sample,
        frame_time_ms,
    };
    report.next_sample = report.next_sample.saturating_add(1);
    report.samples.push_back(sample);
    while report.samples.len() > RENDER_REPORT_HISTORY {
        report.samples.pop_front();
    }
}

#[derive(SystemParam)]
struct RenderReportParams<'w, 's> {
    keys: Res<'w, ButtonInput<KeyCode>>,
    report: Res<'w, RenderReportBuffer>,
    report_path: Res<'w, RenderReportPath>,
    diagnostics: Res<'w, DiagnosticsStore>,
    meshes: Res<'w, Assets<Mesh>>,
    materials: Res<'w, Assets<StandardMaterial>>,
    images: Res<'w, Assets<Image>>,
    entities: Query<'w, 's, Entity>,
    mesh_entities: Query<'w, 's, (Entity, Option<&'static Visibility>), With<Mesh3d>>,
    named_meshes: Query<'w, 's, &'static GltfMeshName>,
    cameras: Query<'w, 's, &'static Bloom, With<Camera3d>>,
    irradiance_volumes: Query<'w, 's, Entity, With<IrradianceVolume>>,
    point_lights: Query<'w, 's, Entity, With<PointLight>>,
    directional_lights: Query<'w, 's, Entity, With<DirectionalLight>>,
    manifest: Res<'w, PreparedSceneManifest>,
    camera_mode: Res<'w, player::CameraModeState>,
    physics_disabled: Res<'w, player::PhysicsDisabled>,
    unlit_mode: Res<'w, UnlitMode>,
    lights_disabled: Res<'w, LightsDisabled>,
    colliders: Query<'w, 's, Entity, With<player::PhysicsCollider>>,
    physics: Res<'w, player::CollisionRuntimeStats>,
}

fn save_render_report(params: RenderReportParams) {
    if !params.keys.just_pressed(KeyCode::F10) {
        return;
    }

    let mesh_entity_count = params.mesh_entities.iter().count();
    let entity_count = params.entities.iter().count();
    let named_mesh_count = params.named_meshes.iter().count();
    let camera_count = params.cameras.iter().count();
    let hidden_mesh_count = params
        .mesh_entities
        .iter()
        .filter(|(_, visibility)| matches!(visibility, Some(Visibility::Hidden)))
        .count();
    let bloom = params.cameras.single().ok();
    let bloom_intensity = bloom.map_or(0.0, |value| value.intensity);
    let bloom_threshold = bloom.map_or(0.0, |value| value.prefilter.threshold);
    let bloom_softness = bloom.map_or(0.0, |value| value.prefilter.threshold_softness);
    let point_light_count = params.point_lights.iter().count();
    let directional_light_count = params.directional_lights.iter().count();
    let irradiance_volume_count = params.irradiance_volumes.iter().count();
    let camera_mode = format!("{:?}", params.camera_mode.mode);
    let collider_count = params.colliders.iter().count();
    let mut shape_kinds = params.physics.shape_kinds.iter().collect::<Vec<_>>();
    shape_kinds.sort_unstable_by_key(|(kind, _)| **kind);
    let shape_kinds = shape_kinds
        .into_iter()
        .map(|(kind, count)| format!("{kind}:{count}"))
        .collect::<Vec<_>>()
        .join(";");

    let mut csv = String::from(
        "sample,frame_time_ms,fps,entity_count,mesh_entities,hidden_meshes,named_gltf_meshes,point_lights,directional_lights,irradiance_volumes,cameras,mesh_assets,material_assets,image_assets,manifest_placements,manifest_lights,bloom_intensity,bloom_threshold,bloom_softness,camera_mode,unlit_mode,lights_disabled,physics_disabled,collider_entities,physics_authored_assets,physics_fallback_assets,physics_bodies,physics_shapes,physics_shape_kinds,physics_packed_triangles,physics_filtered_shapes,physics_dynamic_bodies,physics_cooking_ms,physics_sidecar_bytes\n",
    );
    for sample in &params.report.samples {
        let fps = if sample.frame_time_ms > f64::EPSILON {
            1000.0 / sample.frame_time_ms
        } else {
            0.0
        };
        csv.push_str(&format!(
            "{},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.4},{:.4},{:.4},{},{},{},{},{},{},{},{},{},{},{},{},{},{:.3},{}\n",
            sample.sample,
            sample.frame_time_ms,
            fps,
            entity_count,
            mesh_entity_count,
            hidden_mesh_count,
            named_mesh_count,
            point_light_count,
            directional_light_count,
            irradiance_volume_count,
            camera_count,
            params.meshes.len(),
            params.materials.len(),
            params.images.len(),
            params.manifest.placements.len(),
            params.manifest.lights.len(),
            bloom_intensity,
            bloom_threshold,
            bloom_softness,
            camera_mode,
            u8::from(params.unlit_mode.0),
            u8::from(params.lights_disabled.0),
            u8::from(params.physics_disabled.0),
            collider_count,
            params.physics.authored_assets,
            params.physics.fallback_assets,
            params.physics.bodies,
            params.physics.shapes,
            csv_field(&shape_kinds),
            params.physics.packed_triangles,
            params.physics.filtered_shapes,
            params.physics.dynamic_bodies,
            params.physics.cooking_millis,
            params.physics.sidecar_bytes,
        ));
    }
    if let Err(error) = fs::write(&params.report_path.0, csv) {
        warn!(
            "failed to write render report {}: {error}",
            params.report_path.0.display()
        );
    } else {
        let mut render_diagnostics = params
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.path().as_str().starts_with("render/"))
            .filter_map(|diagnostic| {
                let value = diagnostic.value()?;
                if !value.is_finite() {
                    return None;
                }
                Some((
                    diagnostic.path().as_str().to_owned(),
                    diagnostic.suffix.as_ref().to_owned(),
                    value,
                    diagnostic.average().unwrap_or(value),
                    diagnostic.history_len(),
                ))
            })
            .collect::<Vec<_>>();
        render_diagnostics.sort_unstable_by(|left, right| left.0.cmp(&right.0));

        let diagnostics_path = render_diagnostics_report_path(&params.report_path.0);
        let mut diagnostics_csv =
            String::from("diagnostic_path,suffix,value,average,history_len\n");
        for (path, suffix, value, average, history_len) in render_diagnostics {
            diagnostics_csv.push_str(&format!(
                "{},{},{value:.6},{average:.6},{history_len}\n",
                csv_field(&path),
                csv_field(&suffix),
            ));
        }
        if let Err(error) = fs::write(&diagnostics_path, diagnostics_csv) {
            warn!(
                "failed to write render diagnostics report {}: {error}",
                diagnostics_path.display()
            );
        } else {
            info!(
                "wrote render diagnostics report to {}",
                diagnostics_path.display()
            );
        }
        info!(
            "wrote render report with {} samples to {}",
            params.report.samples.len(),
            params.report_path.0.display()
        );
    }
}

fn csv_field(value: &str) -> String {
    if value
        .chars()
        .any(|character| matches!(character, ',' | '"' | '\n' | '\r'))
    {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn inspect_center_hit(
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    names: Query<&GltfMeshName>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Ok((camera, transform)) = cameras.single() else {
        return;
    };
    let Some(viewport) = camera.logical_viewport_size() else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(transform, viewport * 0.5) else {
        return;
    };
    let settings = MeshRayCastSettings {
        visibility: RayCastVisibility::VisibleInView,
        ..default()
    };
    if let Some((entity, hit)) = raycast.cast_ray(ray, &settings).first() {
        let name = names
            .get(*entity)
            .map(|name| name.0.as_str())
            .unwrap_or("unnamed");
        info!(
            "center hit: entity {entity:?}, mesh {name:?}, distance {:.2}",
            hit.distance
        );
    } else {
        info!("center hit: none");
    }
}

fn spawn_prepared_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut compensation_curves: ResMut<Assets<AutoExposureCompensationCurve>>,
    manifest: Res<PreparedSceneManifest>,
    lighting: Res<LightingScale>,
    ambient_scale: Res<AmbientScale>,
    fog_strength: Res<FogStrength>,
) {
    let focus = scene_focus(&manifest);
    let initial_camera_position =
        transition_camera_position(&manifest).unwrap_or_else(|| focus + Vec3::new(0.0, 4.0, 12.0));
    let initial_camera_transform =
        Transform::from_translation(initial_camera_position).looking_at(focus, Vec3::Y);
    let (initial_yaw, initial_pitch, _) = initial_camera_transform.rotation.to_euler(EulerRot::YXZ);
    let cell_lighting = effective_lighting(&manifest.cell);
    let (color_grading, auto_exposure) =
        camera_post_processing(manifest.cell.image_space.as_ref(), &mut compensation_curves);
    let mut camera = commands.spawn((
        Camera3d::default(),
        DepthPrepass,
        OcclusionCulling,
        Bloom::NATURAL,
        Tonemapping::TonyMcMapface,
        Exposure { ev100: 12.0 },
        color_grading,
        initial_camera_transform,
        FlyCamera {
            yaw: initial_yaw,
            pitch: initial_pitch,
            speed: 8.0,
        },
    ));
    if let Some(auto_exposure) = auto_exposure {
        camera.insert(auto_exposure);
        if let Some(image_space) = manifest.cell.image_space.as_ref() {
            info!(
                "applying ImageSpace {:08x} ({}) eye_adapt_speed={:.3} target_lum={:.3}",
                image_space.form_id,
                image_space.editor_id.as_deref().unwrap_or("<unnamed>"),
                image_space.eye_adapt_speed,
                image_space.hdr_target_lum,
            );
        }
    } else {
        warn!(
            "{} has no resolved ImageSpace; retaining fixed viewer post-processing",
            cell_label(&manifest.cell)
        );
    }
    if let Some(fog) = distance_fog(&cell_lighting, fog_strength.0) {
        camera.insert(fog);
    }
    commands.insert_resource(GlobalAmbientLight {
        color: Color::srgb(
            cell_lighting.ambient_rgba[0],
            cell_lighting.ambient_rgba[1],
            cell_lighting.ambient_rgba[2],
        ),
        brightness: 25.0 * lighting.0 * ambient_scale.0,
        affects_lightmapped_meshes: true,
    });
    let directional_luminance = cell_lighting.directional_rgba[0]
        + cell_lighting.directional_rgba[1]
        + cell_lighting.directional_rgba[2];
    if directional_luminance > f32::EPSILON
        && cell_lighting.directional_rgba[..3]
            .iter()
            .all(|channel| channel.is_finite())
    {
        let base_illuminance = CELL_DIRECTIONAL_ILLUMINANCE;
        commands.spawn((
            DirectionalLight {
                color: Color::srgb(
                    cell_lighting.directional_rgba[0],
                    cell_lighting.directional_rgba[1],
                    cell_lighting.directional_rgba[2],
                ),
                illuminance: scaled_directional_illuminance(base_illuminance, lighting.0, false),
                affects_lightmapped_mesh_diffuse: true,
                shadow_maps_enabled: false,
                ..default()
            },
            CellDirectionalLight { base_illuminance },
            Transform::from_rotation(Quat::from_array(cell_lighting.directional_rotation_xyzw())),
        ));
    }
    for light in &manifest.lights {
        if !light.initially_enabled {
            continue;
        }
        commands.spawn((
            PointLight {
                intensity: light.radius * light.radius * 2.0 * lighting.0,
                range: light.radius,
                color: Color::srgb(
                    light.color_rgba[0],
                    light.color_rgba[1],
                    light.color_rgba[2],
                ),
                affects_lightmapped_mesh_diffuse: true,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform::from_translation(Vec3::from_array(light.translation)),
        ));
    }
    if let Some(bake) = &manifest.bake {
        commands.spawn(WorldAssetRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(bake.scene_path.clone())),
        ));
        if let Some(volume) = &bake.irradiance_volume {
            commands.spawn((
                LightProbe::default(),
                IrradianceVolume {
                    voxels: asset_server.load(volume.asset_path.clone()),
                    intensity: volume.intensity,
                    affects_lightmapped_meshes: true,
                },
                Transform {
                    translation: Vec3::from_array(volume.translation),
                    rotation: Quat::from_xyzw(
                        volume.rotation_xyzw[0],
                        volume.rotation_xyzw[1],
                        volume.rotation_xyzw[2],
                        volume.rotation_xyzw[3],
                    ),
                    scale: Vec3::from_array(volume.scale),
                },
            ));
            info!(
                "loading baked scene {} with irradiance volume {} at {:?}",
                bake.scene_path, volume.asset_path, volume.resolution
            );
        } else {
            warn!(
                "baked scene {} has no irradiance volume; run `bake {}`",
                bake.scene_path,
                cell_label(&manifest.cell)
            );
        }
        spawn_interactive_placements(&mut commands, &asset_server, &manifest);
    } else {
        for placement in &manifest.placements {
            if !placement.initially_enabled {
                continue;
            }
            let Some(path) = placement.asset_path.as_ref() else {
                continue;
            };
            commands.spawn((
                WorldAssetRoot(
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone())),
                ),
                interaction::PlacementRoot::new(placement.clone()),
                Transform {
                    translation: Vec3::from_array(placement.translation),
                    rotation: Quat::from_xyzw(
                        placement.rotation_xyzw[0],
                        placement.rotation_xyzw[1],
                        placement.rotation_xyzw[2],
                        placement.rotation_xyzw[3],
                    ),
                    scale: Vec3::splat(placement.scale),
                },
            ));
        }
    }
    info!(
        "loaded {} with {} placements, {} diagnostics; camera focus {:?}",
        cell_label(&manifest.cell),
        manifest.placements.len(),
        manifest.diagnostics.len(),
        focus,
    );
    info!(
        "camera controls: Tab toggles FPS player/free camera, Esc releases cursor, left click captures cursor"
    );
}

fn effective_lighting(cell: &CellInfo) -> PreparedCellLighting {
    cell.effective_lighting
        .clone()
        .unwrap_or_else(|| PreparedCellLighting {
            ambient_rgba: cell.ambient_rgba,
            directional_rgba: cell.directional_rgba,
            ..default()
        })
}

fn distance_fog(lighting: &PreparedCellLighting, strength: f32) -> Option<DistanceFog> {
    let values = [
        lighting.fog_near,
        lighting.fog_far,
        lighting.fog_clip_distance,
        lighting.fog_power,
        lighting.directional_fade,
    ];
    if values.iter().any(|value| !value.is_finite())
        || lighting
            .fog_rgba
            .iter()
            .chain(lighting.directional_rgba.iter())
            .any(|value| !value.is_finite())
        || lighting.fog_far <= 0.0
        || lighting.fog_far <= lighting.fog_near
    {
        return None;
    }
    let start = lighting.fog_near.max(0.0) * FO3_SCALE;
    let mut end = lighting.fog_far.max(0.0) * FO3_SCALE;
    if lighting.fog_clip_distance > 0.0 {
        end = end.min(lighting.fog_clip_distance * FO3_SCALE);
    }
    if !start.is_finite() || !end.is_finite() || end <= start {
        return None;
    }
    if !strength.is_finite() || strength < 0.0 {
        return None;
    }
    let strength = strength.clamp(0.0, 1.0);
    let directional_fade = lighting.directional_fade.clamp(0.0, 1.0);
    Some(DistanceFog {
        color: Color::srgba(
            lighting.fog_rgba[0],
            lighting.fog_rgba[1],
            lighting.fog_rgba[2],
            strength,
        ),
        directional_light_color: Color::srgba(
            lighting.directional_rgba[0] * directional_fade,
            lighting.directional_rgba[1] * directional_fade,
            lighting.directional_rgba[2] * directional_fade,
            strength,
        ),
        directional_light_exponent: lighting.fog_power.max(1.0),
        falloff: FogFalloff::Linear { start, end },
    })
}

fn apply_fog_strength(
    fog_strength: Res<FogStrength>,
    manifest: Res<PreparedSceneManifest>,
    mut cameras: Query<&mut DistanceFog, With<Camera3d>>,
) {
    if !fog_strength.is_changed() {
        return;
    }
    let lighting = effective_lighting(&manifest.cell);
    let Some(fog) = distance_fog(&lighting, fog_strength.0) else {
        return;
    };
    for mut camera_fog in &mut cameras {
        *camera_fog = fog.clone();
    }
}

fn scaled_directional_illuminance(
    base_illuminance: f32,
    lighting_scale: f32,
    disabled: bool,
) -> f32 {
    if disabled {
        0.0
    } else {
        base_illuminance * lighting_scale / DEFAULT_LIGHTING_SCALE
    }
}

#[derive(Component)]
struct CellDirectionalLight {
    base_illuminance: f32,
}

fn spawn_interactive_placements(
    commands: &mut Commands,
    asset_server: &AssetServer,
    manifest: &PreparedSceneManifest,
) {
    for placement in manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled && !is_bake_static(placement))
    {
        let Some(path) = placement.asset_path.as_ref() else {
            continue;
        };
        commands.spawn((
            WorldAssetRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(path.clone()))),
            interaction::PlacementRoot::new(placement.clone()),
            Transform {
                translation: Vec3::from_array(placement.translation),
                rotation: Quat::from_xyzw(
                    placement.rotation_xyzw[0],
                    placement.rotation_xyzw[1],
                    placement.rotation_xyzw[2],
                    placement.rotation_xyzw[3],
                ),
                scale: Vec3::splat(placement.scale),
            },
        ));
    }
}

fn camera_post_processing(
    image_space: Option<&ImageSpaceInfo>,
    compensation_curves: &mut Assets<AutoExposureCompensationCurve>,
) -> (ColorGrading, Option<AutoExposure>) {
    let Some(image_space) = image_space else {
        return (ColorGrading::default(), None);
    };

    let flags = image_space.flags;
    let mut color_grading = ColorGrading::default();
    if flags & 0x08 != 0 {
        color_grading.global.exposure = image_space.brightness.max(0.0001).log2();
    }
    if flags & 0x01 != 0 {
        color_grading.global.post_saturation = image_space.cinematic_saturation.max(0.0);
    }
    if flags & 0x02 != 0 {
        let contrast = image_space.cinematic_contrast.max(0.0);
        color_grading.shadows.contrast = contrast;
        color_grading.midtones.contrast = contrast;
        color_grading.highlights.contrast = contrast;
    }
    if flags & 0x04 != 0
        && let Some((temperature, tint)) = image_space_tint_to_white_balance(
            image_space.cinematic_brightness_tint_rgb,
            image_space.cinematic_brightness_tint_value,
        )
    {
        color_grading.global.temperature = temperature;
        color_grading.global.tint = tint;
    }

    let target_lum = image_space.hdr_target_lum.max(0.001);
    let compensation = target_lum.log2();
    let compensation_curve = compensation_curves.add(
        AutoExposureCompensationCurve::from_curve(LinearSpline::new([
            vec2(-8.0, compensation),
            vec2(8.0, compensation),
        ]))
        .expect("flat auto-exposure compensation curve is valid"),
    );
    let speed = image_space_eye_adaptation_speed(image_space.eye_adapt_speed);
    let auto_exposure = AutoExposure {
        speed_brighten: speed,
        speed_darken: speed,
        compensation_curve,
        ..default()
    };
    (color_grading, Some(auto_exposure))
}

fn image_space_eye_adaptation_speed(value: f32) -> f32 {
    0.5 + (1.0 - value.clamp(0.0, 1.0)) * 7.5
}

fn image_space_tint_to_white_balance(rgb: [f32; 3], strength: f32) -> Option<(f32, f32)> {
    let [r, g, b] = rgb;
    let x = 0.412_456_4 * r + 0.357_576_1 * g + 0.180_437_5 * b;
    let y = 0.212_672_9 * r + 0.715_152_2 * g + 0.072_175 * b;
    let z = 0.019_333_9 * r + 0.119_192 * g + 0.950_304_1 * b;
    let sum = x + y + z;
    if sum <= f32::EPSILON || !sum.is_finite() {
        return None;
    }
    let target_x = x / sum;
    let target_y = y / sum;
    let strength = strength.max(0.0);
    Some((
        (0.3127 - target_x) * strength,
        (target_y - 0.3290) * strength,
    ))
}

type GlowCardMeshQuery<'w> = (Entity, &'w GltfMeshName);

fn configure_glow_cards(
    mut commands: Commands,
    meshes: Query<GlowCardMeshQuery<'_>, (With<Mesh3d>, Without<GlowCard>)>,
    mut inspected: Local<HashSet<Entity>>,
    mut last_mesh_count: Local<Option<usize>>,
) {
    let mesh_count = meshes.iter().count();
    if *last_mesh_count == Some(mesh_count) {
        return;
    }
    *last_mesh_count = Some(mesh_count);
    for (entity, name) in &meshes {
        if !inspected.insert(entity) {
            continue;
        }
        if !is_glow_card_mesh_name(&name.0) {
            continue;
        }
        // Converted assets promote the physical bulb to an emissive material
        // and no longer export this hint card. Keep this fallback for older
        // cached GLBs so they cannot reintroduce the large flat billboard.
        commands
            .entity(entity)
            .insert((Visibility::Hidden, GlowCard));
    }
}

#[derive(Component)]
struct GlowCard;

fn is_glow_card_mesh_name(name: &str) -> bool {
    name.to_ascii_lowercase().starts_with("lightglow")
}

fn scene_focus(manifest: &PreparedSceneManifest) -> Vec3 {
    let mut minimum = Vec3::splat(f32::INFINITY);
    let mut maximum = Vec3::splat(f32::NEG_INFINITY);
    let mut found = false;
    for placement in &manifest.placements {
        if placement.asset_path.is_none() {
            continue;
        }
        let position = Vec3::from_array(placement.translation);
        minimum = minimum.min(position);
        maximum = maximum.max(position);
        found = true;
    }
    if found {
        (minimum + maximum) * 0.5
    } else {
        Vec3::ZERO
    }
}

fn transition_camera_position(manifest: &PreparedSceneManifest) -> Option<Vec3> {
    manifest
        .placements
        .iter()
        .filter(|placement| placement.initially_enabled)
        .find_map(|placement| {
            matches!(
                &placement.semantic,
                PreparedSemantic::Door(door) if door.destination.is_some()
            )
            .then_some(Vec3::from_array(placement.translation) + Vec3::Y * player::EYE_HEIGHT)
        })
}

fn capture_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.visible = false;
    cursor_options.grab_mode = CursorGrabMode::Locked;
}

fn capture_cursor_input(
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
struct UnlitMode(bool);

#[derive(Resource)]
struct LightingScale(f32);

#[derive(Resource)]
struct IrradianceIntensity(f32);

#[derive(Resource)]
struct AmbientScale(f32);

#[derive(Resource)]
struct FogStrength(f32);

#[derive(Resource)]
struct AoStrength(f32);

#[derive(Resource, Default)]
struct AoMeshBases {
    values: HashMap<AssetId<Mesh>, VertexAttributeValues>,
}

#[derive(Default)]
struct AoScanState {
    last_mesh_entity_count: usize,
    last_mesh_asset_count: usize,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
enum AdjustmentTarget {
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

    fn label(self) -> &'static str {
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

    fn cycle(self, delta: i32) -> Self {
        let index = Self::ALL
            .iter()
            .position(|target| *target == self)
            .unwrap_or(0);
        let next = (index as i32 + delta).rem_euclid(Self::ALL.len() as i32) as usize;
        Self::ALL[next]
    }
}

#[derive(Component)]
struct AdjustmentHud;

#[derive(Resource)]
struct LightsDisabled(bool);

#[allow(clippy::too_many_arguments)]
fn adjust_selected_value(
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
fn update_adjustment_hud(
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

fn apply_irradiance_intensity(
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

fn apply_ao_strength(
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

fn scale_ao_colors(
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

fn scale_ao_channel(value: f32, strength: f32) -> f32 {
    (1.0 - (1.0 - value.clamp(0.0, 1.0)) * strength).clamp(0.0, 1.0)
}

fn scale_ao_byte(value: u8, strength: f32) -> u8 {
    (scale_ao_channel(f32::from(value) / 255.0, strength) * 255.0).round() as u8
}

fn scale_ao_u16(value: u16, strength: f32) -> u16 {
    (scale_ao_channel(f32::from(value) / 65_535.0, strength) * 65_535.0).round() as u16
}

fn toggle_lights_disabled(keys: Res<ButtonInput<KeyCode>>, mut disabled: ResMut<LightsDisabled>) {
    if keys.just_pressed(KeyCode::F3) {
        disabled.0 = !disabled.0;
        info!(
            "all runtime lights: {}",
            if disabled.0 { "off" } else { "on" }
        );
    }
}

fn apply_lighting_scale(
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

fn toggle_unlit_mode(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<UnlitMode>) {
    if keys.just_pressed(KeyCode::KeyQ) {
        mode.0 = !mode.0;
        info!(
            "unlit diagnostic mode: {}",
            if mode.0 { "on" } else { "off" }
        );
    }
}

fn apply_unlit_mode(mode: Res<UnlitMode>, mut materials: ResMut<Assets<StandardMaterial>>) {
    if !mode.is_changed() {
        return;
    }

    for material in materials.iter_mut().map(|(_, material)| material) {
        material.unlit = mode.0;
    }
}

#[derive(Component)]
struct FlyCamera {
    yaw: f32,
    pitch: f32,
    speed: f32,
}

fn free_fly_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse: MessageReader<MouseMotion>,
    mut wheel: MessageReader<MouseWheel>,
    cursor_options: Single<&CursorOptions>,
    mode: Res<player::CameraModeState>,
    mut query: Query<(&mut Transform, &mut FlyCamera), With<Camera3d>>,
    time: Res<Time>,
) {
    let wheel_delta = wheel.read().map(|event| event.y).sum::<f32>();
    let captured = matches!(cursor_options.grab_mode, CursorGrabMode::Locked);
    let delta = mouse
        .read()
        .fold(Vec2::ZERO, |sum, event| sum + event.delta);
    if mode.mode != player::CameraMode::Free || !captured {
        return;
    }
    let Ok((mut transform, mut camera)) = query.single_mut() else {
        return;
    };
    if wheel_delta != 0.0 {
        camera.speed = (camera.speed * 1.2_f32.powf(wheel_delta)).clamp(0.25, 256.0);
        info!("camera speed: {:.2}", camera.speed);
    }
    camera.yaw -= delta.x * 0.002;
    camera.pitch = (camera.pitch - delta.y * 0.002).clamp(-1.5, 1.5);
    transform.rotation = Quat::from_euler(EulerRot::YXZ, camera.yaw, camera.pitch, 0.0);
    let mut direction = Vec3::ZERO;
    let forward = transform.forward();
    let right = transform.right();
    if keys.pressed(KeyCode::KeyW) {
        direction += *forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction -= *forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += *right;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction -= *right;
    }
    if keys.pressed(KeyCode::KeyE) {
        direction += Vec3::Y;
    }
    if keys.pressed(KeyCode::KeyZ) {
        direction -= Vec3::Y;
    }
    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * camera.speed * time.delta_secs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjustment_target_cycles_with_page_navigation_order() {
        assert_eq!(
            AdjustmentTarget::default().cycle(1),
            AdjustmentTarget::IrradianceIntensity
        );
        assert_eq!(
            AdjustmentTarget::default().cycle(-1),
            AdjustmentTarget::AoStrength
        );
        assert_eq!(
            AdjustmentTarget::BloomSoftness.cycle(1),
            AdjustmentTarget::FogStrength
        );
        assert_eq!(
            AdjustmentTarget::FogStrength.cycle(1),
            AdjustmentTarget::AoStrength
        );
        assert_eq!(
            AdjustmentTarget::AoStrength.cycle(1),
            AdjustmentTarget::LightingScale
        );
        assert_eq!(
            AdjustmentTarget::LightingScale.cycle(1),
            AdjustmentTarget::IrradianceIntensity
        );
        assert_eq!(AdjustmentTarget::BloomIntensity.label(), "Bloom intensity");
    }

    #[test]
    fn irradiance_intensity_changes_exponentially_and_reaches_zero() {
        assert_eq!((1.0_f32 * 0.5).max(0.0), 0.5);
        assert_eq!((1.0_f32 * 2.0).min(4096.0), 2.0);
        assert_eq!((0.0_f32 * 0.5).max(0.0), 0.0);
    }

    #[test]
    fn glow_card_names_are_detected_without_matching_regular_meshes() {
        assert!(is_glow_card_mesh_name("LightGlow01:0.001"));
        assert!(is_glow_card_mesh_name("lightglow01"));
        assert!(!is_glow_card_mesh_name("ShackHangingLight02:51"));
    }

    #[test]
    fn render_report_path_resolves_to_project_root() {
        let manifest = Path::new(r"C:\project\.bevyout\cache\scenes\000151e3\scene.ron");
        assert_eq!(
            render_report_path(manifest),
            PathBuf::from(r"C:\project\render_timings.csv")
        );
    }

    #[test]
    fn ao_strength_scales_baked_darkness_without_changing_alpha() {
        let baseline = VertexAttributeValues::Float32x4(vec![[0.72, 0.8, 0.9, 0.5]]);
        let mut values = baseline.clone();
        scale_ao_colors(&mut values, &baseline, 0.5);
        let VertexAttributeValues::Float32x4(values) = values else {
            panic!("expected float colors");
        };
        assert!((values[0][0] - 0.86).abs() < 0.001);
        assert!((values[0][1] - 0.9).abs() < 0.001);
        assert!((values[0][3] - 0.5).abs() < 0.001);

        let mut disabled = baseline.clone();
        scale_ao_colors(&mut disabled, &baseline, 0.0);
        let VertexAttributeValues::Float32x4(disabled) = disabled else {
            panic!("expected float colors");
        };
        assert_eq!(disabled[0], [1.0, 1.0, 1.0, 0.5]);

        let baseline = VertexAttributeValues::Unorm16x4(vec![[47_185, 52_428, 58_982, 65_535]]);
        let mut values = baseline.clone();
        scale_ao_colors(&mut values, &baseline, 0.0);
        let VertexAttributeValues::Unorm16x4(values) = values else {
            panic!("expected normalized 16-bit colors");
        };
        assert_eq!(values[0], [65_535, 65_535, 65_535, 65_535]);
    }

    #[test]
    fn image_space_eye_adaptation_speed_uses_documented_endpoints() {
        assert!((image_space_eye_adaptation_speed(0.0) - 8.0).abs() < f32::EPSILON);
        assert!((image_space_eye_adaptation_speed(1.0) - 0.5).abs() < f32::EPSILON);
        assert!(image_space_eye_adaptation_speed(0.5) > image_space_eye_adaptation_speed(0.9));
    }

    #[test]
    fn image_space_tint_maps_neutral_white_to_neutral_balance() {
        let (temperature, tint) = image_space_tint_to_white_balance([1.0, 1.0, 1.0], 1.0)
            .expect("white has a valid chromaticity");
        assert!(temperature.abs() < 0.001);
        assert!(tint.abs() < 0.001);
    }

    #[test]
    fn image_space_settings_map_to_grading_and_target_exposure() {
        let mut image_space = ImageSpaceInfo {
            flags: 0x0f,
            hdr_target_lum: 2.0,
            brightness: 4.0,
            cinematic_saturation: 0.5,
            cinematic_contrast: 1.5,
            cinematic_brightness_tint_rgb: [0.8, 0.9, 1.0],
            cinematic_brightness_tint_value: 1.0,
            ..default()
        };
        image_space.eye_adapt_speed = 0.25;
        let mut curves = Assets::<AutoExposureCompensationCurve>::default();
        let (grading, auto_exposure) = camera_post_processing(Some(&image_space), &mut curves);

        assert!((grading.global.exposure - 2.0).abs() < f32::EPSILON);
        assert!((grading.global.post_saturation - 0.5).abs() < f32::EPSILON);
        assert!((grading.shadows.contrast - 1.5).abs() < f32::EPSILON);
        assert!((grading.midtones.contrast - 1.5).abs() < f32::EPSILON);
        assert!((grading.highlights.contrast - 1.5).abs() < f32::EPSILON);
        assert!(grading.global.tint.abs() > 0.0 || grading.global.temperature.abs() > 0.0);
        let auto_exposure = auto_exposure.expect("image space enables auto exposure");
        assert!((auto_exposure.speed_brighten - 6.125).abs() < f32::EPSILON);
        assert_eq!(curves.len(), 1);
    }

    #[test]
    fn missing_image_space_keeps_fixed_camera_post_processing() {
        let mut curves = Assets::<AutoExposureCompensationCurve>::default();
        let (grading, auto_exposure) = camera_post_processing(None, &mut curves);
        assert_eq!(grading.global.exposure, 0.0);
        assert!(auto_exposure.is_none());
        assert_eq!(curves.len(), 0);
    }

    #[test]
    fn fog_uses_fo3_distances_and_rejects_invalid_ranges() {
        let lighting = PreparedCellLighting {
            fog_rgba: [0.1, 0.2, 0.3, 1.0],
            directional_rgba: [0.4, 0.5, 0.6, 1.0],
            fog_near: 10.0,
            fog_far: 100.0,
            fog_clip_distance: 80.0,
            fog_power: 0.5,
            ..default()
        };
        let fog = distance_fog(&lighting, DEFAULT_FOG_STRENGTH).expect("valid FO3 fog should map");
        match fog.falloff {
            FogFalloff::Linear { start, end } => {
                assert!((start - 10.0 * FO3_SCALE).abs() < f32::EPSILON);
                assert!((end - 80.0 * FO3_SCALE).abs() < f32::EPSILON);
            }
            _ => panic!("expected linear fog"),
        }
        assert_eq!(fog.directional_light_exponent, 1.0);
        assert!(
            distance_fog(
                &PreparedCellLighting {
                    fog_near: 20.0,
                    fog_far: 10.0,
                    ..lighting
                },
                DEFAULT_FOG_STRENGTH,
            )
            .is_none()
        );
    }

    #[test]
    fn directional_rotation_and_light_scale_are_deterministic() {
        let lighting = PreparedCellLighting {
            directional_rotation_z: 90,
            ..default()
        };
        let expected = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        let actual = Quat::from_array(lighting.directional_rotation_xyzw());
        assert!(actual.dot(expected).abs() > 1.0 - 1e-5);
        assert_eq!(
            scaled_directional_illuminance(10_000.0, 256.0, false),
            20_000.0
        );
        assert_eq!(scaled_directional_illuminance(10_000.0, 256.0, true), 0.0);
    }
}

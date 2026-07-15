//! Opt-in bridge for local agents.
//!
//! Bevy's Remote Protocol owns the transport and ECS scheduling. This module
//! only adds a small amount of bevyout-specific context that is not useful to
//! expose through raw reflection alone.

use std::time::{SystemTime, UNIX_EPOCH};

use bevy::ecs::schedule::Schedules;
use bevy::prelude::*;
use bevy::remote::{
    BrpError, BrpResult, RemotePlugin, error_codes::INVALID_PARAMS, http::RemoteHttpPlugin,
};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use serde_json::{Value, json};

use super::interaction::PlacementRoot;
use super::player::FpsPlayer;
use super::{RenderReportBuffer, diagnostics};
use crate::console::{ConsoleExecutor, ConsoleRegistry, ConsoleRequest, ConsoleSessionId};
use crate::vsa::PreparedSceneManifest;

const DEFAULT_SNAPSHOT_LIMIT: usize = 100;
const MAX_SNAPSHOT_LIMIT: usize = 1_000;
const DEFAULT_FRAME_BUDGET_MS: f64 = 16.667;
const MAX_PERFORMANCE_SAMPLES: usize = 600;
const DEFAULT_CONFLICT_LIMIT: usize = 100;
const MAX_CONFLICT_LIMIT: usize = 1_000;

#[derive(Resource)]
struct AgentBridgeInfo {
    port: u16,
    session_id: String,
}

pub(crate) fn install(app: &mut App, port: u16) {
    let session_id = format!(
        "{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.as_millis())
            .unwrap_or_default()
    );

    let remote = RemotePlugin::default()
        .with_method_main("bevyout.session", session)
        .with_method_main("bevyout.scene_snapshot", scene_snapshot)
        .with_method_main("bevyout.performance_snapshot", performance_snapshot)
        .with_method_main("bevyout.schedule_snapshot", schedule_snapshot)
        .with_method_main("bevyout.capture_viewport", capture_viewport)
        .with_method_main("bevyout.console.exec", console_exec)
        .with_method_main("bevyout.console.help", console_help);
    let http = RemoteHttpPlugin::default()
        .with_address(std::net::Ipv4Addr::LOCALHOST)
        .with_port(port);

    app.insert_resource(AgentBridgeInfo { port, session_id })
        .add_plugins((remote, http));
    info!("agent bridge enabled on http://127.0.0.1:{port}/ (runtime-only ECS access)");
}

fn performance_snapshot(In(params): In<Option<Value>>, world: &mut World) -> BrpResult {
    let params = params.unwrap_or_else(|| json!({}));
    let object = params
        .as_object()
        .ok_or_else(|| invalid_params("params must be an object"))?;
    let after_sample = object.get("after_sample").and_then(Value::as_u64);
    let latest_limit = object
        .get("latest_limit")
        .and_then(Value::as_u64)
        .unwrap_or(MAX_PERFORMANCE_SAMPLES as u64) as usize;
    if !(1..=MAX_PERFORMANCE_SAMPLES).contains(&latest_limit) {
        return Err(invalid_params(format!(
            "latest_limit must be between 1 and {MAX_PERFORMANCE_SAMPLES}"
        )));
    }
    let budget_ms = object
        .get("budget_ms")
        .and_then(Value::as_f64)
        .unwrap_or(DEFAULT_FRAME_BUDGET_MS);
    if !budget_ms.is_finite() || budget_ms <= 0.0 {
        return Err(invalid_params(
            "budget_ms must be finite and greater than zero",
        ));
    }
    let include_samples = object
        .get("include_samples")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let (latest_sample, mut window) = {
        let report = world.resource::<RenderReportBuffer>();
        (
            diagnostics::latest_render_sample(report),
            diagnostics::summarize_render_samples(report, after_sample, latest_limit, budget_ms),
        )
    };
    if !include_samples {
        window.samples.clear();
    }

    let mut diagnostics = world
        .resource::<bevy::diagnostic::DiagnosticsStore>()
        .iter()
        .filter_map(|diagnostic| {
            let value = diagnostic.value()?;
            value.is_finite().then(|| {
                json!({
                    "path": diagnostic.path().as_str(),
                    "suffix": diagnostic.suffix.as_ref(),
                    "value": value,
                    "average": diagnostic.average().filter(|value| value.is_finite()),
                    "smoothed": diagnostic.smoothed().filter(|value| value.is_finite()),
                    "history_len": diagnostic.history_len(),
                })
            })
        })
        .collect::<Vec<_>>();
    diagnostics.sort_unstable_by(|left, right| {
        left["path"]
            .as_str()
            .unwrap_or_default()
            .cmp(right["path"].as_str().unwrap_or_default())
    });

    let entity_count = world.entities().len();
    let mesh_entity_count = world
        .query_filtered::<Entity, With<Mesh3d>>()
        .iter(world)
        .count();
    let point_light_count = world
        .query_filtered::<Entity, With<PointLight>>()
        .iter(world)
        .count();
    let directional_light_count = world
        .query_filtered::<Entity, With<DirectionalLight>>()
        .iter(world)
        .count();

    Ok(json!({
        "latest_sample": latest_sample,
        "raw_samples_included": include_samples,
        "window": window,
        "diagnostics": diagnostics,
        "world": {
            "entities": entity_count,
            "mesh_entities": mesh_entity_count,
            "point_lights": point_light_count,
            "directional_lights": directional_light_count,
        },
    }))
}

fn schedule_snapshot(In(params): In<Option<Value>>, world: &World) -> BrpResult {
    let params = params.unwrap_or_else(|| json!({}));
    let object = params
        .as_object()
        .ok_or_else(|| invalid_params("params must be an object"))?;
    let schedule_contains = object
        .get("schedule_contains")
        .and_then(Value::as_str)
        .map(str::to_ascii_lowercase);
    let include_systems = object
        .get("include_systems")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let conflict_limit = object
        .get("conflict_limit")
        .and_then(Value::as_u64)
        .unwrap_or(DEFAULT_CONFLICT_LIMIT as u64) as usize;
    if conflict_limit > MAX_CONFLICT_LIMIT {
        return Err(invalid_params(format!(
            "conflict_limit must not exceed {MAX_CONFLICT_LIMIT}"
        )));
    }

    let components = world.components();
    let schedules = world.resource::<Schedules>();
    let mut rows = Vec::new();
    let mut total_systems = 0usize;
    let mut total_exclusive_systems = 0usize;
    let mut total_conflicts = 0usize;
    for (label, schedule) in schedules.iter() {
        let label = format!("{label:?}");
        if schedule_contains
            .as_ref()
            .is_some_and(|needle| !label.to_ascii_lowercase().contains(needle))
        {
            continue;
        }

        let Ok(systems) = schedule.systems() else {
            rows.push(json!({
                "label": label,
                "initialized": false,
                "system_count": 0,
                "exclusive_system_count": 0,
                "non_send_system_count": 0,
                "deferred_system_count": 0,
                "conflict_pair_count": 0,
                "conflicts": [],
            }));
            continue;
        };
        let system_rows = systems
            .map(|(key, system)| {
                (
                    key,
                    json!({
                        "name": system.name().to_string(),
                        "exclusive": system.is_exclusive(),
                        "send": system.is_send(),
                        "deferred": system.has_deferred(),
                    }),
                )
            })
            .collect::<Vec<_>>();
        let system_names = system_rows
            .iter()
            .map(|(key, system)| (*key, system["name"].as_str().unwrap_or_default().to_owned()))
            .collect::<std::collections::HashMap<_, _>>();
        let exclusive_system_count = system_rows
            .iter()
            .filter(|(_, system)| system["exclusive"] == true)
            .count();
        let non_send_system_count = system_rows
            .iter()
            .filter(|(_, system)| system["send"] == false)
            .count();
        let deferred_system_count = system_rows
            .iter()
            .filter(|(_, system)| system["deferred"] == true)
            .count();

        let conflicts = schedule.graph().conflicting_systems();
        let conflict_pair_count = conflicts.len();
        let conflict_rows = conflicts
            .iter()
            .take(conflict_limit)
            .map(|(system_a, system_b, conflicting_components)| {
                json!({
                    "system_a": system_names
                        .get(system_a)
                        .cloned()
                        .unwrap_or_else(|| format!("{system_a:?}")),
                    "system_b": system_names
                        .get(system_b)
                        .cloned()
                        .unwrap_or_else(|| format!("{system_b:?}")),
                    "components": conflicting_components
                        .iter()
                        .map(|component| components
                            .get_name(*component)
                            .map(|name| name.to_string())
                            .unwrap_or_else(|| format!("{component:?}")))
                        .collect::<Vec<_>>(),
                    "world_access": conflicting_components.is_empty(),
                })
            })
            .collect::<Vec<_>>();

        total_systems += system_rows.len();
        total_exclusive_systems += exclusive_system_count;
        total_conflicts += conflict_pair_count;
        rows.push(json!({
            "label": label,
            "initialized": true,
            "system_count": system_rows.len(),
            "exclusive_system_count": exclusive_system_count,
            "non_send_system_count": non_send_system_count,
            "deferred_system_count": deferred_system_count,
            "conflict_pair_count": conflict_pair_count,
            "conflicts_truncated": conflict_pair_count > conflict_rows.len(),
            "conflicts": conflict_rows,
            "systems": include_systems.then(|| system_rows
                .into_iter()
                .map(|(_, system)| system)
                .collect::<Vec<_>>()),
        }));
    }
    rows.sort_unstable_by(|left, right| {
        left["label"]
            .as_str()
            .unwrap_or_default()
            .cmp(right["label"].as_str().unwrap_or_default())
    });

    Ok(json!({
        "schedule_count": rows.len(),
        "system_count": total_systems,
        "exclusive_system_count": total_exclusive_systems,
        "conflict_pair_count": total_conflicts,
        "schedules": rows,
    }))
}

fn session(
    In(_params): In<Option<Value>>,
    info: Res<AgentBridgeInfo>,
    manifest: Res<PreparedSceneManifest>,
) -> BrpResult {
    Ok(json!({
        "session_id": info.session_id,
        "port": info.port,
        "persistence": "runtime_only",
        "cell": {
            "form_id": manifest.cell.form_id,
            "editor_id": manifest.cell.editor_id,
            "name": manifest.cell.name,
        },
        "placement_count": manifest.placements.len(),
        "diagnostic_count": manifest.diagnostics.len(),
    }))
}

#[allow(clippy::type_complexity)]
fn scene_snapshot(
    In(params): In<Option<Value>>,
    query: Query<(
        Entity,
        &Transform,
        Option<&GlobalTransform>,
        Option<&ChildOf>,
        Option<&Name>,
        Option<&PlacementRoot>,
        Option<&Camera3d>,
        Option<&PointLight>,
        Option<&DirectionalLight>,
        Option<&FpsPlayer>,
    )>,
) -> BrpResult {
    let params = params.unwrap_or_else(|| json!({}));
    let object = params
        .as_object()
        .ok_or_else(|| invalid_params("params must be an object"))?;
    let offset = object.get("offset").and_then(Value::as_u64).unwrap_or(0) as usize;
    let limit = object
        .get("limit")
        .and_then(Value::as_u64)
        .unwrap_or(DEFAULT_SNAPSHOT_LIMIT as u64)
        .clamp(1, MAX_SNAPSHOT_LIMIT as u64) as usize;
    let include_other = object
        .get("include_other")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let role_filter = object.get("role").and_then(Value::as_str);
    let name_filter = object.get("name_contains").and_then(Value::as_str);

    let mut entities = Vec::new();
    for (
        entity,
        transform,
        global,
        parent,
        name,
        placement,
        camera,
        point_light,
        directional_light,
        player,
    ) in &query
    {
        let role = if placement.is_some() {
            "placement"
        } else if player.is_some() {
            "player"
        } else if camera.is_some() {
            "camera"
        } else if point_light.is_some() || directional_light.is_some() {
            "light"
        } else if include_other {
            "entity"
        } else {
            continue;
        };
        if role_filter.is_some_and(|filter| filter != role) {
            continue;
        }

        let name = name.map(Name::as_str);
        if name_filter.is_some_and(|filter| !name.is_some_and(|value| value.contains(filter))) {
            continue;
        }

        let mut row = json!({
            "entity": entity.to_bits(),
            "role": role,
            "name": name,
            "parent": parent.map(|parent| parent.parent().to_bits()),
            "transform": {
                "translation": [transform.translation.x, transform.translation.y, transform.translation.z],
                "rotation_xyzw": [transform.rotation.x, transform.rotation.y, transform.rotation.z, transform.rotation.w],
                "scale": [transform.scale.x, transform.scale.y, transform.scale.z],
            },
        });

        if let Some(global) = global {
            let (scale, rotation, translation) = global.to_scale_rotation_translation();
            row["global_transform"] = json!({
                "translation": [translation.x, translation.y, translation.z],
                "rotation_xyzw": [rotation.x, rotation.y, rotation.z, rotation.w],
                "scale": [scale.x, scale.y, scale.z],
            });
        }
        if let Some(placement) = placement {
            let placement = placement.placement();
            row["placement"] = json!({
                "reference_form_id": placement.reference_form_id,
                "base_form_id": placement.base_form_id,
                "editor_id": placement.editor_id,
                "display_name": placement.display_name,
                "reference_kind": placement.reference_kind,
                "base_kind": placement.base_kind,
                "semantic": placement.semantic,
                "initially_enabled": placement.initially_enabled,
            });
        }
        entities.push(row);
    }

    let total = entities.len();
    let page = entities
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect::<Vec<_>>();
    let next_offset = (offset + page.len() < total).then_some(offset + page.len());
    Ok(json!({
        "entities": page,
        "total": total,
        "offset": offset,
        "limit": limit,
        "next_offset": next_offset,
    }))
}

fn capture_viewport(In(params): In<Option<Value>>, mut commands: Commands) -> BrpResult {
    let params = params.unwrap_or_else(|| json!({}));
    let token = params
        .get("token")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_params("capture requires a token"))?;
    if token.is_empty()
        || token.len() > 96
        || !token
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    {
        return Err(invalid_params(
            "token must contain only letters, numbers, '-' or '_'",
        ));
    }

    let directory = std::env::temp_dir().join("bevyout-agent");
    std::fs::create_dir_all(&directory).map_err(BrpError::internal)?;
    let path = directory.join(format!("{token}.png"));
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path.clone()));

    Ok(json!({
        "token": token,
        "path": path,
        "persistence": "temporary",
    }))
}

fn console_exec(In(params): In<Option<Value>>, world: &mut World) -> BrpResult {
    let params = params.unwrap_or_else(|| json!({}));
    let object = params
        .as_object()
        .ok_or_else(|| invalid_params("params must be an object"))?;
    let line = object
        .get("line")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_params("console.exec requires a string 'line'"))?;
    if line.len() > 16_384 {
        return Err(invalid_params("console line exceeds 16384 bytes"));
    }
    let session = object
        .get("session")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| world.resource::<AgentBridgeInfo>().session_id.clone());
    if session.is_empty() || session.len() > 128 {
        return Err(invalid_params(
            "console session must contain between 1 and 128 bytes",
        ));
    }
    serde_json::to_value(ConsoleExecutor::execute(
        world,
        ConsoleRequest {
            session: ConsoleSessionId::new(session),
            line: line.to_string(),
        },
    ))
    .map_err(BrpError::internal)
}

fn console_help(In(params): In<Option<Value>>, world: &World) -> BrpResult {
    if let Some(params) = params
        && !params.is_null()
        && !params.as_object().is_some_and(serde_json::Map::is_empty)
    {
        return Err(invalid_params("console.help does not accept parameters"));
    }
    serde_json::to_value(world.resource::<ConsoleRegistry>().metadata()).map_err(BrpError::internal)
}

fn invalid_params(message: impl Into<String>) -> BrpError {
    BrpError {
        code: INVALID_PARAMS,
        message: message.into(),
        data: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::{ConsolePlugin, RefRegistry};

    fn app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, ConsolePlugin))
            .init_resource::<bevy::diagnostic::DiagnosticsStore>()
            .insert_resource(RenderReportBuffer::default())
            .insert_resource(AgentBridgeInfo {
                port: 15_702,
                session_id: "bridge-test".into(),
            });
        let entity = app
            .world_mut()
            .spawn(Transform::from_xyz(1.0, 2.0, 3.0))
            .id();
        app.world_mut()
            .resource_mut::<RefRegistry>()
            .register(entity, 1, Some("TestRef"));
        app.update();
        app
    }

    #[test]
    fn brp_console_result_matches_direct_executor_shape() {
        let mut direct = app();
        let expected = serde_json::to_value(ConsoleExecutor::execute(
            direct.world_mut(),
            ConsoleRequest {
                session: ConsoleSessionId::new("same"),
                line: "00000001.getpos".into(),
            },
        ))
        .unwrap();

        let mut remote = app();
        let actual = console_exec(
            In(Some(
                json!({ "session": "same", "line": "00000001.getpos" }),
            )),
            remote.world_mut(),
        )
        .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn brp_console_help_exposes_registry_metadata() {
        let app = app();
        let value = console_help(In(None), app.world()).unwrap();
        assert!(
            value
                .as_array()
                .unwrap()
                .iter()
                .any(|entry| entry["name"] == "getpos")
        );
    }

    #[test]
    fn performance_snapshot_exposes_a_bounded_empty_window() {
        let mut app = app();
        let value = performance_snapshot(
            In(Some(json!({
                "latest_limit": 32,
                "budget_ms": 20.0,
                "include_samples": true,
            }))),
            app.world_mut(),
        )
        .unwrap();
        assert_eq!(value["window"]["sample_count"], 0);
        assert_eq!(value["window"]["budget_ms"], 20.0);
        assert_eq!(value["raw_samples_included"], true);
        assert!(value["world"]["entities"].as_u64().unwrap() >= 1);
    }

    #[test]
    fn schedule_snapshot_reports_initialized_system_metadata() {
        let app = app();
        let value =
            schedule_snapshot(In(Some(json!({ "include_systems": true }))), app.world()).unwrap();
        assert!(value["schedule_count"].as_u64().unwrap() > 0);
        assert!(
            value["schedules"]
                .as_array()
                .unwrap()
                .iter()
                .any(|schedule| schedule["initialized"] == true)
        );
    }
}

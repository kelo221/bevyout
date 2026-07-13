//! Opt-in bridge for local agents.
//!
//! Bevy's Remote Protocol owns the transport and ECS scheduling. This module
//! only adds a small amount of bevyout-specific context that is not useful to
//! expose through raw reflection alone.

use std::time::{SystemTime, UNIX_EPOCH};

use bevy::prelude::*;
use bevy::remote::{
    BrpError, BrpResult, RemotePlugin, error_codes::INVALID_PARAMS, http::RemoteHttpPlugin,
};
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
use serde_json::{Value, json};

use super::interaction::PlacementRoot;
use super::player::FpsPlayer;
use crate::vsa::PreparedSceneManifest;

const DEFAULT_SNAPSHOT_LIMIT: usize = 100;
const MAX_SNAPSHOT_LIMIT: usize = 1_000;

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
        .with_method_main("bevyout.capture_viewport", capture_viewport);
    let http = RemoteHttpPlugin::default()
        .with_address(std::net::Ipv4Addr::LOCALHOST)
        .with_port(port);

    app.insert_resource(AgentBridgeInfo { port, session_id })
        .add_plugins((remote, http));
    info!("agent bridge enabled on http://127.0.0.1:{port}/ (runtime-only ECS access)");
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

fn invalid_params(message: impl Into<String>) -> BrpError {
    BrpError {
        code: INVALID_PARAMS,
        message: message.into(),
        data: None,
    }
}

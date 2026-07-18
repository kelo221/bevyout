//! Center-screen focus acquisition, placement-root resolution, and probes.

use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn update_focused_placement(
    time: Res<Time>,
    mode: Res<CameraModeState>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    inventory: Res<PlayerInventory>,
    mut state: ResMut<InteractionState>,
    mut prompt: Query<&mut Text, With<InteractionPromptText>>,
    mut raycast_elapsed: Local<f32>,
) {
    if mode.mode != CameraMode::Fps {
        state.focused = None;
        if let Ok(mut prompt) = prompt.single_mut() {
            prompt.0.clear();
        }
        return;
    }
    *raycast_elapsed += time.delta_secs();
    if *raycast_elapsed < FOCUS_RAYCAST_INTERVAL_SECONDS {
        return;
    }
    *raycast_elapsed = 0.0;

    let focused = active_center_ray(&cameras).and_then(|ray| {
        let settings = MeshRayCastSettings {
            visibility: RayCastVisibility::VisibleInView,
            ..default()
        };
        let (hit_entity, hit) = raycast.cast_ray(ray, &settings).first()?;
        if hit.distance > INTERACTION_DISTANCE_METERS {
            return None;
        }
        let root_entity = find_placement_root(*hit_entity, &parents, &roots)?;
        let root = roots.get(root_entity).ok()?;
        interaction_prompt(
            &root.placement,
            state.open.contains(&root_entity),
            &inventory,
        )
        .map(|text| (root_entity, text))
    });

    state.focused = focused.as_ref().map(|(entity, _)| *entity);
    if let Ok(mut prompt) = prompt.single_mut() {
        prompt.0 = focused.map(|(_, text)| text).unwrap_or_default();
    }
}

fn active_center_ray(
    cameras: &Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Option<Ray3d> {
    cameras.iter().find_map(|(camera, transform)| {
        if !camera.is_active {
            return None;
        }
        let viewport = camera.logical_viewport_size()?;
        camera.viewport_to_world(transform, viewport * 0.5).ok()
    })
}

pub(crate) fn find_placement_root(
    mut entity: Entity,
    parents: &Query<&ChildOf>,
    roots: &Query<&PlacementRoot>,
) -> Option<Entity> {
    for _ in 0..MAX_PARENT_DEPTH {
        if roots.contains(entity) {
            return Some(entity);
        }
        entity = parents.get(entity).ok()?.parent();
    }
    warn!("placement hierarchy exceeded {MAX_PARENT_DEPTH} ancestors");
    None
}

pub(super) fn cleanup_removed_placements(
    mut removed: RemovedComponents<PlacementRoot>,
    mut references: ResMut<RefRegistry>,
    mut sessions: ResMut<ConsoleSessionStore>,
) {
    for entity in removed.read() {
        references.unregister(entity);
        sessions.clear_entity(entity);
    }
}

pub(super) fn probe_center_target(
    keys: Res<ButtonInput<KeyCode>>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut raycast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<&PlacementRoot>,
    references: Res<RefRegistry>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }

    let Some(ray) = active_center_ray(&cameras) else {
        info!("{}", probe_status_message(false, None));
        return;
    };
    let settings = MeshRayCastSettings {
        visibility: RayCastVisibility::VisibleInView,
        ..default()
    };
    let Some((hit_entity, _)) = raycast.cast_ray(ray, &settings).first() else {
        info!("{}", probe_status_message(false, None));
        return;
    };
    let Some(root_entity) = find_placement_root(*hit_entity, &parents, &roots) else {
        info!("{}", probe_status_message(true, None));
        return;
    };
    if references.identity(root_entity).is_none() {
        info!("{}", probe_status_message(true, None));
        return;
    }
    let label = references.label(root_entity);
    info!("{}", probe_status_message(true, Some(&label)));
}

pub(super) fn probe_status_message(hit: bool, label: Option<&str>) -> String {
    if !hit {
        "probe: no target".into()
    } else if let Some(label) = label {
        format!("probe: {label}")
    } else {
        "probe: NOT_IMPLEMENTED (static-batched geometry)".into()
    }
}

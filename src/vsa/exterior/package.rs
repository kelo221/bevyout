use bevyout_core::actor::ActorKind;
use bevyout_core::manifest::CellInfo;
use bevyout_core::manifest::PreparedPlacement;
use bevyout_core::manifest::PreparedSemantic;
use bevyout_core::manifest::exterior::{
    EXTERIOR_CELL_PACKAGE_REVISION, EXTERIOR_ENVIRONMENT_REVISION, EXTERIOR_NAVIGATION_REVISION,
    ExteriorBorderPortal, ExteriorCellPackage, ExteriorCoordinatePolicy, ExteriorDiagnostic,
    GridCoordinate, PreparedExteriorActor, PreparedExteriorDoorDestination,
    PreparedExteriorEnvironment, PreparedExteriorLight, PreparedExteriorNavigation,
    PreparedExteriorObject, PreparedWater,
};

use super::super::manifest::PreparedNavGraphSource;
use super::super::openmw_esm4::{ParsedPlugin, ReferenceKind};
use super::terrain_from_land;
use super::{DISTANT_REFERENCE_FLAG, PERSISTENT_REFERENCE_FLAG};
use crate::vsa::paths::{placement_transform, placement_transform_parts};

pub(crate) fn build_cell_package(
    parsed: &ParsedPlugin,
    cell: &CellInfo,
    content_fingerprint: &str,
    navigation_source: Option<&PreparedNavGraphSource>,
    navigation_clearance_ready: bool,
) -> Option<ExteriorCellPackage> {
    let grid = cell.grid.map(|(x, y)| GridCoordinate::new(x, y))?;
    let worldspace_form_id = cell.worldspace_form_id?;
    let policy = ExteriorCoordinatePolicy::default();
    let terrain = parsed
        .land
        .as_ref()
        .map(|land| terrain_from_land(land, grid));
    let mut diagnostics = parsed
        .diagnostics
        .iter()
        .map(|message| ExteriorDiagnostic {
            code: "parser".into(),
            form_id: Some(cell.form_id),
            severity: "info".into(),
            message: message.clone(),
        })
        .collect::<Vec<_>>();
    if terrain
        .as_ref()
        .is_some_and(|terrain| !terrain.is_well_formed())
    {
        diagnostics.push(ExteriorDiagnostic {
            code: "terrain_shape".into(),
            form_id: parsed.land.as_ref().map(|land| land.form_id),
            severity: "error".into(),
            message: "LAND sample arrays do not agree".into(),
        });
    }
    if let Some(land) = parsed.land.as_ref()
        && !land.is_complete()
    {
        diagnostics.push(ExteriorDiagnostic {
            code: "terrain_incomplete".into(),
            form_id: Some(land.form_id),
            severity: "warning".into(),
            message: "LAND is missing one or more source arrays; deterministic fallbacks were used"
                .into(),
        });
    }
    if parsed.road_count > 0 {
        diagnostics.push(ExteriorDiagnostic {
            code: "road_records".into(),
            form_id: Some(cell.form_id),
            severity: "info".into(),
            message: format!(
                "exterior cell contains {} ROAD record(s)",
                parsed.road_count
            ),
        });
    }
    let mut static_objects = Vec::new();
    let mut dynamic_objects = Vec::new();
    let mut distant_objects = Vec::new();
    let mut actors = Vec::new();
    let mut local_lights = Vec::new();
    for reference in &parsed.references {
        let Some(base) = parsed.bases.get(&reference.base_form_id) else {
            diagnostics.push(ExteriorDiagnostic {
                code: "missing_base".into(),
                form_id: Some(reference.base_form_id),
                severity: "warning".into(),
                message: format!("reference {:08x} has no resolved base", reference.form_id),
            });
            continue;
        };
        let (position, rotation_xyzw, scale) = placement_transform(reference);
        let persistent = reference.flags & PERSISTENT_REFERENCE_FLAG != 0;
        // ACHR/ACRE references are gameplay actors, not mesh entries: route
        // them to their own field with room for the resolved actor assembly
        // instead of flattening them into `dynamic_objects` (issue #299).
        if let Some(kind) = actor_kind(reference.kind) {
            if persistent {
                diagnostics.push(ExteriorDiagnostic {
                    code: "persistent_worldspace_reference".into(),
                    form_id: Some(reference.form_id),
                    severity: "info".into(),
                    message:
                        "persistent reference is owned by the worldspace index, not this cell package"
                            .into(),
                });
            } else {
                actors.push(PreparedExteriorActor {
                    reference_form_id: reference.form_id,
                    base_form_id: reference.base_form_id,
                    kind,
                    asset_path: base.model.clone(),
                    physics_asset_path: None,
                    assembly: None,
                    position,
                    rotation_xyzw,
                    scale,
                    initially_enabled: reference.initially_enabled,
                });
            }
            if let Some(light) = &base.light {
                local_lights.push(PreparedExteriorLight {
                    reference_form_id: reference.form_id,
                    position,
                    color_rgba: light.color_rgba,
                    range: light.radius * crate::vsa::paths::FO3_SCALE,
                });
            }
            continue;
        }
        let object = PreparedExteriorObject {
            reference_form_id: reference.form_id,
            base_form_id: reference.base_form_id,
            asset_path: base.model.clone(),
            physics_asset_path: None,
            door_destination: reference.door.as_ref().and_then(|door| {
                door.destination.as_ref().map(|destination| {
                    let (position, rotation, _) =
                        placement_transform_parts(destination.position, destination.rotation, 1.0);
                    PreparedExteriorDoorDestination {
                        door_reference_form_id: destination.door_reference_form_id,
                        cell_form_id: destination.cell_form_id,
                        position,
                        rotation_xyzw: rotation,
                    }
                })
            }),
            position,
            rotation_xyzw,
            scale,
            initially_enabled: reference.initially_enabled,
            persistent,
            dynamic: !matches!(reference.kind, ReferenceKind::Object) || base.kind == "DOOR",
            distant: reference.flags & DISTANT_REFERENCE_FLAG != 0,
        };
        if object.persistent {
            diagnostics.push(ExteriorDiagnostic {
                code: "persistent_worldspace_reference".into(),
                form_id: Some(object.reference_form_id),
                severity: "info".into(),
                message:
                    "persistent reference is owned by the worldspace index, not this cell package"
                        .into(),
            });
        } else if object.distant {
            distant_objects.push(object);
        } else if object.dynamic {
            dynamic_objects.push(object);
        } else {
            static_objects.push(object);
        }
        if let Some(light) = &base.light {
            local_lights.push(PreparedExteriorLight {
                reference_form_id: reference.form_id,
                position,
                color_rgba: light.color_rgba,
                range: light.radius * crate::vsa::paths::FO3_SCALE,
            });
        }
    }
    static_objects.sort_by_key(|object| object.reference_form_id);
    dynamic_objects.sort_by_key(|object| object.reference_form_id);
    actors.sort_by_key(|actor| actor.reference_form_id);
    distant_objects.sort_by_key(|object| object.reference_form_id);
    local_lights.sort_by_key(|light| light.reference_form_id);
    let authored_water_height = cell.water_height;
    let water_height =
        authored_water_height.filter(|height| height.is_finite() && height.abs() < 1_000_000.0);
    if authored_water_height.is_some() && water_height.is_none() {
        diagnostics.push(ExteriorDiagnostic {
            code: "invalid_water_height".into(),
            form_id: Some(cell.form_id),
            severity: "warning".into(),
            message:
                "CELL water height was a non-finite or out-of-range sentinel; water was omitted"
                    .into(),
        });
    }
    let water = water_height.map(|height| PreparedWater {
        form_id: cell.water_form_id,
        height: height * crate::vsa::paths::FO3_SCALE,
        water_type_form_id: cell.water_form_id,
        swim_depth: 1.0,
    });
    let environment = PreparedExteriorEnvironment {
        revision: EXTERIOR_ENVIRONMENT_REVISION.into(),
        climate_form_id: cell
            .day_night_profile
            .as_ref()
            .and_then(|profile| profile.climate_form_id),
        weather_form_id: cell
            .day_night_profile
            .as_ref()
            .map(|profile| profile.weather_form_id),
        image_space_form_id: cell.image_space_form_id,
        water_type_form_id: cell.water_form_id,
        fog_near: cell.effective_lighting.as_ref().map_or(0.0, |lighting| {
            lighting.fog_near * crate::vsa::paths::FO3_SCALE
        }),
        fog_far: cell.effective_lighting.as_ref().map_or(0.0, |lighting| {
            lighting.fog_far * crate::vsa::paths::FO3_SCALE
        }),
        dynamic_lighting_allowed: true,
        timings: cell
            .day_night_profile
            .as_ref()
            .map(|profile| profile.timings)
            .unwrap_or_default(),
        weather_profiles: Vec::new(),
    };
    let navigation = (!parsed.navmeshes.is_empty()).then(|| PreparedExteriorNavigation {
        revision: EXTERIOR_NAVIGATION_REVISION.into(),
        graph_asset_path: navigation_source.map(|source| source.asset_path.clone()),
        graph_hash: navigation_source.map(|source| source.hash.clone()),
        mesh_count: navigation_source.map_or(parsed.navmeshes.len(), |source| source.mesh_count),
        polygon_count: navigation_source.map_or(0, |source| source.polygon_count),
        vertex_count: navigation_source.map_or(0, |source| source.vertex_count),
        door_count: navigation_source.map_or(0, |source| source.door_count),
        external_connection_count: navigation_source
            .map_or(0, |source| source.external_connection_count),
        mesh_merge_count: navigation_source.map_or(0, |source| source.mesh_merge_count),
        clearance_ready: navigation_clearance_ready,
        vertices: parsed
            .navmeshes
            .iter()
            .flat_map(|navmesh| {
                navmesh.vertices.iter().map(|vertex| {
                    policy
                        .plugin_to_bevy([
                            f64::from(vertex[0]),
                            f64::from(vertex[1]),
                            f64::from(vertex[2]),
                        ])
                        .map(|value| value as f32)
                })
            })
            .collect(),
        triangles: navigation_triangles(parsed),
        border_portals: navigation_border_portals(parsed, &policy, grid),
    });
    Some(ExteriorCellPackage {
        revision: EXTERIOR_CELL_PACKAGE_REVISION.into(),
        content_fingerprint: content_fingerprint.into(),
        cell_form_id: cell.form_id,
        worldspace_form_id,
        grid,
        origin: policy.grid_origin(grid).map(|value| value as f32),
        terrain,
        water,
        static_objects,
        dynamic_objects,
        distant_objects,
        actors,
        local_lights,
        navigation,
        environment,
        diagnostics,
    })
}

fn navigation_triangles(parsed: &ParsedPlugin) -> Vec<[u32; 3]> {
    let mut base = 0_u32;
    let mut triangles = Vec::new();
    for navmesh in &parsed.navmeshes {
        for triangle in &navmesh.triangles {
            let indices = triangle.vertex_indices.map(|index| index as i32);
            if indices.iter().all(|index| *index >= 0)
                && indices
                    .iter()
                    .all(|index| (*index as usize) < navmesh.vertices.len())
            {
                triangles.push([
                    base + indices[0] as u32,
                    base + indices[1] as u32,
                    base + indices[2] as u32,
                ]);
            }
        }
        base = base.saturating_add(navmesh.vertices.len() as u32);
    }
    triangles
}

fn navigation_border_portals(
    parsed: &ParsedPlugin,
    policy: &ExteriorCoordinatePolicy,
    grid: GridCoordinate,
) -> Vec<ExteriorBorderPortal> {
    let origin = policy.grid_origin(grid);
    let span = policy.cell_span_metres();
    let min_x = origin[0] as f32;
    let max_x = (origin[0] + span) as f32;
    let max_z = origin[2] as f32;
    let min_z = (origin[2] - span) as f32;
    let tolerance = 0.25_f32;
    let vertices = parsed
        .navmeshes
        .iter()
        .flat_map(|navmesh| {
            navmesh.vertices.iter().map(|vertex| {
                policy
                    .plugin_to_bevy([
                        f64::from(vertex[0]),
                        f64::from(vertex[1]),
                        f64::from(vertex[2]),
                    ])
                    .map(|value| value as f32)
            })
        })
        .collect::<Vec<_>>();
    let triangles = navigation_triangles(parsed);
    let mut portals = Vec::new();
    for triangle in triangles {
        for edge in 0..3 {
            let a = vertices[triangle[edge] as usize];
            let b = vertices[triangle[(edge + 1) % 3] as usize];
            let boundary = if (a[0] - min_x).abs() <= tolerance && (b[0] - min_x).abs() <= tolerance
            {
                Some(1)
            } else if (a[0] - max_x).abs() <= tolerance && (b[0] - max_x).abs() <= tolerance {
                Some(0)
            } else if (a[2] - min_z).abs() <= tolerance && (b[2] - min_z).abs() <= tolerance {
                Some(2)
            } else if (a[2] - max_z).abs() <= tolerance && (b[2] - max_z).abs() <= tolerance {
                Some(3)
            } else {
                None
            };
            if let Some(edge) = boundary {
                portals.push(ExteriorBorderPortal {
                    edge,
                    start: a,
                    end: b,
                    tolerance,
                });
            }
        }
    }
    portals.sort_by(|left, right| {
        left.edge
            .cmp(&right.edge)
            .then_with(|| compare_points(left.start, right.start))
            .then_with(|| compare_points(left.end, right.end))
    });
    portals.dedup();
    portals
}

fn compare_points(left: [f32; 3], right: [f32; 3]) -> std::cmp::Ordering {
    left[0]
        .total_cmp(&right[0])
        .then_with(|| left[1].total_cmp(&right[1]))
        .then_with(|| left[2].total_cmp(&right[2]))
}

fn actor_kind(kind: ReferenceKind) -> Option<ActorKind> {
    match kind {
        ReferenceKind::Npc => Some(ActorKind::Humanoid),
        ReferenceKind::Creature => Some(ActorKind::Creature),
        ReferenceKind::Object => None,
    }
}

pub(crate) fn apply_staged_assets(
    package: &mut ExteriorCellPackage,
    placements: &[PreparedPlacement],
    failed_assets: &std::collections::HashMap<String, String>,
) {
    for actor in package.actors.iter_mut() {
        let Some(placement) = placements
            .iter()
            .find(|placement| placement.reference_form_id == actor.reference_form_id)
        else {
            actor.asset_path = None;
            actor.physics_asset_path = None;
            actor.assembly = None;
            continue;
        };
        // Issue #305 review: defensive mirror of the object loop below. The
        // orchestrator already scrubs a failed placement's `asset_path`
        // before this runs on the exterior path, so `failed_assets` should
        // never actually contain this placement's path here -- but if that
        // ordering ever changes, an actor must not retain a path to a
        // missing GLB either.
        if let Some(asset_path) = placement.asset_path.as_deref()
            && let Some(reason) = failed_assets.get(asset_path)
        {
            actor.asset_path = None;
            actor.physics_asset_path = None;
            actor.assembly = None;
            package.diagnostics.push(ExteriorDiagnostic {
                code: "native_asset_failed".into(),
                form_id: Some(actor.reference_form_id),
                severity: "warning".into(),
                message: reason.clone(),
            });
            continue;
        }
        actor.asset_path = placement.asset_path.clone();
        actor.physics_asset_path = placement.physics_asset_path.clone();
        actor.assembly = match &placement.semantic {
            PreparedSemantic::Npc(prepared_actor) | PreparedSemantic::Creature(prepared_actor) => {
                prepared_actor.assembly.clone()
            }
            _ => None,
        };
        if actor.assembly.is_none()
            && let Some(reason) = placement.error.as_deref()
        {
            package.diagnostics.push(ExteriorDiagnostic {
                code: "exterior_actor_unavailable".into(),
                form_id: Some(actor.reference_form_id),
                severity: "warning".into(),
                message: reason.into(),
            });
        }
    }
    for object in package
        .static_objects
        .iter_mut()
        .chain(package.dynamic_objects.iter_mut())
        .chain(package.distant_objects.iter_mut())
    {
        let Some(placement) = placements
            .iter()
            .find(|placement| placement.reference_form_id == object.reference_form_id)
        else {
            // `base.model` is a Fallout source NIF path. It is useful while
            // assembling the package, but the viewer may only receive staged
            // runtime assets. References intentionally omitted from prepared
            // placements (editor markers and non-rendering effects) therefore
            // must not retain that source path.
            object.asset_path = None;
            object.physics_asset_path = None;
            continue;
        };
        if let Some(asset_path) = placement.asset_path.as_deref() {
            if let Some(reason) = failed_assets.get(asset_path) {
                object.asset_path = None;
                object.physics_asset_path = None;
                package.diagnostics.push(ExteriorDiagnostic {
                    code: "native_asset_failed".into(),
                    form_id: Some(object.reference_form_id),
                    severity: "warning".into(),
                    message: reason.clone(),
                });
            } else {
                object.asset_path = Some(asset_path.into());
                object.physics_asset_path = placement.physics_asset_path.clone();
            }
        } else {
            object.asset_path = None;
            object.physics_asset_path = None;
            if let Some(reason) = placement.error.as_deref() {
                package.diagnostics.push(ExteriorDiagnostic {
                    code: "exterior_asset_unavailable".into(),
                    form_id: Some(object.reference_form_id),
                    severity: "warning".into(),
                    message: reason.into(),
                });
            }
        }
    }
}

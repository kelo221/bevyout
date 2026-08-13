use crate::cli::ExteriorCatalogArgs;
use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;

use bevyout_core::manifest::exterior::{
    EXTERIOR_INDEX_REVISION, ExteriorCellIndexEntry, ExteriorCoordinatePolicy, ExteriorDiagnostic,
    ExteriorPersistentReference, ExteriorWorldspaceIndex, GridCoordinate,
    PreparedWeatherCatalogEntry,
};

use super::{DISTANT_REFERENCE_FLAG, PERSISTENT_REFERENCE_FLAG};
use crate::vsa::openmw_esm4::ParsedContentSet;
use crate::vsa::paths::placement_transform;

pub(crate) fn build_worldspace_indexes(
    parsed: &ParsedContentSet,
    content_fingerprint: &str,
) -> Vec<ExteriorWorldspaceIndex> {
    let policy = ExteriorCoordinatePolicy::default();
    let mut weather_profiles = parsed
        .weathers()
        .map(|(_, weather)| PreparedWeatherCatalogEntry {
            form_id: weather.form_id,
            editor_id: weather.editor_id.clone(),
            sky_upper: weather.sky_upper,
            sky_lower: weather.sky_lower,
            ambient: weather.ambient,
            sunlight: weather.sunlight,
        })
        .collect::<Vec<_>>();
    weather_profiles.sort_by_key(|profile| profile.form_id);
    let mut indexes = parsed
        .worldspaces()
        .map(|(_, worldspace)| ExteriorWorldspaceIndex {
            revision: EXTERIOR_INDEX_REVISION.into(),
            content_fingerprint: content_fingerprint.into(),
            worldspace_form_id: worldspace.form_id,
            editor_id: worldspace.editor_id.clone(),
            name: worldspace.name.clone(),
            climate_form_id: worldspace.climate_form_id,
            coordinate_policy: policy.clone(),
            weather_profiles: weather_profiles.clone(),
            cells: Vec::new(),
            persistent_references: Vec::new(),
            worldspace_lod: Vec::new(),
            diagnostics: Vec::new(),
        })
        .collect::<Vec<_>>();
    let mut by_worldspace = indexes
        .iter_mut()
        .map(|index| (index.worldspace_form_id, index))
        .collect::<BTreeMap<_, _>>();
    let references_by_cell = parsed.references_by_cell();
    let navmesh_counts = parsed.navmesh_counts_by_cell();
    for (cell_form_id, cell) in parsed.cells() {
        if cell.interior {
            continue;
        }
        let Some(worldspace_form_id) = cell.worldspace_form_id else {
            continue;
        };
        let Some(index) = by_worldspace.get_mut(&worldspace_form_id) else {
            continue;
        };
        let Some((x, y)) = cell.grid else {
            index.diagnostics.push(ExteriorDiagnostic {
                code: "missing_grid".into(),
                form_id: Some(*cell_form_id),
                severity: "warning".into(),
                message: "exterior CELL has no XCLC grid".into(),
            });
            continue;
        };
        let grid = GridCoordinate::new(x, y);
        let references = references_by_cell
            .get(cell_form_id)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let persistent = references
            .iter()
            .filter(|reference| reference.flags & PERSISTENT_REFERENCE_FLAG != 0)
            .map(|reference| {
                let (position, rotation_xyzw, scale) = placement_transform(reference);
                ExteriorPersistentReference {
                    reference_form_id: reference.form_id,
                    cell_form_id: *cell_form_id,
                    base_form_id: reference.base_form_id,
                    semantic: reference.kind.as_str().into(),
                    asset_path: parsed.base_model(reference.base_form_id),
                    position,
                    rotation_xyzw,
                    scale,
                    initially_enabled: reference.initially_enabled,
                    distant: reference.flags & DISTANT_REFERENCE_FLAG != 0,
                }
            })
            .collect::<Vec<_>>();
        index.persistent_references.extend(persistent);
        index.cells.push(ExteriorCellIndexEntry {
            cell_form_id: *cell_form_id,
            grid,
            origin: policy.grid_origin(grid).map(|value| value as f32),
            package_path: format!(
                "worldspaces/{worldspace_form_id:08x}/cells/{cell_form_id:08x}.ron"
            ),
            land_form_id: parsed.land_for_cell(*cell_form_id).map(|land| land.form_id),
            road_count: parsed.road_count_for_cell(*cell_form_id),
            navm_count: navmesh_counts
                .get(cell_form_id)
                .copied()
                .unwrap_or_default(),
            persistent_reference_count: references
                .iter()
                .filter(|reference| reference.flags & PERSISTENT_REFERENCE_FLAG != 0)
                .count(),
            distant_reference_count: references
                .iter()
                .filter(|reference| reference.flags & DISTANT_REFERENCE_FLAG != 0)
                .count(),
        });
    }
    for index in &mut indexes {
        index.sort_deterministically();
        let mut seen = BTreeMap::<u32, u32>::new();
        index.persistent_references.retain(|reference| {
            seen.insert(reference.reference_form_id, reference.cell_form_id)
                .is_none()
        });
        let mut grid_owner = BTreeMap::new();
        for cell in &index.cells {
            if let Some(previous) = grid_owner.insert(cell.grid, cell.cell_form_id) {
                index.diagnostics.push(ExteriorDiagnostic {
                    code: "duplicate_grid".into(),
                    form_id: Some(cell.cell_form_id),
                    severity: "error".into(),
                    message: format!("grid {:?} also belongs to {previous:08x}", cell.grid),
                });
            }
        }
        index.sort_deterministically();
    }
    indexes.sort_by_key(|index| index.worldspace_form_id);
    indexes
}

pub fn exterior_catalog(args: ExteriorCatalogArgs) -> Result<()> {
    let path = fs::canonicalize(&args.index)
        .with_context(|| format!("exterior index does not exist: {}", args.index.display()))?;
    let text = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let index: ExteriorWorldspaceIndex = ron::de::from_str(&text)
        .with_context(|| format!("parsing exterior index {}", path.display()))?;
    let mut output = String::new();
    output.push_str(&format!(
        "worldspace {:08x} cells={} persistent={} land={} navm={} roads={}\n",
        index.worldspace_form_id,
        index.cells.len(),
        index.persistent_references.len(),
        index
            .cells
            .iter()
            .filter(|cell| cell.land_form_id.is_some())
            .count(),
        index
            .cells
            .iter()
            .map(|cell| cell.navm_count)
            .sum::<usize>(),
        index
            .cells
            .iter()
            .map(|cell| cell.road_count)
            .sum::<usize>(),
    ));
    output.push_str(&format!(
        "worldspace lod assets={}\n",
        index.worldspace_lod.len()
    ));
    for cell in index.cells {
        output.push_str(&format!(
            "cell {:08x} grid={},{} origin={:.3},{:.3},{:.3} package={}\n",
            cell.cell_form_id,
            cell.grid.x,
            cell.grid.y,
            cell.origin[0],
            cell.origin[1],
            cell.origin[2],
            cell.package_path,
        ));
    }
    match std::io::stdout().write_all(output.as_bytes()) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::BrokenPipe => {}
        Err(error) => return Err(error.into()),
    }
    Ok(())
}

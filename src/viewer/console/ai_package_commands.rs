//! `showpackages` (issue #176): the visible surface for #175's prepared AI
//! package catalog. Per AGENTS.md's "the human must be able to see what a
//! wave shipped" rule -- #175 alone is only inspectable as a RON file.
//!
//! Both the per-cell actor catalog (`actors.ron`) and the content-set-wide
//! package catalog (`packages.ron`) are read from disk on demand when this
//! command runs, the same way `viewer::nav_overlay`'s `tnm` reads
//! `navgraph.ron` on demand rather than through a resource preloaded at
//! `view` startup -- unlike `PreparedItemCatalog`, neither prepared catalog
//! is cell-invariant content-set-wide-and-nothing-else: `actors.ron`
//! changes on every cell swap, so keeping it as a static `Resource` would
//! need its own reload-on-swap wiring this issue does not need to build.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::vsa::{
    ACTOR_CATALOG_REVISION, ActorBlueprint, ActorCatalogEntry, PACKAGE_CATALOG_REVISION,
    PreparedActorCatalog, PreparedPackageCatalog, PreparedPackageEntry,
};

use super::super::ai::lifecycle::{LifecyclePhase, PackageLifecycle};
use super::super::ai::resolution::{
    self, PackageLocation, PackageTarget, ResolutionContext, ResolvedReference,
};
use super::super::ai::selection::{
    self, CandidateOutcome, GameInstant, NoConditionFunctions, PackageCandidate, PackageSchedule,
};
use super::*;

pub(super) struct AiPackageCommandProvider;

impl ConsoleCommandProvider for AiPackageCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        registry.register(ConsoleCommand::new(
            "showpackages",
            "showpackages <actor-reference-or-base-formid> [game-hour]",
            "Print a prepared actor's AI packages in authored priority order, then the runtime layer: the selected package and why higher-priority ones were rejected (#193), the lifecycle phase (#194), and the resolved world position/entity for the selected package's location and target (#195). Optional game-hour (0-24) drives schedule selection; defaults to noon.",
            show_packages,
        ))
    }
}

fn package_type_label(package_type: u8) -> &'static str {
    match package_type {
        0 => "Find",
        1 => "Follow",
        2 => "Escort",
        3 => "Eat",
        4 => "Sleep",
        5 => "Wander",
        6 => "Travel",
        7 => "Accompany",
        8 => "UseItemAt",
        9 => "Ambush",
        10 => "FleeNotCombat",
        11 => "Unknown11",
        12 => "Sandbox",
        13 => "Patrol",
        14 => "Guard",
        15 => "Dialogue",
        16 => "UseWeapon",
        _ => "Unsupported",
    }
}

fn location_type_label(location_type: u32) -> &'static str {
    match location_type {
        0 => "NearReference",
        1 => "InCell",
        2 => "NearCurrentLocation",
        3 => "NearEditorLocation",
        4 => "ObjectId",
        5 => "ObjectType",
        6 => "NearLinkedReference",
        7 => "AtPackageLocation",
        _ => "Unknown",
    }
}

fn target_type_label(target_type: i32) -> &'static str {
    match target_type {
        0 => "SpecificReference",
        1 => "ObjectId",
        2 => "ObjectType",
        3 => "LinkedReference",
        _ => "Unknown",
    }
}

/// Reads and validates the active cell's per-cell actor catalog
/// (`actors.ron`), mirroring `viewer::app::run_view`'s item-catalog
/// validation (revision pinned to this build's compiled constant).
fn load_actor_catalog(world: &World) -> Result<PreparedActorCatalog, ConsoleError> {
    let manifest = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .ok_or_else(|| {
            ConsoleError::new("cell_unavailable", "no active cell manifest is loaded")
        })?;
    let relative = manifest.0.actor_catalog_path.as_deref().ok_or_else(|| {
        ConsoleError::new(
            "no_actor_catalog",
            "this cell has no prepared actor catalog; run `prepare` again",
        )
    })?;
    let path = PathBuf::from(&manifest.0.asset_root)
        .join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
    let text = std::fs::read_to_string(&path).map_err(|error| {
        ConsoleError::new(
            "catalog_unreadable",
            format!("reading actor catalog {}: {error}", path.display()),
        )
    })?;
    let catalog: PreparedActorCatalog = ron::de::from_str(&text).map_err(|error| {
        ConsoleError::new("catalog_invalid", format!("invalid actor catalog: {error}"))
    })?;
    if catalog.revision != ACTOR_CATALOG_REVISION {
        return Err(ConsoleError::new(
            "stale_catalog",
            format!(
                "actor catalog revision {} is stale, expected {ACTOR_CATALOG_REVISION}; run `prepare` again",
                catalog.revision
            ),
        ));
    }
    Ok(catalog)
}

/// Reads and validates the content-set-wide package catalog
/// (`catalogs/<fingerprint>/packages.ron`). Its path is fully deterministic
/// from `source_fingerprint` -- the same construction
/// `package_catalog::write_package_catalog` uses -- so there is no
/// manifest-carried pointer to follow (see that module's `write_package_catalog`
/// doc comment).
fn load_package_catalog(world: &World) -> Result<PreparedPackageCatalog, ConsoleError> {
    let manifest = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .ok_or_else(|| {
            ConsoleError::new("cell_unavailable", "no active cell manifest is loaded")
        })?;
    let relative = PathBuf::from("catalogs")
        .join(&manifest.0.source_fingerprint)
        .join("packages.ron");
    let path = PathBuf::from(&manifest.0.asset_root).join(&relative);
    let text = std::fs::read_to_string(&path).map_err(|error| {
        ConsoleError::new(
            "catalog_unreadable",
            format!("reading package catalog {}: {error}", path.display()),
        )
    })?;
    let catalog: PreparedPackageCatalog = ron::de::from_str(&text).map_err(|error| {
        ConsoleError::new(
            "catalog_invalid",
            format!("invalid package catalog: {error}"),
        )
    })?;
    if catalog.revision != PACKAGE_CATALOG_REVISION {
        return Err(ConsoleError::new(
            "stale_catalog",
            format!(
                "package catalog revision {} is stale, expected {PACKAGE_CATALOG_REVISION}; run `prepare` again",
                catalog.revision
            ),
        ));
    }
    Ok(catalog)
}

/// A selector resolved to something a prepared actor catalog can be looked
/// up by, independent of whether that catalog has actually been loaded --
/// this keeps argument-shape errors (`bad_arity`/`bad_type`/`not_actor`)
/// reachable even with no cell manifest loaded, matching `actorinspect`'s
/// own error priority.
enum ActorLookupKey {
    Reference(u32),
    Base(u32),
}

/// Resolves `selector` to a lookup key: a live placement reference first
/// (matches `actorinspect`'s own resolution, requiring an NPC/creature
/// semantic), falling back to a raw base FormID for an actor this cell's
/// catalog may know about but that is not currently a live placement (e.g.
/// a leveled-list candidate).
fn resolve_actor_lookup_key(world: &World, selector: &str) -> Result<ActorLookupKey, ConsoleError> {
    if let Ok(entity) = resolve_reference(world, selector) {
        let placement = world
            .get::<interaction::PlacementRoot>(entity)
            .ok_or_else(|| ConsoleError::new("not_actor", "reference has no placement root"))?
            .placement();
        if !matches!(
            placement.semantic,
            PreparedSemantic::Npc(_) | PreparedSemantic::Creature(_)
        ) {
            return Err(ConsoleError::new(
                "not_actor",
                "showpackages only accepts NPC or creature references",
            ));
        }
        return Ok(ActorLookupKey::Reference(placement.reference_form_id));
    }
    parse_item_form_id(selector)
        .map(ActorLookupKey::Base)
        .ok_or_else(|| {
            ConsoleError::new(
                "bad_type",
                "showpackages requires an actor reference or a 1-8 hex digit base FormID",
            )
        })
}

/// Looks up the resolved key in the loaded actor catalog, producing the
/// same deterministic `unknown_actor` error for either key shape.
fn find_actor_blueprint<'a>(
    catalog: &'a PreparedActorCatalog,
    key: &ActorLookupKey,
) -> Result<&'a ActorBlueprint, ConsoleError> {
    let (found, error_message) = match key {
        ActorLookupKey::Reference(reference_form_id) => (
            catalog.entries.iter().find_map(|entry| match entry {
                ActorCatalogEntry::Prepared(blueprint)
                    if blueprint.reference_form_id == *reference_form_id =>
                {
                    Some(blueprint.as_ref())
                }
                _ => None,
            }),
            format!(
                "no prepared package data for reference {reference_form_id:08x} (see actor catalog diagnostics)"
            ),
        ),
        ActorLookupKey::Base(base_form_id) => (
            catalog.entries.iter().find_map(|entry| match entry {
                ActorCatalogEntry::Prepared(blueprint)
                    if blueprint.base_form_id == *base_form_id =>
                {
                    Some(blueprint.as_ref())
                }
                _ => None,
            }),
            format!("no prepared actor with base FormID {base_form_id:08x} in this cell"),
        ),
    };
    found.ok_or_else(|| ConsoleError::new("unknown_actor", error_message))
}

pub(super) fn show_packages(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (selector, hour_override) = match invocation.args.as_slice() {
        [selector] => (selector, None),
        [selector, hour] => {
            let hour = hour
                .parse::<f32>()
                .ok()
                .filter(|h| (0.0..=24.0).contains(h));
            let hour = hour.ok_or_else(|| {
                ConsoleError::new("bad_type", "game-hour must be a number in 0..=24")
            })?;
            (selector, Some(hour))
        }
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "showpackages requires an actor reference or base FormID and an optional game-hour",
            ));
        }
    };
    let key = resolve_actor_lookup_key(world, selector)?;
    let actor_catalog = load_actor_catalog(world)?;
    let blueprint = find_actor_blueprint(&actor_catalog, &key)?;

    if blueprint.package_form_ids.is_empty() {
        let summary = format!(
            "showpackages {:08x}: actor {:08x} has no packages",
            blueprint.reference_form_id, blueprint.base_form_id
        );
        return Ok(ConsoleCommandResult::new(
            json!({
                "actor_reference_form_id": blueprint.reference_form_id,
                "actor_base_form_id": blueprint.base_form_id,
                "packages": [],
            }),
            vec![summary],
        ));
    }

    let reference_form_id = blueprint.reference_form_id;
    let base_form_id = blueprint.base_form_id;
    let package_form_ids = blueprint.package_form_ids.clone();

    let package_catalog = load_package_catalog(world)?;
    let total = package_form_ids.len();
    let mut entries = Vec::with_capacity(total);
    let mut lines = Vec::with_capacity(total + 1);
    lines.push(format!(
        "showpackages {reference_form_id:08x}: actor {base_form_id:08x} has {total} package(s) in priority order"
    ));
    // The found catalog entries, in authored priority order -- the candidate
    // list #193 selection runs over.
    let mut candidate_entries: Vec<&PreparedPackageEntry> = Vec::new();
    for (index, form_id) in package_form_ids.iter().copied().enumerate() {
        let found = package_catalog
            .packages
            .iter()
            .find(|entry| entry.form_id == form_id);
        match found {
            Some(entry) => {
                lines.push(format_package_line(index, total, entry));
                entries.push(package_json(index, entry));
                candidate_entries.push(entry);
            }
            None => {
                lines.push(format!(
                    "showpackages {reference_form_id:08x}: #{}/{total} {form_id:08x} not found in package catalog (see actor catalog diagnostics)",
                    index + 1
                ));
                entries.push(json!({
                    "index": index,
                    "form_id": form_id,
                    "found": false,
                }));
            }
        }
    }

    // #193: selection over the found candidates at the requested game hour.
    let now = GameInstant {
        hour: hour_override.unwrap_or(GameInstant::default().hour),
        ..GameInstant::default()
    };
    let candidates: Vec<PackageCandidate> = candidate_entries
        .iter()
        .map(|entry| to_candidate(entry))
        .collect();
    let report = selection::select_package(&candidates, now, &NoConditionFunctions);
    let (selection_json, selection_lines) = report_selection(reference_form_id, now, &report);
    lines.extend(selection_lines);

    // #194: lifecycle phase. A persisted checkpoint resumes deterministically;
    // otherwise the phase is derived from this selection pass.
    let checkpoint = persisted_checkpoint(world, reference_form_id);
    let from_checkpoint = checkpoint.is_some();
    let lifecycle = match checkpoint {
        Some(checkpoint) => PackageLifecycle::from_checkpoint(checkpoint),
        None => {
            let mut lifecycle = PackageLifecycle::new();
            lifecycle.on_select(report.selected);
            lifecycle
        }
    };
    let (lifecycle_json, lifecycle_line) =
        report_lifecycle(reference_form_id, &lifecycle, from_checkpoint);
    lines.push(lifecycle_line);

    // #195: resolve the selected package's location and target into world space.
    let resolution_json = match report.selected.and_then(|selected| {
        candidate_entries
            .iter()
            .find(|entry| entry.form_id == selected)
            .copied()
    }) {
        Some(entry) => {
            let context = build_resolution_context(world, reference_form_id);
            let (resolution_json, resolution_lines) =
                report_resolution(reference_form_id, entry, &context);
            lines.extend(resolution_lines);
            resolution_json
        }
        None => {
            lines.push(format!(
                "showpackages {reference_form_id:08x}: resolution skipped (no selected package)"
            ));
            Value::Null
        }
    };

    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": reference_form_id,
            "actor_base_form_id": base_form_id,
            "packages": entries,
            "selection": selection_json,
            "lifecycle": lifecycle_json,
            "resolution": resolution_json,
        }),
        lines,
    ))
}

/// Maps a prepared catalog entry onto the pure #193 selection input.
fn to_candidate(entry: &PreparedPackageEntry) -> PackageCandidate {
    PackageCandidate {
        form_id: entry.form_id,
        package_type: entry.package_type,
        schedule: entry.schedule.map(|schedule| PackageSchedule {
            month: schedule.month,
            day_of_week: schedule.day_of_week,
            date: schedule.date,
            time: schedule.time,
            duration: schedule.duration,
        }),
        conditions: entry.conditions.clone(),
    }
}

/// Renders the #193 selection report: the selected package and every
/// higher-priority package's concrete rejection reason.
fn report_selection(
    reference_form_id: u32,
    now: GameInstant,
    report: &selection::SelectionReport,
) -> (Value, Vec<String>) {
    let mut lines = Vec::with_capacity(report.evaluations.len() + 2);
    match report.selected {
        Some(selected) => lines.push(format!(
            "showpackages {reference_form_id:08x}: selected package {selected:08x} @ hour {:.1}",
            now.hour
        )),
        None => lines.push(format!(
            "showpackages {reference_form_id:08x}: no package selected @ hour {:.1} (schedule/condition gap)",
            now.hour
        )),
    }
    let mut evaluations_json = Vec::with_capacity(report.evaluations.len());
    for (index, evaluation) in report.evaluations.iter().enumerate() {
        let (status, reason) = match evaluation.outcome {
            CandidateOutcome::Selected => ("selected", None),
            CandidateOutcome::Rejected(reason) => ("rejected", Some(reason.label())),
        };
        lines.push(format!(
            "showpackages {reference_form_id:08x}:   #{} {:08x} {status}{}",
            index + 1,
            evaluation.form_id,
            reason.map_or_else(String::new, |reason| format!(": {reason}"))
        ));
        evaluations_json.push(json!({
            "index": index,
            "form_id": evaluation.form_id,
            "status": status,
            "reason": reason,
        }));
    }
    let counters = &report.counters;
    (
        json!({
            "selected_form_id": report.selected,
            "game_hour": now.hour,
            "evaluations": evaluations_json,
            "counters": {
                "total": counters.total,
                "unsupported_type": counters.unsupported_type,
                "out_of_schedule": counters.out_of_schedule,
                "conditions_false": counters.conditions_false,
                "conditions_unevaluable": counters.conditions_unevaluable,
                "schedule_gap": counters.schedule_gap,
            },
        }),
        lines,
    )
}

/// Renders the #194 lifecycle phase for the selected/persisted package.
fn report_lifecycle(
    reference_form_id: u32,
    lifecycle: &PackageLifecycle,
    from_checkpoint: bool,
) -> (Value, String) {
    let source = if from_checkpoint {
        "checkpoint"
    } else {
        "selection"
    };
    let phase = lifecycle.phase();
    let active = lifecycle.active_form_id();
    let line = format!(
        "showpackages {reference_form_id:08x}: lifecycle phase={} active={} step={} elapsed={:.1} ({source})",
        phase.label(),
        active.map_or_else(|| "none".to_string(), |form_id| format!("{form_id:08x}")),
        lifecycle
            .step()
            .map_or_else(|| "-".to_string(), |step| step.to_string()),
        lifecycle.elapsed_seconds().unwrap_or(0.0),
    );
    (
        json!({
            "phase": phase.label(),
            "active_form_id": active,
            "step": lifecycle.step(),
            "elapsed_seconds": lifecycle.elapsed_seconds(),
            "paused_form_id": lifecycle.paused_form_id(),
            "source": source,
            "resumable": matches!(
                phase,
                LifecyclePhase::Running | LifecyclePhase::Paused | LifecyclePhase::AwaitingRetry
            ),
        }),
        line,
    )
}

/// Renders the #195 resolved world position/entity for the selected package's
/// location and target, or the deterministic diagnostic for an unresolvable one.
fn report_resolution(
    reference_form_id: u32,
    entry: &PreparedPackageEntry,
    context: &ResolutionContext,
) -> (Value, Vec<String>) {
    let mut lines = Vec::with_capacity(2);
    let location_json = match entry.location {
        Some(location) => {
            let location = PackageLocation {
                location_type: location.location_type,
                form_id: location.form_id,
                raw_value: location.raw_value,
                radius: location.radius,
            };
            let result = resolution::resolve_location(&location, context);
            lines.push(resolution_line(
                reference_form_id,
                entry.form_id,
                "location",
                &result,
            ));
            resolution_json(&result)
        }
        None => {
            lines.push(format!(
                "showpackages {reference_form_id:08x}: {:08x} location none",
                entry.form_id
            ));
            Value::Null
        }
    };
    let target_json = match entry.target {
        Some(target) => {
            let target = PackageTarget {
                target_type: target.target_type,
                form_id: target.form_id,
                raw_value: target.raw_value,
                count_or_distance: target.count_or_distance,
            };
            let result = resolution::resolve_target(&target, context);
            lines.push(resolution_line(
                reference_form_id,
                entry.form_id,
                "target",
                &result,
            ));
            resolution_json(&result)
        }
        None => {
            lines.push(format!(
                "showpackages {reference_form_id:08x}: {:08x} target none",
                entry.form_id
            ));
            Value::Null
        }
    };
    (
        json!({
            "selected_form_id": entry.form_id,
            "location": location_json,
            "target": target_json,
        }),
        lines,
    )
}

fn resolution_line(
    reference_form_id: u32,
    package_form_id: u32,
    slot: &str,
    result: &Result<resolution::ResolvedPoint, resolution::ResolutionDiagnostic>,
) -> String {
    match result {
        Ok(point) => format!(
            "showpackages {reference_form_id:08x}: {package_form_id:08x} {slot} resolved via {} to ({:.2},{:.2},{:.2}) entity={} radius={:.1}",
            point.source.label(),
            point.position[0],
            point.position[1],
            point.position[2],
            point
                .entity
                .map_or_else(|| "none".to_string(), |entity| format!("{entity}")),
            point.radius,
        ),
        Err(diagnostic) => format!(
            "showpackages {reference_form_id:08x}: {package_form_id:08x} {slot} unresolved: {diagnostic}"
        ),
    }
}

fn resolution_json(
    result: &Result<resolution::ResolvedPoint, resolution::ResolutionDiagnostic>,
) -> Value {
    match result {
        Ok(point) => json!({
            "resolved": true,
            "source": point.source.label(),
            "position": point.position,
            "entity": point.entity,
            "radius": point.radius,
        }),
        Err(diagnostic) => json!({
            "resolved": false,
            "diagnostic": diagnostic.message,
        }),
    }
}

/// Reads the actor's persisted package checkpoint (`ActiveSaveState`), the
/// same one `setactorpackage` writes and save format v4 round-trips.
fn persisted_checkpoint(
    world: &World,
    reference_form_id: u32,
) -> Option<bevyout_core::actor_state::ActorPackageCheckpoint> {
    world
        .get_resource::<super::super::world::ActiveSaveState>()
        .and_then(|save| {
            save.0.cells.values().find_map(|cell| {
                cell.actors
                    .get(&reference_form_id)
                    .and_then(|state| state.package)
            })
        })
}

/// Snapshots the live placements into the #195 resolution context: every
/// visible reference's position and entity, indexed by base for nearest-of-base
/// resolution, plus the querying actor's own position.
fn build_resolution_context(world: &mut World, actor_reference_form_id: u32) -> ResolutionContext {
    let current_cell_form_id = world
        .get_resource::<crate::viewer::LoadedSceneManifest>()
        .map_or(0, |manifest| manifest.0.cell.form_id);
    let mut references = HashMap::new();
    let mut bases: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut actor_position = [0.0f32; 3];
    let mut query = world.query::<(Entity, &interaction::PlacementRoot)>();
    for (entity, root) in query.iter(world) {
        let placement = root.placement();
        let resolved = ResolvedReference {
            reference_form_id: placement.reference_form_id,
            base_form_id: placement.base_form_id,
            cell_form_id: current_cell_form_id,
            position: placement.translation,
            entity: Some(entity.to_bits()),
        };
        if placement.reference_form_id == actor_reference_form_id {
            actor_position = placement.translation;
        }
        bases
            .entry(placement.base_form_id)
            .or_default()
            .push(placement.reference_form_id);
        references.insert(placement.reference_form_id, resolved);
    }
    ResolutionContext {
        current_cell_form_id,
        actor_position,
        references,
        bases,
        ..ResolutionContext::default()
    }
}

fn package_json(index: usize, entry: &PreparedPackageEntry) -> Value {
    json!({
        "index": index,
        "found": true,
        "form_id": entry.form_id,
        "editor_id": entry.editor_id,
        "package_type": entry.package_type,
        "package_type_label": package_type_label(entry.package_type),
        "schedule": entry.schedule.map(|schedule| json!({
            "month": schedule.month,
            "day_of_week": schedule.day_of_week,
            "date": schedule.date,
            "time": schedule.time,
            "duration": schedule.duration,
        })),
        "location": entry.location.map(|location| json!({
            "location_type": location.location_type,
            "location_type_label": location_type_label(location.location_type),
            "form_id": location.form_id,
            "radius": location.radius,
        })),
        "target": entry.target.map(|target| json!({
            "target_type": target.target_type,
            "target_type_label": target_type_label(target.target_type),
            "form_id": target.form_id,
            "count_or_distance": target.count_or_distance,
        })),
        "condition_count": entry.conditions.len(),
    })
}

fn format_package_line(index: usize, total: usize, entry: &PreparedPackageEntry) -> String {
    let schedule = entry.schedule.map_or_else(
        || "none".to_string(),
        |schedule| {
            format!(
                "month:{} day:{} date:{} time:{} duration:{}",
                schedule.month,
                schedule.day_of_week,
                schedule.date,
                schedule.time,
                schedule.duration
            )
        },
    );
    let location = entry.location.map_or_else(
        || "none".to_string(),
        |location| {
            format!(
                "type:{}({}) target:{} radius:{}",
                location.location_type,
                location_type_label(location.location_type),
                location
                    .form_id
                    .map_or_else(|| "none".to_string(), |form_id| format!("{form_id:08x}")),
                location.radius
            )
        },
    );
    let target = entry.target.map_or_else(
        || "none".to_string(),
        |target| {
            format!(
                "type:{}({}) target:{} count_or_distance:{}",
                target.target_type,
                target_type_label(target.target_type),
                target
                    .form_id
                    .map_or_else(|| "none".to_string(), |form_id| format!("{form_id:08x}")),
                target.count_or_distance
            )
        },
    );
    format!(
        "showpackages: #{}/{total} {:08x} \"{}\" type={}({}) schedule={schedule} location={location} target={target} conditions={}",
        index + 1,
        entry.form_id,
        entry.editor_id.as_deref().unwrap_or("(none)"),
        entry.package_type,
        package_type_label(entry.package_type),
        entry.conditions.len(),
    )
}

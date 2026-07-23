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

use std::path::PathBuf;

use crate::vsa::{
    ACTOR_CATALOG_REVISION, ActorBlueprint, ActorCatalogEntry, PACKAGE_CATALOG_REVISION,
    PreparedActorCatalog, PreparedPackageCatalog, PreparedPackageEntry,
};

use super::super::ai::families::{self, PackageFamily, Waypoint};
use super::super::ai::family_runtime::{
    ActorPackageController, DEFAULT_ARRIVAL_TOLERANCE, PackageInteractionOccupancy,
    build_resolution_context,
};
use super::super::ai::lifecycle::{LifecyclePhase, PackageLifecycle};
use super::super::ai::resolution::{
    self, PackageLocation, PackageTarget, ResolutionContext, ResolvedPoint,
};
use super::super::ai::selection::{
    self, CandidateOutcome, GameInstant, NoConditionFunctions, PackageCandidate, PackageSchedule,
};
use super::super::nav::agent;
use super::*;

/// Follow band maximum (chase radius, metres) when the package's `PTDT`
/// distance is unset; the inner (hold) radius is derived from it.
const DEFAULT_FOLLOW_BAND_MAX: f32 = 4.0;
/// Inner (hold) radius as a fraction of the outer (chase) radius -- the
/// hysteresis dead band.
const FOLLOW_INNER_RATIO: f32 = 0.5;
/// A follow never holds closer than this, so a tiny authored distance still
/// leaves the follower a sane personal-space band.
const FOLLOW_MIN_INNER: f32 = 1.0;
/// Sandbox roam radius (metres) when the package's `PLDT` radius is unset.
const DEFAULT_WANDER_RADIUS: f32 = 8.0;
/// Seconds a sandbox actor idles at each reached roam point.
const DEFAULT_WANDER_IDLE_SECONDS: f32 = 3.0;

pub(super) struct AiPackageCommandProvider;

impl ConsoleCommandProvider for AiPackageCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        registry.register(ConsoleCommand::new(
            "showpackages",
            "showpackages <actor-reference-or-base-formid> [game-hour]",
            "Print a prepared actor's AI packages in authored priority order, then the runtime layer: the selected package and why higher-priority ones were rejected (#193), the lifecycle phase (#194), and the resolved world position/entity for the selected package's location and target (#195). Optional game-hour (0-24) drives schedule selection; defaults to noon.",
            show_packages,
        ))?;
        registry.register(ConsoleCommand::new(
            "runpackage",
            "runpackage <actor-reference> [status|stop]",
            "Drive the selected AI package family (#196 Travel/Patrol, #197 Idle/Eat/Sleep, #198 Follow/Sandbox) on a nav-bound actor (run `tna bind` first). With no action, selects+resolves the actor's package and starts the family running: Travel/Patrol walk the route, Idle/Eat/Sleep occupy and play at the resolved point, Follow trails a moving leader within a distance band, Sandbox roams within a radius. `status` prints the running family's step/target; `stop` releases occupancy and ends it.",
            run_package,
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

/// `runpackage <actor-reference> [status|stop]` (#196/#197): the visible
/// surface for the package families. Starting selects+resolves the actor's
/// package (like `showpackages`) and attaches the runtime driver
/// (`ai::family_runtime`) the per-frame system then advances.
pub(super) fn run_package(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (selector, action) = match invocation.args.as_slice() {
        [selector] => (selector, "start"),
        [selector, action] => (selector, action.as_str()),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "runpackage requires an actor reference and an optional status|stop action",
            ));
        }
    };
    let entity = resolve_reference(world, selector)?;
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
            "runpackage only accepts NPC or creature references",
        ));
    }
    let reference_form_id = placement.reference_form_id;
    match action {
        "start" => start_package(world, entity, reference_form_id, GameInstant::default()),
        "status" => status_package(world, entity, reference_form_id),
        "stop" => stop_package(world, entity, reference_form_id),
        other => Err(ConsoleError::new(
            "bad_type",
            format!("unknown runpackage action '{other}'; expected status or stop"),
        )),
    }
}

/// Selects+resolves the actor's active package and attaches a running family
/// driver. The actor must already own a nav agent (`tna bind`).
///
/// `pub(crate)` and parameterized by `instant` (issue #218): the console
/// `runpackage` command (which hardcodes `GameInstant::default()`, noon) and
/// the autonomous package driver both call this exact function, so there is
/// one select-resolve-start implementation, not two that could drift.
pub(crate) fn start_package(
    world: &mut World,
    entity: Entity,
    reference_form_id: u32,
    instant: GameInstant,
) -> Result<ConsoleCommandResult, ConsoleError> {
    if !agent::is_nav_bound(world, entity) {
        return Err(ConsoleError::new(
            "not_bound",
            format!(
                "actor {reference_form_id:08x} is not nav-bound; run `tna bind <index> {reference_form_id:08x}` first"
            ),
        ));
    }
    let actor_catalog = load_actor_catalog(world)?;
    let blueprint = find_actor_blueprint(
        &actor_catalog,
        &ActorLookupKey::Reference(reference_form_id),
    )?;
    if blueprint.package_form_ids.is_empty() {
        return Err(ConsoleError::new(
            "no_packages",
            format!("actor {reference_form_id:08x} has no packages"),
        ));
    }
    let package_form_ids = blueprint.package_form_ids.clone();
    let package_catalog = load_package_catalog(world)?;
    let candidate_entries: Vec<PreparedPackageEntry> = package_form_ids
        .iter()
        .filter_map(|form_id| {
            package_catalog
                .packages
                .iter()
                .find(|entry| entry.form_id == *form_id)
                .cloned()
        })
        .collect();
    let candidates: Vec<PackageCandidate> = candidate_entries.iter().map(to_candidate).collect();
    let report = selection::select_package(&candidates, instant, &NoConditionFunctions);
    let selected = report.selected.ok_or_else(|| {
        ConsoleError::new(
            "no_selection",
            format!(
                "actor {reference_form_id:08x} has no package selected at hour {:.1} (schedule/condition gap)",
                instant.hour
            ),
        )
    })?;
    let entry = candidate_entries
        .iter()
        .find(|entry| entry.form_id == selected)
        .expect("selected package is one of the candidates");
    let family = PackageFamily::from_package_type(entry.package_type).ok_or_else(|| {
        ConsoleError::new(
            "unsupported_family",
            format!(
                "selected package {selected:08x} type {}({}) is not driven by a family (Follow/Eat/Sleep/Idle/Travel/Sandbox/Patrol only)",
                entry.package_type,
                package_type_label(entry.package_type),
            ),
        )
    })?;

    // Set the nav-owned door policy for this family before dispatching: only
    // Sandbox/Wander refuses to open doors (#198). Nav reads this marker at its
    // door-open seam and never learns the family type itself.
    agent::set_agent_refuses_doors(world, entity, !family.opens_doors());

    let context = build_resolution_context(world, reference_form_id);
    // The dynamic families (#198) resolve differently: Follow needs the live
    // leader entity to trail; Sandbox needs a roam centre + radius.
    if family == PackageFamily::Follow {
        return start_follow_package(world, entity, reference_form_id, selected, entry, &context);
    }
    if family == PackageFamily::Sandbox {
        return start_wander_package(world, entity, reference_form_id, selected, entry, &context);
    }
    // Eat/Sleep pick the nearest free interaction point among the resolved
    // furniture candidates; Patrol (#213) walks the actor's linked-reference
    // marker chain into an ordered waypoint list; the others drive to the
    // single resolved location (falling back to the target slot).
    let waypoints = if matches!(family, PackageFamily::Eat | PackageFamily::Sleep) {
        let candidates = resolve_interaction_candidates(entry, &context, selected);
        if candidates.is_empty() {
            return Err(ConsoleError::new(
                "unresolved_point",
                format!("package {selected:08x} has no resolvable interaction point"),
            ));
        }
        let occupied = &world.resource::<PackageInteractionOccupancy>().0;
        let chosen = families::select_interaction_point(
            &candidates,
            context.actor_position,
            occupied,
        )
        .ok_or_else(|| {
            ConsoleError::new(
                "furniture_busy",
                format!("every resolved interaction point for package {selected:08x} is occupied"),
            )
        })?;
        vec![candidates[chosen]]
    } else if family == PackageFamily::Patrol {
        let start = context.linked_reference.ok_or_else(|| {
            ConsoleError::new(
                "unresolved_point",
                "near-linked-reference location has no linked reference".to_string(),
            )
        })?;
        let markers = resolution::linked_reference_chain(&context, start);
        if markers.is_empty() {
            return Err(ConsoleError::new(
                "unresolved_point",
                format!(
                    "package {selected:08x}'s linked-reference chain from {start:08x} resolved no markers"
                ),
            ));
        }
        markers
            .into_iter()
            .map(|marker| Waypoint::at(marker.position))
            .collect()
    } else {
        let resolved = resolve_family_point(entry, &context, false)?;
        vec![Waypoint::at(resolved.position)]
    };
    let target = waypoints[0].position;

    let controller = ActorPackageController::start(
        reference_form_id,
        selected,
        family,
        waypoints,
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    world.entity_mut(entity).insert(controller);

    let line = format!(
        "runpackage {reference_form_id:08x}: started {} package {selected:08x} -> target ({:.2},{:.2},{:.2})",
        family.label(),
        target[0],
        target[1],
        target[2],
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": reference_form_id,
            "family": family.label(),
            "selected_form_id": selected,
            "target": target,
        }),
        vec![line],
    ))
}

/// Starts a Follow (#198) package: resolves the leader through the target
/// slot, then trails its live entity within a hysteresis distance band.
fn start_follow_package(
    world: &mut World,
    entity: Entity,
    reference_form_id: u32,
    selected: u32,
    entry: &PreparedPackageEntry,
    context: &ResolutionContext,
) -> Result<ConsoleCommandResult, ConsoleError> {
    // Prefer the target slot -- a follow's leader is authored there (specific
    // reference or linked/follow target).
    let point = resolve_family_point(entry, context, true)?;
    let leader_bits = point.entity.ok_or_else(|| {
        ConsoleError::new(
            "no_follow_target",
            format!("follow package {selected:08x} resolved no live leader entity to follow"),
        )
    })?;
    let leader = Entity::from_bits(leader_bits);
    let outer = if point.radius > 0.0 {
        point.radius
    } else {
        DEFAULT_FOLLOW_BAND_MAX
    };
    let inner = (outer * FOLLOW_INNER_RATIO)
        .max(FOLLOW_MIN_INNER)
        .min(outer);
    let controller = ActorPackageController::start_follow(
        reference_form_id,
        selected,
        leader,
        inner,
        outer,
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    world.entity_mut(entity).insert(controller);
    let line = format!(
        "runpackage {reference_form_id:08x}: started follow package {selected:08x} -> leader {leader_bits:016x} band [{inner:.1},{outer:.1}]",
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": reference_form_id,
            "family": "follow",
            "selected_form_id": selected,
            "leader_entity": leader_bits,
            "band_min": inner,
            "band_max": outer,
        }),
        vec![line],
    ))
}

/// Starts a Sandbox/Wander (#198) package: resolves the roam centre through the
/// location slot and roams deterministically within its radius.
fn start_wander_package(
    world: &mut World,
    entity: Entity,
    reference_form_id: u32,
    selected: u32,
    entry: &PreparedPackageEntry,
    context: &ResolutionContext,
) -> Result<ConsoleCommandResult, ConsoleError> {
    // Prefer the location slot -- a sandbox roams around its authored anchor.
    let point = resolve_family_point(entry, context, false)?;
    let radius = if point.radius > 0.0 {
        point.radius
    } else {
        DEFAULT_WANDER_RADIUS
    };
    let controller = ActorPackageController::start_wander(
        reference_form_id,
        selected,
        point.position,
        radius,
        DEFAULT_WANDER_IDLE_SECONDS,
        // A per-actor seed makes the roam deterministic and reproducible.
        u64::from(reference_form_id),
        DEFAULT_ARRIVAL_TOLERANCE,
    );
    world.entity_mut(entity).insert(controller);
    let line = format!(
        "runpackage {reference_form_id:08x}: started sandbox package {selected:08x} -> roam center ({:.2},{:.2},{:.2}) radius {radius:.1}",
        point.position[0], point.position[1], point.position[2],
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": reference_form_id,
            "family": "sandbox",
            "selected_form_id": selected,
            "roam_center": point.position,
            "radius": radius,
        }),
        vec![line],
    ))
}

/// Prints the running family's step and target for an actor.
fn status_package(
    world: &mut World,
    entity: Entity,
    reference_form_id: u32,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let controller = world.get::<ActorPackageController>(entity).ok_or_else(|| {
        ConsoleError::new(
            "no_running_package",
            format!("actor {reference_form_id:08x} has no running package; start one with `runpackage {reference_form_id:08x}`"),
        )
    })?;
    let target = controller.driver.current_target();
    let target_desc = target.map_or_else(
        || "none".to_string(),
        |point| format!("({:.2},{:.2},{:.2})", point[0], point[1], point[2]),
    );
    let occupied = controller
        .driver
        .occupied_point()
        .map_or_else(|| "none".to_string(), |point| format!("{point:08x}"));
    // A follow that gave up on a door it could not open names that door here
    // (#198/#185), so the human sees *why* it stopped following.
    let blocked = controller
        .driver
        .blocked_door()
        .map_or_else(|| "none".to_string(), |door| format!("{door:08x}"));
    let line = format!(
        "runpackage {reference_form_id:08x}: {} package {:08x} phase={} step={} marker={}/{} target={target_desc} occupied={occupied} blocked_door={blocked}",
        controller.driver.family().label(),
        controller.selected_form_id,
        controller.lifecycle.phase().label(),
        controller.driver.step_label(),
        controller.driver.marker_index(),
        controller.driver.marker_count(),
    );
    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": reference_form_id,
            "family": controller.driver.family().label(),
            "selected_form_id": controller.selected_form_id,
            "phase": controller.lifecycle.phase().label(),
            "step": controller.driver.step_label(),
            "marker_index": controller.driver.marker_index(),
            "marker_count": controller.driver.marker_count(),
            "target": target,
            "occupied_point": controller.driver.occupied_point(),
            "blocked_door": controller.driver.blocked_door(),
        }),
        vec![line],
    ))
}

/// Stops a running package: releases occupancy, clears the nav route, and
/// removes the controller.
fn stop_package(
    world: &mut World,
    entity: Entity,
    reference_form_id: u32,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let mut controller = world
        .get_mut::<ActorPackageController>(entity)
        .ok_or_else(|| {
            ConsoleError::new(
                "no_running_package",
                format!("actor {reference_form_id:08x} has no running package"),
            )
        })?;
    let family = controller.driver.family().label();
    let released = controller.driver.release();
    if let Some(point) = released {
        world
            .resource_mut::<PackageInteractionOccupancy>()
            .0
            .remove(&point);
    }
    agent::clear_agent_target(world, entity);
    // Clear the nav-owned door policy and hand facing back to navigation: the
    // package that set them is ending.
    agent::set_agent_refuses_doors(world, entity, false);
    agent::set_facing_authority(world, entity, false);
    world.entity_mut(entity).remove::<ActorPackageController>();
    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": reference_form_id,
            "family": family,
            "released_point": released,
        }),
        vec![format!(
            "runpackage {reference_form_id:08x}: stopped {family} package (released={})",
            released.map_or_else(|| "none".to_string(), |point| format!("{point:08x}")),
        )],
    ))
}

/// Builds the `ai::resolution` mirror types (`PackageLocation`/
/// `PackageTarget`) from a prepared package entry's raw `PLDT`/`PTDT`
/// slots, for `resolution::resolve_family_point` (issue #218: that function
/// moved to `ai::resolution` and no longer knows about `PreparedPackageEntry`
/// -- keeping it decoupled from `vsa` types is what lets it compile verbatim
/// into `tests/features.rs`).
fn location_target_mirrors(
    entry: &PreparedPackageEntry,
) -> (Option<PackageLocation>, Option<PackageTarget>) {
    let location = entry.location.map(|location| PackageLocation {
        location_type: location.location_type,
        form_id: location.form_id,
        raw_value: location.raw_value,
        radius: location.radius,
    });
    let target = entry.target.map(|target| PackageTarget {
        target_type: target.target_type,
        form_id: target.form_id,
        raw_value: target.raw_value,
        count_or_distance: target.count_or_distance,
    });
    (location, target)
}

/// Resolves a package's location/target into a world point for a family,
/// preferring one slot but falling back to the other. A thin wrapper over
/// `resolution::resolve_family_point` (issue #218: moved there so `runpackage`
/// and the autonomous package driver share one implementation), converting
/// its pure `ResolutionDiagnostic` into this layer's `ConsoleError`.
fn resolve_family_point(
    entry: &PreparedPackageEntry,
    context: &ResolutionContext,
    prefer_target: bool,
) -> Result<ResolvedPoint, ConsoleError> {
    let (location, target) = location_target_mirrors(entry);
    resolution::resolve_family_point(location, target, context, prefer_target)
        .map_err(|diagnostic| ConsoleError::new("unresolved_point", diagnostic.message))
}

/// Resolves an eat/sleep package's furniture candidates: the target slot (the
/// furniture reference) and the location slot, each carried as a `Waypoint` with
/// an interaction-point occupancy key (the resolved entity, or the package
/// FormID as a fallback). `families::select_interaction_point` then picks the
/// nearest free one.
fn resolve_interaction_candidates(
    entry: &PreparedPackageEntry,
    context: &ResolutionContext,
    selected: u32,
) -> Vec<Waypoint> {
    let mut candidates = Vec::new();
    let mut push = |point: ResolvedPoint| {
        let interaction_point = point.entity.map_or(selected, |entity| entity as u32);
        candidates.push(Waypoint {
            position: point.position,
            wait_seconds: 0.0,
            orientation_yaw: None,
            interaction_point: Some(interaction_point),
        });
    };
    if let Some(target) = entry.target
        && let Ok(point) = resolution::resolve_target(
            &PackageTarget {
                target_type: target.target_type,
                form_id: target.form_id,
                raw_value: target.raw_value,
                count_or_distance: target.count_or_distance,
            },
            context,
        )
    {
        push(point);
    }
    if let Some(location) = entry.location
        && let Ok(point) = resolution::resolve_location(
            &PackageLocation {
                location_type: location.location_type,
                form_id: location.form_id,
                raw_value: location.raw_value,
                radius: location.radius,
            },
            context,
        )
    {
        push(point);
    }
    candidates
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

// `build_resolution_context` moved to `ai::family_runtime` (issue #218): it
// only reads `World` state, so both `runpackage` (imported above) and the
// autonomous package driver share one implementation.

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

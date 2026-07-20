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

use super::*;

pub(super) struct AiPackageCommandProvider;

impl ConsoleCommandProvider for AiPackageCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        registry.register(ConsoleCommand::new(
            "showpackages",
            "showpackages <actor-reference-or-base-formid>",
            "Print a prepared actor's resolved AI packages in authored priority order: FormID, EditorID, type, schedule, location, target, and condition count.",
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
    let [selector] = invocation.args.as_slice() else {
        return Err(ConsoleError::new(
            "bad_arity",
            "showpackages requires exactly one actor reference or base FormID",
        ));
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

    let package_catalog = load_package_catalog(world)?;
    let total = blueprint.package_form_ids.len();
    let mut entries = Vec::with_capacity(total);
    let mut lines = Vec::with_capacity(total + 1);
    lines.push(format!(
        "showpackages {:08x}: actor {:08x} has {total} package(s) in priority order",
        blueprint.reference_form_id, blueprint.base_form_id
    ));
    for (index, form_id) in blueprint.package_form_ids.iter().copied().enumerate() {
        let found = package_catalog
            .packages
            .iter()
            .find(|entry| entry.form_id == form_id);
        match found {
            Some(entry) => {
                lines.push(format_package_line(index, total, entry));
                entries.push(package_json(index, entry));
            }
            None => {
                lines.push(format!(
                    "showpackages {:08x}: #{}/{total} {form_id:08x} not found in package catalog (see actor catalog diagnostics)",
                    blueprint.reference_form_id,
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
    Ok(ConsoleCommandResult::new(
        json!({
            "actor_reference_form_id": blueprint.reference_form_id,
            "actor_base_form_id": blueprint.base_form_id,
            "packages": entries,
        }),
        lines,
    ))
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

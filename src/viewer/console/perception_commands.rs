//! `perception` (issue #116): the visible surface for deterministic
//! disposition/hostility and target awareness. For a given observer actor and
//! a target (the player by default, or another actor), it reports the resolved
//! disposition value, the hostility verdict *and which rule decided it*, and
//! the observer's live perception/awareness state (distance, line of sight,
//! detection confidence, and the acquired target).
//!
//! Relationship data (factions, `AIDT` aggression, base disposition, and the
//! `FACT` relation table) is read from the per-cell actor catalog on demand,
//! the same way `showpackages` reads `actors.ron`. Awareness is read from the
//! authoritative `ActorAwareness` component maintained by the perception
//! system -- there is no second target authority.

use std::path::PathBuf;

use bevyout_core::disposition::{
    Aggression, DispositionActor, DispositionResult, DispositionTarget, DispositionThresholds,
    FactionMembership, resolve_disposition,
};
use bevyout_core::faction::FactionRelationTable;

use crate::vsa::{ACTOR_CATALOG_REVISION, ActorBlueprint, ActorCatalogEntry, PreparedActorCatalog};

use super::super::perception::ActorAwareness;
use super::*;

pub(super) struct PerceptionCommandProvider;

impl ConsoleCommandProvider for PerceptionCommandProvider {
    fn register_commands(&self, registry: &mut ConsoleRegistry) -> Result<(), ConsoleError> {
        registry.register(ConsoleCommand::new(
            "perception",
            "perception <observer-reference-or-formid> [<target-reference-or-formid>|player]",
            "Report an actor's disposition value, hostility verdict and the deciding rule toward a target (player by default), plus its live awareness: distance, line of sight, confidence, and acquired target.",
            perception,
        ))
    }
}

/// Reads and validates the active cell's per-cell actor catalog, mirroring
/// `showpackages`/`run_view` (revision pinned to this build's constant).
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

/// Resolves a selector to a live actor reference FormID, or a raw base/
/// reference FormID otherwise (matching `showpackages`).
fn resolve_actor_form_id(world: &World, selector: &str) -> Result<u32, ConsoleError> {
    if let Ok(entity) = resolve_reference(world, selector)
        && let Some(runtime) = world.get::<actor::ActorRuntime>(entity)
    {
        return Ok(runtime.reference_form_id);
    }
    parse_item_form_id(selector).ok_or_else(|| {
        ConsoleError::new(
            "bad_type",
            "perception requires an actor reference or a 1-8 hex digit FormID",
        )
    })
}

fn find_blueprint(catalog: &PreparedActorCatalog, form_id: u32) -> Option<&ActorBlueprint> {
    catalog.entries.iter().find_map(|entry| match entry {
        ActorCatalogEntry::Prepared(blueprint)
            if blueprint.reference_form_id == form_id || blueprint.base_form_id == form_id =>
        {
            Some(blueprint.as_ref())
        }
        _ => None,
    })
}

fn disposition_actor(blueprint: &ActorBlueprint) -> DispositionActor {
    DispositionActor {
        factions: blueprint
            .factions
            .iter()
            .map(|membership| FactionMembership {
                faction_form_id: membership.faction_form_id,
                rank: membership.rank,
            })
            .collect(),
        base_disposition: i32::from(blueprint.disposition_base),
        aggression: blueprint
            .ai_data
            .map(|data| Aggression::from_raw(data.aggression))
            .unwrap_or_default(),
        race_disposition_adjust: 0,
    }
}

fn disposition_target(blueprint: &ActorBlueprint, is_self: bool) -> DispositionTarget {
    DispositionTarget {
        factions: blueprint
            .factions
            .iter()
            .map(|membership| FactionMembership {
                faction_form_id: membership.faction_form_id,
                rank: membership.rank,
            })
            .collect(),
        is_self,
    }
}

/// The observer's live awareness, if the actor is currently spawned.
fn observer_awareness(world: &World, reference_form_id: u32) -> Option<ActorAwareness> {
    world.iter_entities().find_map(|entity| {
        let runtime = entity.get::<actor::ActorRuntime>()?;
        (runtime.reference_form_id == reference_form_id)
            .then(|| entity.get::<ActorAwareness>().cloned())
            .flatten()
    })
}

fn perception(
    world: &mut World,
    invocation: &ConsoleInvocation,
) -> Result<ConsoleCommandResult, ConsoleError> {
    let (observer_selector, target_selector) = match invocation.args.as_slice() {
        [observer] => (observer.as_str(), "player"),
        [observer, target] => (observer.as_str(), target.as_str()),
        _ => {
            return Err(ConsoleError::new(
                "bad_arity",
                "perception requires an observer and an optional target (player by default)",
            ));
        }
    };

    let observer_form_id = resolve_actor_form_id(world, observer_selector)?;
    let target_is_player = target_selector.eq_ignore_ascii_case("player");
    let target_form_id = if target_is_player {
        None
    } else {
        Some(resolve_actor_form_id(world, target_selector)?)
    };

    let catalog = load_actor_catalog(world)?;
    let observer = find_blueprint(&catalog, observer_form_id).ok_or_else(|| {
        ConsoleError::new(
            "unknown_actor",
            format!("no prepared actor for observer {observer_form_id:08x} in this cell"),
        )
    })?;

    let table: &FactionRelationTable = &catalog.faction_table;
    let thresholds = DispositionThresholds::default();

    let (target, target_label) = if let Some(target_form_id) = target_form_id {
        let target = find_blueprint(&catalog, target_form_id).ok_or_else(|| {
            ConsoleError::new(
                "unknown_actor",
                format!("no prepared actor for target {target_form_id:08x} in this cell"),
            )
        })?;
        let is_self = target.reference_form_id == observer.reference_form_id;
        (
            disposition_target(target, is_self),
            format!("{target_form_id:08x}"),
        )
    } else {
        // The player carries no prepared faction membership in this viewer.
        (DispositionTarget::default(), "player".to_string())
    };

    let result: DispositionResult =
        resolve_disposition(&disposition_actor(observer), &target, table, &thresholds);

    let awareness = observer_awareness(world, observer.reference_form_id);
    let last_player = awareness.as_ref().and_then(|a| a.last_player);
    let state = awareness.as_ref().map(|a| a.state);

    let summary = format!(
        "perception {:08x} -> {target_label}: disposition={} hostility={} rule={} | awareness: {}",
        observer.reference_form_id,
        result.disposition,
        result.hostility.label(),
        result.decided_by.label(),
        match (&state, &last_player) {
            (Some(state), Some(inputs)) => format!(
                "distance={:.2} los={} confidence={:.2} acquired={}",
                inputs.distance,
                inputs.has_line_of_sight,
                state.confidence,
                state.target().map_or_else(
                    || "none".to_string(),
                    |t| format!("{}:{:08x}", t.class.label(), t.form_id)
                )
            ),
            (Some(state), None) =>
                format!("no target geometry yet, confidence={:.2}", state.confidence),
            _ => "actor not spawned (no live awareness)".to_string(),
        }
    );

    let mut lines = vec![summary];
    for diagnostic in &result.diagnostics {
        lines.push(format!("perception diagnostic: {diagnostic}"));
    }

    Ok(ConsoleCommandResult::new(
        json!({
            "observer_reference_form_id": observer.reference_form_id,
            "observer_base_form_id": observer.base_form_id,
            "target": target_label,
            "disposition": result.disposition,
            "hostility": result.hostility.label(),
            "decided_by": result.decided_by.label(),
            "diagnostics": result.diagnostics,
            "aggression": disposition_actor(observer).aggression.label(),
            "awareness": state.map(|state| json!({
                "confidence": state.confidence,
                "acquired": state.target().map(|t| json!({
                    "class": t.class.label(),
                    "form_id": t.form_id,
                })),
                "time_since_seen": state.time_since_seen,
                "distance": last_player.map(|inputs| inputs.distance),
                "line_of_sight": last_player.map(|inputs| inputs.has_line_of_sight),
            })),
        }),
        lines,
    ))
}

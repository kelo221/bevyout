//! Thin viewer adapter: classify ownership, mark stolen, report one crime.

use bevy::prelude::*;
use bevyout_core::crime::{CrimeEvent, CrimeKind, CrimeLedger, WitnessEvidence, resolve_crime};
use bevyout_core::faction::FactionRelationTable;
use bevyout_core::item_transaction::{HolderId, ItemInstanceId};
use bevyout_core::items::{OwnershipClaim, TakeClassification, TakerFactions, classify_ownership};
use bevyout_core::perception::{TargetClass, TargetId};

use super::actor::ActorRuntime;
use super::actor_state::ActorDefinitionCatalogs;
use super::interaction::CanonicalItemLedger;
use super::perception::ActorAwareness;
use super::stats::PlayerProgression;

#[must_use]
pub(crate) fn classify_claim(
    claim: OwnershipClaim,
    catalogs: Option<&ActorDefinitionCatalogs>,
) -> TakeClassification {
    let mut table = FactionRelationTable::default();
    if let Some(catalogs) = catalogs {
        for (_, catalog) in catalogs.catalogs() {
            for (form_id, faction) in &catalog.faction_table.factions {
                if !table.is_known(*form_id) {
                    table.insert(faction.clone());
                }
            }
        }
    }
    classify_ownership(claim, &TakerFactions::default(), &table)
}

pub(crate) fn report_player_theft(
    progression: &mut PlayerProgression,
    canonical: &mut CanonicalItemLedger,
    item_id: Option<ItemInstanceId>,
    owner: u32,
    witnesses: &[WitnessEvidence],
) {
    let mut item = item_id.and_then(|id| {
        canonical
            .ledger
            .holders()
            .get(&HolderId::Player)
            .and_then(|holder| holder.find(id))
            .map(|item| item.state.clone())
    });
    let mut sorted = witnesses.to_vec();
    let id = CrimeLedger::allocate(TargetId::player(), &mut progression.crime);
    let report = resolve_crime(
        &mut progression.crime,
        CrimeEvent {
            id,
            kind: CrimeKind::Theft,
            victim: TargetId {
                class: TargetClass::Actor,
                form_id: owner,
            },
            item_id,
            owner_form_id: Some(owner),
        },
        &mut sorted,
        item.as_mut(),
    );
    if let (Some(id), Some(state)) = (item_id, item)
        && let Some(live) = canonical
            .ledger
            .holders_mut()
            .get_mut(&HolderId::Player)
            .and_then(|holder| holder.find_mut(id))
    {
        live.state = state;
    }
    if let Some(report) = report {
        info!(
            "crime sequence={} kind={} bounty={} karma={} witnesses={}",
            report.id.sequence,
            report.kind.label(),
            report.bounty,
            report.karma_delta,
            report.witnesses.len()
        );
    }
}

pub(crate) fn maybe_report_theft(
    claim: OwnershipClaim,
    catalogs: Option<&ActorDefinitionCatalogs>,
    progression: Option<&mut PlayerProgression>,
    canonical: &mut CanonicalItemLedger,
    item_id: Option<ItemInstanceId>,
    steal_form_id: u32,
    witnesses: &[WitnessEvidence],
) {
    if let TakeClassification::Steal { owner_form_id } = classify_claim(claim, catalogs) {
        info!("steal {steal_form_id:08x} owner {owner_form_id:08x}");
        if let Some(progression) = progression {
            report_player_theft(progression, canonical, item_id, owner_form_id, witnesses);
        }
    }
}

#[must_use]
pub(crate) fn latest_player_item(
    canonical: &CanonicalItemLedger,
    base_form_id: u32,
) -> Option<ItemInstanceId> {
    canonical
        .ledger
        .holders()
        .get(&HolderId::Player)
        .and_then(|holder| {
            holder
                .items
                .iter()
                .filter(|item| item.base_form_id == base_form_id)
                .map(|item| item.id)
                .max()
        })
}

pub(crate) fn report_theft_in_world(
    world: &mut World,
    claim: OwnershipClaim,
    item_id: Option<ItemInstanceId>,
    steal_form_id: u32,
) {
    let witnesses = {
        let mut query = world.query::<(&ActorRuntime, &ActorAwareness)>();
        live_witnesses(query.iter(world))
    };
    let TakeClassification::Steal { owner_form_id } =
        classify_claim(claim, world.get_resource::<ActorDefinitionCatalogs>())
    else {
        return;
    };
    info!("steal {steal_form_id:08x} owner {owner_form_id:08x}");
    if world.get_resource::<PlayerProgression>().is_none()
        || world.get_resource::<CanonicalItemLedger>().is_none()
    {
        return;
    }
    world.resource_scope(|world, mut progression: Mut<PlayerProgression>| {
        let mut canonical = world.resource_mut::<CanonicalItemLedger>();
        report_player_theft(
            &mut progression,
            &mut canonical,
            item_id,
            owner_form_id,
            &witnesses,
        );
    });
}

#[must_use]
pub(crate) fn live_witnesses<'a>(
    actors: impl Iterator<Item = (&'a ActorRuntime, &'a ActorAwareness)>,
) -> Vec<WitnessEvidence> {
    actors
        .map(|(runtime, awareness)| {
            let inputs = awareness.last_player;
            WitnessEvidence {
                witness: TargetId {
                    class: TargetClass::Actor,
                    form_id: runtime.reference_form_id,
                },
                has_line_of_sight: inputs.is_some_and(|inputs| inputs.has_line_of_sight),
                distance_mm: inputs
                    .map(|inputs| (inputs.distance.max(0.0) * 1_000.0).round() as u32)
                    .unwrap_or(u32::MAX),
                alive: true,
                enabled: true,
                hostile_to_victim: false,
            }
        })
        .collect()
}

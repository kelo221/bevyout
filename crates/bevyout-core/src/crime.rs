//! Crime reports, bounty, and Karma.
//!
//! Classification happens once. Witnesses are sorted by FormID. Multiple
//! eligible witnesses are listed on the receipt but never multiply bounty.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::item_transaction::{ItemInstanceId, ItemState};
use crate::perception::TargetId;

/// Alarm range matches the default perception sight range (40 m).
pub const CRIME_ALARM_RANGE_MM: u32 = 40_000;
pub const THEFT_BOUNTY: u32 = 40;
pub const ASSAULT_BOUNTY: u32 = 40;
pub const MURDER_BOUNTY: u32 = 1_000;
pub const THEFT_KARMA: i32 = -5;
pub const ASSAULT_KARMA: i32 = -10;
pub const MURDER_KARMA: i32 = -100;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrimeId {
    pub actor: TargetId,
    pub sequence: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CrimeKind {
    Theft,
    Assault,
    Murder,
}

impl CrimeKind {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Theft => "theft",
            Self::Assault => "assault",
            Self::Murder => "murder",
        }
    }

    #[must_use]
    pub const fn bounty(self) -> u32 {
        match self {
            Self::Theft => THEFT_BOUNTY,
            Self::Assault => ASSAULT_BOUNTY,
            Self::Murder => MURDER_BOUNTY,
        }
    }

    #[must_use]
    pub const fn karma_delta(self) -> i32 {
        match self {
            Self::Theft => THEFT_KARMA,
            Self::Assault => ASSAULT_KARMA,
            Self::Murder => MURDER_KARMA,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrimeEvent {
    pub id: CrimeId,
    pub kind: CrimeKind,
    pub victim: TargetId,
    pub item_id: Option<ItemInstanceId>,
    pub owner_form_id: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct WitnessEvidence {
    pub witness: TargetId,
    pub has_line_of_sight: bool,
    pub distance_mm: u32,
    pub alive: bool,
    pub enabled: bool,
    pub hostile_to_victim: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrimeReport {
    pub id: CrimeId,
    pub kind: CrimeKind,
    pub victim: TargetId,
    pub witnesses: Vec<TargetId>,
    pub bounty: u32,
    pub karma_delta: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CrimeLedger {
    pub bounty: u32,
    pub karma: i32,
    pub next_sequence: u64,
    pub reported: BTreeSet<CrimeId>,
}

impl CrimeLedger {
    #[must_use]
    pub fn allocate(actor: TargetId, ledger: &mut Self) -> CrimeId {
        let sequence = ledger.next_sequence;
        ledger.next_sequence = ledger.next_sequence.saturating_add(1);
        CrimeId { actor, sequence }
    }
}

#[must_use]
pub fn witness_is_eligible(witness: &WitnessEvidence) -> bool {
    witness.alive
        && witness.enabled
        && witness.has_line_of_sight
        && witness.distance_mm <= CRIME_ALARM_RANGE_MM
        && !witness.hostile_to_victim
}

fn eligible_witnesses(witnesses: &mut [WitnessEvidence]) -> Vec<TargetId> {
    witnesses.sort_by_key(|witness| (witness.witness.class, witness.witness.form_id));
    let mut seen = BTreeSet::new();
    let mut eligible = Vec::new();
    for witness in witnesses.iter() {
        if witness_is_eligible(witness) && seen.insert(witness.witness) {
            eligible.push(witness.witness);
        }
    }
    eligible
}

/// Marks canonical stolen provenance without reporting a crime.
pub fn mark_stolen(state: &mut ItemState, owner_form_id: u32, faction_rank: Option<i32>) {
    state.ownership.origin_owner_form_id = Some(owner_form_id);
    state.ownership.origin_faction_rank = faction_rank;
    state.ownership.stolen = true;
}

/// Resolve one crime. Witnesses are sorted by FormID. Replay of a reported
/// `CrimeId` is a no-op. Unwitnessed theft still marks provenance.
pub fn resolve_crime(
    ledger: &mut CrimeLedger,
    event: CrimeEvent,
    witnesses: &mut [WitnessEvidence],
    item: Option<&mut ItemState>,
) -> Option<CrimeReport> {
    if let (Some(item), Some(owner)) = (item, event.owner_form_id) {
        mark_stolen(item, owner, None);
    }
    if ledger.reported.contains(&event.id) {
        return None;
    }
    let eligible = eligible_witnesses(witnesses);
    if eligible.is_empty() {
        return None;
    }
    ledger.reported.insert(event.id);
    ledger.bounty = ledger.bounty.saturating_add(event.kind.bounty());
    ledger.karma = ledger.karma.saturating_add(event.kind.karma_delta());
    Some(CrimeReport {
        id: event.id,
        kind: event.kind,
        victim: event.victim,
        witnesses: eligible,
        bounty: event.kind.bounty(),
        karma_delta: event.kind.karma_delta(),
    })
}

/// Replace an unreported assault with a witnessed murder. The assault is never
/// billed; the murder uses the same [`CrimeId`].
pub fn escalate_assault_to_murder(
    ledger: &mut CrimeLedger,
    assault: CrimeEvent,
    witnesses: &mut [WitnessEvidence],
) -> Option<CrimeReport> {
    if assault.kind != CrimeKind::Assault || ledger.reported.contains(&assault.id) {
        return None;
    }
    resolve_crime(
        ledger,
        CrimeEvent {
            kind: CrimeKind::Murder,
            ..assault
        },
        witnesses,
        None,
    )
}

#[cfg(test)]
#[path = "tests/crime.rs"]
mod tests;

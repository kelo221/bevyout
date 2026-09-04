//! Ordered lifecycle scheduler driven by one integer clock advance.
//!
//! Tasks due at the same millisecond sort by kind then owner. A single
//! advancement may cross many deadlines; they run in chronological order
//! without Bevy scheduling. Cell reset, encounter-zone lock, and fast
//! travel commit live here so the viewer cannot teleport first and
//! advance time later.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::barter::{
    MERCHANT_RESTOCK_INTERVAL_MS, MerchantRestockState, MerchantStockCatalog, restock_if_due,
};
use crate::chems::Addictions;
use crate::combat::limbs::LimbState;
use crate::combat::medical::{MedicalSource, restore_limbs};
use crate::effects::ActiveEffectsLedger;
use crate::item_transaction::{HolderId, ItemInstance, ItemLedger, ItemState};
use crate::radiation::RadiationPool;
use crate::time::{GameClockState, GameTime, GameTimeAdvanced, TimeAdvanceReason, TimeError};

/// 72 game hours in milliseconds.
pub const CELL_RESET_INTERVAL_MS: u64 = MERCHANT_RESTOCK_INTERVAL_MS;
/// Lifecycle snapshot schema for the optional RPGS LIFE subrecord.
pub const LIFECYCLE_SNAPSHOT_REVISION: u32 = 1;

/// Pinned due-task kinds. Ordinal is execution order at one timestamp.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LifecycleKind {
    Effects,
    Radiation,
    Death,
    Restock,
    CellReset,
    Arrival,
}

/// One scheduled deadline. Owner ids are FormIDs or synthetic test ids.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LifecycleTask {
    pub kind: LifecycleKind,
    pub owner: u32,
    pub due_game_ms: u64,
}

/// Ordered due-task calendar. The map is rebuilt from world state when
/// needed; it is never scanned every frame.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LifecycleScheduler {
    pub tasks: BTreeMap<u64, Vec<LifecycleTask>>,
}

impl LifecycleScheduler {
    pub fn schedule(&mut self, task: LifecycleTask) {
        let slot = self.tasks.entry(task.due_game_ms).or_default();
        if !slot.contains(&task) {
            slot.push(task);
            slot.sort_unstable();
        }
    }

    pub fn due(&self, from_exclusive_ms: u64, to_inclusive_ms: u64) -> Vec<LifecycleTask> {
        self.tasks
            .range((
                std::ops::Bound::Excluded(from_exclusive_ms),
                std::ops::Bound::Included(to_inclusive_ms),
            ))
            .flat_map(|(_, tasks)| tasks.iter().copied())
            .collect()
    }

    pub fn remove_due(
        &mut self,
        from_exclusive_ms: u64,
        to_inclusive_ms: u64,
    ) -> Vec<LifecycleTask> {
        let keys: Vec<u64> = self
            .tasks
            .range((
                std::ops::Bound::Excluded(from_exclusive_ms),
                std::ops::Bound::Included(to_inclusive_ms),
            ))
            .map(|(key, _)| *key)
            .collect();
        let mut due = Vec::new();
        for key in keys {
            if let Some(mut tasks) = self.tasks.remove(&key) {
                due.append(&mut tasks);
            }
        }
        due
    }
}

/// Per-cell last-visited / reset-due / generation state.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellLifecycleState {
    pub cell_form_id: u32,
    pub last_visited_game_ms: u64,
    pub reset_due_game_ms: Option<u64>,
    pub reset_generation: u32,
    pub occupied: bool,
    pub unique_refs: BTreeSet<u32>,
    pub unique_actors: BTreeSet<u32>,
    pub containers: BTreeSet<u32>,
    pub actors: BTreeSet<u32>,
    pub corpses: BTreeSet<u32>,
}

impl CellLifecycleState {
    #[must_use]
    pub fn new(cell_form_id: u32, now_ms: u64, occupied: bool) -> Self {
        Self {
            cell_form_id,
            last_visited_game_ms: now_ms,
            reset_due_game_ms: (!occupied).then_some(now_ms.saturating_add(CELL_RESET_INTERVAL_MS)),
            reset_generation: 0,
            occupied,
            unique_refs: BTreeSet::new(),
            unique_actors: BTreeSet::new(),
            containers: BTreeSet::new(),
            actors: BTreeSet::new(),
            corpses: BTreeSet::new(),
        }
    }

    pub fn mark_occupied(&mut self, now_ms: u64) {
        self.occupied = true;
        self.last_visited_game_ms = now_ms;
        self.reset_due_game_ms = None;
    }

    pub fn mark_vacated(&mut self, now_ms: u64) {
        self.occupied = false;
        self.last_visited_game_ms = now_ms;
        self.reset_due_game_ms = Some(now_ms.saturating_add(CELL_RESET_INTERVAL_MS));
    }
}

/// Receipt proving one due reset executed. The same due event cannot run twice.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct CellResetReceipt {
    pub cell_form_id: u32,
    pub due_game_ms: u64,
    pub generation: u32,
    pub restored_containers: u32,
    pub preserved_containers: u32,
    pub respawned_actors: u32,
    pub surviving_actors: u32,
    pub removed_corpses: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellResetError {
    Occupied,
    AlreadyApplied,
    NotDue,
}

/// Encounter-zone lock captured on first entry.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncounterZoneState {
    pub zone_form_id: u32,
    pub first_entered_game_ms: u64,
    pub locked_level: u8,
    pub min_level: u8,
    pub max_level: u8,
}

impl EncounterZoneState {
    #[must_use]
    pub fn lock_on_first_entry(
        zone_form_id: u32,
        now_ms: u64,
        player_level: u8,
        min_level: u8,
        max_level: u8,
    ) -> Self {
        let lo = min_level.min(max_level);
        let hi = max_level.max(min_level);
        Self {
            zone_form_id,
            first_entered_game_ms: now_ms,
            locked_level: player_level.clamp(lo, hi),
            min_level: lo,
            max_level: hi,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FastTravelBlock {
    Danger,
    Interior,
    Encumbered,
    Combat,
    Radiation,
    Undiscovered,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FastTravelEvidence {
    pub destination_cell: u32,
    pub travel_ms: u64,
    pub discovered: bool,
    pub danger: bool,
    pub interior: bool,
    pub encumbered: bool,
    pub combat: bool,
    pub radiation: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FastTravelPlan {
    pub destination_cell: u32,
    pub travel_ms: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct FastTravelCommit {
    pub destination_cell: u32,
    pub travel_ms: u64,
    pub arrival_requested: bool,
}

pub fn validate_fast_travel(
    evidence: &FastTravelEvidence,
) -> Result<FastTravelPlan, FastTravelBlock> {
    if evidence.danger {
        return Err(FastTravelBlock::Danger);
    }
    if evidence.interior {
        return Err(FastTravelBlock::Interior);
    }
    if evidence.encumbered {
        return Err(FastTravelBlock::Encumbered);
    }
    if evidence.combat {
        return Err(FastTravelBlock::Combat);
    }
    if evidence.radiation {
        return Err(FastTravelBlock::Radiation);
    }
    if !evidence.discovered {
        return Err(FastTravelBlock::Undiscovered);
    }
    Ok(FastTravelPlan {
        destination_cell: evidence.destination_cell,
        travel_ms: evidence.travel_ms,
    })
}

/// World state the scheduler mutates. Viewer adapters project this; they
/// do not keep independent timers.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct LifecycleWorld {
    pub clock: GameClockState,
    pub scheduler: LifecycleScheduler,
    pub effects: ActiveEffectsLedger,
    pub chem_doses_ms: BTreeMap<u32, u32>,
    pub addictions: Addictions,
    pub radiation: RadiationPool,
    pub current_health: Option<f32>,
    pub limbs: LimbState,
    pub restocks: BTreeMap<u32, MerchantRestockState>,
    pub cells: BTreeMap<u32, CellLifecycleState>,
    pub reset_receipts: BTreeSet<(u32, u64)>,
    pub encounter_zones: BTreeMap<u32, EncounterZoneState>,
    pub player_cell: Option<u32>,
    pub last_arrival: Option<u32>,
    pub last_kinds: Vec<LifecycleKind>,
    pub last_owners: Vec<u32>,
    #[serde(skip)]
    pub last_reset: Option<Result<CellResetReceipt, CellResetError>>,
    #[serde(skip)]
    pub last_fast_travel: Option<Result<FastTravelCommit, FastTravelBlock>>,
}

impl LifecycleWorld {
    pub fn schedule_defaults(&mut self) {
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Effects,
            owner: 0,
            due_game_ms: self.clock.absolute_game_ms,
        });
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Radiation,
            owner: 0,
            due_game_ms: self.clock.absolute_game_ms,
        });
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Death,
            owner: 0,
            due_game_ms: self.clock.absolute_game_ms,
        });
    }

    fn schedule_interval_kinds(&mut self, due_game_ms: u64) {
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Effects,
            owner: 0,
            due_game_ms,
        });
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Radiation,
            owner: 0,
            due_game_ms,
        });
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Death,
            owner: 0,
            due_game_ms,
        });
    }

    pub fn schedule_restock(&mut self, owner: u32, due_game_ms: u64) {
        self.restocks
            .entry(owner)
            .or_insert_with(|| MerchantRestockState {
                generation: 0,
                last_restock_game_ms: 0,
                next_restock_game_ms: due_game_ms,
                seed_state: crate::chems::RpgRngState::default(),
            });
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Restock,
            owner,
            due_game_ms,
        });
    }

    pub fn ensure_cell(&mut self, cell_form_id: u32, occupied: bool) -> &mut CellLifecycleState {
        let now = self.clock.absolute_game_ms;
        self.cells
            .entry(cell_form_id)
            .or_insert_with(|| CellLifecycleState::new(cell_form_id, now, occupied))
    }

    pub fn register_cell(&mut self, cell_form_id: u32, occupied: bool) {
        let due = {
            let cell = self.ensure_cell(cell_form_id, occupied);
            cell.reset_due_game_ms
        };
        if let Some(due) = due {
            self.scheduler.schedule(LifecycleTask {
                kind: LifecycleKind::CellReset,
                owner: cell_form_id,
                due_game_ms: due,
            });
        }
    }

    pub fn vacate_cell(&mut self, cell_form_id: u32, now_ms: u64) {
        if let Some(cell) = self.cells.get_mut(&cell_form_id) {
            cell.mark_vacated(now_ms);
            if let Some(due) = cell.reset_due_game_ms {
                self.scheduler.schedule(LifecycleTask {
                    kind: LifecycleKind::CellReset,
                    owner: cell_form_id,
                    due_game_ms: due,
                });
            }
        }
    }

    pub fn enter_encounter_zone(
        &mut self,
        zone_form_id: u32,
        player_level: u8,
        min_level: u8,
        max_level: u8,
    ) -> EncounterZoneState {
        if let Some(existing) = self.encounter_zones.get(&zone_form_id) {
            return *existing;
        }
        let locked = EncounterZoneState::lock_on_first_entry(
            zone_form_id,
            self.clock.absolute_game_ms,
            player_level,
            min_level,
            max_level,
        );
        self.encounter_zones.insert(zone_form_id, locked);
        locked
    }

    pub fn advance(
        &mut self,
        delta_ms: u64,
        reason: TimeAdvanceReason,
        ledger: Option<&mut ItemLedger>,
    ) -> Result<GameTimeAdvanced, TimeError> {
        let advanced = self.clock.advance_game_ms(delta_ms, reason)?;
        self.apply_advance(advanced, ledger);
        Ok(advanced)
    }

    pub fn advance_realtime(
        &mut self,
        real_delta_us: u64,
        ledger: Option<&mut ItemLedger>,
    ) -> Result<GameTimeAdvanced, TimeError> {
        let advanced = self.clock.advance_realtime(real_delta_us)?;
        self.apply_advance(advanced, ledger);
        Ok(advanced)
    }

    fn apply_advance(&mut self, advanced: GameTimeAdvanced, mut ledger: Option<&mut ItemLedger>) {
        self.schedule_interval_kinds(advanced.to_game_ms);
        self.last_kinds.clear();
        self.last_owners.clear();
        if advanced.reason == TimeAdvanceReason::Sleep {
            self.restore_owned_bed();
        }
        let mut effects_ticked = false;
        loop {
            let due = self
                .scheduler
                .remove_due(advanced.from_game_ms, advanced.to_game_ms);
            if due.is_empty() {
                break;
            }
            for task in due {
                self.last_kinds.push(task.kind);
                self.last_owners.push(task.owner);
                match task.kind {
                    LifecycleKind::Effects => {
                        if !effects_ticked {
                            tick_effects(self, advanced.delta_ms());
                            effects_ticked = true;
                        }
                    }
                    LifecycleKind::Radiation | LifecycleKind::Death => {}
                    LifecycleKind::Restock => {
                        if let Some(state) = self.restocks.get_mut(&task.owner) {
                            let outcome = restock_if_due(
                                GameTime::from_ms(task.due_game_ms),
                                state,
                                &MerchantStockCatalog::default(),
                            );
                            if outcome.due {
                                self.scheduler.schedule(LifecycleTask {
                                    kind: LifecycleKind::Restock,
                                    owner: task.owner,
                                    due_game_ms: state.next_restock_game_ms,
                                });
                            }
                        }
                    }
                    LifecycleKind::CellReset => {
                        self.last_reset = Some(reset_cell(
                            self,
                            task.owner,
                            task.due_game_ms,
                            ledger.as_deref_mut(),
                        ));
                    }
                    LifecycleKind::Arrival => {
                        self.last_arrival = Some(task.owner);
                        self.player_cell = Some(task.owner);
                    }
                }
            }
        }
    }

    pub fn apply_cell_reset(
        &mut self,
        cell_form_id: u32,
        due_game_ms: u64,
        ledger: Option<&mut ItemLedger>,
    ) -> Result<CellResetReceipt, CellResetError> {
        let result = reset_cell(self, cell_form_id, due_game_ms, ledger);
        self.last_reset = Some(result);
        result
    }

    pub fn restore_owned_bed(&mut self) {
        restore_limbs(
            &mut self.limbs,
            MedicalSource::OwnedBed,
            None,
            self.clock.now(),
        );
    }

    pub fn commit_fast_travel(
        &mut self,
        evidence: FastTravelEvidence,
        ledger: Option<&mut ItemLedger>,
    ) -> Result<FastTravelCommit, FastTravelBlock> {
        let plan = validate_fast_travel(&evidence)?;
        let arrival_ms = match self.clock.absolute_game_ms.checked_add(plan.travel_ms) {
            Some(ms) => ms,
            None => {
                let blocked = Err(FastTravelBlock::Undiscovered);
                self.last_fast_travel = Some(blocked);
                return blocked;
            }
        };
        self.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::Arrival,
            owner: plan.destination_cell,
            due_game_ms: arrival_ms,
        });
        if self
            .advance(plan.travel_ms, TimeAdvanceReason::FastTravel, ledger)
            .is_err()
        {
            let blocked = Err(FastTravelBlock::Undiscovered);
            self.last_fast_travel = Some(blocked);
            return blocked;
        }
        let commit = FastTravelCommit {
            destination_cell: plan.destination_cell,
            travel_ms: plan.travel_ms,
            arrival_requested: self.last_arrival == Some(plan.destination_cell),
        };
        self.last_fast_travel = Some(Ok(commit));
        Ok(commit)
    }

    #[must_use]
    pub fn snapshot(&self) -> LifecycleSnapshot {
        LifecycleSnapshot {
            revision: LIFECYCLE_SNAPSHOT_REVISION,
            clock: self.clock,
            cells: self.cells.clone(),
            restocks: self.restocks.clone(),
            encounter_zones: self.encounter_zones.clone(),
            reset_receipts: self.reset_receipts.clone(),
        }
    }

    pub fn restore_snapshot(&mut self, snapshot: LifecycleSnapshot) {
        self.clock = snapshot.clock;
        self.cells = snapshot.cells;
        self.restocks = snapshot.restocks;
        self.encounter_zones = snapshot.encounter_zones;
        self.reset_receipts = snapshot.reset_receipts;
        self.scheduler = LifecycleScheduler::default();
        for (owner, state) in &self.restocks {
            self.scheduler.schedule(LifecycleTask {
                kind: LifecycleKind::Restock,
                owner: *owner,
                due_game_ms: state.next_restock_game_ms,
            });
        }
        for cell in self.cells.values() {
            if let Some(due) = cell.reset_due_game_ms {
                self.scheduler.schedule(LifecycleTask {
                    kind: LifecycleKind::CellReset,
                    owner: cell.cell_form_id,
                    due_game_ms: due,
                });
            }
        }
    }
}

/// Persistable lifecycle envelope (optional RPGS LIFE).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct LifecycleSnapshot {
    pub revision: u32,
    pub clock: GameClockState,
    pub cells: BTreeMap<u32, CellLifecycleState>,
    pub restocks: BTreeMap<u32, MerchantRestockState>,
    pub encounter_zones: BTreeMap<u32, EncounterZoneState>,
    pub reset_receipts: BTreeSet<(u32, u64)>,
}

fn tick_effects(world: &mut LifecycleWorld, delta_ms: u64) {
    let mut remaining = delta_ms;
    while remaining > 0 {
        let step = u32::try_from(remaining.min(u64::from(u32::MAX))).unwrap_or(u32::MAX);
        let _expired = world.effects.tick(step);
        let mut finished = Vec::new();
        for (&withdrawal_form_id, remaining_ms) in world.chem_doses_ms.iter_mut() {
            *remaining_ms = remaining_ms.saturating_sub(step);
            if *remaining_ms == 0 {
                finished.push(withdrawal_form_id);
            }
        }
        for withdrawal_form_id in finished {
            world.chem_doses_ms.remove(&withdrawal_form_id);
            world.addictions.begin_withdrawal(withdrawal_form_id);
        }
        remaining -= u64::from(step);
    }
}

fn reset_cell(
    world: &mut LifecycleWorld,
    cell_form_id: u32,
    due_game_ms: u64,
    ledger: Option<&mut ItemLedger>,
) -> Result<CellResetReceipt, CellResetError> {
    if world.reset_receipts.contains(&(cell_form_id, due_game_ms)) {
        return Err(CellResetError::AlreadyApplied);
    }
    let Some(cell) = world.cells.get(&cell_form_id) else {
        return Err(CellResetError::NotDue);
    };
    if cell.occupied {
        return Err(CellResetError::Occupied);
    }
    if cell.reset_due_game_ms != Some(due_game_ms) || due_game_ms > world.clock.absolute_game_ms {
        return Err(CellResetError::NotDue);
    }

    let unique_refs = cell.unique_refs.clone();
    let unique_actors = cell.unique_actors.clone();
    let containers = cell.containers.clone();
    let actors = cell.actors.clone();
    let corpses = cell.corpses.clone();

    let mut restored_containers = 0;
    let mut preserved_containers = 0;
    if let Some(ledger) = ledger {
        for reference in &containers {
            let holder = HolderId::FixtureContainer {
                reference_form_id: *reference,
            };
            if unique_refs.contains(reference) || holder_has_player_owned(ledger, holder) {
                preserved_containers += 1;
                continue;
            }
            // Reset templates are not prepared yet. Count the container as
            // restored for the receipt without emptying live contents.
            restored_containers += 1;
        }
        for reference in &corpses {
            ledger.holders_mut().remove(&HolderId::Corpse {
                actor_reference_form_id: *reference,
            });
        }
    }

    let respawned_actors = actors
        .iter()
        .filter(|reference| !unique_actors.contains(reference))
        .count() as u32;
    let surviving_actors = unique_actors.len() as u32;
    let removed_corpses = corpses.len() as u32;

    let Some(cell) = world.cells.get_mut(&cell_form_id) else {
        return Err(CellResetError::NotDue);
    };
    cell.reset_generation = cell.reset_generation.saturating_add(1);
    cell.last_visited_game_ms = due_game_ms;
    cell.reset_due_game_ms = Some(due_game_ms.saturating_add(CELL_RESET_INTERVAL_MS));
    cell.corpses.clear();
    let generation = cell.reset_generation;
    let next_due = cell.reset_due_game_ms;
    world.reset_receipts.insert((cell_form_id, due_game_ms));
    if let Some(next_due) = next_due {
        world.scheduler.schedule(LifecycleTask {
            kind: LifecycleKind::CellReset,
            owner: cell_form_id,
            due_game_ms: next_due,
        });
    }
    Ok(CellResetReceipt {
        cell_form_id,
        due_game_ms,
        generation,
        restored_containers,
        preserved_containers,
        respawned_actors,
        surviving_actors,
        removed_corpses,
    })
}

fn holder_has_player_owned(ledger: &ItemLedger, holder: HolderId) -> bool {
    ledger
        .holders()
        .get(&holder)
        .is_some_and(|state| state.items.iter().any(item_is_player_owned))
}

fn item_is_player_owned(item: &ItemInstance) -> bool {
    item.state.ownership.origin_owner_form_id == Some(0) || item.state.ownership.stolen
}

pub fn player_owned_item(id: u64, form_id: u32) -> ItemInstance {
    ItemInstance {
        id: crate::item_transaction::ItemInstanceId(id),
        base_form_id: form_id,
        count: 1,
        state: ItemState {
            ownership: crate::item_transaction::OwnershipProvenance {
                origin_owner_form_id: Some(0),
                origin_faction_rank: None,
                stolen: false,
            },
            ..ItemState::default()
        },
    }
}

pub fn unowned_item(id: u64, form_id: u32) -> ItemInstance {
    ItemInstance {
        id: crate::item_transaction::ItemInstanceId(id),
        base_form_id: form_id,
        count: 1,
        state: ItemState::default(),
    }
}

#[cfg(test)]
#[path = "tests/lifecycle.rs"]
mod tests;

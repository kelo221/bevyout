//! Ownership and addressing for console-visible navigation agents.

use bevy::prelude::*;

use crate::viewer::nav::agent::{INITIAL_AGENT_SLOTS, MAX_AGENT_INDEX};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DebugAgentOrigin {
    SpawnedCapsule,
    BoundActor { reference_form_id: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DebugAgentEntry {
    pub(crate) entity: Entity,
    pub(crate) origin: DebugAgentOrigin,
}

/// Console index -> entity address book. The public `entities` projection is
/// retained for the existing diagnostic/test harnesses; `origins` is the
/// ownership authority used by handoff code to distinguish disposable debug
/// capsules from production actors.
#[derive(Resource)]
pub(crate) struct DebugAgentRoster {
    pub(crate) entities: Vec<Option<Entity>>,
    origins: Vec<Option<DebugAgentOrigin>>,
}

impl Default for DebugAgentRoster {
    fn default() -> Self {
        Self {
            entities: vec![None; INITIAL_AGENT_SLOTS],
            origins: vec![None; INITIAL_AGENT_SLOTS],
        }
    }
}

impl DebugAgentRoster {
    pub(crate) fn index_of(&self, entity: Entity) -> Option<usize> {
        self.entities.iter().position(|slot| *slot == Some(entity))
    }

    pub(crate) fn active(&self) -> impl Iterator<Item = (usize, Entity)> + '_ {
        self.entities
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| slot.map(|entity| (index, entity)))
    }

    pub(crate) fn active_entries(&self) -> impl Iterator<Item = (usize, DebugAgentEntry)> + '_ {
        self.active().map(|(index, entity)| {
            (
                index,
                DebugAgentEntry {
                    entity,
                    origin: self
                        .origins
                        .get(index)
                        .and_then(|origin| *origin)
                        .unwrap_or(DebugAgentOrigin::SpawnedCapsule),
                },
            )
        })
    }

    pub(crate) fn get(&self, index: usize) -> Option<Entity> {
        self.entities.get(index).copied().flatten()
    }

    pub(crate) fn entry(&self, index: usize) -> Option<DebugAgentEntry> {
        self.active_entries()
            .find_map(|(entry_index, entry)| (entry_index == index).then_some(entry))
    }

    pub(crate) fn origin_of(&self, entity: Entity) -> Option<DebugAgentOrigin> {
        self.index_of(entity)
            .and_then(|index| self.entry(index).map(|entry| entry.origin))
    }

    pub(crate) fn is_spawned_capsule(&self, index: usize) -> bool {
        matches!(
            self.entry(index).map(|entry| entry.origin),
            Some(DebugAgentOrigin::SpawnedCapsule)
        )
    }

    pub(crate) fn is_occupied(&self, index: usize) -> bool {
        self.get(index).is_some()
    }

    pub(crate) fn set(&mut self, index: usize, value: Option<Entity>) {
        self.set_with_origin(index, value, DebugAgentOrigin::SpawnedCapsule);
    }

    pub(crate) fn set_with_origin(
        &mut self,
        index: usize,
        value: Option<Entity>,
        origin: DebugAgentOrigin,
    ) {
        debug_assert!(index <= MAX_AGENT_INDEX);
        if index >= self.entities.len() {
            self.entities.resize(index + 1, None);
            self.origins.resize(index + 1, None);
        }
        self.entities[index] = value;
        self.origins[index] = value.map(|_| origin);
    }
}

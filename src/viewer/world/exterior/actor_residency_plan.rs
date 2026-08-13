//! Pure exterior actor-residency sequencing (M6 W3-C).
//!
//! This module answers one question only: given what the exterior lifecycle
//! and the live ECS projection currently look like, which residency requests
//! should be made this frame, and in which order. It owns no actor state, no
//! save record, and no entity; the Bevy adapter in `actors.rs` turns each
//! planned request into a `viewer::actor_residency` decision and applies the
//! accepted transition to the existing authorities.
//!
//! Deliberately `std`-only (no Bevy, no `bevyout_core`) so the runtime
//! sequencing is executable from cucumber, per AGENTS.md's testing section.

/// One resident-cell ownership token as observed from `ExteriorStreamState`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct PlannedOwner {
    pub(crate) cell_form_id: u32,
    pub(crate) generation: u64,
}

/// A prepared actor entry carried by one exterior cell package.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlannedActorEntry {
    pub(crate) reference_form_id: u32,
    /// Whether `ActiveSaveState` already holds an `ActorInstanceState` for
    /// this reference. It decides `Bind` (fresh) versus `Restore` (reload).
    pub(crate) has_saved_state: bool,
}

/// One exterior cell as the lifecycle currently reports it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PlannedCell {
    pub(crate) cell_form_id: u32,
    pub(crate) generation: u64,
    pub(crate) grid: (i32, i32),
    /// The full residency predicate: collision-ready, lifecycle `Ready` or
    /// `Resident`, and a decoded package present (plus a spawned root).
    pub(crate) projectable: bool,
    pub(crate) evicting: bool,
    pub(crate) actors: Vec<PlannedActorEntry>,
}

impl PlannedCell {
    fn owner(&self) -> PlannedOwner {
        PlannedOwner {
            cell_form_id: self.cell_form_id,
            generation: self.generation,
        }
    }
}

/// One live projected actor entity as observed from the ECS.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlannedLiveActor {
    pub(crate) reference_form_id: u32,
    pub(crate) owner: PlannedOwner,
    /// The grid cell the actor's live transform currently falls inside.
    pub(crate) grid: (i32, i32),
}

/// A residency request the adapter should put to `decide_actor_residency`,
/// or a duplicate-projection observation the adapter must refuse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlannedRequest {
    Bind {
        reference_form_id: u32,
        destination: PlannedOwner,
    },
    Restore {
        reference_form_id: u32,
        destination: PlannedOwner,
    },
    Handoff {
        reference_form_id: u32,
        source: PlannedOwner,
        destination: PlannedOwner,
    },
    Unload {
        reference_form_id: u32,
        source: PlannedOwner,
    },
    /// Two or more live entities claim one reference. The adapter reports
    /// this to `decide_actor_residency` as an owner slice so the rejection
    /// (`DuplicateOwner`) stays the policy's decision, not the planner's.
    Duplicate {
        reference_form_id: u32,
        owners: usize,
    },
}

impl PlannedRequest {
    pub(crate) const fn reference_form_id(self) -> u32 {
        match self {
            Self::Bind {
                reference_form_id, ..
            }
            | Self::Restore {
                reference_form_id, ..
            }
            | Self::Handoff {
                reference_form_id, ..
            }
            | Self::Unload {
                reference_form_id, ..
            }
            | Self::Duplicate {
                reference_form_id, ..
            } => reference_form_id,
        }
    }
}

/// Plans this frame's residency requests.
///
/// Ordering is fixed and load bearing: duplicates are refused first, then
/// unloads (they must run before `finalize_evictions` despawns their cell
/// root), then border handoffs, then binds/restores. A reference that is
/// already owned by exactly the cell that lists it produces no request at
/// all, which is what makes re-running the plan every frame a no-op.
pub(crate) fn plan_actor_residency(
    cells: &[PlannedCell],
    live: &[PlannedLiveActor],
) -> Vec<PlannedRequest> {
    let mut duplicates = Vec::new();
    let mut unloads = Vec::new();
    let mut handoffs = Vec::new();
    let mut binds = Vec::new();

    let mut references = live
        .iter()
        .map(|actor| actor.reference_form_id)
        .collect::<Vec<_>>();
    references.sort_unstable();
    references.dedup();
    for reference_form_id in references {
        let owners = live
            .iter()
            .filter(|actor| actor.reference_form_id == reference_form_id)
            .collect::<Vec<_>>();
        if owners.len() > 1 {
            duplicates.push(PlannedRequest::Duplicate {
                reference_form_id,
                owners: owners.len(),
            });
            continue;
        }
        let actor = owners[0];
        // The owning cell is gone, evicting, or has already moved on to a
        // newer generation: the live projection must be checkpointed and
        // released before its root disappears.
        let owning_cell = cells
            .iter()
            .find(|cell| cell.owner() == actor.owner && !cell.evicting && cell.projectable);
        if owning_cell.is_none() {
            unloads.push(PlannedRequest::Unload {
                reference_form_id,
                source: actor.owner,
            });
            continue;
        }
        if let Some(destination) = cells.iter().find(|cell| {
            cell.grid == actor.grid
                && cell.projectable
                && !cell.evicting
                && cell.cell_form_id != actor.owner.cell_form_id
        }) {
            handoffs.push(PlannedRequest::Handoff {
                reference_form_id,
                source: actor.owner,
                destination: destination.owner(),
            });
        }
    }

    for cell in cells
        .iter()
        .filter(|cell| cell.projectable && !cell.evicting)
    {
        for entry in &cell.actors {
            let already_live = live
                .iter()
                .any(|actor| actor.reference_form_id == entry.reference_form_id);
            if already_live {
                continue;
            }
            let destination = cell.owner();
            binds.push(if entry.has_saved_state {
                PlannedRequest::Restore {
                    reference_form_id: entry.reference_form_id,
                    destination,
                }
            } else {
                PlannedRequest::Bind {
                    reference_form_id: entry.reference_form_id,
                    destination,
                }
            });
        }
    }

    duplicates
        .into_iter()
        .chain(unloads)
        .chain(handoffs)
        .chain(binds)
        .collect()
}

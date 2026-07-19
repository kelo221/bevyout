//! Shared interaction components and runtime resources.
//!
//! State is centralized here so focus, activation, scripted control, transfer
//! UI, and persistence continue to project the same authorities.

use super::*;

/// Attach this component to the root that owns a prepared placement's scene.
/// Mesh-ray hits are walked through `ChildOf` ancestors until this root is found.
#[derive(Component, Clone, Debug)]
pub(crate) struct PlacementRoot {
    pub(super) placement: PreparedPlacement,
}

impl PlacementRoot {
    pub(crate) fn new(placement: PreparedPlacement) -> Self {
        Self { placement }
    }

    pub(crate) fn uses_quick_ao(&self) -> bool {
        self.placement.ao_mode == "ao-quick-v1"
    }

    pub(crate) fn placement(&self) -> &PreparedPlacement {
        &self.placement
    }

    /// Issue #163 (`setlock`): mutates this door's runtime lock level in
    /// place -- the exact `PreparedDoor` the player's own E-activation path
    /// (`activation::activate_focused_placement` -> `door_is_locked`) reads
    /// off this same component, so a re-lock is visible on the very next
    /// activation attempt. `None` (or a level `<= 0`) clears the lock,
    /// matching `door_is_locked`'s own `lock_level.is_none_or(|level| level
    /// <= 0)` rule. Returns `false` (no-op) for a non-door placement so
    /// `console::world_commands::setlock` can report a clean error instead
    /// of silently doing nothing.
    pub(crate) fn set_door_lock_level(&mut self, lock_level: Option<i8>) -> bool {
        match &mut self.placement.semantic {
            PreparedSemantic::Door(door) => {
                door.lock_level = lock_level;
                true
            }
            _ => false,
        }
    }
}

/// `open` is pub(crate) for issues #60/#61: `world::persist` captures
/// door/container open state on the way out of a cell and re-inserts it on
/// apply. Everything else stays private to this module.
#[derive(Resource, Default)]
pub(crate) struct InteractionState {
    pub(super) focused: Option<Entity>,
    pub(crate) open: HashSet<Entity>,
}

/// Issue #59's notice seam: `world::swap`'s fallback-cancellation and
/// failure-recovery systems reach this same HUD line (`show`, made
/// `pub(crate)` for that) rather than inventing a second notice surface.
#[derive(Resource, Default)]
pub(crate) struct InteractionNotice {
    pub(super) text: String,
    pub(super) remaining_seconds: f32,
}

impl InteractionNotice {
    pub(crate) fn show(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.remaining_seconds = NOTICE_SECONDS;
    }

    /// Read-only view of the currently displayed notice text (issue #59's
    /// swap tests assert on it; nothing outside tests should).
    #[cfg(test)]
    pub(crate) fn text(&self) -> &str {
        &self.text
    }
}

/// Fixed seam (issue #75, wired at wave-2 integration into #76's persistence
/// capture/apply): a container's runtime inventory, keyed by
/// `reference_form_id` -- the same identity the manifest and console/BRP
/// paths already use for containers. `pub(crate)` (and the tuple field with
/// it) so `world::persist` can capture/apply it without this module growing
/// a second, parallel accessor API.
#[derive(Resource, Default)]
pub(crate) struct ContainerStates(pub(crate) HashMap<u32, container_policy::ContainerState>);

impl ContainerStates {
    /// F75.2/T75.2: opens (or reopens) a container's state. `resolve_leveled`
    /// is the #74 resolver seam -- called only when this reference has never
    /// resolved before; a `resolved` state short-circuits straight back
    /// without touching it (never re-rolls).
    pub(super) fn open(
        &mut self,
        reference_form_id: u32,
        entries: &[container_policy::SeedEntry],
        resolve_leveled: impl FnMut(u32) -> Vec<(u32, i32)>,
    ) -> &container_policy::ContainerState {
        let existing = self.0.remove(&reference_form_id);
        let state = container_policy::open_container(existing, entries, resolve_leveled);
        self.0.entry(reference_form_id).or_insert(state)
    }

    pub(crate) fn get(&self, reference_form_id: u32) -> Option<&container_policy::ContainerState> {
        self.0.get(&reference_form_id)
    }

    pub(super) fn get_mut(
        &mut self,
        reference_form_id: u32,
    ) -> Option<&mut container_policy::ContainerState> {
        self.0.get_mut(&reference_form_id)
    }
}

/// The transfer modal's holder kind. Containers and staged corpses share the
/// stack/persistence implementation but retain distinct stable logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LootHolderKind {
    Container,
    Corpse,
}

impl LootHolderKind {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Container => "container",
            Self::Corpse => "corpse",
        }
    }
}

/// F75.2/F118.2: which loot holder the transfer modal is showing, and the
/// best-effort item names collected from its prepared inventory at open
/// time (resolved-leveled or player-only items with no known name fall
/// back to their hex form id -- name/count is all this issue promises, see
/// the plan's non-goals).
pub(crate) struct ActiveContainer {
    pub(super) kind: LootHolderKind,
    pub(super) entity: Entity,
    pub(super) reference_form_id: u32,
    pub(super) name: String,
    pub(super) item_names: HashMap<u32, String>,
    /// `XOWN` owner of the container reference (issue #81): taking from an
    /// owned container logs as theft.
    pub(super) owner_form_id: Option<u32>,
}

#[derive(Resource, Default)]
pub(crate) struct ActiveContainerTarget(pub(crate) Option<ActiveContainer>);

#[derive(Component)]
pub(super) struct InteractionPromptText;

#[derive(Component)]
pub(super) struct InteractionNoticeText;

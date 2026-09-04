//! Thin Bevy adapter over core lockpicking and hacking sessions.

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevyout_core::actor_state::ActorSkill;
use bevyout_core::crime::CrimeLedger;
use bevyout_core::minigames::{
    HackingInput, HackingSession, LockpickConfig, LockpickInput, LockpickPhase, LockpickSession,
    MinigameCommit, MinigameError, MinigameRngState, MinigameSessionId, PickAngleMilliDegrees,
    generate_hacking_board, step_hacking, step_lockpick,
};

use crate::app_state::{GameplayModal, RequestStateTransition};

use super::actor::ActorRuntime;
use super::actor_state::ActorStateRuntime;
use super::crime::live_witnesses;
use super::interaction::{CanonicalItemLedger, PlacementRoot, PlayerInventory};
use super::nav;
use super::perception::ActorAwareness;
use super::stats::PlayerProgression;

const SYNTHETIC_HACKING_BANK: &[&str] = &["VENT", "DOOR", "LOCK", "SAFE", "KEYS"];
const SYNTHETIC_HACKING_PASSWORD: &str = "VENT";

#[derive(Resource, Debug, Clone)]
pub(crate) struct MinigameRuntime {
    pub(crate) rng: MinigameRngState,
    pub(crate) lockpick: Option<LockpickSession>,
    pub(crate) hacking: Option<HackingSession>,
    pub(crate) target: Option<Entity>,
    next_session: u64,
}

impl Default for MinigameRuntime {
    fn default() -> Self {
        Self {
            rng: MinigameRngState::from_seed(0),
            lockpick: None,
            hacking: None,
            target: None,
            next_session: 1,
        }
    }
}

pub(crate) struct MinigamesPlugin;

impl Plugin for MinigamesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MinigameRuntime>()
            .add_systems(OnEnter(GameplayModal::Lockpicking), show_minigame_cursor)
            .add_systems(OnEnter(GameplayModal::Hacking), show_minigame_cursor)
            .add_systems(OnExit(GameplayModal::Lockpicking), exit_lockpicking)
            .add_systems(OnExit(GameplayModal::Hacking), exit_hacking)
            .add_systems(
                Update,
                cancel_on_escape.run_if(
                    in_state(GameplayModal::Lockpicking).or_else(in_state(GameplayModal::Hacking)),
                ),
            );
    }
}

fn show_minigame_cursor(mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn hide_minigame_cursor(cursor: &mut Query<&mut CursorOptions, With<PrimaryWindow>>) {
    if let Ok(mut cursor) = cursor.single_mut() {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }
}

fn exit_lockpicking(
    mut runtime: ResMut<MinigameRuntime>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if runtime
        .lockpick
        .as_ref()
        .is_some_and(LockpickSession::is_active)
        && let Some(session) = runtime.lockpick.as_mut()
    {
        session.phase = LockpickPhase::Cancelled;
    }
    runtime.lockpick = None;
    runtime.target = None;
    hide_minigame_cursor(&mut cursor);
}

fn exit_hacking(
    mut runtime: ResMut<MinigameRuntime>,
    mut cursor: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if runtime
        .hacking
        .as_ref()
        .is_some_and(HackingSession::is_active)
        && let Some(session) = runtime.hacking.as_mut()
    {
        session.phase = bevyout_core::minigames::HackingPhase::Cancelled;
    }
    runtime.hacking = None;
    runtime.target = None;
    hide_minigame_cursor(&mut cursor);
}

fn cancel_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut requests: MessageWriter<RequestStateTransition>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        requests.write(RequestStateTransition::Modal(GameplayModal::None));
    }
}

pub(crate) fn start_lockpick_session(
    runtime: &mut MinigameRuntime,
    skill: u8,
    entity: Entity,
    difficulty: u8,
    owner_form_id: Option<u32>,
) {
    let sweet_spot_milli = (i32::from(difficulty) * 900 - 45_000).clamp(
        bevyout_core::minigames::PICK_ANGLE_MIN_MILLI,
        bevyout_core::minigames::PICK_ANGLE_MAX_MILLI,
    );
    let tolerance_milli = 1_000u32.saturating_add(u32::from(skill) * 40);
    let id = MinigameSessionId(runtime.next_session);
    runtime.next_session = runtime.next_session.saturating_add(1);
    runtime.target = Some(entity);
    runtime.hacking = None;
    runtime.lockpick = Some(LockpickSession::new(
        id,
        LockpickConfig {
            difficulty,
            skill,
            sweet_spot_milli,
            tolerance_milli,
            owner_form_id,
        },
    ));
}

pub(crate) fn start_hacking_session(
    runtime: &mut MinigameRuntime,
    entity: Entity,
) -> Result<(), MinigameError> {
    let bank: Vec<String> = SYNTHETIC_HACKING_BANK
        .iter()
        .map(|word| (*word).to_string())
        .collect();
    let board = generate_hacking_board(&bank, SYNTHETIC_HACKING_PASSWORD, &mut runtime.rng)?;
    let id = MinigameSessionId(runtime.next_session);
    runtime.next_session = runtime.next_session.saturating_add(1);
    runtime.target = Some(entity);
    runtime.lockpick = None;
    runtime.hacking = Some(HackingSession::new(id, board));
    Ok(())
}

#[must_use]
pub(crate) fn cancel_active_sessions(runtime: &mut MinigameRuntime) -> bool {
    let mut cancelled = false;
    if runtime
        .lockpick
        .as_ref()
        .is_some_and(LockpickSession::is_active)
        && let Some(session) = runtime.lockpick.as_mut()
    {
        session.phase = LockpickPhase::Cancelled;
        cancelled = true;
    }
    if runtime
        .hacking
        .as_ref()
        .is_some_and(HackingSession::is_active)
        && let Some(session) = runtime.hacking.as_mut()
    {
        session.phase = bevyout_core::minigames::HackingPhase::Cancelled;
        cancelled = true;
    }
    runtime.lockpick = None;
    runtime.hacking = None;
    runtime.target = None;
    cancelled
}

pub(crate) fn begin_lockpick(
    world: &mut World,
    entity: Entity,
    difficulty: u8,
    owner_form_id: Option<u32>,
) {
    let skill = world
        .get_resource::<PlayerProgression>()
        .map(|progression| progression.stats.skill_value(ActorSkill::Lockpick))
        .unwrap_or(0);
    let mut runtime = world.resource_mut::<MinigameRuntime>();
    start_lockpick_session(&mut runtime, skill, entity, difficulty, owner_form_id);
    world.write_message(RequestStateTransition::Modal(GameplayModal::Lockpicking));
}

pub(crate) fn begin_hacking(world: &mut World, entity: Entity) -> Result<(), MinigameError> {
    {
        let mut runtime = world.resource_mut::<MinigameRuntime>();
        start_hacking_session(&mut runtime, entity)?;
    }
    world.write_message(RequestStateTransition::Modal(GameplayModal::Hacking));
    Ok(())
}

pub(crate) fn apply_lockpick_input(
    world: &mut World,
    input: LockpickInput,
) -> Result<MinigameCommit, MinigameError> {
    let witnesses = {
        let mut query = world.query::<(
            &ActorRuntime,
            &ActorAwareness,
            Option<&ActorStateRuntime>,
            Option<&PlacementRoot>,
        )>();
        live_witnesses(query.iter(world))
    };
    let mut witnesses = witnesses;
    if world.get_resource::<MinigameRuntime>().is_none()
        || world.get_resource::<CanonicalItemLedger>().is_none()
    {
        return Err(MinigameError::Inactive);
    }
    let commit = if world.get_resource::<PlayerProgression>().is_some() {
        world.resource_scope(|world, mut runtime: Mut<MinigameRuntime>| {
            world.resource_scope(|world, mut canonical: Mut<CanonicalItemLedger>| {
                world.resource_scope(|world, mut progression: Mut<PlayerProgression>| {
                    let _ = world;
                    let MinigameRuntime { rng, lockpick, .. } = &mut *runtime;
                    let Some(session) = lockpick.as_mut() else {
                        return Err(MinigameError::Inactive);
                    };
                    step_lockpick(
                        session,
                        input,
                        &mut canonical.ledger,
                        rng,
                        &mut progression.crime,
                        &mut witnesses,
                    )
                })
            })
        })?
    } else {
        world.resource_scope(|world, mut runtime: Mut<MinigameRuntime>| {
            world.resource_scope(|world, mut canonical: Mut<CanonicalItemLedger>| {
                let _ = world;
                let MinigameRuntime { rng, lockpick, .. } = &mut *runtime;
                let Some(session) = lockpick.as_mut() else {
                    return Err(MinigameError::Inactive);
                };
                let mut crime = CrimeLedger::default();
                step_lockpick(
                    session,
                    input,
                    &mut canonical.ledger,
                    rng,
                    &mut crime,
                    &mut witnesses,
                )
            })
        })?
    };
    if commit.pin_consumed {
        let snapshot = world
            .get_resource::<CanonicalItemLedger>()
            .and_then(|ledger| ledger.player_legacy_snapshot());
        if let (Some(snapshot), Some(mut inventory)) =
            (snapshot, world.get_resource_mut::<PlayerInventory>())
        {
            *inventory = PlayerInventory::from_stack_states(snapshot.stacks());
        }
    }
    if commit.lock_unlocked {
        unlock_target_door(world);
        world.write_message(RequestStateTransition::Modal(GameplayModal::None));
    } else if matches!(input, LockpickInput::Cancel) {
        world.write_message(RequestStateTransition::Modal(GameplayModal::None));
    }
    Ok(commit)
}

pub(crate) fn apply_hacking_input(
    world: &mut World,
    input: HackingInput,
) -> Result<MinigameCommit, MinigameError> {
    let commit = world.resource_scope(|_world, mut runtime: Mut<MinigameRuntime>| {
        let MinigameRuntime { rng, hacking, .. } = &mut *runtime;
        let Some(session) = hacking.as_mut() else {
            return Err(MinigameError::Inactive);
        };
        step_hacking(session, input, rng)
    })?;
    if commit.terminal_unlocked
        || commit.terminal_locked_out
        || matches!(input, HackingInput::Cancel)
    {
        world.write_message(RequestStateTransition::Modal(GameplayModal::None));
    }
    Ok(commit)
}

fn unlock_target_door(world: &mut World) {
    let Some(entity) = world
        .get_resource::<MinigameRuntime>()
        .and_then(|runtime| runtime.target)
    else {
        return;
    };
    let Some(form_id) = world
        .get::<PlacementRoot>(entity)
        .map(|root| root.placement().reference_form_id)
    else {
        return;
    };
    if let Some(mut root) = world.get_mut::<PlacementRoot>(entity) {
        root.set_door_lock_level(None);
    }
    nav::api::set_door_lock_level(world, form_id.into(), None);
}

#[cfg(test)]
pub(crate) fn grant_runtime_bobby_pins(world: &mut World, count: u32) -> Result<(), MinigameError> {
    {
        let mut canonical = world
            .get_resource_mut::<CanonicalItemLedger>()
            .ok_or(MinigameError::NoBobbyPin)?;
        bevyout_core::minigames::grant_bobby_pins(&mut canonical.ledger, count)?;
    }
    let snapshot = world
        .get_resource::<CanonicalItemLedger>()
        .and_then(|ledger| ledger.player_legacy_snapshot());
    if let (Some(snapshot), Some(mut inventory)) =
        (snapshot, world.get_resource_mut::<PlayerInventory>())
    {
        *inventory = PlayerInventory::from_stack_states(snapshot.stacks());
    }
    Ok(())
}

pub(crate) fn lockpick_snapshot(world: &World) -> Option<&LockpickSession> {
    world
        .get_resource::<MinigameRuntime>()
        .and_then(|runtime| runtime.lockpick.as_ref())
}

pub(crate) fn hacking_snapshot(world: &World) -> Option<&HackingSession> {
    world
        .get_resource::<MinigameRuntime>()
        .and_then(|runtime| runtime.hacking.as_ref())
}

pub(crate) fn parse_pick_angle(value: &str) -> Result<PickAngleMilliDegrees, MinigameError> {
    let milli = value
        .parse::<i32>()
        .map_err(|_| MinigameError::PickAngleOutOfRange)?;
    PickAngleMilliDegrees::new(milli)
}

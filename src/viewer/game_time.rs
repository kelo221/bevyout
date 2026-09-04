//! Authoritative integer game clock adapter (M9 wave 9).
//!
//! Lighting [`super::day_night::GameClock`] remains a projection of
//! [`bevyout_core::time::GameClockState::hour_as_f32`]. Realtime frames,
//! `passtime`, sleep, and fast travel all advance one [`LifecycleWorld`].

use bevy::prelude::*;
use bevyout_core::lifecycle::{
    FastTravelBlock, FastTravelCommit, FastTravelEvidence, LifecycleWorld,
};
use bevyout_core::time::{GameTimeAdvanced, TimeAdvanceReason};

use super::day_night::GameClock;
use super::effects::{ActiveEffectsList, Addictions, PlayerRadiation, PlayerVitals};
use super::interaction::CanonicalItemLedger;
use super::player::FpsPlayer;
use super::plugins::ViewerSet;
use super::stats::PlayerProgression;

#[derive(Resource, Debug, Clone, PartialEq)]
pub(crate) struct GameTimeRuntime {
    pub(crate) world: LifecycleWorld,
    pub(crate) last_advance: Option<GameTimeAdvanced>,
}

impl Default for GameTimeRuntime {
    fn default() -> Self {
        let mut world = LifecycleWorld::default();
        world.clock.timescale = 0;
        Self {
            world,
            last_advance: None,
        }
    }
}

pub(crate) struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        let mut world = LifecycleWorld::default();
        world.clock.timescale = 0;
        app.insert_resource(GameTimeRuntime {
            world,
            last_advance: None,
        })
        .add_systems(
            Update,
            (
                advance_realtime_clock,
                project_lighting_hour,
                project_runtime_player_state,
            )
                .chain()
                .in_set(ViewerSet::WorldSync)
                .before(super::effects::EffectsSet::Mutate),
        );
    }
}

fn advance_realtime_clock(
    real_time: Res<Time<Real>>,
    virtual_time: Res<Time<Virtual>>,
    mut runtime: ResMut<GameTimeRuntime>,
    mut ledger: Option<ResMut<CanonicalItemLedger>>,
    mut progression: Option<ResMut<PlayerProgression>>,
) {
    if virtual_time.is_paused() {
        runtime.last_advance = None;
        return;
    }
    let delta_us = (real_time.delta_secs_f64() * 1_000_000.0).round() as u64;
    if delta_us == 0 {
        runtime.last_advance = None;
        return;
    }
    sync_from_progression(&mut runtime.world, progression.as_deref());
    let ledger = ledger.as_mut().map(|canonical| &mut canonical.ledger);
    match runtime.world.advance_realtime(delta_us, ledger) {
        Ok(advanced) => {
            runtime.last_advance = Some(advanced);
            if let Some(progression) = progression.as_mut() {
                sync_to_progression(&runtime.world, progression);
            }
        }
        Err(_) => runtime.last_advance = None,
    }
}

fn project_lighting_hour(runtime: Res<GameTimeRuntime>, mut clock: Option<ResMut<GameClock>>) {
    if runtime
        .last_advance
        .is_some_and(|advanced| advanced.delta_ms() > 0)
        && let Some(clock) = clock.as_mut()
    {
        clock.hour = runtime.world.clock.hour_as_f32();
    }
}

fn project_runtime_player_state(
    runtime: Res<GameTimeRuntime>,
    mut progression: Option<ResMut<PlayerProgression>>,
    mut players: Query<
        (
            &mut ActiveEffectsList,
            &mut Addictions,
            &mut PlayerRadiation,
            Option<&mut PlayerVitals>,
        ),
        With<FpsPlayer>,
    >,
) {
    if !runtime
        .last_advance
        .is_some_and(|advanced| advanced.delta_ms() > 0)
    {
        return;
    }
    if let Some(progression) = progression.as_mut() {
        sync_to_progression(&runtime.world, progression);
    }
    for (mut effects, mut addictions, mut radiation, vitals) in &mut players {
        effects.ledger = runtime.world.effects.clone();
        effects.chem_doses_ms = runtime.world.chem_doses_ms.clone();
        addictions.0 = runtime.world.addictions.clone();
        radiation.0 = runtime.world.radiation;
        if let Some(mut vitals) = vitals
            && let Some(health) = runtime.world.current_health
        {
            vitals.current_health = health;
        }
    }
}

pub(crate) fn sync_from_progression(
    world: &mut LifecycleWorld,
    progression: Option<&PlayerProgression>,
) {
    if let Some(progression) = progression {
        world.effects = progression.effects.clone();
        world.chem_doses_ms = progression.chem_doses_ms.clone();
        world.addictions = progression.addictions.clone();
        world.radiation = progression.radiation;
        world.current_health = progression.current_health;
        world.limbs = progression.limbs.clone();
    }
}

pub(crate) fn sync_to_progression(world: &LifecycleWorld, progression: &mut PlayerProgression) {
    progression.effects = world.effects.clone();
    progression.chem_doses_ms = world.chem_doses_ms.clone();
    progression.addictions = world.addictions.clone();
    progression.radiation = world.radiation;
    progression.current_health = world.current_health;
    progression.limbs = world.limbs.clone();
}

pub(crate) fn passtime_ms(
    runtime: &mut GameTimeRuntime,
    delta_ms: u64,
    reason: TimeAdvanceReason,
    ledger: Option<&mut bevyout_core::item_transaction::ItemLedger>,
    progression: Option<&mut PlayerProgression>,
) -> Result<GameTimeAdvanced, bevyout_core::time::TimeError> {
    sync_from_progression(&mut runtime.world, progression.as_deref());
    let advanced = runtime.world.advance(delta_ms, reason, ledger)?;
    runtime.last_advance = Some(advanced);
    if let Some(progression) = progression {
        sync_to_progression(&runtime.world, progression);
        if reason == TimeAdvanceReason::Sleep {
            super::stats::restore_owned_bed(progression, runtime.world.clock.now());
        }
    }
    Ok(advanced)
}

pub(crate) fn commit_fast_travel(
    runtime: &mut GameTimeRuntime,
    evidence: FastTravelEvidence,
    ledger: Option<&mut bevyout_core::item_transaction::ItemLedger>,
    progression: Option<&mut PlayerProgression>,
) -> Result<FastTravelCommit, FastTravelBlock> {
    sync_from_progression(&mut runtime.world, progression.as_deref());
    let commit = runtime.world.commit_fast_travel(evidence, ledger)?;
    if let Some(progression) = progression {
        sync_to_progression(&runtime.world, progression);
    }
    Ok(commit)
}

pub(crate) fn project_runtime_to_world(world: &mut World, runtime: &GameTimeRuntime) {
    if let Some(mut progression) = world.get_resource_mut::<PlayerProgression>() {
        sync_to_progression(&runtime.world, &mut progression);
    }
    let entities: Vec<Entity> = world
        .query_filtered::<Entity, With<FpsPlayer>>()
        .iter(world)
        .collect();
    for entity in entities {
        if let Some(mut effects) = world.get_mut::<ActiveEffectsList>(entity) {
            effects.ledger = runtime.world.effects.clone();
            effects.chem_doses_ms = runtime.world.chem_doses_ms.clone();
        }
        if let Some(mut addictions) = world.get_mut::<Addictions>(entity) {
            addictions.0 = runtime.world.addictions.clone();
        }
        if let Some(mut radiation) = world.get_mut::<PlayerRadiation>(entity) {
            radiation.0 = runtime.world.radiation;
        }
        if let Some(health) = runtime.world.current_health
            && let Some(mut vitals) = world.get_mut::<PlayerVitals>(entity)
        {
            vitals.current_health = health;
        }
    }
}

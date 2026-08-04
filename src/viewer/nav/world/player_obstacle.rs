use bevy::prelude::*;
use bevy_landmass::prelude::*;

use crate::viewer::nav::debug::player::player_transform_query;
use crate::viewer::nav::world::state::NavArchipelagoState;
use crate::viewer::player;

/// Spawns the landmass `Character3d` mirroring the FPS player (issue #114
/// added scope, wave 5): a non-agent RVO obstacle nav agents steer around
/// but that landmass itself never moves. `Character<CS>` requires
/// `Transform`/`Velocity3d` (`bevy_landmass`'s own `#[require(...)]`), so
/// this only needs to seed the bundle plus a starting `Transform` -- an
/// initial placement at the player's current position so the entity is
/// never left at the origin for even one tick; `sync_player_nav_character`
/// takes over every fixed tick after that, before `LandmassSystems::
/// SyncValues` reads it. `player_transform_query` returning `None` (no FPS
/// player yet -- e.g. before `initialize_default_fps` has run) is not an
/// error here: the character still needs to exist so agents already routed
/// have something to sync onto once the player does appear.
pub(crate) fn spawn_player_nav_character(world: &mut World, archipelago_entity: Entity) -> Entity {
    let position = player_transform_query(world).unwrap_or(Vec3::ZERO);
    world
        .spawn((
            Character3dBundle {
                character: default(),
                settings: CharacterSettings {
                    radius: player::CAPSULE_RADIUS,
                },
                archipelago_ref: ArchipelagoRef3d::new(archipelago_entity),
            },
            Transform::from_translation(position),
        ))
        .id()
}

/// Mirrors the FPS player onto its landmass character every fixed tick
/// (issue #114 added scope, wave 5), before `LandmassSystems::SyncValues`
/// reads `Transform`/`Velocity3d`: agents predict and avoid the player using
/// its *actual* post-collision KCC velocity, matching
/// `apply_agent_physics_movement`'s own physics-authoritative feedback
/// convention rather than desired input. A no-op whenever no archipelago has
/// ever been built (`tna spawn` never ran -- the common case) or the FPS
/// player does not currently exist (startup, or a console-harness test
/// world) -- never panics either way.
pub(crate) fn sync_player_nav_character(
    archipelago_state: Res<NavArchipelagoState>,
    mut characters: Query<(&mut Transform, &mut Velocity3d)>,
    players: Query<(&GlobalTransform, &player::KccState), With<player::FpsPlayer>>,
) {
    let Some(character_entity) = archipelago_state.player_character else {
        return;
    };
    let Ok((player_transform, kcc)) = players.single() else {
        return;
    };
    let Ok((mut transform, mut velocity)) = characters.get_mut(character_entity) else {
        return;
    };
    transform.translation = player_transform.translation();
    velocity.velocity = kcc.velocity;
}

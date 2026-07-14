use super::*;
use bevy::ecs::system::RunSystemOnce;

fn spawn_animated_door(world: &mut World, clip_seconds: &[(&str, f32)]) -> (Entity, Entity) {
    let player_entity = world.spawn(AnimationPlayer::default()).id();
    let root_entity = world
        .spawn(AnimatedPlacement::for_test(player_entity, clip_seconds))
        .id();
    (root_entity, player_entity)
}

// F57.2: playing a clip starts it on the placement's `AnimationPlayer` and
// records it as the placement's current clip.
#[test]
fn playing_a_clip_starts_it_on_the_players_animation_player() {
    let mut world = World::new();
    world.init_resource::<Messages<PlayPlacementAnimation>>();
    let (root, player_entity) = spawn_animated_door(&mut world, &[("Open", 1.3), ("Close", 1.3)]);
    world.write_message(PlayPlacementAnimation {
        root,
        transition: ClipTransition::Opening,
        lead_ms: 0.0,
    });
    world
        .run_system_once(play_placement_animations)
        .expect("system should run");

    let animated = world
        .get::<AnimatedPlacement>(root)
        .expect("AnimatedPlacement should still be present");
    assert_eq!(animated.current_clip.as_deref(), Some("Open"));
    let player = world
        .get::<AnimationPlayer>(player_entity)
        .expect("AnimationPlayer should still be present");
    assert!(player.playing_animations().next().is_some());
}

// F57.4: reversing mid-animation stops the previous clip and switches to
// the opposite one -- both clips are never simultaneously active.
#[test]
fn reversing_mid_animation_switches_to_the_opposite_clip() {
    let mut world = World::new();
    world.init_resource::<Messages<PlayPlacementAnimation>>();
    let (root, player_entity) = spawn_animated_door(&mut world, &[("Open", 1.0), ("Close", 1.0)]);
    world.write_message(PlayPlacementAnimation {
        root,
        transition: ClipTransition::Opening,
        lead_ms: 0.0,
    });
    world
        .run_system_once(play_placement_animations)
        .expect("system should run");
    world.write_message(PlayPlacementAnimation {
        root,
        transition: ClipTransition::Closing,
        lead_ms: 0.0,
    });
    world
        .run_system_once(play_placement_animations)
        .expect("system should run");

    let animated = world
        .get::<AnimatedPlacement>(root)
        .expect("AnimatedPlacement should still be present");
    assert_eq!(animated.current_clip.as_deref(), Some("Close"));
    let player = world
        .get::<AnimationPlayer>(player_entity)
        .expect("AnimationPlayer should still be present");
    assert_eq!(player.playing_animations().count(), 1);
}

// F57.2: an activator whose only clip isn't named "Open" still plays it
// (the alphabetically-first fallback from `policy::select_clip`).
#[test]
fn activator_without_an_open_clip_plays_its_only_clip() {
    let mut world = World::new();
    world.init_resource::<Messages<PlayPlacementAnimation>>();
    let (root, player_entity) = spawn_animated_door(&mut world, &[("Activate", 0.5)]);
    world.write_message(PlayPlacementAnimation {
        root,
        transition: ClipTransition::Opening,
        lead_ms: 0.0,
    });
    world
        .run_system_once(play_placement_animations)
        .expect("system should run");

    let animated = world
        .get::<AnimatedPlacement>(root)
        .expect("AnimatedPlacement should still be present");
    assert_eq!(animated.current_clip.as_deref(), Some("Activate"));
    let player = world
        .get::<AnimationPlayer>(player_entity)
        .expect("AnimationPlayer should still be present");
    assert!(player.playing_animations().next().is_some());
}

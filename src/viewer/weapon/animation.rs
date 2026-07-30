//! Authored first-person weapon-part animation discovery and reload control.
//!
//! The prepared weapon GLBs contain short clips on named part nodes. This
//! module keeps the discovery/graph plumbing local to the weapon presentation
//! and drives those clips from the engine-agnostic weapon action progress.

use std::collections::HashMap;

use bevy::animation::graph::{AnimationGraph, AnimationGraphHandle, AnimationNodeIndex};
use bevy::animation::{AnimationClip, AnimationPlayer, RepeatAnimation};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevyout_core::weapon::WeaponAction;

use super::{
    PlayerWeaponRuntime, presentation::WeaponViewmodelRoot, presentation::WeaponViewmodelSource,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(super) enum ReloadPart {
    LaserSideLatch,
    LaserEnergyCell,
    TenMmSlide,
    TenMmClip,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum ReloadPhase {
    #[default]
    Idle,
    Opening,
    Holding,
    Inserting,
    Closing,
}

/// Normalize Blender/glTF names such as `##LPSideLatch:direct` to a stable
/// policy key. The direct-animation suffix is metadata, not part identity.
fn canonical_clip_name(name: &str) -> String {
    name.split(':')
        .next()
        .unwrap_or(name)
        .trim_start_matches('#')
        .to_ascii_lowercase()
}

pub(super) fn classify_reload_part(name: &str) -> Option<ReloadPart> {
    match canonical_clip_name(name).as_str() {
        "lpsidelatch" => Some(ReloadPart::LaserSideLatch),
        "lpsmallenergycell" => Some(ReloadPart::LaserEnergyCell),
        "slide" => Some(ReloadPart::TenMmSlide),
        "clip" => Some(ReloadPart::TenMmClip),
        // Trigger/Hammer and any future firing-only clips are deliberately not
        // part of reload choreography.
        _ => None,
    }
}

pub(super) fn reload_phase(action: WeaponAction, progress: f32) -> ReloadPhase {
    if action != WeaponAction::Reloading {
        return ReloadPhase::Idle;
    }
    let progress = progress.clamp(0.0, 1.0);
    if progress < 0.20 {
        ReloadPhase::Opening
    } else if progress < 0.55 {
        ReloadPhase::Holding
    } else if progress < 0.80 {
        ReloadPhase::Inserting
    } else {
        ReloadPhase::Closing
    }
}

pub(super) fn has_authored_reload_parts(names: &[&str]) -> bool {
    names
        .iter()
        .any(|name| classify_reload_part(name).is_some())
}

#[derive(Clone, Copy)]
struct WeaponClip {
    node: AnimationNodeIndex,
    duration: f32,
}

/// State attached to the active viewmodel root after its GLTF animations have
/// loaded. Holding this state on the root means a weapon swap removes the
/// graph and all clip state together with the scene hierarchy.
#[derive(Component)]
pub(super) struct WeaponViewmodelAnimations {
    player: Entity,
    clips: HashMap<ReloadPart, WeaponClip>,
    #[allow(dead_code)]
    gltf: Handle<Gltf>,
    phase: ReloadPhase,
}

struct PendingWeaponDiscovery {
    player: Entity,
    root: Entity,
    gltf: Handle<Gltf>,
}

#[derive(Resource, Default)]
pub(super) struct PendingWeaponAnimationDiscovery(Vec<PendingWeaponDiscovery>);

/// Find the viewmodel root above a scene-spawned AnimationPlayer.
fn find_viewmodel_root(
    player: Entity,
    parents: &Query<&ChildOf>,
    roots: &Query<(), With<WeaponViewmodelRoot>>,
) -> Option<Entity> {
    let mut current = player;
    for _ in 0..64 {
        if roots.contains(current) {
            return Some(current);
        }
        let Ok(parent) = parents.get(current) else {
            break;
        };
        current = parent.0;
    }
    None
}

pub(super) fn discover_animation_players(
    new_players: Query<Entity, Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    roots: Query<(), With<WeaponViewmodelRoot>>,
    sources: Query<&WeaponViewmodelSource>,
    animated: Query<&WeaponViewmodelAnimations>,
    live_players: Query<(), With<AnimationPlayer>>,
    mut pending: ResMut<PendingWeaponAnimationDiscovery>,
) {
    for player in &new_players {
        let Some(root) = find_viewmodel_root(player, &parents, &roots) else {
            continue;
        };
        if animated
            .get(root)
            .is_ok_and(|state| live_players.contains(state.player))
        {
            continue;
        }
        let Ok(source) = sources.get(root) else {
            continue;
        };
        if pending
            .0
            .iter()
            .any(|entry| entry.root == root && entry.player == player)
        {
            continue;
        }
        pending.0.push(PendingWeaponDiscovery {
            player,
            root,
            gltf: source.0.clone(),
        });
    }
}

pub(super) fn resolve_pending_animation_discovery(
    mut commands: Commands,
    mut pending: ResMut<PendingWeaponAnimationDiscovery>,
    gltfs: Res<Assets<Gltf>>,
    clips: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    live_roots: Query<(), With<WeaponViewmodelRoot>>,
    live_players: Query<(), With<AnimationPlayer>>,
) {
    if pending.0.is_empty() {
        return;
    }
    let mut still_pending = Vec::new();
    for entry in pending.0.drain(..) {
        if !live_roots.contains(entry.root) || !live_players.contains(entry.player) {
            continue;
        }
        let Some(gltf) = gltfs.get(&entry.gltf) else {
            still_pending.push(entry);
            continue;
        };
        if !gltf
            .named_animations
            .values()
            .all(|handle| clips.get(handle).is_some())
        {
            still_pending.push(entry);
            continue;
        }

        let mut names: Vec<String> = gltf
            .named_animations
            .keys()
            .map(ToString::to_string)
            .collect();
        names.sort_unstable();
        let mut graph = AnimationGraph::new();
        let graph_root = graph.root;
        let mut reload_clips = HashMap::new();
        let authored_names: Vec<&str> = names.iter().map(String::as_str).collect();
        let has_authored_parts = has_authored_reload_parts(&authored_names);
        for name in &names {
            let handle = gltf.named_animations[name.as_str()].clone();
            let duration = clips
                .get(&handle)
                .map(AnimationClip::duration)
                .unwrap_or_default();
            let node = graph.add_clip(handle, 1.0, graph_root);
            if has_authored_parts && let Some(part) = classify_reload_part(name) {
                reload_clips.insert(part, WeaponClip { node, duration });
            }
        }

        if !names.is_empty() {
            let graph_handle = graphs.add(graph);
            commands
                .entity(entry.player)
                .insert(AnimationGraphHandle(graph_handle));
        }
        commands
            .entity(entry.root)
            .insert(WeaponViewmodelAnimations {
                player: entry.player,
                clips: reload_clips,
                gltf: entry.gltf,
                phase: ReloadPhase::Idle,
            });
    }
    pending.0 = still_pending;
}

pub(super) fn drive_viewmodel_animations(
    runtime: Res<PlayerWeaponRuntime>,
    mut roots: Query<&mut WeaponViewmodelAnimations>,
    mut players: Query<&mut AnimationPlayer>,
) {
    let (action, progress) = runtime
        .state
        .as_ref()
        .map_or((WeaponAction::Idle, 0.0), |state| {
            (state.action(), state.action_progress())
        });
    let next_phase = reload_phase(action, progress);

    for mut state in &mut roots {
        if state.phase == next_phase {
            continue;
        }
        let Ok(mut player) = players.get_mut(state.player) else {
            continue;
        };

        // A very large frame can skip the early threshold. Always begin a
        // reload by opening/removing the authored parts before applying the
        // phase visible at the current progress.
        if state.phase == ReloadPhase::Idle && next_phase != ReloadPhase::Idle {
            for clip in state.clips.values() {
                play_reverse(&mut player, *clip);
            }
        }
        match next_phase {
            ReloadPhase::Idle => {
                for clip in state.clips.values() {
                    settle_forward(&mut player, *clip);
                }
            }
            ReloadPhase::Opening => {
                for clip in state.clips.values() {
                    play_reverse(&mut player, *clip);
                }
            }
            ReloadPhase::Holding => {}
            ReloadPhase::Inserting => {
                for (part, clip) in &state.clips {
                    if matches!(
                        part,
                        ReloadPart::LaserEnergyCell
                            | ReloadPart::TenMmSlide
                            | ReloadPart::TenMmClip
                    ) {
                        play_forward(&mut player, *clip);
                    }
                }
            }
            ReloadPhase::Closing => {
                for (part, clip) in &state.clips {
                    if *part == ReloadPart::LaserSideLatch {
                        play_forward(&mut player, *clip);
                    } else {
                        settle_forward(&mut player, *clip);
                    }
                }
            }
        }
        state.phase = next_phase;
    }
}

fn play_reverse(player: &mut AnimationPlayer, clip: WeaponClip) {
    player
        .start(clip.node)
        .set_repeat(RepeatAnimation::Never)
        .set_speed(-1.0)
        .seek_to(clip.duration.max(0.0));
}

fn play_forward(player: &mut AnimationPlayer, clip: WeaponClip) {
    player
        .start(clip.node)
        .set_repeat(RepeatAnimation::Never)
        .set_speed(1.0)
        .seek_to(0.0);
}

fn settle_forward(player: &mut AnimationPlayer, clip: WeaponClip) {
    player
        .start(clip.node)
        .set_repeat(RepeatAnimation::Never)
        .set_speed(1.0)
        .set_seek_time(clip.duration.max(0.0));
}

#[cfg(test)]
#[path = "tests/animation.rs"]
mod tests;

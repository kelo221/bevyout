//! Issue #57: discovers a prepared placement's NIF-controller animations
//! (imported as named glTF clips, see `vsa::assets::blender_script.py`'s
//! `NLA_TRACKS` export) and plays them in sync with door/container
//! open-state changes and activator activation.
//!
//! `policy.rs` is a pure, std-only module (clip selection, the travel
//! open-lead, and the mid-animation reversal seek) kept dependency-free like
//! `world::policy` so `tests/features.rs` can include it verbatim.
//! Everything below is the Bevy-side glue: reacting to a spawned scene's
//! `AnimationPlayer`, mapping its `Gltf` asset's `named_animations` onto an
//! `AnimationGraph`, and driving playback from `PlayPlacementAnimation`
//! messages that `interaction.rs` writes on door/container/activator
//! activation.

use bevy::gltf::Gltf;

use super::interaction;
use super::*;

mod policy;

pub(crate) use policy::{ClipTransition, OPEN_LEAD_CAP_SECONDS, open_lead_seconds};

pub(crate) fn install(app: &mut App) {
    app.init_resource::<PendingAnimationDiscovery>()
        .add_message::<PlayPlacementAnimation>()
        .add_systems(
            Update,
            (
                discover_animation_players,
                resolve_pending_animation_discovery,
                play_placement_animations,
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
}

/// F57.2: written by `interaction.rs` on a door/container open-state change
/// or an activator activation. `lead_ms` is purely informational (the
/// `DoorTravelRequested` deferral itself is `interaction.rs`'s own pending
/// state) and only nonzero for a travel door's `Opening` transition.
#[derive(Message, Clone, Copy, Debug)]
pub(crate) struct PlayPlacementAnimation {
    pub(crate) root: Entity,
    pub(crate) transition: ClipTransition,
    pub(crate) lead_ms: f32,
}

/// F57.2: attached to a `PlacementRoot` entity once its scene's animation
/// clips have been discovered and graphed. Placements whose GLB has no
/// animations never get this component, so `interaction.rs` naturally sees
/// "no clip" for them (F57.2's "assets without animations behave exactly as
/// today").
#[derive(Component, Debug)]
pub(crate) struct AnimatedPlacement {
    player: Entity,
    clip_nodes: HashMap<String, AnimationNodeIndex>,
    clip_durations: HashMap<String, f32>,
    current_clip: Option<String>,
    /// Keeps the root `Gltf` asset alive for this placement's lifetime.
    /// Never read: without a resident holder, the root is freed once
    /// discovery resolves, and the next rediscovery's `load::<Gltf>`
    /// re-runs the whole glTF loader — which fires `Modified` for every
    /// subasset, respawns every scene instance of the asset (re-`Added`
    /// `AnimationPlayer`s ⇒ more discoveries), and re-uploads its
    /// meshes/images to the GPU, in a self-sustaining loop (measured at
    /// 16k reloads/10 s in the #55 profile; the A8 "scene respawns" and
    /// A9 "smaller chunks are worse" mysteries were both this).
    #[allow(dead_code)]
    gltf: Option<Handle<Gltf>>,
}

impl AnimatedPlacement {
    /// Duration in seconds of the named clip, if this placement has one.
    /// `interaction.rs` uses this for `Open` to compute a travel door's lead
    /// (F57.3).
    pub(crate) fn clip_seconds(&self, name: &str) -> Option<f32> {
        self.clip_durations.get(name).copied()
    }
}

#[cfg(test)]
impl AnimatedPlacement {
    /// T57.4 cross-module test seam: `interaction::tests`'s door-travel-lead
    /// tests need an `AnimatedPlacement` with a known clip duration without
    /// going through the real `Gltf`/`AnimationClip` discovery pipeline
    /// (`discover_animation_players`/`resolve_pending_animation_discovery`,
    /// which this module's own tests cover instead).
    pub(crate) fn for_test(player: Entity, clip_seconds: &[(&str, f32)]) -> Self {
        let mut clip_nodes = HashMap::new();
        let mut clip_durations = HashMap::new();
        for (index, (name, seconds)) in clip_seconds.iter().enumerate() {
            clip_nodes.insert((*name).to_string(), AnimationNodeIndex::new(index));
            clip_durations.insert((*name).to_string(), *seconds);
        }
        Self {
            player,
            clip_nodes,
            clip_durations,
            current_clip: None,
            gltf: None,
        }
    }
}

struct PendingDiscovery {
    player: Entity,
    root: Entity,
    gltf: Handle<Gltf>,
}

/// Placements whose `AnimationPlayer` has been seen but whose `Gltf` asset
/// (needed for `named_animations`) hasn't finished loading yet.
#[derive(Resource, Default)]
struct PendingAnimationDiscovery(Vec<PendingDiscovery>);

/// F57.2: Bevy's glTF scene spawner adds `AnimationPlayer` to the scene
/// hierarchy on load; walking ancestors from there to the owning
/// `PlacementRoot` (via `interaction::find_placement_root`) is how this
/// reacts without editing `scene.rs`.
fn discover_animation_players(
    new_players: Query<Entity, Added<AnimationPlayer>>,
    parents: Query<&ChildOf>,
    roots: Query<&interaction::PlacementRoot>,
    already_animated: Query<&AnimatedPlacement>,
    live_players: Query<(), With<AnimationPlayer>>,
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingAnimationDiscovery>,
) {
    for player_entity in &new_players {
        let Some(root_entity) = interaction::find_placement_root(player_entity, &parents, &roots)
        else {
            continue;
        };
        // A preloaded cell's scene respawns under a surviving root (evict →
        // re-preload, and the swap's scene rebuild): the recorded player is
        // then a dead entity, so rediscover instead of skipping.
        if already_animated
            .get(root_entity)
            .is_ok_and(|animated| live_players.contains(animated.player))
        {
            continue;
        }
        let Some(path) = roots
            .get(root_entity)
            .ok()
            .and_then(|root| root.placement().asset_path.clone())
        else {
            continue;
        };
        let gltf = asset_server.load::<Gltf>(path);
        pending.0.push(PendingDiscovery {
            player: player_entity,
            root: root_entity,
            gltf,
        });
    }
}

/// Resolves each pending discovery once its `Gltf` asset (and every
/// animation clip it names) has finished loading, building a per-placement
/// `AnimationGraph` from the sorted clip names (deterministic regardless of
/// `named_animations`' `HashMap` iteration order).
fn resolve_pending_animation_discovery(
    mut commands: Commands,
    mut pending: ResMut<PendingAnimationDiscovery>,
    gltfs: Res<Assets<Gltf>>,
    clips: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    if pending.0.is_empty() {
        return;
    }
    let mut still_pending = Vec::new();
    for entry in pending.0.drain(..) {
        let Some(gltf) = gltfs.get(&entry.gltf) else {
            still_pending.push(entry);
            continue;
        };
        let ready = gltf
            .named_animations
            .values()
            .all(|handle| clips.get(handle).is_some());
        if !ready {
            still_pending.push(entry);
            continue;
        }
        let mut names: Vec<String> = gltf
            .named_animations
            .keys()
            .map(|name| name.to_string())
            .collect();
        names.sort();
        if names.is_empty() {
            continue;
        }
        let mut graph = AnimationGraph::new();
        let graph_root = graph.root;
        let mut clip_nodes = HashMap::new();
        let mut clip_durations = HashMap::new();
        for name in &names {
            let handle = gltf.named_animations[name.as_str()].clone();
            let duration = clips
                .get(&handle)
                .map(AnimationClip::duration)
                .unwrap_or(0.0);
            let node = graph.add_clip(handle, 1.0, graph_root);
            clip_nodes.insert(name.clone(), node);
            clip_durations.insert(name.clone(), duration);
        }
        let graph_handle = graphs.add(graph);
        commands
            .entity(entry.player)
            .insert(AnimationGraphHandle(graph_handle));
        commands.entity(entry.root).insert(AnimatedPlacement {
            player: entry.player,
            clip_nodes,
            clip_durations,
            current_clip: None,
            gltf: Some(entry.gltf),
        });
    }
    pending.0 = still_pending;
}

/// F57.2/F57.4: plays the clip `policy::select_clip` picks for each
/// `PlayPlacementAnimation`, holding the final pose (the default
/// `RepeatAnimation::Never` simply stops advancing once finished -- no
/// explicit rewind needed). Re-activating mid-animation stops whatever was
/// playing and seeks the new clip via `policy::reversal_seek_seconds`;
/// re-requesting the clip already held replays it from the start (useful
/// for a repeatable activator).
fn play_placement_animations(
    mut events: MessageReader<PlayPlacementAnimation>,
    mut animated: Query<&mut AnimatedPlacement>,
    mut players: Query<&mut AnimationPlayer>,
    roots: Query<&interaction::PlacementRoot>,
) {
    for event in events.read() {
        let Ok(mut animated) = animated.get_mut(event.root) else {
            // Routine: placements whose GLB has no animations still get
            // play requests from activation.
            continue;
        };
        let mut clip_names: Vec<String> = animated.clip_nodes.keys().cloned().collect();
        clip_names.sort();
        let Some(clip_name) = policy::select_clip(event.transition, &clip_names) else {
            continue;
        };
        let Some(&node) = animated.clip_nodes.get(&clip_name) else {
            continue;
        };
        let new_duration = animated
            .clip_durations
            .get(&clip_name)
            .copied()
            .unwrap_or(0.0);
        let Ok(mut player) = players.get_mut(animated.player) else {
            // Should be unreachable: discovery refreshes a dead player.
            warn!(
                "door anim: stale AnimationPlayer {:?} for root {:?}",
                animated.player, event.root
            );
            continue;
        };

        match animated.current_clip.clone() {
            Some(previous_name) if previous_name == clip_name => {
                if let Some(active) = player.animation_mut(node) {
                    active.replay();
                } else {
                    player.play(node);
                }
            }
            Some(previous_name) => {
                let previous_node = animated.clip_nodes.get(&previous_name).copied();
                let previous_seek = previous_node
                    .and_then(|previous_node| player.animation(previous_node))
                    .map(|active| active.seek_time())
                    .unwrap_or(0.0);
                if let Some(previous_node) = previous_node {
                    player.stop(previous_node);
                }
                let seek = policy::reversal_seek_seconds(new_duration, previous_seek);
                player.play(node).seek_to(seek);
            }
            None => {
                player.play(node);
            }
        }
        animated.current_clip = Some(clip_name.clone());

        let reference_form_id = roots
            .get(event.root)
            .map(|root| root.placement().reference_form_id)
            .unwrap_or_default();
        info!(
            "door anim {:08x} {} lead_ms={:.1}",
            reference_form_id, clip_name, event.lead_ms
        );
    }
}

#[cfg(test)]
#[path = "animation/tests/mod.rs"]
mod tests;

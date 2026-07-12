[bevy](../index.html)::[animation](index.html)

# Function animate\_targets 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#1082-1093)

```rust
pub fn animate_targets(
    par_commands: ParallelCommands<'_, '_>,
    clips: Res<'_, Assets<AnimationClip>>,
    graphs: Res<'_, Assets<AnimationGraph>>,
    threaded_animation_graphs: Res<'_, ThreadedAnimationGraphs>,
    players: Query<'_, '_, (&AnimationPlayer, &AnimationGraphHandle)>,
    targets: Query<'_, '_, (Entity, &AnimationTargetId, &AnimatedBy, EntityMutExcept<'_, '_, (AnimationTargetId, AnimatedBy, AnimationPlayer, AnimationGraphHandle)>), Without<IsResource>>,
    animation_evaluation_state: Local<'_, ThreadLocal<RefCell<AnimationEvaluationState>>>,
)
```

A system that modifies animation targets (e.g. bones in a skinned mesh) according to the currently-playing animations.
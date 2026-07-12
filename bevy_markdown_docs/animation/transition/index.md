[bevy](../../index.html)::[animation](../index.html)

# Module transition 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#18)

Animation transitions.

Please note that this is an unstable temporary API. It may be replaced by a state machine in the future.

## Structs

[AnimationTransition](struct.AnimationTransition.html "struct bevy::animation::transition::AnimationTransition")

An animation that is being faded out as part of a transition

[AnimationTransitions](struct.AnimationTransitions.html "struct bevy::animation::transition::AnimationTransitions")

Manages fade-out of animation blend factors, allowing for smooth transitions between animations.

## Functions

[advance\_transitions](fn.advance_transitions.html "fn bevy::animation::transition::advance_transitions")

A system that alters the weight of currently-playing transitions based on the current time and decline amount.

[expire\_completed\_transitions](fn.expire_completed_transitions.html "fn bevy::animation::transition::expire_completed_transitions")

A system that removed transitions that have completed from the [`AnimationTransitions`](../../prelude/struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions") object.
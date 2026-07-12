[bevy](../index.html)::[animation](index.html)

# Trait AnimationEvent 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_event.rs.html#16)

```rust
pub trait AnimationEvent: Clone + for<'a> Event<Trigger<'a> = AnimationEventTrigger> { }
```

An [`Event`](../prelude/trait.Event.html "trait bevy::prelude::Event") that an [`AnimationPlayer`](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") or an [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId") can trigger when playing an [`AnimationClip`](../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip").

*   If you used [`AnimationClip::add_event`](../prelude/struct.AnimationClip.html#method.add_event "method bevy::prelude::AnimationClip::add_event"), this will be triggered by the [`AnimationPlayer`](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer").
*   If you used [`AnimationClip::add_event_to_target`](../prelude/struct.AnimationClip.html#method.add_event_to_target "method bevy::prelude::AnimationClip::add_event_to_target"), this will be triggered by the [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId").

This trait can be derived.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
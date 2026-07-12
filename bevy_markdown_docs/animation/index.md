[bevy](../index.html)

# Crate animation 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#1-1785)

Animation for the game engine Bevy

## Modules

[animatable](animatable/index.html "mod bevy::animation::animatable")

Traits and type for interpolating between values.

[animation\_curves](animation_curves/index.html "mod bevy::animation::animation_curves")

The [`AnimationCurve`](../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") trait and adaptors that allow curves to implement it.

[gltf\_curves](gltf_curves/index.html "mod bevy::animation::gltf_curves")

Concrete curve structures used to load glTF curves into the animation system.

[graph](graph/index.html "mod bevy::animation::graph")

The animation graph, which allows animations to be blended together.

[prelude](prelude/index.html "mod bevy::animation::prelude")

The animation prelude.

[transition](transition/index.html "mod bevy::animation::transition")

Animation transitions.

## Macros

[animated\_field](macro.animated_field.html "macro bevy::animation::animated_field")

Returns an [`AnimatedField`](../prelude/struct.AnimatedField.html "struct bevy::prelude::AnimatedField") with a given `$component` and `$field`.

## Structs

[ActiveAnimation](struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

An animation that an [`AnimationPlayer`](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") is currently either playing or was playing, but is presently paused.

[AnimatedBy](struct.AnimatedBy.html "struct bevy::animation::AnimatedBy")

A component that links an animated entity to an entity containing an [`AnimationPlayer`](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer"). Typically used alongside the [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId") component - the linked `AnimationPlayer` plays [`AnimationClip`](../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip") assets, and the `AnimationTargetId` identifies which curves in the `AnimationClip` will affect the target entity.

[AnimationClip](struct.AnimationClip.html "struct bevy::animation::AnimationClip")

A list of [`VariableCurve`](../prelude/struct.VariableCurve.html "struct bevy::prelude::VariableCurve")s and the [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")s to which they apply.

[AnimationEvaluationState](struct.AnimationEvaluationState.html "struct bevy::animation::AnimationEvaluationState")

Temporary data that the [`animate_targets`](fn.animate_targets.html "fn bevy::animation::animate_targets") system maintains.

[AnimationEventTrigger](struct.AnimationEventTrigger.html "struct bevy::animation::AnimationEventTrigger")

The [`Trigger`](../ecs/event/trait.Trigger.html "trait bevy::ecs::event::Trigger") implementation for [`AnimationEvent`](trait.AnimationEvent.html "trait bevy::animation::AnimationEvent"). This passes in either the [`AnimationPlayer`](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") or the [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId") context, and uses that to run any observers that target that entity. See [`AnimationEvent`](trait.AnimationEvent.html "trait bevy::animation::AnimationEvent") for when which entity is used.

[AnimationPlayer](struct.AnimationPlayer.html "struct bevy::animation::AnimationPlayer")

Animation controls.

[AnimationPlugin](struct.AnimationPlugin.html "struct bevy::animation::AnimationPlugin")

Adds animation support to an app

[AnimationTargetId](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

A component that identifies which parts of an [`AnimationClip`](../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip") asset can be applied to an entity. Typically used alongside the [`AnimatedBy`](struct.AnimatedBy.html "struct bevy::animation::AnimatedBy") component.

[VariableCurve](struct.VariableCurve.html "struct bevy::animation::VariableCurve")

Contains an [animation curve](../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") which is used to animate a property of an entity.

## Enums

[AnimationEvaluationError](enum.AnimationEvaluationError.html "enum bevy::animation::AnimationEvaluationError")

Why Bevy failed to evaluate an animation.

[RepeatAnimation](enum.RepeatAnimation.html "enum bevy::animation::RepeatAnimation")

Repetition behavior of an animation.

## Statics

[ANIMATION\_TARGET\_NAMESPACE](static.ANIMATION_TARGET_NAMESPACE.html "static bevy::animation::ANIMATION_TARGET_NAMESPACE")

The [UUID namespace](https://en.wikipedia.org/wiki/Universally_unique_identifier#Versions_3_and_5_\(namespace_name-based\)) of animation targets (e.g. bones).

## Traits

[AnimationEvent](trait.AnimationEvent.html "trait bevy::animation::AnimationEvent")

An [`Event`](../prelude/trait.Event.html "trait bevy::prelude::Event") that an [`AnimationPlayer`](../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") or an [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId") can trigger when playing an [`AnimationClip`](../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip").

## Functions

[advance\_animations](fn.advance_animations.html "fn bevy::animation::advance_animations")

A system that advances the time for all playing animations.

[animate\_targets](fn.animate_targets.html "fn bevy::animation::animate_targets")

A system that modifies animation targets (e.g. bones in a skinned mesh) according to the currently-playing animations.

## Type Aliases

[AnimationCurves](type.AnimationCurves.html "type bevy::animation::AnimationCurves")

A mapping from [`AnimationTargetId`](struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId") (e.g. bone in a skinned mesh) to the animation curves.

[AnimationEntityMut](type.AnimationEntityMut.html "type bevy::animation::AnimationEntityMut")

A type alias for [`EntityMutExcept`](../ecs/world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept") as used in animation.

## Derive Macros

[AnimationEvent](derive.AnimationEvent.html "derive bevy::animation::AnimationEvent")

Implements the `AnimationEvent` trait for a type - see the trait docs for an example usage.
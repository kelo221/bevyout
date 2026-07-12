[bevy](../../index.html)::[animation](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#57)

The animation prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[AnimatableCurve](struct.AnimatableCurve.html "struct bevy::animation::prelude::AnimatableCurve")

This type allows the conversion of a [curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") valued in the [property type](../../prelude/trait.AnimatableProperty.html#associatedtype.Property "associated type bevy::prelude::AnimatableProperty::Property") of an [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") into an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") which animates that property.

[AnimatableCurveEvaluator](struct.AnimatableCurveEvaluator.html "struct bevy::animation::prelude::AnimatableCurveEvaluator")

An [`AnimatableCurveEvaluator`](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator") for [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") instances.

[AnimatableKeyframeCurve](struct.AnimatableKeyframeCurve.html "struct bevy::animation::prelude::AnimatableKeyframeCurve")

A [curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") defined by keyframes with values in an [animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") type.

[AnimatedField](struct.AnimatedField.html "struct bevy::animation::prelude::AnimatedField")

A [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") field that can be animated, defined by a function that reads the component and returns the accessed field / property.

[AnimationClip](struct.AnimationClip.html "struct bevy::animation::prelude::AnimationClip")

A list of [`VariableCurve`](../../prelude/struct.VariableCurve.html "struct bevy::prelude::VariableCurve")s and the [`AnimationTargetId`](../struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")s to which they apply.

[AnimationGraph](struct.AnimationGraph.html "struct bevy::animation::prelude::AnimationGraph")

A graph structure that describes how animation clips are to be blended together.

[AnimationGraphAssetLoader](struct.AnimationGraphAssetLoader.html "struct bevy::animation::prelude::AnimationGraphAssetLoader")

An [`AssetLoader`](../../asset/trait.AssetLoader.html "trait bevy::asset::AssetLoader") that can load [`AnimationGraph`](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")s as assets.

[AnimationGraphHandle](struct.AnimationGraphHandle.html "struct bevy::animation::prelude::AnimationGraphHandle")

A [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle") to the [`AnimationGraph`](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph") to be used by the [`AnimationPlayer`](../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer") on the same entity.

[AnimationGraphHandleTemplate](struct.AnimationGraphHandleTemplate.html "struct bevy::animation::prelude::AnimationGraphHandleTemplate")

[AnimationGraphNode](struct.AnimationGraphNode.html "struct bevy::animation::prelude::AnimationGraphNode")

An individual node within an animation graph.

[AnimationPlayer](struct.AnimationPlayer.html "struct bevy::animation::prelude::AnimationPlayer")

Animation controls.

[AnimationPlugin](struct.AnimationPlugin.html "struct bevy::animation::prelude::AnimationPlugin")

Adds animation support to an app

[AnimationTransition](struct.AnimationTransition.html "struct bevy::animation::prelude::AnimationTransition")

An animation that is being faded out as part of a transition

[AnimationTransitions](struct.AnimationTransitions.html "struct bevy::animation::prelude::AnimationTransitions")

Manages fade-out of animation blend factors, allowing for smooth transitions between animations.

[BlendInput](struct.BlendInput.html "struct bevy::animation::prelude::BlendInput")

An individual input for [`Animatable::blend`](../../prelude/trait.Animatable.html#tymethod.blend "associated function bevy::prelude::Animatable::blend").

[NonPathHandleError](struct.NonPathHandleError.html "struct bevy::animation::prelude::NonPathHandleError")

Error for when only path [`Handle`](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")s are supported.

[SerializedAnimationGraph](struct.SerializedAnimationGraph.html "struct bevy::animation::prelude::SerializedAnimationGraph")

A version of [`AnimationGraph`](../../prelude/struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph") suitable for serializing as an asset.

[SerializedAnimationGraphNode](struct.SerializedAnimationGraphNode.html "struct bevy::animation::prelude::SerializedAnimationGraphNode")

A version of [`AnimationGraphNode`](../../prelude/struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode") suitable for serializing as an asset.

[ThreadedAnimationGraph](struct.ThreadedAnimationGraph.html "struct bevy::animation::prelude::ThreadedAnimationGraph")

An acceleration structure for an animation graph that allows Bevy to evaluate it quickly.

[ThreadedAnimationGraphs](struct.ThreadedAnimationGraphs.html "struct bevy::animation::prelude::ThreadedAnimationGraphs")

Acceleration structures for animation graphs that allows Bevy to evaluate them quickly.

[VariableCurve](struct.VariableCurve.html "struct bevy::animation::prelude::VariableCurve")

Contains an [animation curve](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") which is used to animate a property of an entity.

[WeightsCurve](struct.WeightsCurve.html "struct bevy::animation::prelude::WeightsCurve")

This type allows an [`IterableCurve`](../../prelude/iterable/trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve") valued in `f32` to be used as an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") that animates [morph weights](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights").

[WeightsCurveSample](struct.WeightsCurveSample.html "struct bevy::animation::prelude::WeightsCurveSample")

Type indicating that the sampled value from an animation curve is coming from a [`WeightsCurve`](../../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve").

## Enums

[AnimationGraphLoadError](enum.AnimationGraphLoadError.html "enum bevy::animation::prelude::AnimationGraphLoadError")

Errors that can occur when deserializing animation graphs from RON.

[AnimationGraphSaveError](enum.AnimationGraphSaveError.html "enum bevy::animation::prelude::AnimationGraphSaveError")

Errors that can occur when serializing animation graphs to RON.

[AnimationNodeType](enum.AnimationNodeType.html "enum bevy::animation::prelude::AnimationNodeType")

Animation node data specific to the type of node (clip, blend, or add).

[EvaluatorId](enum.EvaluatorId.html "enum bevy::animation::prelude::EvaluatorId")

The [`EvaluatorId`](../../prelude/enum.EvaluatorId.html "enum bevy::prelude::EvaluatorId") is used to look up the [`AnimationCurveEvaluator`](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator") for an [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty"). For a given animated property, this ID should always be the same to allow things like animation blending to occur.

[SerializedAnimationNodeType](enum.SerializedAnimationNodeType.html "enum bevy::animation::prelude::SerializedAnimationNodeType")

A version of [`AnimationNodeType`](../../prelude/enum.AnimationNodeType.html "enum bevy::prelude::AnimationNodeType") suitable for serializing as part of a [`SerializedAnimationGraphNode`](../../prelude/struct.SerializedAnimationGraphNode.html "struct bevy::prelude::SerializedAnimationGraphNode") asset.

## Traits

[Animatable](trait.Animatable.html "trait bevy::animation::prelude::Animatable")

An animatable value type.

[AnimatableProperty](trait.AnimatableProperty.html "trait bevy::animation::prelude::AnimatableProperty")

A trait for exposing a value in an entity so that it can be animated.

[AnimationCompatibleCurve](trait.AnimationCompatibleCurve.html "trait bevy::animation::prelude::AnimationCompatibleCurve")

This trait collects the additional requirements on top of [`Curve<T>`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") needed for a curve to be used as an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve").

[AnimationCurve](trait.AnimationCurve.html "trait bevy::animation::prelude::AnimationCurve")

A low-level trait that provides control over how curves are actually applied to entities by the animation system.

[AnimationCurveEvaluator](trait.AnimationCurveEvaluator.html "trait bevy::animation::prelude::AnimationCurveEvaluator")

A low-level trait for use in [`VariableCurve`](../../prelude/struct.VariableCurve.html "struct bevy::prelude::VariableCurve") that provides fine control over how animations are evaluated.

## Functions

[advance\_transitions](fn.advance_transitions.html "fn bevy::animation::prelude::advance_transitions")

A system that alters the weight of currently-playing transitions based on the current time and decline amount.

[expire\_completed\_transitions](fn.expire_completed_transitions.html "fn bevy::animation::prelude::expire_completed_transitions")

A system that removed transitions that have completed from the [`AnimationTransitions`](../../prelude/struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions") object.

[interpolate\_with\_cubic\_bezier](fn.interpolate_with_cubic_bezier.html "fn bevy::animation::prelude::interpolate_with_cubic_bezier")

Evaluates a cubic Bézier curve at a value `t`, given two endpoints and the derivatives at those endpoints.

## Type Aliases

[AnimationDiGraph](type.AnimationDiGraph.html "type bevy::animation::prelude::AnimationDiGraph")

A type alias for the `petgraph` data structure that defines the animation graph.

[AnimationMask](type.AnimationMask.html "type bevy::animation::prelude::AnimationMask")

The type of an animation mask bitfield.

[AnimationNodeIndex](type.AnimationNodeIndex.html "type bevy::animation::prelude::AnimationNodeIndex")

The index of either an animation or blend node in the animation graph.
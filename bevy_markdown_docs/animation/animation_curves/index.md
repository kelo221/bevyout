[bevy](../../index.html)::[animation](../index.html)

# Module animation\_curves 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#13)

The [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") trait and adaptors that allow curves to implement it.

## Overview

The flow of curves into the animation system generally begins with something that implements the [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") trait. Let’s imagine, for example, that we have some `Curve<Vec3>` that we want to use to animate something. That could be defined in a number of different ways, but let’s imagine that we’ve defined it [using a function](../../prelude/struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve"):

```rust
let wobble_curve = FunctionCurve::new(
    Interval::UNIT,
    |t| { vec3(t.cos(), 0.0, 0.0) },
);
```

Okay, so we have a curve, but the animation system also needs to know, in some way, how the values from this curve should actually be used. That is, it needs to know what to animate! That’s what [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") is for. In particular, what we need to do is take our curve and turn it into an `AnimationCurve` which will be usable by the animation system.

For instance, let’s imagine that we want to use the `Vec3` output from our curve to animate the [translation component of a `Transform`](../../prelude/struct.Transform.html#structfield.translation "field bevy::prelude::Transform::translation"). For this, there is the adaptor [`AnimatableCurve`](../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve"), which wraps any [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") and [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") and turns it into an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") that will use the given curve to animate the entity’s property:

```rust
let wobble_animation = AnimatableCurve::new(animated_field!(Transform::translation), wobble_curve);
```

And finally, this [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") needs to be added to an [`AnimationClip`](../../prelude/struct.AnimationClip.html "struct bevy::prelude::AnimationClip") in order to actually animate something. This is what that looks like:

```rust
let mut animation_clip = AnimationClip::default();
animation_clip.add_curve_to_target(
    animation_target_id,
    wobble_animation,
);
```

## Making animation curves

The overview showed one example, but in general there are a few different ways of going from a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve"), which produces time-related data of some kind, to an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve"), which knows how to apply that data to an entity.

### Animated Fields

The [`animated_field`](../macro.animated_field.html "macro bevy::animation::animated_field") macro (which returns an [`AnimatedField`](../../prelude/struct.AnimatedField.html "struct bevy::prelude::AnimatedField")), in combination with [`AnimatableCurve`](../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve") is the easiest way to make an animation curve (see the example above).

This will select a field on a component and pass it to a [`Curve`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") with a type that matches the field.

### Animatable Properties

Animation of arbitrary aspects of entities can be accomplished using [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") in conjunction with [`AnimatableCurve`](../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve"). See the documentation [there](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") for details.

### Custom [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") and [`AnimationCurveEvaluator`](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")

This is the lowest-level option with the most control, but it is also the most complicated.

## Structs

[AnimatableCurve](struct.AnimatableCurve.html "struct bevy::animation::animation_curves::AnimatableCurve")

This type allows the conversion of a [curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") valued in the [property type](../../prelude/trait.AnimatableProperty.html#associatedtype.Property "associated type bevy::prelude::AnimatableProperty::Property") of an [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") into an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") which animates that property.

[AnimatableCurveEvaluator](struct.AnimatableCurveEvaluator.html "struct bevy::animation::animation_curves::AnimatableCurveEvaluator")

An [`AnimatableCurveEvaluator`](../../prelude/struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator") for [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") instances.

[AnimatableKeyframeCurve](struct.AnimatableKeyframeCurve.html "struct bevy::animation::animation_curves::AnimatableKeyframeCurve")

A [curve](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") defined by keyframes with values in an [animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") type.

[AnimatedField](struct.AnimatedField.html "struct bevy::animation::animation_curves::AnimatedField")

A [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") field that can be animated, defined by a function that reads the component and returns the accessed field / property.

[WeightsCurve](struct.WeightsCurve.html "struct bevy::animation::animation_curves::WeightsCurve")

This type allows an [`IterableCurve`](../../prelude/iterable/trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve") valued in `f32` to be used as an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") that animates [morph weights](../../prelude/struct.MorphWeights.html "struct bevy::prelude::MorphWeights").

[WeightsCurveSample](struct.WeightsCurveSample.html "struct bevy::animation::animation_curves::WeightsCurveSample")

Type indicating that the sampled value from an animation curve is coming from a [`WeightsCurve`](../../prelude/struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve").

## Enums

[EvaluatorId](enum.EvaluatorId.html "enum bevy::animation::animation_curves::EvaluatorId")

The [`EvaluatorId`](../../prelude/enum.EvaluatorId.html "enum bevy::prelude::EvaluatorId") is used to look up the [`AnimationCurveEvaluator`](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator") for an [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty"). For a given animated property, this ID should always be the same to allow things like animation blending to occur.

## Traits

[AnimatableProperty](trait.AnimatableProperty.html "trait bevy::animation::animation_curves::AnimatableProperty")

A trait for exposing a value in an entity so that it can be animated.

[AnimationCompatibleCurve](trait.AnimationCompatibleCurve.html "trait bevy::animation::animation_curves::AnimationCompatibleCurve")

This trait collects the additional requirements on top of [`Curve<T>`](../../prelude/trait.Curve.html "trait bevy::prelude::Curve") needed for a curve to be used as an [`AnimationCurve`](../../prelude/trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve").

[AnimationCurve](trait.AnimationCurve.html "trait bevy::animation::animation_curves::AnimationCurve")

A low-level trait that provides control over how curves are actually applied to entities by the animation system.

[AnimationCurveEvaluator](trait.AnimationCurveEvaluator.html "trait bevy::animation::animation_curves::AnimationCurveEvaluator")

A low-level trait for use in [`VariableCurve`](../../prelude/struct.VariableCurve.html "struct bevy::prelude::VariableCurve") that provides fine control over how animations are evaluated.
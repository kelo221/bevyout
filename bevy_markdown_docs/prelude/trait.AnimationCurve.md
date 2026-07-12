[bevy](../index.html)::[prelude](index.html)

# Trait AnimationCurve 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#566)

```rust
pub trait AnimationCurve:
    Debug
    + Send
    + Sync
    + 'static {
    // Required methods
    fn clone_value(&self) -> Box<dyn AnimationCurve>;
    fn domain(&self) -> Interval;
    fn evaluator_id(&self) -> EvaluatorId<'_>;
    fn create_evaluator(&self) -> Box<dyn AnimationCurveEvaluator>;
    fn apply(
        &self,
        curve_evaluator: &mut (dyn AnimationCurveEvaluator + 'static),
        t: f32,
        weight: f32,
        graph_node: NodeIndex,
    ) -> Result<(), AnimationEvaluationError>;
    fn sample_clamped(&self, t: f32) -> Box<dyn Any>;
}
```

A low-level trait that provides control over how curves are actually applied to entities by the animation system.

Typically, this will not need to be implemented manually, since it is automatically implemented by [`AnimatableCurve`](struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve") and other curves used by the animation system (e.g. those that animate parts of transforms or morph weights). However, this can be implemented manually when `AnimatableCurve` is not sufficiently expressive.

In many respects, this behaves like a type-erased form of [`Curve`](trait.Curve.html "trait bevy::prelude::Curve"), where the output type of the curve is remembered only in the components that are mutated in the implementation of [`apply`](trait.AnimationCurve.html#tymethod.apply "method bevy::prelude::AnimationCurve::apply").

## Required Methods

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#568)

#### fn [clone\_value](#tymethod.clone_value)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [AnimationCurve](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve")\>

Returns a boxed clone of this value.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#571)

#### fn [domain](#tymethod.domain)(&self) -> [Interval](struct.Interval.html "struct bevy::prelude::Interval")

The range of times for which this animation is defined.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#577)

#### fn [evaluator\_id](#tymethod.evaluator_id)(&self) -> [EvaluatorId](enum.EvaluatorId.html "enum bevy::prelude::EvaluatorId")<'\_>

Returns the type ID of the [`AnimationCurveEvaluator`](trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator").

This must match the type returned by [`Self::create_evaluator`](trait.AnimationCurve.html#tymethod.create_evaluator "method bevy::prelude::AnimationCurve::create_evaluator"). It must be a single type that doesn’t depend on the type of the curve.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#585)

#### fn [create\_evaluator](#tymethod.create_evaluator)(&self) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [AnimationCurveEvaluator](trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator")\>

Returns a newly-instantiated [`AnimationCurveEvaluator`](trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator") for use with this curve.

All curve types must return the same type of [`AnimationCurveEvaluator`](trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator"). The returned value must match the type returned by [`Self::evaluator_id`](trait.AnimationCurve.html#tymethod.evaluator_id "method bevy::prelude::AnimationCurve::evaluator_id").

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#601-607)

#### fn [apply](#tymethod.apply)( &self, curve\_evaluator: &mut (dyn [AnimationCurveEvaluator](trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator") + 'static), t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), weight: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html), graph\_node: [NodeIndex](https://docs.rs/petgraph/0.8.3/x86_64-unknown-linux-gnu/petgraph/graph_impl/struct.NodeIndex.html "struct petgraph::graph_impl::NodeIndex"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [AnimationEvaluationError](../animation/enum.AnimationEvaluationError.html "enum bevy::animation::AnimationEvaluationError")\>

Samples the curve at the given time `t`, and pushes the sampled value onto the evaluation stack of the `curve_evaluator`.

The `curve_evaluator` parameter points to the value returned by [`Self::create_evaluator`](trait.AnimationCurve.html#tymethod.create_evaluator "method bevy::prelude::AnimationCurve::create_evaluator"), upcast to an `&mut dyn AnimationCurveEvaluator`. Typically, implementations of [`Self::apply`](trait.AnimationCurve.html#tymethod.apply "method bevy::prelude::AnimationCurve::apply") will want to downcast the `curve_evaluator` parameter to the concrete type [`Self::evaluator_id`](trait.AnimationCurve.html#tymethod.evaluator_id "method bevy::prelude::AnimationCurve::evaluator_id") in order to push values of the appropriate type onto its evaluation stack.

Be sure not to confuse the `t` and `weight` values. The former determines the position at which the _curve_ is sampled, while `weight` ultimately determines how much the _stack values_ will be blended together (see the definition of [`AnimationCurveEvaluator::blend`](trait.AnimationCurveEvaluator.html#tymethod.blend "method bevy::prelude::AnimationCurveEvaluator::blend")).

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#610)

#### fn [sample\_clamped](#tymethod.sample_clamped)(&self, t: [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)) -> [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any")\>

Samples the curve at the given time `t` and returns a Boxed value.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#67-69)

### impl<C> [AnimationCurve](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") for [WeightsCurve](struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>

where C: [IterableCurve](iterable/trait.IterableCurve.html "trait bevy::prelude::iterable::IterableCurve")<[f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\> + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflectable](../reflect/trait.Reflectable.html "trait bevy::reflect::Reflectable"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#347-350)

### impl<P, C> [AnimationCurve](trait.AnimationCurve.html "trait bevy::prelude::AnimationCurve") for [AnimatableCurve](struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>

where P: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [AnimatableProperty](trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), C: [AnimationCompatibleCurve](trait.AnimationCompatibleCurve.html "trait bevy::prelude::AnimationCompatibleCurve")<<P as [AnimatableProperty](trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty")\>::[Property](trait.AnimatableProperty.html#associatedtype.Property "type bevy::prelude::AnimatableProperty::Property")\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),
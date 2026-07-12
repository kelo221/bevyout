[bevy](../../index.html)::[animation](../index.html)::[prelude](index.html)

# Trait AnimatableProperty 

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#190)

```rust
pub trait AnimatableProperty:
    Send
    + Sync
    + 'static {
    type Property: Animatable;

    // Required methods
    fn get_mut<'a>(
        &self,
        entity: &'a mut EntityMutExcept<'_, '_, (AnimationTargetId, AnimatedBy, AnimationPlayer, AnimationGraphHandle)>,
    ) -> Result<&'a mut Self::Property, AnimationEvaluationError>;
    fn evaluator_id(&self) -> EvaluatorId<'_>;
}
```

A trait for exposing a value in an entity so that it can be animated.

`AnimatableProperty` allows any value contained in an entity to be animated as long as it can be obtained by mutable reference. This makes it more flexible than [`animated_field`](../macro.animated_field.html "macro bevy::animation::animated_field").

Here, `AnimatableProperty` is used to animate a value inside an `Option`, returning an error if the option is `None`.

```rust
#[derive(Component)]
struct ExampleComponent {
    power_level: Option<f32>
}

#[derive(Clone)]
struct PowerLevelProperty;

impl AnimatableProperty for PowerLevelProperty {
    type Property = f32;
    fn get_mut<'a>(
        &self,
        entity: &'a mut AnimationEntityMut
    ) -> Result<&'a mut Self::Property, AnimationEvaluationError> {
        let component = entity
            .get_mut::<ExampleComponent>()
            .ok_or(AnimationEvaluationError::ComponentNotPresent(
              TypeId::of::<ExampleComponent>()
            ))?
            .into_inner();
        component.power_level.as_mut().ok_or(AnimationEvaluationError::PropertyNotPresent(
            TypeId::of::<Option<f32>>()
        ))
    }

    fn evaluator_id(&self) -> EvaluatorId {
        EvaluatorId::Type(TypeId::of::<Self>())
    }
}
```

You can then create an [`AnimatableCurve`](../../prelude/struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve") to animate this property like so:

```rust
AnimatableCurve::new(
    PowerLevelProperty,
    AnimatableKeyframeCurve::new([
        (0.0, 0.0),
        (1.0, 9001.0),
    ]).expect("Failed to create power level curve")
);
```

## Required Associated Types

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#192)

#### type [Property](#associatedtype.Property): [Animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable")

The animated property type.

## Required Methods

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#195-198)

#### fn [get\_mut](#tymethod.get_mut)<'a>( &self, entity: &'a mut [EntityMutExcept](../../ecs/world/struct.EntityMutExcept.html "struct bevy::ecs::world::EntityMutExcept")<'\_, '\_, ([AnimationTargetId](../struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId"), [AnimatedBy](../struct.AnimatedBy.html "struct bevy::animation::AnimatedBy"), [AnimationPlayer](../../prelude/struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer"), [AnimationGraphHandle](../../prelude/struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle"))>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<&'a mut Self::[Property](../../prelude/trait.AnimatableProperty.html#associatedtype.Property "type bevy::prelude::AnimatableProperty::Property"), [AnimationEvaluationError](../enum.AnimationEvaluationError.html "enum bevy::animation::AnimationEvaluationError")\>

Retrieves the property from the given `entity`.

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#202)

#### fn [evaluator\_id](#tymethod.evaluator_id)(&self) -> [EvaluatorId](../../prelude/enum.EvaluatorId.html "enum bevy::prelude::EvaluatorId")<'\_>

The [`EvaluatorId`](../../prelude/enum.EvaluatorId.html "enum bevy::prelude::EvaluatorId") used to look up the [`AnimationCurveEvaluator`](../../prelude/trait.AnimationCurveEvaluator.html "trait bevy::prelude::AnimationCurveEvaluator") for this [`AnimatableProperty`](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty"). For a given animated property, this ID should always be the same to allow things like animation blending to occur.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#222-226)

### impl<C, A, F> [AnimatableProperty](../../prelude/trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty") for [AnimatedField](../../prelude/struct.AnimatedField.html "struct bevy::prelude::AnimatedField")<C, A, F>

where C: [Component](../../prelude/trait.Component.html "trait bevy::prelude::Component")<Mutability = [Mutable](../../ecs/component/struct.Mutable.html "struct bevy::ecs::component::Mutable")\>, A: [Animatable](../../prelude/trait.Animatable.html "trait bevy::prelude::Animatable") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug"), F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")([&mut C](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [&mut A](https://doc.rust-lang.org/nightly/std/primitive.reference.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#228)

#### type [Property](#associatedtype.Property) = A
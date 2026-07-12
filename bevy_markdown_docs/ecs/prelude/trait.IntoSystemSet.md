[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Trait IntoSystemSet 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#270)

```rust
pub trait IntoSystemSet<Marker>: Sized {
    type Set: SystemSet;

    // Required method
    fn into_system_set(self) -> Self::Set;
}
```

Types that can be converted into a [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet").

## Usage notes

This trait should only be used as a bound for trait implementations or as an argument to a function. If a system set needs to be returned from a function or stored somewhere, use [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") instead of this trait.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#272)

#### type [Set](#associatedtype.Set): [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")

The type of [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") this instance converts into.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#275)

#### fn [into\_system\_set](#tymethod.into_system_set)(self) -> Self::[Set](../../prelude/trait.IntoSystemSet.html#associatedtype.Set "type bevy::prelude::IntoSystemSet::Set")

Converts this instance to its associated [`SystemSet`](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet") type.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#226)

### impl [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for [ApplyDeferred](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/executor/mod.rs.html#227)

#### type [Set](#associatedtype.Set) = [SystemTypeSet](../schedule/struct.SystemTypeSet.html "struct bevy::ecs::schedule::SystemTypeSet")<[ApplyDeferred](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#303-307)

### impl<Marker, F> [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<(IsExclusiveFunctionSystem, Marker)> for F

where Marker: 'static, <F as [ExclusiveSystemParamFunction](../system/trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>>::[Out](../system/trait.ExclusiveSystemParamFunction.html#associatedtype.Out "type bevy::ecs::system::ExclusiveSystemParamFunction::Out"): [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, F: [ExclusiveSystemParamFunction](../system/trait.ExclusiveSystemParamFunction.html "trait bevy::ecs::system::ExclusiveSystemParamFunction")<Marker>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#309)

#### type [Set](#associatedtype.Set) = [SystemTypeSet](../schedule/struct.SystemTypeSet.html "struct bevy::ecs::schedule::SystemTypeSet")<F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#289-292)

### impl<Marker, F> [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<(IsFunctionSystem, Marker)> for F

where Marker: 'static, F: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[In](../../prelude/trait.SystemParamFunction.html#associatedtype.In "type bevy::prelude::SystemParamFunction::In"): [FromInput](../system/trait.FromInput.html "trait bevy::ecs::system::FromInput")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Out](../../prelude/trait.SystemParamFunction.html#associatedtype.Out "type bevy::prelude::SystemParamFunction::Out"): [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#294)

#### type [Set](#associatedtype.Set) = [SystemTypeSet](../schedule/struct.SystemTypeSet.html "struct bevy::ecs::schedule::SystemTypeSet")<F>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#279)

### impl<S> [IntoSystemSet](../../prelude/trait.IntoSystemSet.html "trait bevy::prelude::IntoSystemSet")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for S

where S: [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#280)

#### type [Set](#associatedtype.Set) = S
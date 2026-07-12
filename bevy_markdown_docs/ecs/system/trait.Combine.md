[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait Combine 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#93)

```rust
pub trait Combine<A, B>where
    A: System,
    B: System,{
    type In: SystemInput;
    type Out;

    // Required method
    fn combine<T>(
        input: <Self::In as SystemInput>::Inner<'_>,
        data: &mut T,
        a: impl FnOnce(<<A as System>::In as SystemInput>::Inner<'_>, &mut T) -> Result<<A as System>::Out, RunSystemError>,
        b: impl FnOnce(<<B as System>::In as SystemInput>::Inner<'_>, &mut T) -> Result<<B as System>::Out, RunSystemError>,
    ) -> Result<Self::Out, RunSystemError>;
}
```

Customizes the behavior of a [`CombinatorSystem`](struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem").

## Examples

```rust
use bevy_ecs::prelude::*;
use bevy_ecs::system::{CombinatorSystem, Combine, RunSystemError};

// A system combinator that performs an exclusive-or (XOR)
// operation on the output of two systems.
pub type Xor<A, B> = CombinatorSystem<XorMarker, A, B>;

// This struct is used to customize the behavior of our combinator.
pub struct XorMarker;

impl<A, B> Combine<A, B> for XorMarker
where
    A: System<In = (), Out = bool>,
    B: System<In = (), Out = bool>,
{
    type In = ();
    type Out = bool;

    fn combine<T>(
        _input: Self::In,
        data: &mut T,
        a: impl FnOnce(A::In, &mut T) -> Result<A::Out, RunSystemError>,
        b: impl FnOnce(B::In, &mut T) -> Result<B::Out, RunSystemError>,
    ) -> Result<Self::Out, RunSystemError> {
        Ok(a((), data).unwrap_or(false) ^ b((), data).unwrap_or(false))
    }
}

app.add_systems(my_system.run_if(Xor::new(
    IntoSystem::into_system(resource_equals(A(1))),
    IntoSystem::into_system(resource_equals(B(1))),
    // The name of the combined system.
    "a ^ b".into(),
)));
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#95)

#### type [In](#associatedtype.In): [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")

The [input](../../prelude/trait.System.html#associatedtype.In "associated type bevy::prelude::System::In") type for a [`CombinatorSystem`](struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#98)

#### type [Out](#associatedtype.Out)

The [output](../../prelude/trait.System.html#associatedtype.Out "associated type bevy::prelude::System::Out") type for a [`CombinatorSystem`](struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#104-109)

#### fn [combine](#tymethod.combine)<T>( input: <Self::[In](trait.Combine.html#associatedtype.In "type bevy::ecs::system::Combine::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, data: [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html), a: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(<<A as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<A as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>, b: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(<<B as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, [&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<B as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Out](trait.Combine.html#associatedtype.Out "type bevy::ecs::system::Combine::Out"), [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

When used in a [`CombinatorSystem`](struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem"), this function customizes how the two composite systems are invoked and their outputs are combined.

See the trait-level docs for [`Combine`](trait.Combine.html "trait bevy::ecs::system::Combine") for an example implementation.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
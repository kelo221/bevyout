[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait Adapt 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#52)

```rust
pub trait Adapt<S>:
    Send
    + Sync
    + 'staticwhere
    S: System,{
    type In: SystemInput;
    type Out;

    // Required method
    fn adapt(
        &mut self,
        input: <Self::In as SystemInput>::Inner<'_>,
        run_system: impl FnOnce(<<S as System>::In as SystemInput>::Inner<'_>) -> Result<<S as System>::Out, RunSystemError>,
    ) -> Result<Self::Out, RunSystemError>;
}
```

Customizes the behavior of an [`AdapterSystem`](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem")

## Examples

```rust
use bevy_ecs::system::{Adapt, AdapterSystem, RunSystemError};

// A system adapter that inverts the result of a system.
// NOTE: Instead of manually implementing this, you can just use `bevy_ecs::schedule::common_conditions::not`.
pub type NotSystem<S> = AdapterSystem<NotMarker, S>;

// This struct is used to customize the behavior of our adapter.
pub struct NotMarker;

impl<S> Adapt<S> for NotMarker
where
    S: System,
    S::Out: std::ops::Not,
{
    type In = S::In;
    type Out = <S::Out as std::ops::Not>::Output;

    fn adapt(
        &mut self,
        input: <Self::In as SystemInput>::Inner<'_>,
        run_system: impl FnOnce(SystemIn<'_, S>) -> Result<S::Out, RunSystemError>,
    ) -> Result<Self::Out, RunSystemError> {
        let result = run_system(input)?;
        Ok(!result)
    }
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#54)

#### type [In](#associatedtype.In): [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")

The [input](../../prelude/trait.System.html#associatedtype.In "associated type bevy::prelude::System::In") type for an [`AdapterSystem`](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#56)

#### type [Out](#associatedtype.Out)

The [output](../../prelude/trait.System.html#associatedtype.Out "associated type bevy::prelude::System::Out") type for an [`AdapterSystem`](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#60-64)

#### fn [adapt](#tymethod.adapt)( &mut self, input: <Self::[In](trait.Adapt.html#associatedtype.In "type bevy::ecs::system::Adapt::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, run\_system: impl [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")(<<S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Out](trait.Adapt.html#associatedtype.Out "type bevy::ecs::system::Adapt::Out"), [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

When used in an [`AdapterSystem`](struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem"), this function customizes how the system is run and how its inputs/outputs are adapted.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#194-197)

### impl<F, S, Out> [Adapt](trait.Adapt.html "trait bevy::ecs::system::Adapt")<S> for F

where F: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(<S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out")) -> Out, S: [System](../../prelude/trait.System.html "trait bevy::prelude::System"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#199)

#### type [In](#associatedtype.In) = <S as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#200)

#### type [Out](#associatedtype.Out) = Out
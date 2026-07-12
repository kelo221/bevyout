[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Trait ReadOnlySystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#206)

```rust
pub unsafe trait ReadOnlySystem: System {
    // Provided method
    fn run_readonly(
        &mut self,
        input: <Self::In as SystemInput>::Inner<'_>,
        world: &World,
    ) -> Result<Self::Out, RunSystemError> { ... }
}
```

[`System`](../../prelude/trait.System.html "trait bevy::prelude::System") types that do not modify the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") when run. This is implemented for any systems whose parameters all implement [`ReadOnlySystemParam`](../system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam").

Note that systems which perform [deferred](../../prelude/trait.System.html#tymethod.apply_deferred "method bevy::prelude::System::apply_deferred") mutations (such as with [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")) may implement this trait.

## Safety

This must only be implemented for system types which do not mutate the `World` when [`System::run_unsafe`](../../prelude/trait.System.html#tymethod.run_unsafe "method bevy::prelude::System::run_unsafe") is called.

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system.rs.html#211-215)

#### fn [run\_readonly](#method.run_readonly)( &mut self, input: <Self::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In") as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'\_>, world: &[World](../../prelude/struct.World.html "struct bevy::prelude::World"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out"), [RunSystemError](../system/enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Runs this system with the given input in the world.

Unlike [`System::run`](../../prelude/trait.System.html#method.run "method bevy::prelude::System::run"), this can be called with a shared reference to the world, since this system is known not to modify the world.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#454-458)

### impl<A, B> [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") for [PipeSystem](../system/struct.PipeSystem.html "struct bevy::ecs::system::PipeSystem")<A, B>

where A: [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem"), B: [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem"), <B as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[In](../../prelude/trait.System.html#associatedtype.In "type bevy::prelude::System::In"): for<'a> [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")<Inner<'a> = <A as [System](../../prelude/trait.System.html "trait bevy::prelude::System")\>::[Out](../../prelude/trait.System.html#associatedtype.Out "type bevy::prelude::System::Out")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/combinator.rs.html#267-271)

### impl<Func, A, B> [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") for [CombinatorSystem](../system/struct.CombinatorSystem.html "struct bevy::ecs::system::CombinatorSystem")<Func, A, B>

where Func: [Combine](../system/trait.Combine.html "trait bevy::ecs::system::Combine")<A, B> + 'static, A: [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem"), B: [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/adapter_system.rs.html#187-190)

### impl<Func, S> [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") for [AdapterSystem](../system/struct.AdapterSystem.html "struct bevy::ecs::system::AdapterSystem")<Func, S>

where Func: [Adapt](../system/trait.Adapt.html "trait bevy::ecs::system::Adapt")<S>, S: [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#772-782)

### impl<Marker, In, Out, F> [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") for [FunctionSystem](../system/struct.FunctionSystem.html "struct bevy::ecs::system::FunctionSystem")<Marker, In, Out, F>

where Marker: 'static, In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, Out: 'static, F: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[In](../../prelude/trait.SystemParamFunction.html#associatedtype.In "type bevy::prelude::SystemParamFunction::In"): [FromInput](../system/trait.FromInput.html "trait bevy::ecs::system::FromInput")<In>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Out](../../prelude/trait.SystemParamFunction.html#associatedtype.Out "type bevy::prelude::SystemParamFunction::Out"): [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>, <F as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Param](../../prelude/trait.SystemParamFunction.html#associatedtype.Param "type bevy::prelude::SystemParamFunction::Param"): [ReadOnlySystemParam](../system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/builder.rs.html#474-483)

### impl<Marker, In, Out, Func, Builder> [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem") for [BuilderSystem](../system/struct.BuilderSystem.html "struct bevy::ecs::system::BuilderSystem")<Marker, In, Out, Func, Builder>

where Marker: 'static, In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, Out: 'static, Func: [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>, <Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[In](../../prelude/trait.SystemParamFunction.html#associatedtype.In "type bevy::prelude::SystemParamFunction::In"): [FromInput](../system/trait.FromInput.html "trait bevy::ecs::system::FromInput")<In>, <Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Out](../../prelude/trait.SystemParamFunction.html#associatedtype.Out "type bevy::prelude::SystemParamFunction::Out"): [IntoResult](../system/trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<Out>, Builder: [SystemParamBuilder](../../prelude/trait.SystemParamBuilder.html "trait bevy::prelude::SystemParamBuilder")<<Func as [SystemParamFunction](../../prelude/trait.SystemParamFunction.html "trait bevy::prelude::SystemParamFunction")<Marker>>::[Param](../../prelude/trait.SystemParamFunction.html#associatedtype.Param "type bevy::prelude::SystemParamFunction::Param")\> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, [FunctionSystem](../system/struct.FunctionSystem.html "struct bevy::ecs::system::FunctionSystem")<Marker, In, Out, Func>: [ReadOnlySystem](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem"),
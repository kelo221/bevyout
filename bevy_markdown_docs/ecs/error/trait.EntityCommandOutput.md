[bevy](../../index.html)::[ecs](../index.html)::[error](index.html)

# Trait EntityCommandOutput 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#47)

```rust
pub trait EntityCommandOutput {
    type Out;
    type Error: Into<BevyError> + From<EntityMutableFetchError>;

    // Required method
    fn into_result(self) -> Result<Self::Out, Self::Error>;
}
```

A trait implemented for types that can be used as the output of an [`EntityCommand`](../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand").

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#49)

#### type [Out](#associatedtype.Out)

The type returned when the command is successfully applied.

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#54)

#### type [Error](#associatedtype.Error): [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\> + [From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html "trait core::convert::From")<[EntityMutableFetchError](../world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")\>

The error type returned when the command fails to apply. The type must be convertible into a [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError") and constructible from an [`EntityMutableFetchError`](../world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#57)

#### fn [into\_result](#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Out](trait.EntityCommandOutput.html#associatedtype.Out "type bevy::ecs::error::EntityCommandOutput::Out"), Self::[Error](trait.EntityCommandOutput.html#associatedtype.Error "type bevy::ecs::error::EntityCommandOutput::Error")\>

Converts the output into a `Result` containing either the successful output or an error.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#60)

### impl [EntityCommandOutput](trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#61)

#### type [Out](#associatedtype.Out) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#62)

#### type [Error](#associatedtype.Error) = [EntityMutableFetchError](../world/error/enum.EntityMutableFetchError.html "enum bevy::ecs::world::error::EntityMutableFetchError")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#65)

#### fn [into\_result](#tymethod.into_result)( self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [EntityCommandOutput](trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput")\>::[Out](trait.EntityCommandOutput.html#associatedtype.Out "type bevy::ecs::error::EntityCommandOutput::Out"), <[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [EntityCommandOutput](trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput")\>::[Error](trait.EntityCommandOutput.html#associatedtype.Error "type bevy::ecs::error::EntityCommandOutput::Error")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#70-72)

### impl<T, E> [EntityCommandOutput](trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#74)

#### type [Out](#associatedtype.Out) = T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#75)

#### type [Error](#associatedtype.Error) = [EntityCommandError](../system/entity_command/enum.EntityCommandError.html "enum bevy::ecs::system::entity_command::EntityCommandError")<E>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#78)

#### fn [into\_result](#tymethod.into_result)( self, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<<[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E> as [EntityCommandOutput](trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput")\>::[Out](trait.EntityCommandOutput.html#associatedtype.Out "type bevy::ecs::error::EntityCommandOutput::Out"), <[Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E> as [EntityCommandOutput](trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput")\>::[Error](trait.EntityCommandOutput.html#associatedtype.Error "type bevy::ecs::error::EntityCommandOutput::Error")\>

## Implementors
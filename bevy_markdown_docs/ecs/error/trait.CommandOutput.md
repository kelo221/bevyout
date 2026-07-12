[bevy](../../index.html)::[ecs](../index.html)::[error](index.html)

# Trait CommandOutput 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#15)

```rust
pub trait CommandOutput: Sized {
    // Required method
    fn to_err(self) -> Option<BevyError>;
}
```

A trait implemented for types that can be used as the output of a [`Command`](../../prelude/trait.Command.html "trait bevy::prelude::Command").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#17)

#### fn [to\_err](#tymethod.to_err)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

Converts the output into an optional [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#37)

### impl [CommandOutput](trait.CommandOutput.html "trait bevy::ecs::error::CommandOutput") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#39)

#### fn [to\_err](#tymethod.to_err)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#30)

### impl [CommandOutput](trait.CommandOutput.html "trait bevy::ecs::error::CommandOutput") for <[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> [!](https://doc.rust-lang.org/nightly/std/primitive.never.html) as FnRet>::Output

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#32)

#### fn [to\_err](#tymethod.to_err)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#20-22)

### impl<T, E> [CommandOutput](trait.CommandOutput.html "trait bevy::ecs::error::CommandOutput") for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, E>

where E: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/command_handling.rs.html#25)

#### fn [to\_err](#tymethod.to_err)(self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

## Implementors
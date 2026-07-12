[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait IntoResult 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#591)

```rust
pub trait IntoResult<Out>: Sized {
    // Required method
    fn into_result(self) -> Result<Out, RunSystemError>;
}
```

A type that may be converted to the output of a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System"). This is used to allow systems to return either a plain value or a [`Result`](../../prelude/type.Result.html "type bevy::prelude::Result").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#593)

#### fn [into\_result](#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Out, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

Converts this type into the system output type.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#618)

### impl [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\> for <[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> [!](https://doc.rust-lang.org/nightly/std/primitive.never.html) as FnRet>::Output

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#619)

#### fn [into\_result](#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#624)

### impl [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)\> for <[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> [!](https://doc.rust-lang.org/nightly/std/primitive.never.html) as FnRet>::Output

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#625)

#### fn [into\_result](#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html), [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#608)

### impl<T> [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#609)

#### fn [into\_result](#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#602)

### impl<T> [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#603)

#### fn [into\_result](#tymethod.into_result)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<T, [RunSystemError](enum.RunSystemError.html "enum bevy::ecs::system::RunSystemError")\>

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/function_system.rs.html#596)

### impl<T> [IntoResult](trait.IntoResult.html "trait bevy::ecs::system::IntoResult")<T> for T
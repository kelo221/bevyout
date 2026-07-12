[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait FromInput 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#63)

```rust
pub trait FromInput<In>: SystemInputwhere
    In: SystemInput,{
    // Required method
    fn from_inner<'i>(inner: <In as SystemInput>::Inner<'i>) -> Self::Inner<'i>;
}
```

A type that may be constructed from the input of a [`System`](../../prelude/trait.System.html "trait bevy::prelude::System"). This is used to allow systems whose first parameter is a `StaticSystemInput<In>` to take an `In` as input, and can be implemented for user types to allow similar conversions.

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#66)

#### fn [from\_inner](#tymethod.from_inner)<'i>(inner: <In as [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput")\>::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'i>) -> Self::[Inner](../../prelude/trait.SystemInput.html#associatedtype.Inner "type bevy::prelude::SystemInput::Inner")<'i>

Converts the system input’s inner representation into this type’s inner representation.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#76)

### impl<'a, In> [FromInput](trait.FromInput.html "trait bevy::ecs::system::FromInput")<In> for [StaticSystemInput](struct.StaticSystemInput.html "struct bevy::ecs::system::StaticSystemInput")<'a, In>

where In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/input.rs.html#69)

### impl<In> [FromInput](trait.FromInput.html "trait bevy::ecs::system::FromInput")<In> for In

where In: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput"),
[bevy](../index.html)::[reflect](index.html)

# Trait Reflectable 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#31)

```rust
pub trait Reflectable:
    Reflect
    + GetTypeRegistration
    + Typed
    + TypePath { }
```

A catch-all trait that is bound by the core reflection traits, useful to simplify reflection-based generic type bounds.

You do _not_ need to implement this trait manually. It is automatically implemented for all types that implement its supertraits. And these supertraits are all automatically derived with the [`Reflect` derive macro](../prelude/derive.Reflect.html "derive bevy::prelude::Reflect").

This should namely be used to bound generic arguments to the necessary traits for reflection. Doing this has the added benefit of reducing migration costs, as a change to the required traits is automatically handled by this trait.

For now, the supertraits of this trait includes:

*   [`Reflect`](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect")
*   [`GetTypeRegistration`](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration")
*   [`Typed`](trait.Typed.html "trait bevy::reflect::Typed")
*   [`TypePath`](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath")

### Example

```rust
#[derive(Reflect)]
struct MyStruct<T: Reflectable> {
    value: T
}
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/reflectable.rs.html#33)

### impl<T> [Reflectable](trait.Reflectable.html "trait bevy::reflect::Reflectable") for T

where T: [Reflect](../prelude/trait.Reflect.html "trait bevy::prelude::Reflect") + [GetTypeRegistration](trait.GetTypeRegistration.html "trait bevy::reflect::GetTypeRegistration") + [Typed](trait.Typed.html "trait bevy::reflect::Typed") + [TypePath](../prelude/trait.TypePath.html "trait bevy::prelude::TypePath"),
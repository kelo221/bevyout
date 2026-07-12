[bevy](../index.html)::[reflect](index.html)

# Macro impl\_reflect 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#737)

```rust
impl_reflect!() { /* proc-macro */ }
```

A replacement for `#[derive(Reflect)]` to be used with foreign types which the definitions of cannot be altered.

This macro is an alternative to [`impl_reflect_opaque!`](macro.impl_reflect_opaque.html "macro bevy::reflect::impl_reflect_opaque") and [`impl_from_reflect_opaque!`](macro.impl_from_reflect_opaque.html "macro bevy::reflect::impl_from_reflect_opaque") which implement foreign types as Opaque types. Note that there is no `impl_from_reflect`, as this macro will do the job of both. This macro implements them using one of the reflect variant traits (`bevy_reflect::{Struct, TupleStruct, Enum}`, etc.), which have greater functionality. The type being reflected must be in scope, as you cannot qualify it in the macro as e.g. `bevy::prelude::Vec3`.

It is necessary to add a `#[type_path = "my_crate::foo"]` attribute to all types.

It may be necessary to add `#[reflect(Default)]` for some types, specifically non-constructible foreign types. Without `Default` reflected for such types, you will usually get an arcane error message and fail to compile. If the type does not implement `Default`, it may not be possible to reflect without extending the macro.

## Example

Implementing `Reflect` for `bevy::prelude::Vec3` as a struct type:

[ⓘ](# "This example is not tested")

```rust
use bevy::prelude::Vec3;

impl_reflect!(
    #[reflect(PartialEq, Serialize, Deserialize, Default)]
    #[type_path = "bevy::prelude"]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32
    }
);
```
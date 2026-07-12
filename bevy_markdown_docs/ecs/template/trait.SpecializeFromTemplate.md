[bevy](../../index.html)::[ecs](../index.html)::[template](index.html)

# Trait SpecializeFromTemplate 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#416)

```rust
pub trait SpecializeFromTemplate: Sized { }
```

This is used to help improve error messages related to [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") specialization. Developers should generally just ignore this trait and read the error message when they encounter it.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
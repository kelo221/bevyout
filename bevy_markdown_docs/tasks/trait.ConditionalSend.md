[bevy](../index.html)::[tasks](index.html)

# Trait ConditionalSend 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#49)

```rust
pub trait ConditionalSend: Send { }
```

Use [`ConditionalSend`](trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") to mark an optional Send trait bound. Useful as on certain platforms (eg. Wasm), futures aren’t Send.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#50)

### impl<T> [ConditionalSend](trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend") for T

where T: [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send"),
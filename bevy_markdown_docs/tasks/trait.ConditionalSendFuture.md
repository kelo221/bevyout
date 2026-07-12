[bevy](../index.html)::[tasks](index.html)

# Trait ConditionalSendFuture 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#56)

```rust
pub trait ConditionalSendFuture: Future + ConditionalSend { }
```

Use [`ConditionalSendFuture`](trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture") for a future with an optional Send trait bound, as on certain platforms (eg. Wasm), futures aren’t Send.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#58)

### impl<T> [ConditionalSendFuture](trait.ConditionalSendFuture.html "trait bevy::tasks::ConditionalSendFuture") for T

where T: [Future](futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future") + [ConditionalSend](trait.ConditionalSend.html "trait bevy::tasks::ConditionalSend"),
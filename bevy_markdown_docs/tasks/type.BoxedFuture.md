[bevy](../index.html)::[tasks](index.html)

# Type Alias BoxedFuture 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#63)

```rust
pub type BoxedFuture<'a, T> = Pin<Box<dyn ConditionalSendFuture<Output = T> + 'a>>;
```

An owned and dynamically typed Future used when you can’t statically type your result or need to add some indirection.

## Aliased Type

```rust
pub struct BoxedFuture<'a, T> { /* private fields */ }
```
[bevy](../../index.html)::[ecs](../index.html)::[error](index.html)

# Type Alias ErrorHandler 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/handler.rs.html#105)

```rust
pub type ErrorHandler = fn(BevyError, ErrorContext);
```

Defines how Bevy reacts to errors.
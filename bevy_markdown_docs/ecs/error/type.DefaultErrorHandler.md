[bevy](../../index.html)::[ecs](../index.html)::[error](index.html)

# Type Alias DefaultErrorHandler 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/handler.rs.html#125)

```rust
pub type DefaultErrorHandler = FallbackErrorHandler;
```

👎Deprecated since 0.19.0:

Renamed to `FallbackErrorHandler`.

Deprecated alias for [`FallbackErrorHandler`](struct.FallbackErrorHandler.html "struct bevy::ecs::error::FallbackErrorHandler").

## Aliased Type

```rust
pub struct DefaultErrorHandler(pub fn(BevyError, ErrorContext));
```

## Tuple Fields

`0: [fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)([BevyError](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError"), [ErrorContext](enum.ErrorContext.html "enum bevy::ecs::error::ErrorContext"))`
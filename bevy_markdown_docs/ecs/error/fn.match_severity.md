[bevy](../../index.html)::[ecs](../index.html)::[error](index.html)

# Function match\_severity 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/handler.rs.html#130)

```rust
pub fn match_severity(err: BevyError, ctx: ErrorContext)
```

Error handler that defers to an error’s [`Severity`](../../prelude/enum.Severity.html "enum bevy::prelude::Severity").
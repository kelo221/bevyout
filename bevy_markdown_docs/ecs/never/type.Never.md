[bevy](../../index.html)::[ecs](../index.html)::[never](index.html)

# Type Alias Never 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/never.rs.html#39)

```rust
pub type Never = <fn() -> ! as FnRet>::Output;
```

A hacky type alias for the `!` (never) type.

This knowingly opts out of rustc’s stability guarantees. Read the module documentation carefully before using this!
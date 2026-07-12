[bevy](../../index.html)::[ecs](../index.html)::[lifecycle](index.html)

# Type Alias ComponentHook 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#80)

```rust
pub type ComponentHook = for<'w> fn(DeferredWorld<'w>, HookContext);
```

The type used for [`Component`](../../prelude/trait.Component.html "trait bevy::prelude::Component") lifecycle hooks such as `on_add`, `on_insert` or `on_remove`.
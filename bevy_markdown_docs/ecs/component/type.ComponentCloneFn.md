[bevy](../../index.html)::[ecs](../index.html)::[component](index.html)

# Type Alias ComponentCloneFn 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/clone.rs.html#7)

```rust
pub type ComponentCloneFn = fn(&SourceComponent<'_>, &mut ComponentCloneCtx<'_, '_>);
```

Function type that can be used to clone a component of an entity.
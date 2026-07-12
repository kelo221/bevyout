[bevy](../../index.html)::[ecs](../index.html)::[component](index.html)

# Function component\_clone\_ignore 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/clone.rs.html#183)

```rust
pub fn component_clone_ignore(
    _source: &SourceComponent<'_>,
    _ctx: &mut ComponentCloneCtx<'_, '_>,
)
```

Noop implementation of component clone handler function.

See [`EntityClonerBuilder`](../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder") for details.
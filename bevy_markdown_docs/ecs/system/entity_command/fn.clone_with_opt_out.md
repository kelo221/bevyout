[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function clone\_with\_opt\_out 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#295-298)

```rust
pub fn clone_with_opt_out(
    target: Entity,
    config: impl FnOnce(&mut EntityClonerBuilder<'_, OptOut>) + Send + Sync + 'static,
) -> impl EntityCommand
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that clones parts of an entity onto another entity, configured through [`EntityClonerBuilder`](../../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

This builder tries to clone every component from the source entity except for components that were explicitly denied, for example by using the [`deny`](../../entity/struct.EntityClonerBuilder.html#method.deny "method bevy::ecs::entity::EntityClonerBuilder::deny") method.

Required components are not considered by denied components and must be explicitly denied as well if desired.
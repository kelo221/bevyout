[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function clone\_with\_opt\_in 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#312-315)

```rust
pub fn clone_with_opt_in(
    target: Entity,
    config: impl FnOnce(&mut EntityClonerBuilder<'_, OptIn>) + Send + Sync + 'static,
) -> impl EntityCommand
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that clones parts of an entity onto another entity, configured through [`EntityClonerBuilder`](../../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

This builder tries to clone every component that was explicitly allowed from the source entity, for example by using the [`allow`](../../entity/struct.EntityClonerBuilder.html#method.allow "method bevy::ecs::entity::EntityClonerBuilder::allow") method.

Required components are also cloned when the target entity does not contain them.
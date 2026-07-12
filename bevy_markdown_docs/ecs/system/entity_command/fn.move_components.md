[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function move\_components 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#343)

```rust
pub fn move_components<B>(target: Entity) -> impl EntityCommandwhere
    B: Bundle,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") moves the specified components of this entity into another entity.

Components with [`Ignore`](../../component/enum.ComponentCloneBehavior.html#variant.Ignore "variant bevy::ecs::component::ComponentCloneBehavior::Ignore") clone behavior will not be moved, while components that have a [`Custom`](../../component/enum.ComponentCloneBehavior.html#variant.Custom "variant bevy::ecs::component::ComponentCloneBehavior::Custom") clone behavior will be cloned using it and then removed from the source entity. All other components will be moved without any other special handling.

Note that this will trigger `on_remove` hooks/observers on this entity and `on_insert`/`on_add` hooks/observers on the target entity.

## Panics

The command will panic when applied if the target entity does not exist.
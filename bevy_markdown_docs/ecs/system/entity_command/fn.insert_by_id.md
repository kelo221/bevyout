[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function insert\_by\_id 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#147-151)

```rust
pub unsafe fn insert_by_id<T>(
    component_id: ComponentId,
    value: T,
    mode: InsertMode,
) -> impl EntityCommandwhere
    T: Send + 'static,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds a dynamic component to an entity.

## Safety

*   [`ComponentId`](../../component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId") must be from the same world as the target entity.
*   `T` must have the same layout as the one passed during `component_id` creation.
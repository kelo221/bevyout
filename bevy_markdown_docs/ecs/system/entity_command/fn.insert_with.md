[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function insert\_with 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#194-196)

```rust
pub fn insert_with<T, F>(
    component_fn: F,
    mode: InsertMode,
) -> impl EntityCommandwhere
    T: Component,
    F: FnOnce() -> T + Send + 'static,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds a component to an entity using some function that returns the component.

The function will only be invoked if the component will actually be inserted. In other words, the function will _not_ be invoked if `mode` is [`InsertMode::Keep`](../../bundle/enum.InsertMode.html#variant.Keep "variant bevy::ecs::bundle::InsertMode::Keep") and the entity already has the component.
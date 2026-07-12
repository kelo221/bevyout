[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Function insert\_from\_world 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#176)

```rust
pub fn insert_from_world<T>(mode: InsertMode) -> impl EntityCommandwhere
    T: Component + FromWorld,
```

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds a component to an entity using the component’s [`FromWorld`](../../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") implementation.

`T::from_world` will only be invoked if the component will actually be inserted. In other words, `T::from_world` will _not_ be invoked if `mode` is [`InsertMode::Keep`](../../bundle/enum.InsertMode.html#variant.Keep "variant bevy::ecs::bundle::InsertMode::Keep") and the entity already has the component.
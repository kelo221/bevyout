[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Function system\_value 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#417-419)

```rust
pub fn system_value<I, O, M>(
    system: impl IntoSystem<I, O, M>,
) -> SystemHandleTemplate<I, O>where
    I: SystemInput + 'static,
    O: 'static,
```

This will create a new [`SystemHandleTemplate`](enum.SystemHandleTemplate.html "enum bevy::ecs::system::SystemHandleTemplate") for the given `system` value. This makes it possible to define systems “inline” in templates / scenes that produce a [`SystemHandle`](enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle").
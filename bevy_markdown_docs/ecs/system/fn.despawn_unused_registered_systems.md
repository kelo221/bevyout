[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Function despawn\_unused\_registered\_systems 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#104-109)

```rust
pub fn despawn_unused_registered_systems(
    despawner: If<Res<'_, RegisteredSystemDespawner>>,
    commands: Commands<'_, '_>,
)
```

A system that despawns any registered system entities whose [`SystemHandle`](enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle") reference count has reached zero.
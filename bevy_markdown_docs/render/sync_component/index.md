[bevy](../../index.html)::[render](../index.html)

# Module sync\_component 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#63)

## Structs

[SyncComponentPlugin](struct.SyncComponentPlugin.html "struct bevy::render::sync_component::SyncComponentPlugin")

Plugin that registers a component for automatic sync to the render world. See [`SyncWorldPlugin`](../sync_world/struct.SyncWorldPlugin.html "struct bevy::render::sync_world::SyncWorldPlugin") for more information.

## Traits

[SyncComponent](trait.SyncComponent.html "trait bevy::render::sync_component::SyncComponent")

Trait that links components from the main world with output components in the render world. It is used by [`SyncComponentPlugin`](struct.SyncComponentPlugin.html "struct bevy::render::sync_component::SyncComponentPlugin").
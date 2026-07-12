[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Trait SystemBuffer 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1052)

```rust
pub trait SystemBuffer:
    FromWorld
    + Send
    + 'static {
    // Required method
    fn queue(&mut self, _system_meta: &SystemMeta, _world: DeferredWorld<'_>);

    // Provided method
    fn apply(&mut self, system_meta: &SystemMeta, world: &mut World) { ... }
}
```

Types that can be used with [`Deferred<T>`](../../prelude/struct.Deferred.html "struct bevy::prelude::Deferred") in systems. This allows storing system-local data which is used to defer [`World`](../../prelude/struct.World.html "struct bevy::prelude::World") mutations.

Types that implement `SystemBuffer` should take care to perform as many computations up-front as possible. Buffers cannot be applied in parallel, so you should try to minimize the time spent in [`SystemBuffer::apply`](trait.SystemBuffer.html#method.apply "method bevy::ecs::system::SystemBuffer::apply").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1061)

#### fn [queue](#tymethod.queue)(&mut self, \_system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), \_world: [DeferredWorld](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld")<'\_>)

Queues any deferred mutations to be applied at the next [`ApplyDeferred`](../../prelude/struct.ApplyDeferred.html "struct bevy::prelude::ApplyDeferred").

To queue structural changes to [`DeferredWorld`](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld"), a command queue of the [`DeferredWorld`](../world/struct.DeferredWorld.html "struct bevy::ecs::world::DeferredWorld") should be used via [`commands`](../world/struct.DeferredWorld.html#method.commands "method bevy::ecs::world::DeferredWorld::commands").

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#1054)

#### fn [apply](#method.apply)(&mut self, system\_meta: &[SystemMeta](struct.SystemMeta.html "struct bevy::ecs::system::SystemMeta"), world: &mut [World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Applies any deferred mutations to the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/world/command_queue.rs.html#337)

### impl [SystemBuffer](trait.SystemBuffer.html "trait bevy::ecs::system::SystemBuffer") for [CommandQueue](../world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/renderer/render_context.rs.html#106)

### impl [SystemBuffer](trait.SystemBuffer.html "trait bevy::ecs::system::SystemBuffer") for [RenderContextState](../../render/renderer/struct.RenderContextState.html "struct bevy::render::renderer::RenderContextState")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#344-347)

### impl<Config, Clear> [SystemBuffer](trait.SystemBuffer.html "trait bevy::ecs::system::SystemBuffer") for [GizmoBuffer](../../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),
[bevy](../index.html)::[scene](index.html)

# Trait SpawnSystem 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn_system.rs.html#6)

```rust
pub trait SpawnSystem {
    // Required method
    fn spawn(self) -> impl FnMut(&mut World);
}
```

Returns a system that spawns the given [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"). This should generally only be added to schedules that run once, such as [`Startup`](../prelude/struct.Startup.html "struct bevy::prelude::Startup").

## Required Methods

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn_system.rs.html#9)

#### fn [spawn](#tymethod.spawn)(self) -> impl [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")(&mut [World](../prelude/struct.World.html "struct bevy::prelude::World"))

Returns a system that spawns the given [`Scene`](../prelude/trait.Scene.html "trait bevy::prelude::Scene"). This should generally only be added to schedules that run once, such as [`Startup`](../prelude/struct.Startup.html "struct bevy::prelude::Startup").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/spawn_system.rs.html#12)

### impl<F, S> [SpawnSystem](../prelude/trait.SpawnSystem.html "trait bevy::prelude::SpawnSystem") for F

where F: [FnMut](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html "trait core::ops::function::FnMut")() -> S + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, S: [Scene](../prelude/trait.Scene.html "trait bevy::prelude::Scene"),
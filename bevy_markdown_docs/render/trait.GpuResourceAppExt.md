[bevy](../index.html)::[render](index.html)

# Trait GpuResourceAppExt 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#224)

```rust
pub trait GpuResourceAppExt {
    // Required method
    fn init_gpu_resource<R>(&mut self) -> &mut Self
       where R: Resource + FromWorld;
}
```

Convenience methods for render-recovery-aware resource initialization.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#233)

#### fn [init\_gpu\_resource](#tymethod.init_gpu_resource)<R>(&mut self) -> &mut Self

where R: [Resource](../prelude/trait.Resource.html "trait bevy::prelude::Resource") + [FromWorld](../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld"),

Causes the provided GPU resource to be re-initialized during [`RenderStartup`](struct.RenderStartup.html "struct bevy::render::RenderStartup").

This is useful when recovering from lost render devices.

Shorthand for:

[ⓘ](# "This example is not tested")

```rust
app.add_systems(RenderStartup, init_gpu_resource::<R>.ambiguous_with_all());
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#236)

### impl [GpuResourceAppExt](trait.GpuResourceAppExt.html "trait bevy::render::GpuResourceAppExt") for [SubApp](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")
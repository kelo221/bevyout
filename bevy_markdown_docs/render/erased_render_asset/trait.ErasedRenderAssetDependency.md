[bevy](../../index.html)::[render](../index.html)::[erased\_render\_asset](index.html)

# Trait ErasedRenderAssetDependency 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#147)

```rust
pub trait ErasedRenderAssetDependency {
    // Required method
    fn register_system(
        render_app: &mut SubApp,
        system: ScheduleConfigs<Box<dyn System<Out = (), In = ()>>>,
    );
}
```

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#148)

#### fn [register\_system](#tymethod.register_system)( render\_app: &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp"), system: [ScheduleConfigs](../../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>, )

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#151)

### impl [ErasedRenderAssetDependency](trait.ErasedRenderAssetDependency.html "trait bevy::render::erased_render_asset::ErasedRenderAssetDependency") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#152)

#### fn [register\_system](#tymethod.register_system)( render\_app: &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp"), system: [ScheduleConfigs](../../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>, )

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#157)

### impl<A> [ErasedRenderAssetDependency](trait.ErasedRenderAssetDependency.html "trait bevy::render::erased_render_asset::ErasedRenderAssetDependency") for A

where A: [ErasedRenderAsset](trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset"),
[bevy](../../index.html)::[render](../index.html)::[render\_asset](index.html)

# Trait RenderAssetDependency 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#157)

```rust
pub trait RenderAssetDependency {
    // Required method
    fn register_system(
        render_app: &mut SubApp,
        system: ScheduleConfigs<Box<dyn System<Out = (), In = ()>>>,
    );
}
```

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#158)

#### fn [register\_system](#tymethod.register_system)( render\_app: &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp"), system: [ScheduleConfigs](../../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>, )

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#161)

### impl [RenderAssetDependency](trait.RenderAssetDependency.html "trait bevy::render::render_asset::RenderAssetDependency") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#162)

#### fn [register\_system](#tymethod.register_system)( render\_app: &mut [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp"), system: [ScheduleConfigs](../../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>, )

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#167)

### impl<A> [RenderAssetDependency](trait.RenderAssetDependency.html "trait bevy::render::render_asset::RenderAssetDependency") for A

where A: [RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset"),
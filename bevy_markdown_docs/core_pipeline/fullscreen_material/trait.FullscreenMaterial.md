[bevy](../../index.html)::[core\_pipeline](../index.html)::[fullscreen\_material](index.html)

# Trait FullscreenMaterial 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_material.rs.html#77-78)

```rust
pub trait FullscreenMaterial:
    Component
    + ExtractComponent
    + Clone
    + Copy
    + ShaderType
    + WriteInto
    + Default {
    // Required method
    fn fragment_shader() -> ShaderRef;

    // Provided methods
    fn schedule() -> impl ScheduleLabel + Clone { ... }
    fn schedule_configs(
        system: ScheduleConfigs<Box<dyn System<Out = (), In = ()>>>,
    ) -> ScheduleConfigs<Box<dyn System<Out = (), In = ()>>> { ... }
}
```

A trait to define a material that will render to the entire screen using a fullscreen triangle.

## Required Methods

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_material.rs.html#81)

#### fn [fragment\_shader](#tymethod.fragment_shader)() -> [ShaderRef](../../shader/enum.ShaderRef.html "enum bevy::shader::ShaderRef")

The shader that will run on the entire screen using a fullscreen triangle.

## Provided Methods

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_material.rs.html#86)

#### fn [schedule](#method.schedule)() -> impl [ScheduleLabel](../../ecs/schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone")

The schedule this effect runs in.

Defaults to [`Core3d`](../struct.Core3d.html "struct bevy::core_pipeline::Core3d") for 3D post-processing effects.

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/fullscreen_material.rs.html#93)

#### fn [schedule\_configs](#method.schedule_configs)( system: [ScheduleConfigs](../../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>, ) -> [ScheduleConfigs](../../ecs/schedule/enum.ScheduleConfigs.html "enum bevy::ecs::schedule::ScheduleConfigs")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [System](../../prelude/trait.System.html "trait bevy::prelude::System")<Out = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), In = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>>>

Configures this effect’s system set and system order.

By default it’s in [`Core3dSystems::PostProcess`](../enum.Core3dSystems.html#variant.PostProcess "variant bevy::core_pipeline::Core3dSystems::PostProcess") and before [`tonemapping`](../tonemapping/fn.tonemapping.html "fn bevy::core_pipeline::tonemapping::tonemapping").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
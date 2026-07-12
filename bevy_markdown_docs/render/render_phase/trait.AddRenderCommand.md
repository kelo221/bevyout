[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait AddRenderCommand 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#351)

```rust
pub trait AddRenderCommand {
    // Required method
    fn add_render_command<P, C>(&mut self) -> &mut Self
       where P: PhaseItem,
             C: RenderCommand<P> + Send + Sync + 'static,
             <C as RenderCommand<P>>::Param: ReadOnlySystemParam;
}
```

Registers a [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") as a [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") function. They are stored inside the [`DrawFunctions`](struct.DrawFunctions.html "struct bevy::render::render_phase::DrawFunctions") resource of the app.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#353-357)

#### fn [add\_render\_command](#tymethod.add_render_command)<P, C>(&mut self) -> &mut Self

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), C: [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <C as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[Param](trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param"): [ReadOnlySystemParam](../../ecs/system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),

Adds the [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") for the specified render phase to the app.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#383)

### impl [AddRenderCommand](trait.AddRenderCommand.html "trait bevy::render::render_phase::AddRenderCommand") for [App](../../prelude/struct.App.html "struct bevy::prelude::App")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#360)

### impl [AddRenderCommand](trait.AddRenderCommand.html "trait bevy::render::render_phase::AddRenderCommand") for [SubApp](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp")
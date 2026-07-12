[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait Draw 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#24)

```rust
pub trait Draw<P>:
    Send
    + Sync
    + 'staticwhere
    P: PhaseItem,{
    // Required method
    fn draw<'w>(
        &mut self,
        world: &'w World,
        pass: &mut TrackedRenderPass<'w>,
        view: Entity,
        item: &P,
    ) -> Result<(), DrawError>;

    // Provided method
    fn prepare(&mut self, world: &World) { ... }
}
```

A draw function used to draw [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")s.

The draw function can retrieve and query the required ECS data from the render world.

This trait can either be implemented directly or implicitly composed out of multiple modular [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")s. For more details and an example see the [`RenderCommand`](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") documentation.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#35-41)

#### fn [draw](#tymethod.draw)<'w>( &mut self, world: &'w [World](../../prelude/struct.World.html "struct bevy::prelude::World"), pass: &mut [TrackedRenderPass](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass")<'w>, view: [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), item: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [DrawError](enum.DrawError.html "enum bevy::render::render_phase::DrawError")\>

Draws a [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") by issuing zero or more `draw` calls via the [`TrackedRenderPass`](struct.TrackedRenderPass.html "struct bevy::render::render_phase::TrackedRenderPass").

## Provided Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#32)

#### fn [prepare](#method.prepare)(&mut self, world: &[World](../../prelude/struct.World.html "struct bevy::prelude::World"))

Prepares the draw function to be used. This is called once and only once before the phase begins. There may be zero or more [`draw`](trait.Draw.html#tymethod.draw "method bevy::render::render_phase::Draw::draw") calls following a call to this function. Implementing this is optional.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/draw.rs.html#310-312)

### impl<P, C> [Draw](trait.Draw.html "trait bevy::render::render_phase::Draw")<P> for [RenderCommandState](struct.RenderCommandState.html "struct bevy::render::render_phase::RenderCommandState")<P, C>

where P: [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem"), C: [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P> + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static, <C as [RenderCommand](trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand")<P>>::[Param](trait.RenderCommand.html#associatedtype.Param "type bevy::render::render_phase::RenderCommand::Param"): [ReadOnlySystemParam](../../ecs/system/trait.ReadOnlySystemParam.html "trait bevy::ecs::system::ReadOnlySystemParam"),
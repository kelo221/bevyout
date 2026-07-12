[bevy](../../index.html)::[render](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#72)

The render prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[ExtractSchedule](struct.ExtractSchedule.html "struct bevy::render::prelude::ExtractSchedule")

Schedule in which data from the main world is ‘extracted’ into the render world.

[ManualTextureViews](struct.ManualTextureViews.html "struct bevy::render::prelude::ManualTextureViews")

Resource that stores manually managed [`ManualTextureView`](../texture/struct.ManualTextureView.html "struct bevy::render::texture::ManualTextureView")s for use as a [`RenderTarget`](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget"). This type dereferences to a `HashMap<ManualTextureViewHandle, ManualTextureView>`. To add a new texture view, pick a new [`ManualTextureViewHandle`](../../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle") and insert it into the map. Then, to render to the view, set a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")s `target` to `RenderTarget::TextureView(handle)`.

[RenderGraph](struct.RenderGraph.html "struct bevy::render::prelude::RenderGraph")

Schedule label for the root render graph schedule. This schedule runs once per frame in the [`render_system`](../renderer/fn.render_system.html "fn bevy::render::renderer::render_system") system and is responsible for driving the entire rendering process.

## Enums

[Msaa](enum.Msaa.html "enum bevy::render::prelude::Msaa")

Component for configuring the number of samples for [Multi-Sample Anti-Aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing) for a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera").

## Traits

[\_](trait._.html "trait bevy::render::prelude::_")
[bevy](../../index.html)::[render](../index.html)

# Module texture 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#65)

## Structs

[CachedTexture](struct.CachedTexture.html "struct bevy::render::texture::CachedTexture")

A cached GPU [`Texture`](../render_resource/struct.Texture.html "struct bevy::render::render_resource::Texture") with corresponding [`TextureView`](../render_resource/struct.TextureView.html "struct bevy::render::render_resource::TextureView").

[ColorAttachment](struct.ColorAttachment.html "struct bevy::render::texture::ColorAttachment")

A wrapper for a [`CachedTexture`](struct.CachedTexture.html "struct bevy::render::texture::CachedTexture") that is used as a [`RenderPassColorAttachment`](../render_resource/struct.RenderPassColorAttachment.html "struct bevy::render::render_resource::RenderPassColorAttachment").

[DefaultImageSampler](struct.DefaultImageSampler.html "struct bevy::render::texture::DefaultImageSampler")

A rendering resource for the default image sampler which is set during renderer initialization.

[DepthAttachment](struct.DepthAttachment.html "struct bevy::render::texture::DepthAttachment")

A wrapper for a [`TextureView`](../render_resource/struct.TextureView.html "struct bevy::render::render_resource::TextureView") that is used as a depth-only [`RenderPassDepthStencilAttachment`](../render_resource/struct.RenderPassDepthStencilAttachment.html "struct bevy::render::render_resource::RenderPassDepthStencilAttachment").

[FallbackImage](struct.FallbackImage.html "struct bevy::render::texture::FallbackImage")

A [`RenderApp`](../struct.RenderApp.html "struct bevy::render::RenderApp") resource that contains the default “fallback image”, which can be used in situations where an image was not explicitly defined. The most common use case is [`AsBindGroup`](../render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") implementations (such as materials) that support optional textures.

[FallbackImageCubemap](struct.FallbackImageCubemap.html "struct bevy::render::texture::FallbackImageCubemap")

A [`RenderApp`](../struct.RenderApp.html "struct bevy::render::RenderApp") resource that contains a “cubemap fallback image”, which can be used in situations where an image was not explicitly defined. The most common use case is [`AsBindGroup`](../render_resource/trait.AsBindGroup.html "trait bevy::render::render_resource::AsBindGroup") implementations (such as materials) that support optional textures.

[FallbackImageFormatMsaaCache](struct.FallbackImageFormatMsaaCache.html "struct bevy::render::texture::FallbackImageFormatMsaaCache")

A Cache of fallback textures that uses the sample count and `TextureFormat` as a key

[FallbackImageMsaa](struct.FallbackImageMsaa.html "struct bevy::render::texture::FallbackImageMsaa")

[FallbackImageZero](struct.FallbackImageZero.html "struct bevy::render::texture::FallbackImageZero")

A [`RenderApp`](../struct.RenderApp.html "struct bevy::render::RenderApp") resource that contains a _zero-filled_ “fallback image”, which can be used in place of [`FallbackImage`](struct.FallbackImage.html "struct bevy::render::texture::FallbackImage"), when a fully transparent or black fallback is required instead of fully opaque white.

[GpuImage](struct.GpuImage.html "struct bevy::render::texture::GpuImage")

The GPU-representation of an [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image"). Consists of the [`Texture`](../render_resource/struct.Texture.html "struct bevy::render::render_resource::Texture"), its [`TextureView`](../render_resource/struct.TextureView.html "struct bevy::render::render_resource::TextureView") and the corresponding [`Sampler`](../render_resource/struct.Sampler.html "struct bevy::render::render_resource::Sampler"), and the texture’s size.

[ManualTextureView](struct.ManualTextureView.html "struct bevy::render::texture::ManualTextureView")

A manually managed [`TextureView`](../render_resource/struct.TextureView.html "struct bevy::render::render_resource::TextureView") for use as a [`bevy_camera::RenderTarget`](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget").

[ManualTextureViews](struct.ManualTextureViews.html "struct bevy::render::texture::ManualTextureViews")

Resource that stores manually managed [`ManualTextureView`](struct.ManualTextureView.html "struct bevy::render::texture::ManualTextureView")s for use as a [`RenderTarget`](../../camera/enum.RenderTarget.html "enum bevy::camera::RenderTarget"). This type dereferences to a `HashMap<ManualTextureViewHandle, ManualTextureView>`. To add a new texture view, pick a new [`ManualTextureViewHandle`](../../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle") and insert it into the map. Then, to render to the view, set a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera")s `target` to `RenderTarget::TextureView(handle)`.

[OutputColorAttachment](struct.OutputColorAttachment.html "struct bevy::render::texture::OutputColorAttachment")

A wrapper for a [`TextureView`](../render_resource/struct.TextureView.html "struct bevy::render::render_resource::TextureView") that is used as a [`RenderPassColorAttachment`](../render_resource/struct.RenderPassColorAttachment.html "struct bevy::render::render_resource::RenderPassColorAttachment") for a view target’s final output texture.

[TextureCache](struct.TextureCache.html "struct bevy::render::texture::TextureCache")

This resource caches textures that are created repeatedly in the rendering process and are only required for one frame.

[TexturePlugin](struct.TexturePlugin.html "struct bevy::render::texture::TexturePlugin")

## Functions

[update\_texture\_cache\_system](fn.update_texture_cache_system.html "fn bevy::render::texture::update_texture_cache_system")

Updates the [`TextureCache`](struct.TextureCache.html "struct bevy::render::texture::TextureCache") to only retains recently used textures.
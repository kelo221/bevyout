[bevy](../../index.html)::[render](../index.html)::[camera](index.html)

# Trait NormalizedRenderTargetExt 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#195)

```rust
pub trait NormalizedRenderTargetExt {
    // Required methods
    fn get_texture_view<'a>(
        &self,
        windows: &'a ExtractedWindows,
        images: &'a RenderAssets<GpuImage>,
        manual_texture_views: &'a ManualTextureViews,
    ) -> Option<&'a TextureView>;
    fn get_texture_view_format<'a>(
        &self,
        windows: &'a ExtractedWindows,
        images: &'a RenderAssets<GpuImage>,
        manual_texture_views: &'a ManualTextureViews,
    ) -> Option<TextureFormat>;
    fn get_render_target_info<'a>(
        &self,
        resolutions: impl IntoIterator<Item = (Entity, &'a Window)>,
        images: &Assets<Image>,
        manual_texture_views: &ManualTextureViews,
    ) -> Result<RenderTargetInfo, MissingRenderTargetInfoError>;
    fn is_changed(
        &self,
        changed_window_ids: &EntityHashSet,
        changed_image_handles: &HashSet<&AssetId<Image>>,
    ) -> bool;
}
```

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#196-201)

#### fn [get\_texture\_view](#tymethod.get_texture_view)<'a>( &self, windows: &'a [ExtractedWindows](../view/struct.ExtractedWindows.html "struct bevy::render::view::ExtractedWindows"), images: &'a [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>, manual\_texture\_views: &'a [ManualTextureViews](../../prelude/struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [TextureView](../render_resource/struct.TextureView.html "struct bevy::render::render_resource::TextureView")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#204-209)

#### fn [get\_texture\_view\_format](#tymethod.get_texture_view_format)<'a>( &self, windows: &'a [ExtractedWindows](../view/struct.ExtractedWindows.html "struct bevy::render::view::ExtractedWindows"), images: &'a [RenderAssets](../render_asset/struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")<[GpuImage](../texture/struct.GpuImage.html "struct bevy::render::texture::GpuImage")\>, manual\_texture\_views: &'a [ManualTextureViews](../../prelude/struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[TextureFormat](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat")\>

Retrieves the [`TextureFormat`](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") of this render target, if it exists.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#211-216)

#### fn [get\_render\_target\_info](#tymethod.get_render_target_info)<'a>( &self, resolutions: impl [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")<Item = ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), &'a [Window](../../prelude/struct.Window.html "struct bevy::prelude::Window"))>, images: &[Assets](../../prelude/struct.Assets.html "struct bevy::prelude::Assets")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>, manual\_texture\_views: &[ManualTextureViews](../../prelude/struct.ManualTextureViews.html "struct bevy::prelude::ManualTextureViews"), ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[RenderTargetInfo](../../camera/struct.RenderTargetInfo.html "struct bevy::camera::RenderTargetInfo"), [MissingRenderTargetInfoError](enum.MissingRenderTargetInfoError.html "enum bevy::render::camera::MissingRenderTargetInfoError")\>

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#219-223)

#### fn [is\_changed](#tymethod.is_changed)( &self, changed\_window\_ids: &[EntityHashSet](../../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet"), changed\_image\_handles: &[HashSet](../../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<&[AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<[Image](../../prelude/struct.Image.html "struct bevy::prelude::Image")\>>, ) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#226)

### impl [NormalizedRenderTargetExt](../../prelude/trait._.html "trait bevy::prelude::_") for [NormalizedRenderTarget](../../camera/enum.NormalizedRenderTarget.html "enum bevy::camera::NormalizedRenderTarget")
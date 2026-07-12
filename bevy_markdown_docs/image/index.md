[bevy](../index.html)

# Crate image 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/lib.rs.html#1-60)

The Bevy game engine’s GPU-oriented image type.

## Modules

[prelude](prelude/index.html "mod bevy::image::prelude")

The image prelude.

## Structs

[CompressedImageFormatSupport](struct.CompressedImageFormatSupport.html "struct bevy::image::CompressedImageFormatSupport")

For defining which compressed image formats are supported. This will be initialized from available device features in `finish()` of the bevy `RenderPlugin`, but is left for the user to specify if not using the `RenderPlugin`, or the WGPU backend.

[CompressedImageFormats](struct.CompressedImageFormats.html "struct bevy::image::CompressedImageFormats")

A set of flags describing the compressed formats supported by a render device.

[CompressedImageSaver](struct.CompressedImageSaver.html "struct bevy::image::CompressedImageSaver")

An [`AssetSaver`](../asset/saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") that writes compressed basis universal (.ktx2) files.

[DynamicTextureAtlasBuilder](struct.DynamicTextureAtlasBuilder.html "struct bevy::image::DynamicTextureAtlasBuilder")

Helper utility to update [`TextureAtlasLayout`](../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout") on the fly.

[ExrTextureLoader](struct.ExrTextureLoader.html "struct bevy::image::ExrTextureLoader")`exr`

Loads EXR textures as Texture assets

[ExrTextureLoaderSettings](struct.ExrTextureLoaderSettings.html "struct bevy::image::ExrTextureLoaderSettings")`exr`

Settings for [`ExrTextureLoader`](struct.ExrTextureLoader.html "struct bevy::image::ExrTextureLoader").

[FileTextureError](struct.FileTextureError.html "struct bevy::image::FileTextureError")

An error that occurs when loading a texture from a file.

[HdrTextureLoader](struct.HdrTextureLoader.html "struct bevy::image::HdrTextureLoader")

Loads HDR textures as Texture assets

[HdrTextureLoaderSettings](struct.HdrTextureLoaderSettings.html "struct bevy::image::HdrTextureLoaderSettings")

Settings for [`HdrTextureLoader`](struct.HdrTextureLoader.html "struct bevy::image::HdrTextureLoader").

[Image](struct.Image.html "struct bevy::image::Image")

An image, optimized for usage in rendering.

[ImageLoader](struct.ImageLoader.html "struct bevy::image::ImageLoader")

Loader for images that can be read by the `image` crate.

[ImageLoaderSettings](struct.ImageLoaderSettings.html "struct bevy::image::ImageLoaderSettings")

Settings for loading an [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") using an [`ImageLoader`](struct.ImageLoader.html "struct bevy::image::ImageLoader").

[ImagePlugin](struct.ImagePlugin.html "struct bevy::image::ImagePlugin")

Adds the [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") as an asset and makes sure that they are extracted and prepared for the GPU.

[ImageSamplerDescriptor](struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor")

Indicates to an `ImageLoader` how an [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") should be sampled.

[ImageSaver](struct.ImageSaver.html "struct bevy::image::ImageSaver")

[`AssetSaver`](../asset/saver/trait.AssetSaver.html "trait bevy::asset::saver::AssetSaver") for images that can be saved by the `image` crate.

[ImageSaverSettings](struct.ImageSaverSettings.html "struct bevy::image::ImageSaverSettings")

Settings for how to save an image.

[SerializedImage](struct.SerializedImage.html "struct bevy::image::SerializedImage")

A version of [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") suitable for serializing for short-term transfer.

[TextureAtlas](struct.TextureAtlas.html "struct bevy::image::TextureAtlas")

An index into a [`TextureAtlasLayout`](../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout"), which corresponds to a specific section of a texture.

[TextureAtlasBuilder](struct.TextureAtlasBuilder.html "struct bevy::image::TextureAtlasBuilder")

A builder which is used to create a texture atlas from many individual sprites.

[TextureAtlasLayout](struct.TextureAtlasLayout.html "struct bevy::image::TextureAtlasLayout")

Stores a map used to lookup the position of a texture in a [`TextureAtlas`](../prelude/struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas"). This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.

[TextureAtlasPlugin](struct.TextureAtlasPlugin.html "struct bevy::image::TextureAtlasPlugin")

Adds support for texture atlases.

[TextureAtlasSources](struct.TextureAtlasSources.html "struct bevy::image::TextureAtlasSources")

Stores a mapping from sub texture handles to the related area index.

[TextureAtlasTemplate](struct.TextureAtlasTemplate.html "struct bevy::image::TextureAtlasTemplate")

## Enums

[CompressedImageSaverError](enum.CompressedImageSaverError.html "enum bevy::image::CompressedImageSaverError")

Errors encountered when writing compressed images.

[DynamicTextureAtlasBuilderError](enum.DynamicTextureAtlasBuilderError.html "enum bevy::image::DynamicTextureAtlasBuilderError")

An error produced by [`DynamicTextureAtlasBuilder`](../prelude/struct.DynamicTextureAtlasBuilder.html "struct bevy::prelude::DynamicTextureAtlasBuilder") when trying to add a new texture to a [`TextureAtlasLayout`](../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout").

[ExrTextureLoaderError](enum.ExrTextureLoaderError.html "enum bevy::image::ExrTextureLoaderError")`exr`

Possible errors that can be produced by [`ExrTextureLoader`](struct.ExrTextureLoader.html "struct bevy::image::ExrTextureLoader")

[HdrTextureLoaderError](enum.HdrTextureLoaderError.html "enum bevy::image::HdrTextureLoaderError")

Possible errors that can be produced by [`HdrTextureLoader`](struct.HdrTextureLoader.html "struct bevy::image::HdrTextureLoader")

[ImageAddressMode](enum.ImageAddressMode.html "enum bevy::image::ImageAddressMode")

How edges should be handled in texture addressing.

[ImageArrayLayout](enum.ImageArrayLayout.html "enum bevy::image::ImageArrayLayout")

How to interpret the image as an array of textures.

[ImageCompareFunction](enum.ImageCompareFunction.html "enum bevy::image::ImageCompareFunction")

Comparison function used for depth and stencil operations.

[ImageFilterMode](enum.ImageFilterMode.html "enum bevy::image::ImageFilterMode")

Texel mixing mode when sampling between texels.

[ImageFormat](enum.ImageFormat.html "enum bevy::image::ImageFormat")

The format of an on-disk image asset.

[ImageFormatSetting](enum.ImageFormatSetting.html "enum bevy::image::ImageFormatSetting")

How to determine an image’s format when loading.

[ImageLoaderError](enum.ImageLoaderError.html "enum bevy::image::ImageLoaderError")

An error when loading an image using [`ImageLoader`](struct.ImageLoader.html "struct bevy::image::ImageLoader").

[ImageSampler](enum.ImageSampler.html "enum bevy::image::ImageSampler")

Used in [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image"), this determines what image sampler to use when rendering. The default setting, [`ImageSampler::Default`](enum.ImageSampler.html#variant.Default "variant bevy::image::ImageSampler::Default"), will read the sampler from the `ImagePlugin` at setup. Setting this to [`ImageSampler::Descriptor`](enum.ImageSampler.html#variant.Descriptor "variant bevy::image::ImageSampler::Descriptor") will override the global default descriptor for this [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image").

[ImageSamplerBorderColor](enum.ImageSamplerBorderColor.html "enum bevy::image::ImageSamplerBorderColor")

Color variation to use when the sampler addressing mode is [`ImageAddressMode::ClampToBorder`](enum.ImageAddressMode.html#variant.ClampToBorder "variant bevy::image::ImageAddressMode::ClampToBorder").

[ImageType](enum.ImageType.html "enum bevy::image::ImageType")

The type of a raw image buffer.

[IntoDynamicImageError](enum.IntoDynamicImageError.html "enum bevy::image::IntoDynamicImageError")

Errors that occur while converting an [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") into a [`DynamicImage`](https://docs.rs/image/0.25.9/x86_64-unknown-linux-gnu/image/images/dynimage/enum.DynamicImage.html "enum image::images::dynimage::DynamicImage")

[SaveImageError](enum.SaveImageError.html "enum bevy::image::SaveImageError")

An error while saving an image.

[SaveImageFormatSetting](enum.SaveImageFormatSetting.html "enum bevy::image::SaveImageFormatSetting")

The setting for how to choose which file-format to use.

[TextureAccessError](enum.TextureAccessError.html "enum bevy::image::TextureAccessError")

An error that occurs when accessing specific pixels in a texture.

[TextureAtlasBuilderError](enum.TextureAtlasBuilderError.html "enum bevy::image::TextureAtlasBuilderError")

Errors returned by [`TextureAtlasBuilder`](../prelude/struct.TextureAtlasBuilder.html "struct bevy::prelude::TextureAtlasBuilder").

[TextureChannelLayout](enum.TextureChannelLayout.html "enum bevy::image::TextureChannelLayout")

A [UASTC](https://github.com/BinomialLLC/basis_universal/wiki/UASTC-Texture-Specification/b624c07ad3c659e7b0f0badcb36e9a6b8820a99d) texture channel layout

[TextureError](enum.TextureError.html "enum bevy::image::TextureError")

An error that occurs when loading a texture.

[TextureReinterpretationError](enum.TextureReinterpretationError.html "enum bevy::image::TextureReinterpretationError")

An error that occurs when reinterpreting an image.

[TranscodeFormat](enum.TranscodeFormat.html "enum bevy::image::TranscodeFormat")

Texture data need to be transcoded from this format for use with `wgpu`.

## Constants

[TRANSPARENT\_IMAGE\_HANDLE](constant.TRANSPARENT_IMAGE_HANDLE.html "constant bevy::image::TRANSPARENT_IMAGE_HANDLE")

A handle to a 1 x 1 transparent white image.

## Traits

[BevyDefault](trait.BevyDefault.html "trait bevy::image::BevyDefault")Deprecated

Trait used to provide default values for Bevy-external types that do not implement [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default").

[TextureFormatPixelInfo](trait.TextureFormatPixelInfo.html "trait bevy::image::TextureFormatPixelInfo")

Extends the wgpu [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") with information about the pixel.

[TextureSrgbViewFormats](trait.TextureSrgbViewFormats.html "trait bevy::image::TextureSrgbViewFormats")

Trait used to provide texture srgb view formats with static lifetime for `TextureDescriptor.view_formats`.

[ToExtents](trait.ToExtents.html "trait bevy::image::ToExtents")

A trait for creating [`Extent3d`](../render/render_resource/struct.Extent3d.html "struct bevy::render::render_resource::Extent3d") values.

## Functions

[dds\_buffer\_to\_image](fn.dds_buffer_to_image.html "fn bevy::image::dds_buffer_to_image")`dds`

Converts DDS bytes to a bevy [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") using the given compressed format support.

[dds\_format\_to\_texture\_format](fn.dds_format_to_texture_format.html "fn bevy::image::dds_format_to_texture_format")`dds`

Gets a [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") from a [`Dds`](https://docs.rs/ddsfile/0.5.2/x86_64-unknown-linux-gnu/ddsfile/struct.Dds.html "struct ddsfile::Dds") file.

[get\_transcoded\_formats](fn.get_transcoded_formats.html "fn bevy::image::get_transcoded_formats")`basis-universal`

Determines an appropriate wgpu-compatible format based on compressed format support, and a basis universal [`TextureChannelLayout`](enum.TextureChannelLayout.html "enum bevy::image::TextureChannelLayout").

[ktx2\_buffer\_to\_image](fn.ktx2_buffer_to_image.html "fn bevy::image::ktx2_buffer_to_image")`ktx2`

Converts KTX2 bytes to a bevy [`Image`](../prelude/struct.Image.html "struct bevy::prelude::Image") using the given compressed format support.

[ktx2\_dfd\_header\_to\_texture\_format](fn.ktx2_dfd_header_to_texture_format.html "fn bevy::image::ktx2_dfd_header_to_texture_format")`ktx2`

Reads the [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") from a KTX2 data format descriptor header.

[ktx2\_format\_to\_texture\_format](fn.ktx2_format_to_texture_format.html "fn bevy::image::ktx2_format_to_texture_format")`ktx2`

Converts a KTX2 texture format identifier to a [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat").

[ktx2\_get\_texture\_format](fn.ktx2_get_texture_format.html "fn bevy::image::ktx2_get_texture_format")`ktx2`

Reads the [`TextureFormat`](../render/render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") from a [`ktx2::Reader`](https://docs.rs/ktx2/0.5.0/x86_64-unknown-linux-gnu/ktx2/struct.Reader.html "struct ktx2::Reader").

## Type Aliases

[TextureAtlasBuilderResult](type.TextureAtlasBuilderResult.html "type bevy::image::TextureAtlasBuilderResult")

The [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") type used by [`TextureAtlasBuilder`](../prelude/struct.TextureAtlasBuilder.html "struct bevy::prelude::TextureAtlasBuilder").
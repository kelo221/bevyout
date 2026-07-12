[bevy](../../index.html)::[image](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/lib.rs.html#6)

The image prelude.

## Structs

[DynamicTextureAtlasBuilder](struct.DynamicTextureAtlasBuilder.html "struct bevy::image::prelude::DynamicTextureAtlasBuilder")

Helper utility to update [`TextureAtlasLayout`](../../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout") on the fly.

[Image](struct.Image.html "struct bevy::image::prelude::Image")

An image, optimized for usage in rendering.

[ImagePlugin](struct.ImagePlugin.html "struct bevy::image::prelude::ImagePlugin")

Adds the [`Image`](../../prelude/struct.Image.html "struct bevy::prelude::Image") as an asset and makes sure that they are extracted and prepared for the GPU.

[TextureAtlas](struct.TextureAtlas.html "struct bevy::image::prelude::TextureAtlas")

An index into a [`TextureAtlasLayout`](../../prelude/struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout"), which corresponds to a specific section of a texture.

[TextureAtlasBuilder](struct.TextureAtlasBuilder.html "struct bevy::image::prelude::TextureAtlasBuilder")

A builder which is used to create a texture atlas from many individual sprites.

[TextureAtlasLayout](struct.TextureAtlasLayout.html "struct bevy::image::prelude::TextureAtlasLayout")

Stores a map used to lookup the position of a texture in a [`TextureAtlas`](../../prelude/struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas"). This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.

[TextureAtlasSources](struct.TextureAtlasSources.html "struct bevy::image::prelude::TextureAtlasSources")

Stores a mapping from sub texture handles to the related area index.

## Enums

[ImageFormat](enum.ImageFormat.html "enum bevy::image::prelude::ImageFormat")

The format of an on-disk image asset.

[TextureError](enum.TextureError.html "enum bevy::image::prelude::TextureError")

An error that occurs when loading a texture.
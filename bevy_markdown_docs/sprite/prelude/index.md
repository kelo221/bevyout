[bevy](../../index.html)::[sprite](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/lib.rs.html#24)

The sprite prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[BorderRect](struct.BorderRect.html "struct bevy::sprite::prelude::BorderRect")

Defines border insets that shrink a rectangle from its minimum and maximum corners.

[Sprite](struct.Sprite.html "struct bevy::sprite::prelude::Sprite")

Describes a sprite to be rendered to a 2D camera

[SpriteMesh](struct.SpriteMesh.html "struct bevy::sprite::prelude::SpriteMesh")

This is a carbon copy of [`Sprite`](../../prelude/struct.Sprite.html "struct bevy::prelude::Sprite") that uses the Mesh backend instead of the Sprite backend.

[SpritePickingCamera](struct.SpritePickingCamera.html "struct bevy::sprite::prelude::SpritePickingCamera")

An optional component that marks cameras that should be used in the [`SpritePickingPlugin`](../../prelude/struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin").

[SpritePickingPlugin](struct.SpritePickingPlugin.html "struct bevy::sprite::prelude::SpritePickingPlugin")

Enables the sprite picking backend, allowing you to click on, hover over and drag sprites.

[SpritePickingSettings](struct.SpritePickingSettings.html "struct bevy::sprite::prelude::SpritePickingSettings")

Runtime settings for the [`SpritePickingPlugin`](../../prelude/struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin").

[Text2d](struct.Text2d.html "struct bevy::sprite::prelude::Text2d")

The top-level 2D text component.

[TextureSlice](struct.TextureSlice.html "struct bevy::sprite::prelude::TextureSlice")

Single texture slice, representing a texture rect to draw in a given area

[TextureSlicer](struct.TextureSlicer.html "struct bevy::sprite::prelude::TextureSlicer")

Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes without needing to prepare multiple assets. The associated texture will be split into nine portions, so that on resize the different portions scale or tile in different ways to keep the texture in proportion.

## Enums

[SliceScaleMode](enum.SliceScaleMode.html "enum bevy::sprite::prelude::SliceScaleMode")

Defines how a texture slice scales when resized

[SpriteImageMode](enum.SpriteImageMode.html "enum bevy::sprite::prelude::SpriteImageMode")

Controls how the image is altered when scaled.

[SpritePickingMode](enum.SpritePickingMode.html "enum bevy::sprite::prelude::SpritePickingMode")

How should the [`SpritePickingPlugin`](../../prelude/struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin") handle picking and how should it handle transparent pixels

[SpriteScalingMode](enum.SpriteScalingMode.html "enum bevy::sprite::prelude::SpriteScalingMode")

Represents various modes for proportional scaling of a texture.

## Type Aliases

[Text2dReader](type.Text2dReader.html "type bevy::sprite::prelude::Text2dReader")

2d alias for [`TextReader`](../../text/struct.TextReader.html "struct bevy::text::TextReader").

[Text2dWriter](type.Text2dWriter.html "type bevy::sprite::prelude::Text2dWriter")

2d alias for [`TextWriter`](../../text/struct.TextWriter.html "struct bevy::text::TextWriter").
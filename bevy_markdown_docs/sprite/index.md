[bevy](../index.html)

# Crate sprite 

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/lib.rs.html#1-437)

Provides 2D sprite functionality.

## Modules

[prelude](prelude/index.html "mod bevy::sprite::prelude")

The sprite prelude.

## Structs

[Anchor](struct.Anchor.html "struct bevy::sprite::Anchor")

Normalized (relative to its size) offset of a 2d renderable entity from its [`Transform`](../prelude/struct.Transform.html "struct bevy::prelude::Transform").

[BorderRect](struct.BorderRect.html "struct bevy::sprite::BorderRect")

Defines border insets that shrink a rectangle from its minimum and maximum corners.

[Sprite](struct.Sprite.html "struct bevy::sprite::Sprite")

Describes a sprite to be rendered to a 2D camera

[SpriteMesh](struct.SpriteMesh.html "struct bevy::sprite::SpriteMesh")

This is a carbon copy of [`Sprite`](../prelude/struct.Sprite.html "struct bevy::prelude::Sprite") that uses the Mesh backend instead of the Sprite backend.

[SpriteMeshTemplate](struct.SpriteMeshTemplate.html "struct bevy::sprite::SpriteMeshTemplate")

[SpritePickingCamera](struct.SpritePickingCamera.html "struct bevy::sprite::SpritePickingCamera")

An optional component that marks cameras that should be used in the [`SpritePickingPlugin`](../prelude/struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin").

[SpritePickingPlugin](struct.SpritePickingPlugin.html "struct bevy::sprite::SpritePickingPlugin")

Enables the sprite picking backend, allowing you to click on, hover over and drag sprites.

[SpritePickingSettings](struct.SpritePickingSettings.html "struct bevy::sprite::SpritePickingSettings")

Runtime settings for the [`SpritePickingPlugin`](../prelude/struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin").

[SpritePlugin](struct.SpritePlugin.html "struct bevy::sprite::SpritePlugin")

Adds support for 2D sprites.

[SpriteTemplate](struct.SpriteTemplate.html "struct bevy::sprite::SpriteTemplate")

[Text2d](struct.Text2d.html "struct bevy::sprite::Text2d")

The top-level 2D text component.

[Text2dShadow](struct.Text2dShadow.html "struct bevy::sprite::Text2dShadow")

Adds a shadow behind `Text2d` text

[TextureSlice](struct.TextureSlice.html "struct bevy::sprite::TextureSlice")

Single texture slice, representing a texture rect to draw in a given area

[TextureSlicer](struct.TextureSlicer.html "struct bevy::sprite::TextureSlicer")

Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes without needing to prepare multiple assets. The associated texture will be split into nine portions, so that on resize the different portions scale or tile in different ways to keep the texture in proportion.

## Enums

[SliceScaleMode](enum.SliceScaleMode.html "enum bevy::sprite::SliceScaleMode")

Defines how a texture slice scales when resized

[SpriteAlphaMode](enum.SpriteAlphaMode.html "enum bevy::sprite::SpriteAlphaMode")

[SpriteImageMode](enum.SpriteImageMode.html "enum bevy::sprite::SpriteImageMode")

Controls how the image is altered when scaled.

[SpritePickingMode](enum.SpritePickingMode.html "enum bevy::sprite::SpritePickingMode")

How should the [`SpritePickingPlugin`](../prelude/struct.SpritePickingPlugin.html "struct bevy::prelude::SpritePickingPlugin") handle picking and how should it handle transparent pixels

[SpriteScalingMode](enum.SpriteScalingMode.html "enum bevy::sprite::SpriteScalingMode")

Represents various modes for proportional scaling of a texture.

[SpriteSystems](enum.SpriteSystems.html "enum bevy::sprite::SpriteSystems")

System set for sprite rendering.

## Functions

[calculate\_bounds\_2d](fn.calculate_bounds_2d.html "fn bevy::sprite::calculate_bounds_2d")

System calculating and inserting an [`Aabb`](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component to entities with either:

[calculate\_bounds\_text2d](fn.calculate_bounds_text2d.html "fn bevy::sprite::calculate_bounds_text2d")

System calculating and inserting an [`Aabb`](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb") component to entities with some [`TextLayoutInfo`](../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo") and [`Anchor`](struct.Anchor.html "struct bevy::sprite::Anchor") components, and without a [`NoFrustumCulling`](../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling") component.

[update\_text2d\_layout](fn.update_text2d_layout.html "fn bevy::sprite::update_text2d_layout")

Updates the layout and size information whenever the text or style is changed. This information is computed by the [`TextPipeline`](../text/struct.TextPipeline.html "struct bevy::text::TextPipeline") on insertion, then stored.

## Type Aliases

[Text2dReader](type.Text2dReader.html "type bevy::sprite::Text2dReader")

2d alias for [`TextReader`](../text/struct.TextReader.html "struct bevy::text::TextReader").

[Text2dWriter](type.Text2dWriter.html "type bevy::sprite::Text2dWriter")

2d alias for [`TextWriter`](../text/struct.TextWriter.html "struct bevy::text::TextWriter").
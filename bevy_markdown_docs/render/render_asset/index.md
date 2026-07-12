[bevy](../../index.html)::[render](../index.html)

# Module render\_asset 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#56)

## Structs

[AssetExtractionSystems](struct.AssetExtractionSystems.html "struct bevy::render::render_asset::AssetExtractionSystems")

The system set during which we extract modified assets to the render world.

[ExtractedAssets](struct.ExtractedAssets.html "struct bevy::render::render_asset::ExtractedAssets")

Temporarily stores the extracted and removed assets of the current frame.

[PrepareNextFrameAssets](struct.PrepareNextFrameAssets.html "struct bevy::render::render_asset::PrepareNextFrameAssets")

All assets that should be prepared next frame.

[RenderAssetBytesPerFrame](struct.RenderAssetBytesPerFrame.html "struct bevy::render::render_asset::RenderAssetBytesPerFrame")

A resource that defines the amount of data allowed to be transferred from CPU to GPU each frame, preventing choppy frames at the cost of waiting longer for GPU assets to become available.

[RenderAssetBytesPerFrameLimiter](struct.RenderAssetBytesPerFrameLimiter.html "struct bevy::render::render_asset::RenderAssetBytesPerFrameLimiter")

A render-world resource that facilitates limiting the data transferred from CPU to GPU each frame, preventing choppy frames at the cost of waiting longer for GPU assets to become available.

[RenderAssetPlugin](struct.RenderAssetPlugin.html "struct bevy::render::render_asset::RenderAssetPlugin")

This plugin extracts the changed assets from the “app world” into the “render world” and prepares them for the GPU. They can then be accessed from the [`RenderAssets`](struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets") resource.

[RenderAssets](struct.RenderAssets.html "struct bevy::render::render_asset::RenderAssets")

Stores all GPU representations ([`RenderAsset`](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset")) of [`RenderAsset::SourceAsset`](trait.RenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::render_asset::RenderAsset::SourceAsset") as long as they exist.

## Enums

[AssetExtractionError](enum.AssetExtractionError.html "enum bevy::render::render_asset::AssetExtractionError")

Error returned when an asset due for extraction has already been extracted

[PrepareAssetError](enum.PrepareAssetError.html "enum bevy::render::render_asset::PrepareAssetError")

## Traits

[RenderAsset](trait.RenderAsset.html "trait bevy::render::render_asset::RenderAsset")

Describes how an asset gets extracted and prepared for rendering.

[RenderAssetDependency](trait.RenderAssetDependency.html "trait bevy::render::render_asset::RenderAssetDependency")

## Functions

[extract\_render\_asset\_bytes\_per\_frame](fn.extract_render_asset_bytes_per_frame.html "fn bevy::render::render_asset::extract_render_asset_bytes_per_frame")

[prepare\_assets](fn.prepare_assets.html "fn bevy::render::render_asset::prepare_assets")

This system prepares all assets of the corresponding [`RenderAsset::SourceAsset`](trait.RenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::render_asset::RenderAsset::SourceAsset") type which where extracted this frame for the GPU.

[reset\_render\_asset\_bytes\_per\_frame](fn.reset_render_asset_bytes_per_frame.html "fn bevy::render::render_asset::reset_render_asset_bytes_per_frame")
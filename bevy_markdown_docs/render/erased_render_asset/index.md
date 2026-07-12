[bevy](../../index.html)::[render](../index.html)

# Module erased\_render\_asset 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#42)

## Structs

[AssetExtractionSystems](struct.AssetExtractionSystems.html "struct bevy::render::erased_render_asset::AssetExtractionSystems")

The system set during which we extract modified assets to the render world.

[ErasedRenderAssetPlugin](struct.ErasedRenderAssetPlugin.html "struct bevy::render::erased_render_asset::ErasedRenderAssetPlugin")

This plugin extracts the changed assets from the “app world” into the “render world” and prepares them for the GPU. They can then be accessed from the [`ErasedRenderAssets`](struct.ErasedRenderAssets.html "struct bevy::render::erased_render_asset::ErasedRenderAssets") resource.

[ErasedRenderAssets](struct.ErasedRenderAssets.html "struct bevy::render::erased_render_asset::ErasedRenderAssets")

Stores all GPU representations ([`ErasedRenderAsset`](trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset")) of [`ErasedRenderAsset::SourceAsset`](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset") as long as they exist.

[ExtractedAssets](struct.ExtractedAssets.html "struct bevy::render::erased_render_asset::ExtractedAssets")

Temporarily stores the extracted and removed assets of the current frame.

[PrepareNextFrameAssets](struct.PrepareNextFrameAssets.html "struct bevy::render::erased_render_asset::PrepareNextFrameAssets")

All assets that should be prepared next frame.

## Enums

[PrepareAssetError](enum.PrepareAssetError.html "enum bevy::render::erased_render_asset::PrepareAssetError")

## Traits

[ErasedRenderAsset](trait.ErasedRenderAsset.html "trait bevy::render::erased_render_asset::ErasedRenderAsset")

Describes how an asset gets extracted and prepared for rendering.

[ErasedRenderAssetDependency](trait.ErasedRenderAssetDependency.html "trait bevy::render::erased_render_asset::ErasedRenderAssetDependency")

## Functions

[prepare\_erased\_assets](fn.prepare_erased_assets.html "fn bevy::render::erased_render_asset::prepare_erased_assets")

This system prepares all assets of the corresponding [`ErasedRenderAsset::SourceAsset`](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset") type which where extracted this frame for the GPU.
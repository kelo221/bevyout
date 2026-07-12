[bevy](../../index.html)::[render](../index.html)::[erased\_render\_asset](index.html)

# Function prepare\_erased\_assets 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/erased_render_asset.rs.html#372-378)

```rust
pub fn prepare_erased_assets<A>(
    extracted_assets: ResMut<'_, ExtractedAssets<A>>,
    render_assets: ResMut<'_, ErasedRenderAssets<<A as ErasedRenderAsset>::ErasedAsset>>,
    prepare_next_frame: ResMut<'_, PrepareNextFrameAssets<A>>,
    param: StaticSystemParam<'_, '_, <A as ErasedRenderAsset>::Param>,
    bpf: Res<'_, RenderAssetBytesPerFrameLimiter>,
)where
    A: ErasedRenderAsset,
```

This system prepares all assets of the corresponding [`ErasedRenderAsset::SourceAsset`](trait.ErasedRenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::erased_render_asset::ErasedRenderAsset::SourceAsset") type which where extracted this frame for the GPU.
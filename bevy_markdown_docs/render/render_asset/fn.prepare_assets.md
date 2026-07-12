[bevy](../../index.html)::[render](../index.html)::[render\_asset](index.html)

# Function prepare\_assets 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#380-386)

```rust
pub fn prepare_assets<A>(
    extracted_assets: ResMut<'_, ExtractedAssets<A>>,
    render_assets: ResMut<'_, RenderAssets<A>>,
    prepare_next_frame: ResMut<'_, PrepareNextFrameAssets<A>>,
    param: StaticSystemParam<'_, '_, <A as RenderAsset>::Param>,
    bpf: Res<'_, RenderAssetBytesPerFrameLimiter>,
)where
    A: RenderAsset,
```

This system prepares all assets of the corresponding [`RenderAsset::SourceAsset`](trait.RenderAsset.html#associatedtype.SourceAsset "associated type bevy::render::render_asset::RenderAsset::SourceAsset") type which where extracted this frame for the GPU.
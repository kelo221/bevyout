[bevy](../../index.html)::[render](../index.html)::[render\_asset](index.html)

# Function extract\_render\_asset\_bytes\_per\_frame 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_asset.rs.html#485-488)

```rust
pub fn extract_render_asset_bytes_per_frame(
    bpf: Extract<'_, '_, Res<'_, RenderAssetBytesPerFrame>>,
    bpf_limiter: ResMut<'_, RenderAssetBytesPerFrameLimiter>,
)
```
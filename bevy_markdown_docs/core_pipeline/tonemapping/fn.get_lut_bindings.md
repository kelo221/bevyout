[bevy](../../index.html)::[core\_pipeline](../index.html)::[tonemapping](index.html)

# Function get\_lut\_bindings 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/tonemapping/mod.rs.html#389-394)

```rust
pub fn get_lut_bindings<'a>(
    images: &'a RenderAssets<GpuImage>,
    tonemapping_luts: &'a TonemappingLuts,
    tonemapping: &Tonemapping,
    fallback_image: &'a FallbackImage,
) -> (&'a TextureView, &'a Sampler)
```
[bevy](../index.html)::[pbr](index.html)

# Function get\_image\_texture 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#2776-2780)

```rust
pub fn get_image_texture<'a>(
    dummy_white_gpu_image: &'a GpuImage,
    gpu_images: &'a RenderAssets<GpuImage>,
    handle_option: &Option<Handle<Image>>,
) -> Option<(&'a TextureView, &'a Sampler)>
```
[bevy](../index.html)::[pbr](index.html)

# Constant MESH\_PIPELINE\_VIEW\_LAYOUT\_SAFE\_MAX\_TEXTURES 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#142)

```rust
pub const MESH_PIPELINE_VIEW_LAYOUT_SAFE_MAX_TEXTURES: usize = 10; // 10usize
```

Available on **debug-assertions enabled** only.

How many textures are allowed in the view bind group layout (`@group(0)`) before broader compatibility with WebGL and WebGPU is at risk, due to the minimum guaranteed values for `MAX_TEXTURE_IMAGE_UNITS` (in WebGL) and `maxSampledTexturesPerShaderStage` (in WebGPU), currently both at 16.

We use 10 here because it still leaves us, in a worst case scenario, with 6 textures for the other bind groups.

See: [https://gpuweb.github.io/gpuweb/#limits](https://gpuweb.github.io/gpuweb/#limits)
[bevy](../../index.html)::[core\_pipeline](../index.html)::[core\_3d](index.html)

# Constant DEPTH\_PREPASS\_TEXTURE\_SUPPORTED 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#31)

```rust
pub const DEPTH_PREPASS_TEXTURE_SUPPORTED: bool = true;
```

Available on **crate feature `webgpu` or non-WebAssembly** only.

True if multisampled depth textures are supported on this platform.

WebGL 2:

*   doesn’t support `copy_texture_to_texture` for depth textures yet, thus it doesn’t support `DepthPrepass`.
*   doesn’t support creating multisampled textures if they are not pure `RENDER_ATTACHMENT`, so it doesn’t support Msaa when reading `ViewDepthTexture`.
*   shadow sampler `texture_depth_2d` doesn’t support sampling, only supports comparison.

To read depth texture on WebGL 2, we can only use `ViewDepthTexture` with `Msaa::Off` and bind depth texture as unfilterable `texture_2d<f32>`. Therefore we disable depth of field and screen space reflections entirely on WebGL 2.
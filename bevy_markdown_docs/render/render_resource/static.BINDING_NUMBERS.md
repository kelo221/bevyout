[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Static BINDING\_NUMBERS 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bindless.rs.html#42)

```rust
pub static BINDING_NUMBERS: [(BindlessResourceType, BindingNumber); 9]
```

The binding numbers for the built-in binding arrays of each bindless resource type.

In the case of materials, the material allocator manages these binding arrays.

`bindless.wgsl` contains declarations of these arrays for use in your shaders. If you change these, make sure to update that file as well.
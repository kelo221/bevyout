[bevy](../index.html)::[shader](index.html)

# Macro load\_shader\_library 

[Source](https://docs.rs/bevy_shader/0.19.0/x86_64-unknown-linux-gnu/src/bevy_shader/lib.rs.html#28)

```rust
macro_rules! load_shader_library {
    ($asset_server_provider: expr, $path: literal $(, $settings: expr)?) => { ... };
}
```

Inline shader as an `embedded_asset` and load it permanently.

This works around a limitation of the shader loader not properly loading dependencies of shaders.
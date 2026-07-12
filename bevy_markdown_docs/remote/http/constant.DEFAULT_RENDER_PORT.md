[bevy](../../index.html)::[remote](../index.html)::[http](index.html)

# Constant DEFAULT\_RENDER\_PORT 

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/http.rs.html#57)

```rust
pub const DEFAULT_RENDER_PORT: u16 = 15703; // 15_703u16
```

Available on **crate feature `http` and non-`target_family=wasm`** only.

The default port that Bevy will listen on for the render subapp.

The render subapp is available for requests if the `bevy_render` feature is enabled.
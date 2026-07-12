[bevy](../../index.html)::[core\_pipeline](../index.html)::[skybox](index.html)

# Function extract\_skybox 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/skybox/mod.rs.html#68-72)

```rust
pub fn extract_skybox(
    commands: Commands<'_, '_>,
    previous_len: Local<'_, usize>,
    query: Extract<'_, '_, Query<'_, '_, (RenderEntity, &Skybox, Option<&Exposure>)>>,
)
```
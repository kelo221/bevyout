[bevy](../../index.html)::[render](../index.html)::[texture](index.html)

# Function update\_texture\_cache\_system 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/texture/texture_cache.rs.html#106)

```rust
pub fn update_texture_cache_system(texture_cache: ResMut<'_, TextureCache>)
```

Updates the [`TextureCache`](struct.TextureCache.html "struct bevy::render::texture::TextureCache") to only retains recently used textures.
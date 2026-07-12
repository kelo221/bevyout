[bevy](../index.html)::[sprite\_render](index.html)

# Function update\_tilemap\_chunk\_indices 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#193-205)

```rust
pub fn update_tilemap_chunk_indices(
    query: Query<'_, '_, (Entity, &TilemapChunk, &TilemapChunkTileData, &MeshMaterial2d<TilemapChunkMaterial>), Changed<TilemapChunkTileData>>,
    materials: ResMut<'_, Assets<TilemapChunkMaterial>>,
    images: ResMut<'_, Assets<Image>>,
)
```
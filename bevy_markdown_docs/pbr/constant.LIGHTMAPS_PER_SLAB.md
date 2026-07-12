[bevy](../index.html)::[pbr](index.html)

# Constant LIGHTMAPS\_PER\_SLAB 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#75)

```rust
pub const LIGHTMAPS_PER_SLAB: usize = 4; // 4usize
```

The number of lightmaps that we store in a single slab, if bindless textures are in use.

If bindless textures aren’t in use, then only a single lightmap can be bound at a time.
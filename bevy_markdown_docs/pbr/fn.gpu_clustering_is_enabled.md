[bevy](../index.html)::[pbr](index.html)

# Function gpu\_clustering\_is\_enabled 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#380)

```rust
pub fn gpu_clustering_is_enabled(
    global_cluster_settings: Res<'_, GlobalClusterSettings>,
) -> bool
```

A run condition that tests whether GPU clustering is enabled.

This is the version for use in non-extraction systems.
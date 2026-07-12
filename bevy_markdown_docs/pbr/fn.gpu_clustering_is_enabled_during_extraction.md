[bevy](../index.html)::[pbr](index.html)

# Function gpu\_clustering\_is\_enabled\_during\_extraction 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#371-373)

```rust
pub fn gpu_clustering_is_enabled_during_extraction(
    global_cluster_settings: Extract<'_, '_, Res<'_, GlobalClusterSettings>>,
) -> bool
```

A run condition that tests whether GPU clustering is enabled.

This is the version for use in extraction systems.
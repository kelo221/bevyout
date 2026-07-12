[bevy](../index.html)::[pbr](index.html)

# Constant GPU\_CLUSTERING\_INITIAL\_INDEX\_LIST\_CAPACITY 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#58)

```rust
pub const GPU_CLUSTERING_INITIAL_INDEX_LIST_CAPACITY: usize = 65536; // 65_536usize
```

The initial capacity of the clustered object index list.

The application can override this by setting [`GlobalClusterGpuSettings::initial_index_list_capacity`](../light/cluster/struct.GlobalClusterGpuSettings.html#structfield.initial_index_list_capacity "field bevy::light::cluster::GlobalClusterGpuSettings::initial_index_list_capacity").
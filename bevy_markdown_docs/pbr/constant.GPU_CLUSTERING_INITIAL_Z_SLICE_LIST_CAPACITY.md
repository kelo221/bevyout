[bevy](../index.html)::[pbr](index.html)

# Constant GPU\_CLUSTERING\_INITIAL\_Z\_SLICE\_LIST\_CAPACITY 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/cluster/mod.rs.html#52)

```rust
pub const GPU_CLUSTERING_INITIAL_Z_SLICE_LIST_CAPACITY: usize = 1024; // 1_024usize
```

The initial capacity of the Z slice list.

The application can override this by setting [`GlobalClusterGpuSettings::initial_z_slice_list_capacity`](../light/cluster/struct.GlobalClusterGpuSettings.html#structfield.initial_z_slice_list_capacity "field bevy::light::cluster::GlobalClusterGpuSettings::initial_z_slice_list_capacity").
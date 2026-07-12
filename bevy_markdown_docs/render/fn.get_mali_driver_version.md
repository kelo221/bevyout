[bevy](../index.html)::[render](index.html)

# Function get\_mali\_driver\_version 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#548)

```rust
pub fn get_mali_driver_version(adapter_info: &RenderAdapterInfo) -> Option<u32>
```

Get the Mali driver version if the adapter is a Mali GPU.
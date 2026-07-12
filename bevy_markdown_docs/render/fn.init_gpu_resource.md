[bevy](../index.html)::[render](index.html)

# Function init\_gpu\_resource 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#218)

```rust
pub fn init_gpu_resource<R>(world: &mut World)where
    R: Resource + FromWorld,
```

Constructs a `T` resource with `from_world` and inserts it.
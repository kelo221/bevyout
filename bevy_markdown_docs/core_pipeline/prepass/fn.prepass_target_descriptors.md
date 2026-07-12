[bevy](../../index.html)::[core\_pipeline](../index.html)::[prepass](index.html)

# Function prepass\_target\_descriptors 

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#386-390)

```rust
pub fn prepass_target_descriptors(
    normal_prepass: bool,
    motion_vector_prepass: bool,
    deferred_prepass: bool,
) -> Vec<Option<ColorTargetState>>
```
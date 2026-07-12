[bevy](../index.html)::[pbr](index.html)

# Constant MAX\_VIEW\_LIGHT\_PROBES 

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/light_probe/mod.rs.html#52)

```rust
pub const MAX_VIEW_LIGHT_PROBES: usize = 8; // 8usize
```

The maximum number of each type of light probe that each view will consider.

Because the fragment shader does a linear search through the list for each fragment, this number needs to be relatively small.
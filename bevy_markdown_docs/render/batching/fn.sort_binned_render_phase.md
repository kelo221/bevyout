[bevy](../../index.html)::[render](../index.html)::[batching](index.html)

# Function sort\_binned\_render\_phase 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/batching/mod.rs.html#198-200)

```rust
pub fn sort_binned_render_phase<BPI>(
    phases: ResMut<'_, ViewBinnedRenderPhases<BPI>>,
)where
    BPI: BinnedPhaseItem,
```

Sorts a render phase that uses bins.
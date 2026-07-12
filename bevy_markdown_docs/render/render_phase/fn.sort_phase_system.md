[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Function sort\_phase\_system 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2174-2178)

```rust
pub fn sort_phase_system<I>(
    views: Query<'_, '_, &ExtractedView>,
    render_phases: ResMut<'_, ViewSortedRenderPhases<I>>,
)where
    I: SortedPhaseItem,
```

This system sorts the [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")s of all [`SortedRenderPhase`](struct.SortedRenderPhase.html "struct bevy::render::render_phase::SortedRenderPhase")s of this type.
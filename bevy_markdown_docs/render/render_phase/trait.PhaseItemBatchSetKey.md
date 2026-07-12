[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait PhaseItemBatchSetKey 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2070)

```rust
pub trait PhaseItemBatchSetKey:
    Clone
    + Send
    + Sync
    + PartialEq
    + Eq
    + Ord
    + Hash {
    // Required method
    fn indexed(&self) -> bool;
}
```

A key used to combine batches into batch sets.

A _batch set_ is a set of meshes that can potentially be multi-drawn together.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2076)

#### fn [indexed](#tymethod.indexed)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Returns true if this batch set key describes indexed meshes or false if it describes non-indexed meshes.

Bevy uses this in order to determine which kind of indirect draw parameters to use, if indirect drawing is enabled.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#196)

### impl [PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey") for [BatchSetKey2d](../../core_pipeline/core_2d/struct.BatchSetKey2d.html "struct bevy::core_pipeline::core_2d::BatchSetKey2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#217)

### impl [PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey") for [Opaque3dBatchSetKey](../../core_pipeline/core_3d/struct.Opaque3dBatchSetKey.html "struct bevy::core_pipeline::core_3d::Opaque3dBatchSetKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#224)

### impl [PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey") for [OpaqueNoLightmap3dBatchSetKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBatchSetKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBatchSetKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2678)

### impl [PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey") for [ShadowBatchSetKey](../../pbr/struct.ShadowBatchSetKey.html "struct bevy::pbr::ShadowBatchSetKey")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#265)

### impl [PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey") for [Wireframe2dBatchSetKey](../../sprite_render/struct.Wireframe2dBatchSetKey.html "struct bevy::sprite_render::Wireframe2dBatchSetKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#303)

### impl [PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey") for [Wireframe3dBatchSetKey](../../pbr/wireframe/struct.Wireframe3dBatchSetKey.html "struct bevy::pbr::wireframe::Wireframe3dBatchSetKey")
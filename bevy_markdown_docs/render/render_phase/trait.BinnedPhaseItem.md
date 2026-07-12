[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait BinnedPhaseItem 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2037)

```rust
pub trait BinnedPhaseItem: PhaseItem {
    type BinKey: Clone + Send + Sync + PartialEq + Eq + Ord + Hash;
    type BatchSetKey: PhaseItemBatchSetKey;

    // Required method
    fn new(
        batch_set_key: Self::BatchSetKey,
        bin_key: Self::BinKey,
        representative_entity: (Entity, MainEntity),
        batch_range: Range<u32>,
        extra_index: PhaseItemExtraIndex,
    ) -> Self;
}
```

Represents phase items that are placed into bins. The `BinKey` specifies which bin they’re to be placed in. Bin keys are sorted, and items within the same bin are eligible to be batched together. The elements within the bins aren’t themselves sorted.

An example of a binned phase item is `Opaque3d`, for which the rendering order isn’t critical.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2044)

#### type [BinKey](#associatedtype.BinKey): [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash")

The key used for binning [`PhaseItem`](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem")s into bins. Order the members of [`BinnedPhaseItem::BinKey`](trait.BinnedPhaseItem.html#associatedtype.BinKey "associated type bevy::render::render_phase::BinnedPhaseItem::BinKey") by the order of binding for best performance. For example, pipeline id, draw function id, mesh asset id, lowest variable bind group id such as the material bind group id, and its dynamic offsets if any, next bind group and offsets, etc. This reduces the need for rebinding between bins and improves performance.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2050)

#### type [BatchSetKey](#associatedtype.BatchSetKey): [PhaseItemBatchSetKey](trait.PhaseItemBatchSetKey.html "trait bevy::render::render_phase::PhaseItemBatchSetKey")

The key used to combine batches into batch sets.

A _batch set_ is a set of meshes that can potentially be multi-drawn together.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2057-2063)

#### fn [new](#tymethod.new)( batch\_set\_key: Self::[BatchSetKey](trait.BinnedPhaseItem.html#associatedtype.BatchSetKey "type bevy::render::render_phase::BinnedPhaseItem::BatchSetKey"), bin\_key: Self::[BinKey](trait.BinnedPhaseItem.html#associatedtype.BinKey "type bevy::render::render_phase::BinnedPhaseItem::BinKey"), representative\_entity: ([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")), batch\_range: [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>, extra\_index: [PhaseItemExtraIndex](enum.PhaseItemExtraIndex.html "enum bevy::render::render_phase::PhaseItemExtraIndex"), ) -> Self

Creates a new binned phase item from the key and per-entity data.

Unlike [`SortedPhaseItem`](trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem")s, this is generally called “just in time” before rendering. The resulting phase item isn’t stored in any data structures, resulting in significant memory savings.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#279)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [AlphaMask2d](../../core_pipeline/core_2d/struct.AlphaMask2d.html "struct bevy::core_pipeline::core_2d::AlphaMask2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#282)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [BatchSetKey2d](../../core_pipeline/core_2d/struct.BatchSetKey2d.html "struct bevy::core_pipeline::core_2d::BatchSetKey2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#284)

#### type [BinKey](#associatedtype.BinKey) = [AlphaMask2dBinKey](../../core_pipeline/core_2d/struct.AlphaMask2dBinKey.html "struct bevy::core_pipeline::core_2d::AlphaMask2dBinKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#348)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [AlphaMask3d](../../core_pipeline/core_3d/struct.AlphaMask3d.html "struct bevy::core_pipeline::core_3d::AlphaMask3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#349)

#### type [BinKey](#associatedtype.BinKey) = [OpaqueNoLightmap3dBinKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBinKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBinKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#350)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [OpaqueNoLightmap3dBatchSetKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBatchSetKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBatchSetKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#160)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [AlphaMask3dDeferred](../../core_pipeline/deferred/struct.AlphaMask3dDeferred.html "struct bevy::core_pipeline::deferred::AlphaMask3dDeferred")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#161)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [OpaqueNoLightmap3dBatchSetKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBatchSetKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBatchSetKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#162)

#### type [BinKey](#associatedtype.BinKey) = [OpaqueNoLightmap3dBinKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBinKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBinKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#357)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [AlphaMask3dPrepass](../../core_pipeline/prepass/struct.AlphaMask3dPrepass.html "struct bevy::core_pipeline::prepass::AlphaMask3dPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#358)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [OpaqueNoLightmap3dBatchSetKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBatchSetKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBatchSetKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#359)

#### type [BinKey](#associatedtype.BinKey) = [OpaqueNoLightmap3dBinKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBinKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBinKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#164)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [Opaque2d](../../core_pipeline/core_2d/struct.Opaque2d.html "struct bevy::core_pipeline::core_2d::Opaque2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#167)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [BatchSetKey2d](../../core_pipeline/core_2d/struct.BatchSetKey2d.html "struct bevy::core_pipeline::core_2d::BatchSetKey2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#169)

#### type [BinKey](#associatedtype.BinKey) = [Opaque2dBinKey](../../core_pipeline/core_2d/struct.Opaque2dBinKey.html "struct bevy::core_pipeline::core_2d::Opaque2dBinKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#270)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [Opaque3d](../../core_pipeline/core_3d/struct.Opaque3d.html "struct bevy::core_pipeline::core_3d::Opaque3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#271)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [Opaque3dBatchSetKey](../../core_pipeline/core_3d/struct.Opaque3dBatchSetKey.html "struct bevy::core_pipeline::core_3d::Opaque3dBatchSetKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#272)

#### type [BinKey](#associatedtype.BinKey) = [Opaque3dBinKey](../../core_pipeline/core_3d/struct.Opaque3dBinKey.html "struct bevy::core_pipeline::core_3d::Opaque3dBinKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#76)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [Opaque3dDeferred](../../core_pipeline/deferred/struct.Opaque3dDeferred.html "struct bevy::core_pipeline::deferred::Opaque3dDeferred")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#77)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [OpaqueNoLightmap3dBatchSetKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBatchSetKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBatchSetKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#78)

#### type [BinKey](#associatedtype.BinKey) = [OpaqueNoLightmap3dBinKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBinKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBinKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#274)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [Opaque3dPrepass](../../core_pipeline/prepass/struct.Opaque3dPrepass.html "struct bevy::core_pipeline::prepass::Opaque3dPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#275)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [OpaqueNoLightmap3dBatchSetKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBatchSetKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBatchSetKey")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#276)

#### type [BinKey](#associatedtype.BinKey) = [OpaqueNoLightmap3dBinKey](../../core_pipeline/prepass/struct.OpaqueNoLightmap3dBinKey.html "struct bevy::core_pipeline::prepass::OpaqueNoLightmap3dBinKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2727)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [Shadow](../../pbr/struct.Shadow.html "struct bevy::pbr::Shadow")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2728)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [ShadowBatchSetKey](../../pbr/struct.ShadowBatchSetKey.html "struct bevy::pbr::ShadowBatchSetKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2729)

#### type [BinKey](#associatedtype.BinKey) = [ShadowBinKey](../../pbr/struct.ShadowBinKey.html "struct bevy::pbr::ShadowBinKey")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#221)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [Wireframe2dPhaseItem](../../sprite_render/struct.Wireframe2dPhaseItem.html "struct bevy::sprite_render::Wireframe2dPhaseItem")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#222)

#### type [BinKey](#associatedtype.BinKey) = [Wireframe2dBinKey](../../sprite_render/struct.Wireframe2dBinKey.html "struct bevy::sprite_render::Wireframe2dBinKey")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#223)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [Wireframe2dBatchSetKey](../../sprite_render/struct.Wireframe2dBatchSetKey.html "struct bevy::sprite_render::Wireframe2dBatchSetKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#257)

### impl [BinnedPhaseItem](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem") for [Wireframe3d](../../pbr/wireframe/struct.Wireframe3d.html "struct bevy::pbr::wireframe::Wireframe3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#258)

#### type [BinKey](#associatedtype.BinKey) = [Wireframe3dBinKey](../../pbr/wireframe/struct.Wireframe3dBinKey.html "struct bevy::pbr::wireframe::Wireframe3dBinKey")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#259)

#### type [BatchSetKey](#associatedtype.BatchSetKey) = [Wireframe3dBatchSetKey](../../pbr/wireframe/struct.Wireframe3dBatchSetKey.html "struct bevy::pbr::wireframe::Wireframe3dBatchSetKey")
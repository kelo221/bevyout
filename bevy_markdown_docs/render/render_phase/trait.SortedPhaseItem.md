[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait SortedPhaseItem 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2085)

```rust
pub trait SortedPhaseItem: PhaseItem {
    type SortKey: Ord;

    // Required methods
    fn sort_key(&self) -> Self::SortKey;
    fn recalculate_sort_keys(
        items: &mut IndexMap<(Entity, MainEntity), Self, EntityHash>,
        view: &ExtractedView,
    );
    fn indexed(&self) -> bool;

    // Provided method
    fn sort(items: &mut IndexMap<(Entity, MainEntity), Self, EntityHash>) { ... }
}
```

Represents phase items that must be sorted. The `SortKey` specifies the order that these items are drawn in. These are placed into a single array, and the array as a whole is then sorted.

An example of a sorted phase item is `Transparent3d`, which must be sorted back to front in order to correctly render with the painter’s algorithm.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2089)

#### type [SortKey](#associatedtype.SortKey): [Ord](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html "trait core::cmp::Ord")

The type used for ordering the items. The smallest values are drawn first. This order can be calculated using the [`ViewRangefinder3d`](struct.ViewRangefinder3d.html "struct bevy::render::render_phase::ViewRangefinder3d"), based on the view-space `Z` value of the corresponding view matrix.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2092)

#### fn [sort\_key](#tymethod.sort_key)(&self) -> Self::[SortKey](trait.SortedPhaseItem.html#associatedtype.SortKey "type bevy::render::render_phase::SortedPhaseItem::SortKey")

Determines the order in which the items are drawn.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2118-2121)

#### fn [recalculate\_sort\_keys](#tymethod.recalculate_sort_keys)( items: &mut [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")), Self, [EntityHash](../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>, view: &[ExtractedView](../view/struct.ExtractedView.html "struct bevy::render::view::ExtractedView"), )

Populates whatever internal fields are necessary in order to perform the sort.

The renderer calls this method right before calling [`Self::sort`](trait.SortedPhaseItem.html#method.sort "associated function bevy::render::render_phase::SortedPhaseItem::sort"). For 3D transparent phases that need to be depth sorted, it populates the `distance` field with the actual distance from the view. For other phases, this method is generally a no-op.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2132)

#### fn [indexed](#tymethod.indexed)(&self) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Whether this phase item targets indexed meshes (those with both vertex and index buffers as opposed to just vertex buffers).

Bevy needs this information in order to properly group phase items together for multi-draw indirect, because the GPU layout of indirect commands differs between indexed and non-indexed meshes.

If you’re implementing a custom phase item that doesn’t describe a mesh, you can safely return false here.

## Provided Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#2107)

#### fn [sort](#method.sort)(items: &mut [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<([Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity"), [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")), Self, [EntityHash](../../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>)

Sorts a slice of phase items into render order. Generally if the same type is batched this should use a stable sort like [`slice::sort_by_key`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_by_key "method slice::sort_by_key"). In almost all other cases, this should not be altered from the default, which uses an unstable sort, as this provides the best balance of CPU and GPU performance.

Implementers can optionally not sort the list at all. This is generally advisable if and only if the renderer supports a depth prepass, which is by default not supported by the rest of Bevy’s first party rendering crates. Even then, this may have a negative impact on GPU-side performance due to overdraw.

It’s advised to always profile for performance changes when changing this implementation.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/phase.rs.html#85)

### impl [SortedPhaseItem](trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem") for [Transmissive3d](../../pbr/struct.Transmissive3d.html "struct bevy::pbr::Transmissive3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/phase.rs.html#87)

#### type [SortKey](#associatedtype.SortKey) = [FloatOrd](../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#361)

### impl [SortedPhaseItem](trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem") for [Transparent2d](../../core_pipeline/core_2d/struct.Transparent2d.html "struct bevy::core_pipeline::core_2d::Transparent2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#362)

#### type [SortKey](#associatedtype.SortKey) = [FloatOrd](../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#426)

### impl [SortedPhaseItem](trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem") for [Transparent3d](../../core_pipeline/core_3d/struct.Transparent3d.html "struct bevy::core_pipeline::core_3d::Transparent3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#428)

#### type [SortKey](#associatedtype.SortKey) = [FloatOrd](../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#124)

### impl [SortedPhaseItem](trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem") for [TransparentUi](../../ui_render/struct.TransparentUi.html "struct bevy::ui_render::TransparentUi")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#125)

#### type [SortKey](#associatedtype.SortKey) = [FloatOrd](../../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")
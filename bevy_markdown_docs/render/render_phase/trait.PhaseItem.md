[bevy](../../index.html)::[render](../index.html)::[render\_phase](index.html)

# Trait PhaseItem 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1925)

```rust
pub trait PhaseItem:
    Sized
    + Send
    + Sync
    + 'static {
    const AUTOMATIC_BATCHING: bool = true;

    // Required methods
    fn entity(&self) -> Entity;
    fn main_entity(&self) -> MainEntity;
    fn draw_function(&self) -> DrawFunctionId;
    fn batch_range(&self) -> &Range<u32> ⓘ;
    fn batch_range_mut(&mut self) -> &mut Range<u32> ⓘ;
    fn extra_index(&self) -> PhaseItemExtraIndex;
    fn batch_range_and_extra_index_mut(
        &mut self,
    ) -> (&mut Range<u32>, &mut PhaseItemExtraIndex);
}
```

An item (entity of the render world) which will be drawn to a texture or the screen, as part of a render phase.

The data required for rendering an entity is extracted from the main world in the [`ExtractSchedule`](../../prelude/struct.ExtractSchedule.html "struct bevy::prelude::ExtractSchedule"). Then it has to be queued up for rendering during the [`RenderSystems::Queue`](../enum.RenderSystems.html#variant.Queue "variant bevy::render::RenderSystems::Queue"), by adding a corresponding phase item to a render phase. Afterwards it will be possibly sorted and rendered automatically in the [`RenderSystems::PhaseSort`](../enum.RenderSystems.html#variant.PhaseSort "variant bevy::render::RenderSystems::PhaseSort") and [`RenderSystems::Render`](../enum.RenderSystems.html#variant.Render "variant bevy::render::RenderSystems::Render"), respectively.

`PhaseItem`s come in two flavors: [`BinnedPhaseItem`](trait.BinnedPhaseItem.html "trait bevy::render::render_phase::BinnedPhaseItem")s and [`SortedPhaseItem`](trait.SortedPhaseItem.html "trait bevy::render::render_phase::SortedPhaseItem")s.

*   Binned phase items have a `BinKey` which specifies what bin they’re to be placed in. All items in the same bin are eligible to be batched together. The `BinKey`s are sorted, but the individual bin items aren’t. Binned phase items are good for opaque meshes, in which the order of rendering isn’t important. Generally, binned phase items are faster than sorted phase items.
    
*   Sorted phase items, on the other hand, are placed into one large buffer and then sorted all at once. This is needed for transparent meshes, which have to be sorted back-to-front to render with the painter’s algorithm. These types of phase items are generally slower than binned phase items.
    

## Provided Associated Constants

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1927)

#### const [AUTOMATIC\_BATCHING](#associatedconstant.AUTOMATIC_BATCHING): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

Whether or not this `PhaseItem` should be subjected to automatic batching. (Default: `true`)

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1933)

#### fn [entity](#tymethod.entity)(&self) -> [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

The corresponding entity that will be drawn.

This is used to fetch the render data of the entity, required by the draw function, from the render world .

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1936)

#### fn [main\_entity](#tymethod.main_entity)(&self) -> [MainEntity](../sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

The main world entity represented by this `PhaseItem`.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1939)

#### fn [draw\_function](#tymethod.draw_function)(&self) -> [DrawFunctionId](../../material/labels/struct.DrawFunctionId.html "struct bevy::material::labels::DrawFunctionId")

Specifies the [`Draw`](trait.Draw.html "trait bevy::render::render_phase::Draw") function used to render the item.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1944)

#### fn [batch\_range](#tymethod.batch_range)(&self) -> &[Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> [ⓘ](#)

The range of instances that the batch covers. After doing a batched draw, batch range length phase items will be skipped. This design is to avoid having to restructure the render phase unnecessarily.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1945)

#### fn [batch\_range\_mut](#tymethod.batch_range_mut)(&mut self) -> &mut [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\> [ⓘ](#)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1951)

#### fn [extra\_index](#tymethod.extra_index)(&self) -> [PhaseItemExtraIndex](enum.PhaseItemExtraIndex.html "enum bevy::render::render_phase::PhaseItemExtraIndex")

Returns the [`PhaseItemExtraIndex`](enum.PhaseItemExtraIndex.html "enum bevy::render::render_phase::PhaseItemExtraIndex").

If present, this is either a dynamic offset or an indirect parameters index.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_phase/mod.rs.html#1955)

#### fn [batch\_range\_and\_extra\_index\_mut](#tymethod.batch_range_and_extra_index_mut)( &mut self, ) -> (&mut [Range](https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html "struct core::ops::range::Range")<[u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)\>, &mut [PhaseItemExtraIndex](enum.PhaseItemExtraIndex.html "enum bevy::render::render_phase::PhaseItemExtraIndex"))

Returns a pair of mutable references to both the batch range and extra index.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#244)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [AlphaMask2d](../../core_pipeline/core_2d/struct.AlphaMask2d.html "struct bevy::core_pipeline::core_2d::AlphaMask2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#312)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [AlphaMask3d](../../core_pipeline/core_3d/struct.AlphaMask3d.html "struct bevy::core_pipeline::core_3d::AlphaMask3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#123)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [AlphaMask3dDeferred](../../core_pipeline/deferred/struct.AlphaMask3dDeferred.html "struct bevy::core_pipeline::deferred::AlphaMask3dDeferred")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#321)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [AlphaMask3dPrepass](../../core_pipeline/prepass/struct.AlphaMask3dPrepass.html "struct bevy::core_pipeline::prepass::AlphaMask3dPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#130)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Opaque2d](../../core_pipeline/core_2d/struct.Opaque2d.html "struct bevy::core_pipeline::core_2d::Opaque2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#235)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Opaque3d](../../core_pipeline/core_3d/struct.Opaque3d.html "struct bevy::core_pipeline::core_3d::Opaque3d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/deferred/mod.rs.html#40)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Opaque3dDeferred](../../core_pipeline/deferred/struct.Opaque3dDeferred.html "struct bevy::core_pipeline::deferred::Opaque3dDeferred")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#238)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Opaque3dPrepass](../../core_pipeline/prepass/struct.Opaque3dPrepass.html "struct bevy::core_pipeline::prepass::Opaque3dPrepass")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/light.rs.html#2691)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Shadow](../../pbr/struct.Shadow.html "struct bevy::pbr::Shadow")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/phase.rs.html#37)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Transmissive3d](../../pbr/struct.Transmissive3d.html "struct bevy::pbr::Transmissive3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/phase.rs.html#47)

#### const [AUTOMATIC\_BATCHING](#associatedconstant.AUTOMATIC_BATCHING): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = false

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_2d/mod.rs.html#324)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Transparent2d](../../core_pipeline/core_2d/struct.Transparent2d.html "struct bevy::core_pipeline::core_2d::Transparent2d")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/core_3d/mod.rs.html#390)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Transparent3d](../../core_pipeline/core_3d/struct.Transparent3d.html "struct bevy::core_pipeline::core_3d::Transparent3d")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/render_pass.rs.html#88)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [TransparentUi](../../ui_render/struct.TransparentUi.html "struct bevy::ui_render::TransparentUi")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#185)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Wireframe2dPhaseItem](../../sprite_render/struct.Wireframe2dPhaseItem.html "struct bevy::sprite_render::Wireframe2dPhaseItem")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#221)

### impl [PhaseItem](trait.PhaseItem.html "trait bevy::render::render_phase::PhaseItem") for [Wireframe3d](../../pbr/wireframe/struct.Wireframe3d.html "struct bevy::pbr::wireframe::Wireframe3d")

{"&Range<u32>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/range/trait.Step.html\\" title=\\"trait core::iter::range::Step\\">Step</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = A;</div>","&mut Range<u32>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;A&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/range/struct.Range.html\\" title=\\"struct core::ops::range::Range\\">Range</a>&lt;A&gt;<div class=\\"where\\">where\\n A: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/range/trait.Step.html\\" title=\\"trait core::iter::range::Step\\">Step</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = A;</div>"}
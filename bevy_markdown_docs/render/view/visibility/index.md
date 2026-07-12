[bevy](../../../index.html)::[render](../../index.html)::[view](../index.html)

# Module visibility 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#1)

## Structs

[RenderExtractedShadowMapVisibleEntities](struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::visibility::RenderExtractedShadowMapVisibleEntities")

The entities that the CPU has determined are visible from a single shadow-casting light.

[RenderExtractedVisibleEntities](struct.RenderExtractedVisibleEntities.html "struct bevy::render::view::visibility::RenderExtractedVisibleEntities")

The entities that the CPU has determined are visible from a single view or subview.

[RenderExtractedVisibleEntitiesClass](struct.RenderExtractedVisibleEntitiesClass.html "struct bevy::render::view::visibility::RenderExtractedVisibleEntitiesClass")

The entities that the CPU has determined are visible from a single view or subview, for a single [`VisibilityClass`](../../../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass").

[RenderShadowMapVisibleEntities](struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::visibility::RenderShadowMapVisibleEntities")

Collection of entities visible from a single light.

[RenderVisibilityRangePlugin](struct.RenderVisibilityRangePlugin.html "struct bevy::render::view::visibility::RenderVisibilityRangePlugin")

A plugin that enables [`RenderVisibilityRanges`](../struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges")s, which allow entities to be hidden or shown based on distance to the camera.

[RenderVisibilityRanges](struct.RenderVisibilityRanges.html "struct bevy::render::view::visibility::RenderVisibilityRanges")

Stores information related to [`VisibilityRange`](../../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")s in the render world.

[RenderVisibleEntities](struct.RenderVisibleEntities.html "struct bevy::render::view::visibility::RenderVisibleEntities")

Stores a list of all entities that are visible from a single view or subview, as well as the change lists.

[RenderVisibleEntitiesClass](struct.RenderVisibleEntitiesClass.html "struct bevy::render::view::visibility::RenderVisibleEntitiesClass")

Stores a list of all entities that are visible from a single view for a single [`VisibilityClass`](../../../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass"), as well as the change lists.

[VisibilityExtractionSystemParam](struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::visibility::VisibilityExtractionSystemParam")

A system parameter that goes on any render-world system that needs to extract entities into [`RenderVisibleEntities`](../struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities").

## Constants

[VISIBILITY\_RANGES\_STORAGE\_BUFFER\_COUNT](constant.VISIBILITY_RANGES_STORAGE_BUFFER_COUNT.html "constant bevy::render::view::visibility::VISIBILITY_RANGES_STORAGE_BUFFER_COUNT")

We need at least 4 storage buffer bindings available to enable the visibility range buffer.

## Functions

[collect\_visible\_cpu\_culled\_entities](fn.collect_visible_cpu_culled_entities.html "fn bevy::render::view::visibility::collect_visible_cpu_culled_entities")

Updates the [`RenderVisibleEntities`](../struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities") and [`RenderShadowMapVisibleEntities`](../struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities") components with the contents of the [`RenderExtractedVisibleEntities`](../struct.RenderExtractedVisibleEntities.html "struct bevy::render::view::RenderExtractedVisibleEntities") and the [`RenderExtractedShadowMapVisibleEntities`](../struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::RenderExtractedShadowMapVisibleEntities") components respectively.

[extract\_visibility\_ranges](fn.extract_visibility_ranges.html "fn bevy::render::view::visibility::extract_visibility_ranges")

Extracts all [`VisibilityRange`](../../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange") components from the main world to the render world and inserts them into [`RenderVisibilityRanges`](../struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges").

[write\_render\_visibility\_ranges](fn.write_render_visibility_ranges.html "fn bevy::render::view::visibility::write_render_visibility_ranges")

Writes the [`RenderVisibilityRanges`](../struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges") table to the GPU.

## Type Aliases

[VisibilityExtractionNoCpuCullingChangedQuery](type.VisibilityExtractionNoCpuCullingChangedQuery.html "type bevy::render::view::visibility::VisibilityExtractionNoCpuCullingChangedQuery")

The query, part of [`VisibilityExtractionSystemParam`](../struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::VisibilityExtractionSystemParam"), that searches for entities with [`NoCpuCulling`](../../../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling") that might have changed visibility.
[bevy](../../index.html)::[render](../index.html)

# Module view 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#67)

## Modules

[visibility](visibility/index.html "mod bevy::render::view::visibility")

[window](window/index.html "mod bevy::render::view::window")

## Structs

[ColorGrading](struct.ColorGrading.html "struct bevy::render::view::ColorGrading")

Configures filmic color grading parameters to adjust the image appearance.

[ColorGradingGlobal](struct.ColorGradingGlobal.html "struct bevy::render::view::ColorGradingGlobal")

Filmic color grading values applied to the image as a whole (as opposed to individual sections, like shadows and highlights).

[ColorGradingSection](struct.ColorGradingSection.html "struct bevy::render::view::ColorGradingSection")

A section of color grading values that can be selectively applied to shadows, midtones, and highlights.

[ColorGradingUniform](struct.ColorGradingUniform.html "struct bevy::render::view::ColorGradingUniform")

The [`ColorGrading`](struct.ColorGrading.html "struct bevy::render::view::ColorGrading") structure, packed into the most efficient form for the GPU.

[ExtractedView](struct.ExtractedView.html "struct bevy::render::view::ExtractedView")

Describes a view in the render world.

[ExtractedWindow](struct.ExtractedWindow.html "struct bevy::render::view::ExtractedWindow")

[ExtractedWindows](struct.ExtractedWindows.html "struct bevy::render::view::ExtractedWindows")

[NoIndirectDrawing](struct.NoIndirectDrawing.html "struct bevy::render::view::NoIndirectDrawing")

Add this component to a camera to disable _indirect mode_.

[PostProcessWrite](struct.PostProcessWrite.html "struct bevy::render::view::PostProcessWrite")

[RenderExtractedShadowMapVisibleEntities](struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::RenderExtractedShadowMapVisibleEntities")

The entities that the CPU has determined are visible from a single shadow-casting light.

[RenderExtractedVisibleEntities](struct.RenderExtractedVisibleEntities.html "struct bevy::render::view::RenderExtractedVisibleEntities")

The entities that the CPU has determined are visible from a single view or subview.

[RenderExtractedVisibleEntitiesClass](struct.RenderExtractedVisibleEntitiesClass.html "struct bevy::render::view::RenderExtractedVisibleEntitiesClass")

The entities that the CPU has determined are visible from a single view or subview, for a single [`VisibilityClass`](../../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass").

[RenderShadowLodOrigin](struct.RenderShadowLodOrigin.html "struct bevy::render::view::RenderShadowLodOrigin")

A resource, part of the render world, that stores the resolved origin for LOD selection for shadow maps of point and spot lights.

[RenderShadowMapVisibleEntities](struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities")

Collection of entities visible from a single light.

[RenderVisibilityRangePlugin](struct.RenderVisibilityRangePlugin.html "struct bevy::render::view::RenderVisibilityRangePlugin")

A plugin that enables [`RenderVisibilityRanges`](struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges")s, which allow entities to be hidden or shown based on distance to the camera.

[RenderVisibilityRanges](struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges")

Stores information related to [`VisibilityRange`](../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")s in the render world.

[RenderVisibleEntities](struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities")

Stores a list of all entities that are visible from a single view or subview, as well as the change lists.

[RenderVisibleEntitiesClass](struct.RenderVisibleEntitiesClass.html "struct bevy::render::view::RenderVisibleEntitiesClass")

Stores a list of all entities that are visible from a single view for a single [`VisibilityClass`](../../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass"), as well as the change lists.

[RetainedViewEntity](struct.RetainedViewEntity.html "struct bevy::render::view::RetainedViewEntity")

An identifier for a view that is stable across frames.

[ViewDepthTexture](struct.ViewDepthTexture.html "struct bevy::render::view::ViewDepthTexture")

[ViewPlugin](struct.ViewPlugin.html "struct bevy::render::view::ViewPlugin")

[ViewTarget](struct.ViewTarget.html "struct bevy::render::view::ViewTarget")

[ViewTargetAttachments](struct.ViewTargetAttachments.html "struct bevy::render::view::ViewTargetAttachments")

Contains [`OutputColorAttachment`](../texture/struct.OutputColorAttachment.html "struct bevy::render::texture::OutputColorAttachment") used for each target present on any view in the current frame, after being prepared by [`prepare_view_attachments`](fn.prepare_view_attachments.html "fn bevy::render::view::prepare_view_attachments"). Users that want to override the default output color attachment for a specific target can do so by adding a [`OutputColorAttachment`](../texture/struct.OutputColorAttachment.html "struct bevy::render::texture::OutputColorAttachment") to this resource before [`prepare_view_targets`](fn.prepare_view_targets.html "fn bevy::render::view::prepare_view_targets") is called.

[ViewUniform](struct.ViewUniform.html "struct bevy::render::view::ViewUniform")

[ViewUniformOffset](struct.ViewUniformOffset.html "struct bevy::render::view::ViewUniformOffset")

[ViewUniforms](struct.ViewUniforms.html "struct bevy::render::view::ViewUniforms")

[VisibilityExtractionSystemParam](struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::VisibilityExtractionSystemParam")

A system parameter that goes on any render-world system that needs to extract entities into [`RenderVisibleEntities`](struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities").

[WindowRenderPlugin](struct.WindowRenderPlugin.html "struct bevy::render::view::WindowRenderPlugin")

[WindowSurfaces](struct.WindowSurfaces.html "struct bevy::render::view::WindowSurfaces")

## Enums

[Msaa](enum.Msaa.html "enum bevy::render::view::Msaa")

Component for configuring the number of samples for [Multi-Sample Anti-Aliasing](https://en.wikipedia.org/wiki/Multisample_anti-aliasing) for a [`Camera`](../../prelude/struct.Camera.html "struct bevy::prelude::Camera").

## Constants

[COLOR\_TARGET\_FORMAT\_MASK\_BITS](constant.COLOR_TARGET_FORMAT_MASK_BITS.html "constant bevy::render::view::COLOR_TARGET_FORMAT_MASK_BITS")

Mask bits (5-bit) for use in pipeline key bitfields.

[VISIBILITY\_RANGES\_STORAGE\_BUFFER\_COUNT](constant.VISIBILITY_RANGES_STORAGE_BUFFER_COUNT.html "constant bevy::render::view::VISIBILITY_RANGES_STORAGE_BUFFER_COUNT")

We need at least 4 storage buffer bindings available to enable the visibility range buffer.

## Functions

[cleanup\_view\_targets\_for\_resize](fn.cleanup_view_targets_for_resize.html "fn bevy::render::view::cleanup_view_targets_for_resize")

[clear\_view\_attachments](fn.clear_view_attachments.html "fn bevy::render::view::clear_view_attachments")

Clears the view target [`OutputColorAttachment`](../texture/struct.OutputColorAttachment.html "struct bevy::render::texture::OutputColorAttachment")s.

[collect\_visible\_cpu\_culled\_entities](fn.collect_visible_cpu_culled_entities.html "fn bevy::render::view::collect_visible_cpu_culled_entities")

Updates the [`RenderVisibleEntities`](struct.RenderVisibleEntities.html "struct bevy::render::view::RenderVisibleEntities") and [`RenderShadowMapVisibleEntities`](struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities") components with the contents of the [`RenderExtractedVisibleEntities`](struct.RenderExtractedVisibleEntities.html "struct bevy::render::view::RenderExtractedVisibleEntities") and the [`RenderExtractedShadowMapVisibleEntities`](struct.RenderExtractedShadowMapVisibleEntities.html "struct bevy::render::view::RenderExtractedShadowMapVisibleEntities") components respectively.

[create\_surfaces](fn.create_surfaces.html "fn bevy::render::view::create_surfaces")

Creates window surfaces.

[extract\_visibility\_ranges](fn.extract_visibility_ranges.html "fn bevy::render::view::extract_visibility_ranges")

Extracts all [`VisibilityRange`](../../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange") components from the main world to the render world and inserts them into [`RenderVisibilityRanges`](struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges").

[need\_surface\_configuration](fn.need_surface_configuration.html "fn bevy::render::view::need_surface_configuration")

[prepare\_view\_attachments](fn.prepare_view_attachments.html "fn bevy::render::view::prepare_view_attachments")

Prepares the view target [`OutputColorAttachment`](../texture/struct.OutputColorAttachment.html "struct bevy::render::texture::OutputColorAttachment") for each view in the current frame.

[prepare\_view\_targets](fn.prepare_view_targets.html "fn bevy::render::view::prepare_view_targets")

[prepare\_view\_uniforms](fn.prepare_view_uniforms.html "fn bevy::render::view::prepare_view_uniforms")

[prepare\_windows](fn.prepare_windows.html "fn bevy::render::view::prepare_windows")

(re)configures window surfaces, and obtains a swapchain texture for rendering.

[texture\_format\_from\_code](fn.texture_format_from_code.html "fn bevy::render::view::texture_format_from_code")

Decode a 5-bit code back into a [`TextureFormat`](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat").

[texture\_format\_to\_code](fn.texture_format_to_code.html "fn bevy::render::view::texture_format_to_code")

Encode a [`TextureFormat`](../render_resource/enum.TextureFormat.html "enum bevy::render::render_resource::TextureFormat") as a 5-bit code for use in pipeline key bitfields.

[write\_render\_visibility\_ranges](fn.write_render_visibility_ranges.html "fn bevy::render::view::write_render_visibility_ranges")

Writes the [`RenderVisibilityRanges`](struct.RenderVisibilityRanges.html "struct bevy::render::view::RenderVisibilityRanges") table to the GPU.

## Type Aliases

[VisibilityExtractionNoCpuCullingChangedQuery](type.VisibilityExtractionNoCpuCullingChangedQuery.html "type bevy::render::view::VisibilityExtractionNoCpuCullingChangedQuery")

The query, part of [`VisibilityExtractionSystemParam`](struct.VisibilityExtractionSystemParam.html "struct bevy::render::view::VisibilityExtractionSystemParam"), that searches for entities with [`NoCpuCulling`](../../camera/visibility/struct.NoCpuCulling.html "struct bevy::camera::visibility::NoCpuCulling") that might have changed visibility.
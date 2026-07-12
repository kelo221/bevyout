[bevy](../index.html)

# Crate ui\_render 

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#1-1978)

Provides rendering functionality for `bevy_ui`.

## Modules

[box\_shadow](box_shadow/index.html "mod bevy::ui_render::box_shadow")

Box shadows rendering

[prelude](prelude/index.html "mod bevy::ui_render::prelude")

[render\_pass](render_pass/index.html "mod bevy::ui_render::render_pass")

[shader\_flags](shader_flags/index.html "mod bevy::ui_render::shader_flags")

The values here should match the values for the constants in `ui.wgsl`

[stack\_z\_offsets](stack_z_offsets/index.html "mod bevy::ui_render::stack_z_offsets")

Local Z offsets of “extracted nodes” for a given entity. These exist to allow rendering multiple “extracted nodes” for a given source entity (ex: render both a background color _and_ a custom material for a given node).

[ui\_material](ui_material/index.html "mod bevy::ui_render::ui_material")

[ui\_texture\_slice\_pipeline](ui_texture_slice_pipeline/index.html "mod bevy::ui_render::ui_texture_slice_pipeline")

## Structs

[BoxShadowSamples](struct.BoxShadowSamples.html "struct bevy::ui_render::BoxShadowSamples")

Number of shadow samples. A larger value will result in higher quality shadows. Default is 4, values higher than ~10 offer diminishing returns.

[DrawUiMaterialNode](struct.DrawUiMaterialNode.html "struct bevy::ui_render::DrawUiMaterialNode")

[DrawUiNode](struct.DrawUiNode.html "struct bevy::ui_render::DrawUiNode")

[ExtractedGlyph](struct.ExtractedGlyph.html "struct bevy::ui_render::ExtractedGlyph")

[ExtractedUiMaterialNode](struct.ExtractedUiMaterialNode.html "struct bevy::ui_render::ExtractedUiMaterialNode")

[ExtractedUiMaterialNodes](struct.ExtractedUiMaterialNodes.html "struct bevy::ui_render::ExtractedUiMaterialNodes")

[ExtractedUiNode](struct.ExtractedUiNode.html "struct bevy::ui_render::ExtractedUiNode")

[ExtractedUiNodes](struct.ExtractedUiNodes.html "struct bevy::ui_render::ExtractedUiNodes")

[GlobalUiDebugOptions](struct.GlobalUiDebugOptions.html "struct bevy::ui_render::GlobalUiDebugOptions")

Configuration for the UI debug overlay

[ImageNodeBindGroups](struct.ImageNodeBindGroups.html "struct bevy::ui_render::ImageNodeBindGroups")

[PreparedUiMaterial](struct.PreparedUiMaterial.html "struct bevy::ui_render::PreparedUiMaterial")

[SetMatUiViewBindGroup](struct.SetMatUiViewBindGroup.html "struct bevy::ui_render::SetMatUiViewBindGroup")

[SetUiMaterialBindGroup](struct.SetUiMaterialBindGroup.html "struct bevy::ui_render::SetUiMaterialBindGroup")

[SetUiTextureBindGroup](struct.SetUiTextureBindGroup.html "struct bevy::ui_render::SetUiTextureBindGroup")

[SetUiViewBindGroup](struct.SetUiViewBindGroup.html "struct bevy::ui_render::SetUiViewBindGroup")

[TransparentUi](struct.TransparentUi.html "struct bevy::ui_render::TransparentUi")

[UiBatch](struct.UiBatch.html "struct bevy::ui_render::UiBatch")

[UiCameraMap](struct.UiCameraMap.html "struct bevy::ui_render::UiCameraMap")

[UiCameraMapper](struct.UiCameraMapper.html "struct bevy::ui_render::UiCameraMapper")

Helper for mapping UI target camera entities to their corresponding render entities, with caching to avoid repeated lookups for the same camera.

[UiCameraView](struct.UiCameraView.html "struct bevy::ui_render::UiCameraView")

A render-world component that lives on the main render target view and specifies the corresponding UI view.

[UiDebugOptions](struct.UiDebugOptions.html "struct bevy::ui_render::UiDebugOptions")

Configuration for the UI debug overlay

[UiMaterialBatch](struct.UiMaterialBatch.html "struct bevy::ui_render::UiMaterialBatch")

[UiMaterialMeta](struct.UiMaterialMeta.html "struct bevy::ui_render::UiMaterialMeta")

[UiMaterialPipeline](struct.UiMaterialPipeline.html "struct bevy::ui_render::UiMaterialPipeline")

Render pipeline data for a given [`UiMaterial`](../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial")

[UiMaterialPlugin](struct.UiMaterialPlugin.html "struct bevy::ui_render::UiMaterialPlugin")

Adds the necessary ECS resources and render logic to enable rendering entities using the given [`UiMaterial`](../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") asset type (which includes [`UiMaterial`](../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial") types).

[UiMaterialVertex](struct.UiMaterialVertex.html "struct bevy::ui_render::UiMaterialVertex")

[UiMeta](struct.UiMeta.html "struct bevy::ui_render::UiMeta")

[UiPipeline](struct.UiPipeline.html "struct bevy::ui_render::UiPipeline")

[UiPipelineKey](struct.UiPipelineKey.html "struct bevy::ui_render::UiPipelineKey")

[UiRenderPlugin](struct.UiRenderPlugin.html "struct bevy::ui_render::UiRenderPlugin")

[UiViewTarget](struct.UiViewTarget.html "struct bevy::ui_render::UiViewTarget")

A render-world component that lives on the UI view and specifies the corresponding main render target view.

## Enums

[ExtractedUiItem](enum.ExtractedUiItem.html "enum bevy::ui_render::ExtractedUiItem")

[NodeType](enum.NodeType.html "enum bevy::ui_render::NodeType")

The type of UI node. This is used to determine how to render the UI node.

[RenderUiSystems](enum.RenderUiSystems.html "enum bevy::ui_render::RenderUiSystems")

[UiAntiAlias](enum.UiAntiAlias.html "enum bevy::ui_render::UiAntiAlias")

Marker for controlling whether UI is rendered with or without anti-aliasing in a camera. By default, UI is always anti-aliased.

## Functions

[extract\_text\_decorations](fn.extract_text_decorations.html "fn bevy::ui_render::extract_text_decorations")

[extract\_text\_sections](fn.extract_text_sections.html "fn bevy::ui_render::extract_text_sections")

[extract\_text\_shadows](fn.extract_text_shadows.html "fn bevy::ui_render::extract_text_shadows")

[extract\_ui\_camera\_view](fn.extract_ui_camera_view.html "fn bevy::ui_render::extract_ui_camera_view")

Extracts all UI elements associated with a camera into the render world.

[extract\_ui\_material\_nodes](fn.extract_ui_material_nodes.html "fn bevy::ui_render::extract_ui_material_nodes")

[extract\_uinode\_background\_colors](fn.extract_uinode_background_colors.html "fn bevy::ui_render::extract_uinode_background_colors")

[extract\_uinode\_borders](fn.extract_uinode_borders.html "fn bevy::ui_render::extract_uinode_borders")

[extract\_uinode\_images](fn.extract_uinode_images.html "fn bevy::ui_render::extract_uinode_images")

[extract\_viewport\_nodes](fn.extract_viewport_nodes.html "fn bevy::ui_render::extract_viewport_nodes")

[init\_ui\_material\_pipeline](fn.init_ui_material_pipeline.html "fn bevy::ui_render::init_ui_material_pipeline")

[init\_ui\_pipeline](fn.init_ui_pipeline.html "fn bevy::ui_render::init_ui_pipeline")

[prepare\_uimaterial\_nodes](fn.prepare_uimaterial_nodes.html "fn bevy::ui_render::prepare_uimaterial_nodes")

[prepare\_uinodes](fn.prepare_uinodes.html "fn bevy::ui_render::prepare_uinodes")

[queue\_ui\_material\_nodes](fn.queue_ui_material_nodes.html "fn bevy::ui_render::queue_ui_material_nodes")

[queue\_uinodes](fn.queue_uinodes.html "fn bevy::ui_render::queue_uinodes")

[ui\_pass](fn.ui_pass.html "fn bevy::ui_render::ui_pass")

## Type Aliases

[DrawUi](type.DrawUi.html "type bevy::ui_render::DrawUi")

[DrawUiMaterial](type.DrawUiMaterial.html "type bevy::ui_render::DrawUiMaterial")
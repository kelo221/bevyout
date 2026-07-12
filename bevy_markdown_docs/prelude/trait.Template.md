[bevy](../index.html)::[prelude](index.html)

# Trait Template 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#32)

```rust
pub trait Template {
    type Output;

    // Required methods
    fn build_template(
        &self,
        context: &mut TemplateContext<'_, '_>,
    ) -> Result<Self::Output, BevyError>;
    fn clone_template(&self) -> Self;
}
```

A [`Template`](trait.Template.html "trait bevy::prelude::Template") is something that, given a spawn context (target [`Entity`](struct.Entity.html "struct bevy::prelude::Entity"), [`World`](struct.World.html "struct bevy::prelude::World"), etc), can produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[`Template`](trait.Template.html "trait bevy::prelude::Template") is the cornerstone of scene systems. It enables define types (and hierarchies) that require no [`World`](struct.World.html "struct bevy::prelude::World") or [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") context to define, but can _use_ that context to produce the final runtime state. A [`Template`](trait.Template.html "trait bevy::prelude::Template") is notably:

*   **Repeatable**: Building a [`Template`](trait.Template.html "trait bevy::prelude::Template") does not consume it. This enables reusing “baked” scenes / avoids rebuilding scenes each time we want to spawn one.
*   **Clone-able**: Templates can be duplicated via [`Template::clone_template`](trait.Template.html#tymethod.clone_template "method bevy::prelude::Template::clone_template"), enabling scenes to be duplicated, supporting copy-on-write behaviors, etc.
*   **(Often) Serializable**: Templates are intended to be easily serialized and deserialized, as they are typically composed of raw data.

Asset handles and [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") are two commonly [`Template`](trait.Template.html "trait bevy::prelude::Template")\-ed types. Asset handles are often “loaded” from an “asset path”. The “asset path” would be the [`Template`](trait.Template.html "trait bevy::prelude::Template"). Likewise [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") on its own has no reasonable default. A type with an [`Entity`](struct.Entity.html "struct bevy::prelude::Entity") reference could use an “entity path” template to point to a specific entity, relative to the current spawn context.

See [`FromTemplate`](trait.FromTemplate.html "trait bevy::prelude::FromTemplate"), which defines the canonical [`Template`](trait.Template.html "trait bevy::prelude::Template") for a type. This can be derived, which will generate a [`Template`](trait.Template.html "trait bevy::prelude::Template") for the deriving type.

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#34)

#### type [Output](#associatedtype.Output)

The type of value produced by this [`Template`](trait.Template.html "trait bevy::prelude::Template").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#37)

#### fn [build\_template](#tymethod.build_template)( &self, context: &mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>, ) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<Self::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\>

Uses this template and the given `entity` context to produce a [`Template::Output`](trait.Template.html#associatedtype.Output "associated type bevy::prelude::Template::Output").

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#40)

#### fn [clone\_template](#tymethod.clone_template)(&self) -> Self

Clones this template. See [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone").

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [AnimationGraphHandleTemplate](struct.AnimationGraphHandleTemplate.html "struct bevy::prelude::AnimationGraphHandleTemplate")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

#### type [Output](#associatedtype.Output) = [AnimationGraphHandle](struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#32)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [AtmosphereTemplate](../light/atmosphere/struct.AtmosphereTemplate.html "struct bevy::light::atmosphere::AtmosphereTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#32)

#### type [Output](#associatedtype.Output) = [Atmosphere](../light/struct.Atmosphere.html "struct bevy::light::Atmosphere")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#94)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [ChildOfTemplate](../ecs/hierarchy/struct.ChildOfTemplate.html "struct bevy::ecs::hierarchy::ChildOfTemplate")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#94)

#### type [Output](#associatedtype.Output) = [ChildOf](struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#12)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [CustomCursorImageTemplate](../window/struct.CustomCursorImageTemplate.html "struct bevy::window::CustomCursorImageTemplate")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#12)

#### type [Output](#associatedtype.Output) = [CustomCursorImage](../window/struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#68)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [CustomCursorTemplate](../window/enum.CustomCursorTemplate.html "enum bevy::window::CustomCursorTemplate")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#68)

#### type [Output](#associatedtype.Output) = [CustomCursor](../window/enum.CustomCursor.html "enum bevy::window::CustomCursor")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [DynamicWorldRootTemplate](../world_serialization/struct.DynamicWorldRootTemplate.html "struct bevy::world_serialization::DynamicWorldRootTemplate")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

#### type [Output](#associatedtype.Output) = [DynamicWorldRoot](struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [EntityCursorTemplate](../feathers/cursor/enum.EntityCursorTemplate.html "enum bevy::feathers::cursor::EntityCursorTemplate")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

#### type [Output](#associatedtype.Output) = [EntityCursor](../feathers/cursor/enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#450)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [EntityTemplate](../ecs/template/enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#451)

#### type [Output](#associatedtype.Output) = [Entity](struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#46)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [FeathersCheckboxTemplate](../feathers/controls/struct.FeathersCheckboxTemplate.html "struct bevy::feathers::controls::FeathersCheckboxTemplate")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#46)

#### type [Output](#associatedtype.Output) = [FeathersCheckbox](../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [FeathersColorPlaneTemplate](../feathers/controls/enum.FeathersColorPlaneTemplate.html "enum bevy::feathers::controls::FeathersColorPlaneTemplate")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

#### type [Output](#associatedtype.Output) = [FeathersColorPlane](../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [FontSourceTemplate](../text/enum.FontSourceTemplate.html "enum bevy::text::FontSourceTemplate")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

#### type [Output](#associatedtype.Output) = [FontSource](enum.FontSource.html "enum bevy::prelude::FontSource")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#279)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [GizmoMeshConfigTemplate](../gizmos/config/struct.GizmoMeshConfigTemplate.html "struct bevy::gizmos::config::GizmoMeshConfigTemplate")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#279)

#### type [Output](#associatedtype.Output) = [GizmoMeshConfig](../gizmos/config/struct.GizmoMeshConfig.html "struct bevy::gizmos::config::GizmoMeshConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [GizmoTemplate](../gizmos/retained/struct.GizmoTemplate.html "struct bevy::gizmos::retained::GizmoTemplate")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

#### type [Output](#associatedtype.Output) = [Gizmo](struct.Gizmo.html "struct bevy::prelude::Gizmo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [ImageNodeTemplate](../ui/widget/struct.ImageNodeTemplate.html "struct bevy::ui::widget::ImageNodeTemplate")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

#### type [Output](#associatedtype.Output) = [ImageNode](struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [InheritableFontTemplate](../feathers/font_styles/struct.InheritableFontTemplate.html "struct bevy::feathers::font_styles::InheritableFontTemplate")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

#### type [Output](#associatedtype.Output) = [InheritableFont](../feathers/font_styles/struct.InheritableFont.html "struct bevy::feathers::font_styles::InheritableFont")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [LightmapTemplate](../pbr/struct.LightmapTemplate.html "struct bevy::pbr::LightmapTemplate")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

#### type [Output](#associatedtype.Output) = [Lightmap](../pbr/struct.Lightmap.html "struct bevy::pbr::Lightmap")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#977)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [ManualTextureViewHandleTemplate](../camera/struct.ManualTextureViewHandleTemplate.html "struct bevy::camera::ManualTextureViewHandleTemplate")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#977)

#### type [Output](#associatedtype.Output) = [ManualTextureViewHandle](../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [Mesh2dTemplate](../mesh/struct.Mesh2dTemplate.html "struct bevy::mesh::Mesh2dTemplate")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

#### type [Output](#associatedtype.Output) = [Mesh2d](struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [Mesh2dWireframeTemplate](../sprite_render/struct.Mesh2dWireframeTemplate.html "struct bevy::sprite_render::Mesh2dWireframeTemplate")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

#### type [Output](#associatedtype.Output) = [Mesh2dWireframe](../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [Mesh3dTemplate](../mesh/struct.Mesh3dTemplate.html "struct bevy::mesh::Mesh3dTemplate")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

#### type [Output](#associatedtype.Output) = [Mesh3d](struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [Mesh3dWireframeTemplate](../pbr/wireframe/struct.Mesh3dWireframeTemplate.html "struct bevy::pbr::wireframe::Mesh3dWireframeTemplate")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

#### type [Output](#associatedtype.Output) = [Mesh3dWireframe](../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [MeshletMesh3dTemplate](../pbr/experimental/meshlet/struct.MeshletMesh3dTemplate.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3dTemplate")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

#### type [Output](#associatedtype.Output) = [MeshletMesh3d](../pbr/experimental/meshlet/struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [ReadbackTemplate](../render/gpu_readback/enum.ReadbackTemplate.html "enum bevy::render::gpu_readback::ReadbackTemplate")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

#### type [Output](#associatedtype.Output) = [Readback](../render/gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#109)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [ScenePatchInstanceTemplate](../scene/struct.ScenePatchInstanceTemplate.html "struct bevy::scene::ScenePatchInstanceTemplate")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#109)

#### type [Output](#associatedtype.Output) = [ScenePatchInstance](struct.ScenePatchInstance.html "struct bevy::prelude::ScenePatchInstance")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [ScrollbarTemplate](../ui_widgets/struct.ScrollbarTemplate.html "struct bevy::ui_widgets::ScrollbarTemplate")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

#### type [Output](#associatedtype.Output) = [Scrollbar](../ui_widgets/struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [SimplifiedMeshTemplate](../picking/mesh_picking/ray_cast/struct.SimplifiedMeshTemplate.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMeshTemplate")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

#### type [Output](#associatedtype.Output) = [SimplifiedMesh](../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [SkinnedMeshTemplate](../mesh/skinning/struct.SkinnedMeshTemplate.html "struct bevy::mesh::skinning::SkinnedMeshTemplate")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

#### type [Output](#associatedtype.Output) = [SkinnedMesh](../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [SpriteMeshTemplate](../sprite/struct.SpriteMeshTemplate.html "struct bevy::sprite::SpriteMeshTemplate")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

#### type [Output](#associatedtype.Output) = [SpriteMesh](struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [SpriteTemplate](../sprite/struct.SpriteTemplate.html "struct bevy::sprite::SpriteTemplate")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

#### type [Output](#associatedtype.Output) = [Sprite](struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [TextFontTemplate](../text/struct.TextFontTemplate.html "struct bevy::text::TextFontTemplate")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

#### type [Output](#associatedtype.Output) = [TextFont](struct.TextFont.html "struct bevy::prelude::TextFont")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#208)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [TextureAtlasTemplate](../image/struct.TextureAtlasTemplate.html "struct bevy::image::TextureAtlasTemplate")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#208)

#### type [Output](#associatedtype.Output) = [TextureAtlas](struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [TilemapChunkTemplate](../sprite_render/struct.TilemapChunkTemplate.html "struct bevy::sprite_render::TilemapChunkTemplate")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

#### type [Output](#associatedtype.Output) = [TilemapChunk](../sprite_render/struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

### impl [Template](trait.Template.html "trait bevy::prelude::Template") for [WorldAssetRootTemplate](../world_serialization/struct.WorldAssetRootTemplate.html "struct bevy::world_serialization::WorldAssetRootTemplate")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

#### type [Output](#associatedtype.Output) = [WorldAssetRoot](struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#482)

### impl<F, O> [Template](trait.Template.html "trait bevy::prelude::Template") for [FnTemplate](../ecs/template/struct.FnTemplate.html "struct bevy::ecs::template::FnTemplate")<F, O>

where F: [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&mut [TemplateContext](../ecs/template/struct.TemplateContext.html "struct bevy::ecs::template::TemplateContext")<'\_, '\_>) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<O, [BevyError](struct.BevyError.html "struct bevy::prelude::BevyError")\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#483)

#### type [Output](#associatedtype.Output) = O

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#519-520)

### impl<I, E, B, M> [Template](trait.Template.html "trait bevy::prelude::Template") for [OnTemplate](../scene/struct.OnTemplate.html "struct bevy::scene::OnTemplate")<I, E, B, M>

where I: [IntoObserverSystem](../ecs/system/trait.IntoObserverSystem.html "trait bevy::ecs::system::IntoObserverSystem")<E, B, M> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), E: [EntityEvent](trait.EntityEvent.html "trait bevy::prelude::EntityEvent"), B: [Bundle](trait.Bundle.html "trait bevy::prelude::Bundle"), M: 'static,

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#522)

#### type [Output](#associatedtype.Output) = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#352)

### impl<I, O> [Template](trait.Template.html "trait bevy::prelude::Template") for [SystemHandleTemplate](../ecs/system/enum.SystemHandleTemplate.html "enum bevy::ecs::system::SystemHandleTemplate")<I, O>

where I: [SystemInput](trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#353)

#### type [Output](#associatedtype.Output) = [SystemHandle](../ecs/system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle")<I, O>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#167)

### impl<M> [Template](trait.Template.html "trait bevy::prelude::Template") for [MaterialNodeTemplate](struct.MaterialNodeTemplate.html "struct bevy::prelude::MaterialNodeTemplate")<M>

where M: [UiMaterial](trait.UiMaterial.html "trait bevy::prelude::UiMaterial"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#167)

#### type [Output](#associatedtype.Output) = [MaterialNode](struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

### impl<M> [Template](trait.Template.html "trait bevy::prelude::Template") for [MeshMaterial2dTemplate](../sprite_render/struct.MeshMaterial2dTemplate.html "struct bevy::sprite_render::MeshMaterial2dTemplate")<M>

where M: [Material2d](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

#### type [Output](#associatedtype.Output) = [MeshMaterial2d](struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

### impl<M> [Template](trait.Template.html "trait bevy::prelude::Template") for [MeshMaterial3dTemplate](../pbr/struct.MeshMaterial3dTemplate.html "struct bevy::pbr::MeshMaterial3dTemplate")<M>

where M: [Material](trait.Material.html "trait bevy::prelude::Material"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

#### type [Output](#associatedtype.Output) = [MeshMaterial3d](struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

### impl<Source> [Template](trait.Template.html "trait bevy::prelude::Template") for [AudioPlayerTemplate](../audio/struct.AudioPlayerTemplate.html "struct bevy::audio::AudioPlayerTemplate")<Source>

where Source: [Asset](trait.Asset.html "trait bevy::prelude::Asset") + [Decodable](trait.Decodable.html "trait bevy::prelude::Decodable"),

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

#### type [Output](#associatedtype.Output) = [AudioPlayer](struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"), T5: [Template](trait.Template.html "trait bevy::prelude::Template"), T6: [Template](trait.Template.html "trait bevy::prelude::Template"), T7: [Template](trait.Template.html "trait bevy::prelude::Template"), T8: [Template](trait.Template.html "trait bevy::prelude::Template"), T9: [Template](trait.Template.html "trait bevy::prelude::Template"), T10: [Template](trait.Template.html "trait bevy::prelude::Template"), T11: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T5 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T6 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T7 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T8 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T9 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T10 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T11 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"), T5: [Template](trait.Template.html "trait bevy::prelude::Template"), T6: [Template](trait.Template.html "trait bevy::prelude::Template"), T7: [Template](trait.Template.html "trait bevy::prelude::Template"), T8: [Template](trait.Template.html "trait bevy::prelude::Template"), T9: [Template](trait.Template.html "trait bevy::prelude::Template"), T10: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T5 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T6 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T7 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T8 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T9 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T10 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"), T5: [Template](trait.Template.html "trait bevy::prelude::Template"), T6: [Template](trait.Template.html "trait bevy::prelude::Template"), T7: [Template](trait.Template.html "trait bevy::prelude::Template"), T8: [Template](trait.Template.html "trait bevy::prelude::Template"), T9: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T5 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T6 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T7 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T8 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T9 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4, T5, T6, T7, T8)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"), T5: [Template](trait.Template.html "trait bevy::prelude::Template"), T6: [Template](trait.Template.html "trait bevy::prelude::Template"), T7: [Template](trait.Template.html "trait bevy::prelude::Template"), T8: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T5 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T6 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T7 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T8 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4, T5, T6, T7> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4, T5, T6, T7)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"), T5: [Template](trait.Template.html "trait bevy::prelude::Template"), T6: [Template](trait.Template.html "trait bevy::prelude::Template"), T7: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T5 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T6 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T7 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4, T5, T6> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4, T5, T6)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"), T5: [Template](trait.Template.html "trait bevy::prelude::Template"), T6: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T5 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T6 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4, T5> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4, T5)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"), T5: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T5 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3, T4> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3, T4)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"), T4: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T4 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2, T3> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2, T3)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"), T3: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T3 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1, T2> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1, T2)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"), T2: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T2 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0, T1> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0, T1)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"), T1: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"), <T1 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"))

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

### impl<T0> [Template](trait.Template.html "trait bevy::prelude::Template") for [TemplateTuple](../ecs/template/struct.TemplateTuple.html "struct bevy::ecs::template::TemplateTuple")<[(T0,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>

where T0: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#386)

#### type [Output](#associatedtype.Output) = (<T0 as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output"),)

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#341)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for [HandleTemplate](../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>

where T: [Asset](trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#342)

#### type [Output](#associatedtype.Output) = [Handle](enum.Handle.html "enum bevy::prelude::Handle")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#545)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for [OptionTemplate](../ecs/template/enum.OptionTemplate.html "enum bevy::ecs::template::OptionTemplate")<T>

where T: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#546)

#### type [Output](#associatedtype.Output) = [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<<T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output")\>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#390)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#391)

#### type [Output](#associatedtype.Output) = T

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#572)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for [VecTemplate](../ecs/template/struct.VecTemplate.html "struct bevy::ecs::template::VecTemplate")<T>

where T: [Template](trait.Template.html "trait bevy::prelude::Template"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#573)

#### type [Output](#associatedtype.Output) = [Vec](struct.Vec.html "struct bevy::prelude::Vec")<<T as [Template](trait.Template.html "trait bevy::prelude::Template")\>::[Output](trait.Template.html#associatedtype.Output "type bevy::prelude::Template::Output")\>

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#20)

### impl<T> [Template](trait.Template.html "trait bevy::prelude::Template") for [VirtualKeyboardTemplate](../feathers/controls/struct.VirtualKeyboardTemplate.html "struct bevy::feathers::controls::VirtualKeyboardTemplate")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#20)

#### type [Output](#associatedtype.Output) = [VirtualKeyboard](../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>
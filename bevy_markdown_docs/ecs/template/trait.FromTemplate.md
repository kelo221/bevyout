[bevy](../../index.html)::[ecs](../index.html)::[template](index.html)

# Trait FromTemplate 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#348)

```rust
pub trait FromTemplate: Sized {
    type Template: Template<Output = Self>;
}
```

[`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") is implemented for types that can be produced by a specific, canonical [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"). This creates a way to correlate to the [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") using the desired template output type. This is used by Bevy’s scene system.

Both [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") and [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") are blanket implemented for types that implement [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") and [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), meaning most types you would want to use _already have templates_.

It is best to think of [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") as an alternative to [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") for types that require world/spawn context to instantiate. Note that because of the blanket impl, you cannot implement [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"), [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default"), and [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") together on the same type, as it would result in two conflicting [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") impls. This is also why [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") has its own [`Template::clone_template`](../../prelude/trait.Template.html#tymethod.clone_template "method bevy::prelude::Template::clone_template") method (to avoid using the [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") impl, which would pull in the auto-impl).

You can _and should_ prefer deriving [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") and [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") instead of an explicit [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") impl, unless your type uses something that requires (or uses) a [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template"). Handles in an asset system or [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") are examples of “templated” types. If you want your type to support templates of them, you probably want to derive [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate").

[`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") can be derived for types whose fields _also_ implement [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"):

```rust
#[derive(FromTemplate)]
struct Player {
    image: Handle<Image>
}
```

Deriving [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") will generate a [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") type for the deriving type. The example above would generate a `PlayerTemplate` like this:

```rust
struct Player {
    image: Handle<Image>
}

impl FromTemplate for Player {
    type Template = PlayerTemplate;
}

struct PlayerTemplate {
    image: HandleTemplate<Image>,
}

impl Template for PlayerTemplate {
    type Output = Player;
    fn build_template(&self, context: &mut TemplateContext) -> Result<Self::Output> {
        Ok(Player {
            image: self.image.build_template(context)?,
        })
    }

    fn clone_template(&self) -> Self {
        PlayerTemplate {
            image: self.image.clone_template(),
        }
    }
}
```

[`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") derives can specify custom templates to use instead of a canonical [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"):

```rust
#[derive(FromTemplate)]
struct Counter {
    #[template(Always10)]
    count: usize
}

#[derive(Default)]
struct Always10;

impl Template for Always10 {
    type Output = usize;

    fn build_template(&self, context: &mut TemplateContext) -> Result<Self::Output> {
        Ok(10)
    }

    fn clone_template(&self) -> Self {
        Always10
    }
}
```

[`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") is automatically implemented for anything that is [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") and [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"). “Built in” collection types like [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") and [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec") pick up this “blanket” implementation, which is generally a good thing because it means these collection types work with [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") derives by default. However if the items in the collection have a custom [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") impl (ex: a manual implementation like `Handle<T>` for assets or an explicit [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") derive), then relying on a [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") / [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") implementation doesn’t work, as that won’t run the template logic!

Therefore, cases like [`Option<Handle<T>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") need something other than [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") to determine the type. One option is to specify the template manually:

```rust
#[derive(FromTemplate)]
struct Widget {
    #[template(OptionTemplate<HandleTemplate<Image>>)]
    image: Option<Handle<Image>>
}
```

However that is a bit of a mouthful! This is where [`BuiltInTemplate`](trait.BuiltInTemplate.html "trait bevy::ecs::template::BuiltInTemplate") comes in. It fills the same role as [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"), but has no blanket implementation for [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") and [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"), meaning we can have custom implementations for types like [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") and [`Vec`](../../prelude/struct.Vec.html "struct bevy::prelude::Vec").

If you are deriving [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") and you have a “built in” type like [`Option<Handle<T>>`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") which has custom template logic, annotate it with the `template(built_in)` attribute to use [`BuiltInTemplate`](trait.BuiltInTemplate.html "trait bevy::ecs::template::BuiltInTemplate") instead of [`FromTemplate`](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate"):

```rust
#[derive(FromTemplate)]
struct Widget {
    #[template(built_in)]
    image: Option<Handle<Image>>
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#350)

#### type [Template](#associatedtype.Template): [Template](../../prelude/trait.Template.html "trait bevy::prelude::Template")<Output = Self>

The [`Template`](../../prelude/trait.Template.html "trait bevy::prelude::Template") for this type.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [AnimationGraphHandle](../../prelude/struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

#### type [Template](#associatedtype.Template) = [AnimationGraphHandleTemplate](../../prelude/struct.AnimationGraphHandleTemplate.html "struct bevy::prelude::AnimationGraphHandleTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#32)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Atmosphere](../../light/struct.Atmosphere.html "struct bevy::light::Atmosphere")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/atmosphere.rs.html#32)

#### type [Template](#associatedtype.Template) = [AtmosphereTemplate](../../light/atmosphere/struct.AtmosphereTemplate.html "struct bevy::light::atmosphere::AtmosphereTemplate")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#94)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [ChildOf](../../prelude/struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#94)

#### type [Template](#associatedtype.Template) = [ChildOfTemplate](../hierarchy/struct.ChildOfTemplate.html "struct bevy::ecs::hierarchy::ChildOfTemplate")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#68)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [CustomCursor](../../window/enum.CustomCursor.html "enum bevy::window::CustomCursor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#68)

#### type [Template](#associatedtype.Template) = [CustomCursorTemplate](../../window/enum.CustomCursorTemplate.html "enum bevy::window::CustomCursorTemplate")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#12)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [CustomCursorImage](../../window/struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#12)

#### type [Template](#associatedtype.Template) = [CustomCursorImageTemplate](../../window/struct.CustomCursorImageTemplate.html "struct bevy::window::CustomCursorImageTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#173)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [DirectionalLightTexture](../../light/struct.DirectionalLightTexture.html "struct bevy::light::DirectionalLightTexture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#173)

#### type [Template](#associatedtype.Template) = DirectionalLightTextureTemplate

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [DynamicWorldRoot](../../prelude/struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

#### type [Template](#associatedtype.Template) = [DynamicWorldRootTemplate](../../world_serialization/struct.DynamicWorldRootTemplate.html "struct bevy::world_serialization::DynamicWorldRootTemplate")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#474)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Entity](../../prelude/struct.Entity.html "struct bevy::prelude::Entity")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#475)

#### type [Template](#associatedtype.Template) = [EntityTemplate](enum.EntityTemplate.html "enum bevy::ecs::template::EntityTemplate")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [EntityCursor](../../feathers/cursor/enum.EntityCursor.html "enum bevy::feathers::cursor::EntityCursor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#32)

#### type [Template](#associatedtype.Template) = [EntityCursorTemplate](../../feathers/cursor/enum.EntityCursorTemplate.html "enum bevy::feathers::cursor::EntityCursorTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#105)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [EnvironmentMapLight](../../prelude/struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#105)

#### type [Template](#associatedtype.Template) = EnvironmentMapLightTemplate

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#46)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [FeathersCheckbox](../../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#46)

#### type [Template](#associatedtype.Template) = [FeathersCheckboxTemplate](../../feathers/controls/struct.FeathersCheckboxTemplate.html "struct bevy::feathers::controls::FeathersCheckboxTemplate")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [FeathersColorPlane](../../feathers/controls/enum.FeathersColorPlane.html "enum bevy::feathers::controls::FeathersColorPlane")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#47)

#### type [Template](#associatedtype.Template) = [FeathersColorPlaneTemplate](../../feathers/controls/enum.FeathersColorPlaneTemplate.html "enum bevy::feathers::controls::FeathersColorPlaneTemplate")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [FontSource](../../prelude/enum.FontSource.html "enum bevy::prelude::FontSource")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#267)

#### type [Template](#associatedtype.Template) = [FontSourceTemplate](../../text/enum.FontSourceTemplate.html "enum bevy::text::FontSourceTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#261)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [GeneratedEnvironmentMapLight](../../prelude/struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#261)

#### type [Template](#associatedtype.Template) = GeneratedEnvironmentMapLightTemplate

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Gizmo](../../prelude/struct.Gizmo.html "struct bevy::prelude::Gizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

#### type [Template](#associatedtype.Template) = [GizmoTemplate](../../gizmos/retained/struct.GizmoTemplate.html "struct bevy::gizmos::retained::GizmoTemplate")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#279)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [GizmoMeshConfig](../../gizmos/config/struct.GizmoMeshConfig.html "struct bevy::gizmos::config::GizmoMeshConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#279)

#### type [Template](#associatedtype.Template) = [GizmoMeshConfigTemplate](../../gizmos/config/struct.GizmoMeshConfigTemplate.html "struct bevy::gizmos::config::GizmoMeshConfigTemplate")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [ImageNode](../../prelude/struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

#### type [Template](#associatedtype.Template) = [ImageNodeTemplate](../../ui/widget/struct.ImageNodeTemplate.html "struct bevy::ui::widget::ImageNodeTemplate")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [InheritableFont](../../feathers/font_styles/struct.InheritableFont.html "struct bevy::feathers::font_styles::InheritableFont")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

#### type [Template](#associatedtype.Template) = [InheritableFontTemplate](../../feathers/font_styles/struct.InheritableFontTemplate.html "struct bevy::feathers::font_styles::InheritableFontTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#329)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [IrradianceVolume](../../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#329)

#### type [Template](#associatedtype.Template) = IrradianceVolumeTemplate

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Lightmap](../../pbr/struct.Lightmap.html "struct bevy::pbr::Lightmap")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

#### type [Template](#associatedtype.Template) = [LightmapTemplate](../../pbr/struct.LightmapTemplate.html "struct bevy::pbr::LightmapTemplate")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#977)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [ManualTextureViewHandle](../../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#977)

#### type [Template](#associatedtype.Template) = [ManualTextureViewHandleTemplate](../../camera/struct.ManualTextureViewHandleTemplate.html "struct bevy::camera::ManualTextureViewHandleTemplate")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Mesh2d](../../prelude/struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

#### type [Template](#associatedtype.Template) = [Mesh2dTemplate](../../mesh/struct.Mesh2dTemplate.html "struct bevy::mesh::Mesh2dTemplate")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Mesh2dWireframe](../../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

#### type [Template](#associatedtype.Template) = [Mesh2dWireframeTemplate](../../sprite_render/struct.Mesh2dWireframeTemplate.html "struct bevy::sprite_render::Mesh2dWireframeTemplate")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Mesh3d](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

#### type [Template](#associatedtype.Template) = [Mesh3dTemplate](../../mesh/struct.Mesh3dTemplate.html "struct bevy::mesh::Mesh3dTemplate")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Mesh3dWireframe](../../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

#### type [Template](#associatedtype.Template) = [Mesh3dWireframeTemplate](../../pbr/wireframe/struct.Mesh3dWireframeTemplate.html "struct bevy::pbr::wireframe::Mesh3dWireframeTemplate")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [MeshletMesh3d](../../pbr/experimental/meshlet/struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

#### type [Template](#associatedtype.Template) = [MeshletMesh3dTemplate](../../pbr/experimental/meshlet/struct.MeshletMesh3dTemplate.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3dTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#159)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [PointLightTexture](../../light/struct.PointLightTexture.html "struct bevy::light::PointLightTexture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#159)

#### type [Template](#associatedtype.Template) = PointLightTextureTemplate

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/types.rs.html#19)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [RaytracingMesh3d](../../solari/scene/struct.RaytracingMesh3d.html "struct bevy::solari::scene::RaytracingMesh3d")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/types.rs.html#19)

#### type [Template](#associatedtype.Template) = RaytracingMesh3dTemplate

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Readback](../../render/gpu_readback/enum.Readback.html "enum bevy::render::gpu_readback::Readback")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#76)

#### type [Template](#associatedtype.Template) = [ReadbackTemplate](../../render/gpu_readback/enum.ReadbackTemplate.html "enum bevy::render::gpu_readback::ReadbackTemplate")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#109)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [ScenePatchInstance](../../prelude/struct.ScenePatchInstance.html "struct bevy::prelude::ScenePatchInstance")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_patch.rs.html#109)

#### type [Template](#associatedtype.Template) = [ScenePatchInstanceTemplate](../../scene/struct.ScenePatchInstanceTemplate.html "struct bevy::scene::ScenePatchInstanceTemplate")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Scrollbar](../../ui_widgets/struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

#### type [Template](#associatedtype.Template) = [ScrollbarTemplate](../../ui_widgets/struct.ScrollbarTemplate.html "struct bevy::ui_widgets::ScrollbarTemplate")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [SimplifiedMesh](../../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

#### type [Template](#associatedtype.Template) = [SimplifiedMeshTemplate](../../picking/mesh_picking/ray_cast/struct.SimplifiedMeshTemplate.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMeshTemplate")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [SkinnedMesh](../../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

#### type [Template](#associatedtype.Template) = [SkinnedMeshTemplate](../../mesh/skinning/struct.SkinnedMeshTemplate.html "struct bevy::mesh::skinning::SkinnedMeshTemplate")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#227)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Skybox](../../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#227)

#### type [Template](#associatedtype.Template) = SkyboxTemplate

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#204)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [SpotLightTexture](../../light/struct.SpotLightTexture.html "struct bevy::light::SpotLightTexture")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#204)

#### type [Template](#associatedtype.Template) = SpotLightTextureTemplate

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Sprite](../../prelude/struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

#### type [Template](#associatedtype.Template) = [SpriteTemplate](../../sprite/struct.SpriteTemplate.html "struct bevy::sprite::SpriteTemplate")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [SpriteMesh](../../prelude/struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

#### type [Template](#associatedtype.Template) = [SpriteMeshTemplate](../../sprite/struct.SpriteMeshTemplate.html "struct bevy::sprite::SpriteMeshTemplate")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [TextFont](../../prelude/struct.TextFont.html "struct bevy::prelude::TextFont")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

#### type [Template](#associatedtype.Template) = [TextFontTemplate](../../text/struct.TextFontTemplate.html "struct bevy::text::TextFontTemplate")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#208)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [TextureAtlas](../../prelude/struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#208)

#### type [Template](#associatedtype.Template) = [TextureAtlasTemplate](../../image/struct.TextureAtlasTemplate.html "struct bevy::image::TextureAtlasTemplate")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [TilemapChunk](../../sprite_render/struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

#### type [Template](#associatedtype.Template) = [TilemapChunkTemplate](../../sprite_render/struct.TilemapChunkTemplate.html "struct bevy::sprite_render::TilemapChunkTemplate")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

### impl [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [WorldAssetRoot](../../prelude/struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

#### type [Template](#associatedtype.Template) = [WorldAssetRootTemplate](../../world_serialization/struct.WorldAssetRootTemplate.html "struct bevy::world_serialization::WorldAssetRootTemplate")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#307)

### impl<I, O> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [SystemHandle](../system/enum.SystemHandle.html "enum bevy::ecs::system::SystemHandle")<I, O>

where I: [SystemInput](../../prelude/trait.SystemInput.html "trait bevy::prelude::SystemInput") + 'static, O: 'static,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_registry.rs.html#308)

#### type [Template](#associatedtype.Template) = [SystemHandleTemplate](../system/enum.SystemHandleTemplate.html "enum bevy::ecs::system::SystemHandleTemplate")<I, O>

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#167)

### impl<M> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [MaterialNode](../../prelude/struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

where M: [UiMaterial](../../prelude/trait.UiMaterial.html "trait bevy::prelude::UiMaterial"),

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#167)

#### type [Template](#associatedtype.Template) = [MaterialNodeTemplate](../../prelude/struct.MaterialNodeTemplate.html "struct bevy::prelude::MaterialNodeTemplate")<M>

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

### impl<M> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [MeshMaterial2d](../../prelude/struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

where M: [Material2d](../../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d"),

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

#### type [Template](#associatedtype.Template) = [MeshMaterial2dTemplate](../../sprite_render/struct.MeshMaterial2dTemplate.html "struct bevy::sprite_render::MeshMaterial2dTemplate")<M>

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

### impl<M> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [MeshMaterial3d](../../prelude/struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

where M: [Material](../../prelude/trait.Material.html "trait bevy::prelude::Material"),

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

#### type [Template](#associatedtype.Template) = [MeshMaterial3dTemplate](../../pbr/struct.MeshMaterial3dTemplate.html "struct bevy::pbr::MeshMaterial3dTemplate")<M>

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

### impl<Source> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [AudioPlayer](../../prelude/struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>

where Source: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset") + [Decodable](../../prelude/trait.Decodable.html "trait bevy::prelude::Decodable"),

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

#### type [Template](#associatedtype.Template) = [AudioPlayerTemplate](../../audio/struct.AudioPlayerTemplate.html "struct bevy::audio::AudioPlayerTemplate")<Source>

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#244)

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<T>

where T: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#245)

#### type [Template](#associatedtype.Template) = [HandleTemplate](../../asset/enum.HandleTemplate.html "enum bevy::asset::HandleTemplate")<T>

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#404)

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for T

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#405)

#### type [Template](#associatedtype.Template) = T

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#20)

### impl<T> [FromTemplate](../../prelude/trait.FromTemplate.html "trait bevy::prelude::FromTemplate") for [VirtualKeyboard](../../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#20)

#### type [Template](#associatedtype.Template) = [VirtualKeyboardTemplate](../../feathers/controls/struct.VirtualKeyboardTemplate.html "struct bevy::feathers::controls::VirtualKeyboardTemplate")<T>
[bevy](../index.html)::[prelude](index.html)

# Trait TupleStruct 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#47)

```rust
pub trait TupleStruct: PartialReflect {
    // Required methods
    fn field(&self, index: usize) -> Option<&(dyn PartialReflect + 'static)>;
    fn field_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn field_len(&self) -> usize;
    fn iter_fields(&self) -> TupleStructFieldIter<'_> ⓘ;

    // Provided methods
    fn to_dynamic_tuple_struct(&self) -> DynamicTupleStruct { ... }
    fn get_represented_tuple_struct_info(
        &self,
    ) -> Option<&'static TupleStructInfo> { ... }
}
```

A trait used to power [tuple struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types) operations via [reflection](../reflect/index.html "mod bevy::reflect").

This trait uses the [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") trait to allow implementors to have their fields be dynamically addressed by index.

When using [`#[derive(Reflect)]`](derive.Reflect.html "derive bevy::prelude::Reflect") on a tuple struct, this trait will be automatically implemented.

## Example

```rust
use bevy_reflect::{PartialReflect, Reflect, tuple_struct::TupleStruct};

#[derive(Reflect)]
struct Foo(u32);

let foo = Foo(123);

assert_eq!(foo.field_len(), 1);

let field: &dyn PartialReflect = foo.field(0).unwrap();
assert_eq!(field.try_downcast_ref::<u32>(), Some(&123));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#50)

#### fn [field](#tymethod.field)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a reference to the value of the field with index `index` as a `&dyn Reflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#54)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Returns a mutable reference to the value of the field with index `index` as a `&mut dyn Reflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#57)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the tuple struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#60)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [TupleStructFieldIter](../reflect/tuple_struct/struct.TupleStructFieldIter.html "struct bevy::reflect::tuple_struct::TupleStructFieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the tuple struct’s fields.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#63)

#### fn [to\_dynamic\_tuple\_struct](#method.to_dynamic_tuple_struct)(&self) -> [DynamicTupleStruct](../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

Creates a new [`DynamicTupleStruct`](../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct") from this tuple struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#71)

#### fn [get\_represented\_tuple\_struct\_info](#method.get_represented_tuple_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [TupleStructInfo](../reflect/tuple_struct/struct.TupleStructInfo.html "struct bevy::reflect::tuple_struct::TupleStructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

## Trait Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#219)

### impl [GetTupleStructField](trait.GetTupleStructField.html "trait bevy::prelude::GetTupleStructField") for dyn [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#220)

#### fn [get\_field](trait.GetTupleStructField.html#tymethod.get_field)<T>(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a reference to the value of the field with index `index`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#225)

#### fn [get\_field\_mut](trait.GetTupleStructField.html#tymethod.get_field_mut)<T>(&mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Returns a mutable reference to the value of the field with index `index`, downcast to `T`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#110)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [AccessibilityRequested](../a11y/struct.AccessibilityRequested.html "struct bevy::a11y::AccessibilityRequested")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/accessibility.rs.html#185)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [AccessibleLabel](struct.AccessibleLabel.html "struct bevy::prelude::AccessibleLabel")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#49)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ActiveDescendant](../ui_widgets/struct.ActiveDescendant.html "struct bevy::ui_widgets::ActiveDescendant")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#254)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Anchor](../sprite/struct.Anchor.html "struct bevy::sprite::Anchor")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#213)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [AnimatedBy](../animation/struct.AnimatedBy.html "struct bevy::animation::AnimatedBy")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#135)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [AnimationGraphHandle](struct.AnimationGraphHandle.html "struct bevy::prelude::AnimationGraphHandle")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#184)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [AnimationTargetId](../animation/struct.AnimationTargetId.html "struct bevy::animation::AnimationTargetId")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/aspect_ratio.rs.html#14)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [AspectRatio](../math/struct.AspectRatio.html "struct bevy::math::AspectRatio")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2222)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [BackgroundColor](struct.BackgroundColor.html "struct bevy::prelude::BackgroundColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#526)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [BackgroundGradient](struct.BackgroundGradient.html "struct bevy::prelude::BackgroundGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#542)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [BorderGradient](struct.BorderGradient.html "struct bevy::prelude::BorderGradient")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2831)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [BoxShadow](struct.BoxShadow.html "struct bevy::prelude::BoxShadow")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/lib.rs.html#186)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [BoxShadowSamples](struct.BoxShadowSamples.html "struct bevy::prelude::BoxShadowSamples")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#41)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Camera3dDepthTextureUsage](../camera/struct.Camera3dDepthTextureUsage.html "struct bevy::camera::Camera3dDepthTextureUsage")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#95)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ChildOf](struct.ChildOf.html "struct bevy::prelude::ChildOf")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/hierarchy.rs.html#149)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Children](struct.Children.html "struct bevy::prelude::Children")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/clear_color.rs.html#53)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ClearColor](struct.ClearColor.html "struct bevy::prelude::ClearColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_plane.rs.html#68)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ColorPlaneValue](../feathers/controls/struct.ColorPlaneValue.html "struct bevy::feathers::controls::ColorPlaneValue")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#33)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ColorSwatchValue](../feathers/controls/struct.ColorSwatchValue.html "struct bevy::feathers::controls::ColorSwatchValue")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/component/info.rs.html#178)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ComponentId](../ecs/component/struct.ComponentId.html "struct bevy::ecs::component::ComponentId")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#17)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ComputedStackIndex](../ui/struct.ComputedStackIndex.html "struct bevy::ui::ComputedStackIndex")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#23)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DefaultCursor](../feathers/cursor/struct.DefaultCursor.html "struct bevy::feathers::cursor::DefaultCursor")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material.rs.html#1358)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DefaultOpaqueRendererMethod](../pbr/struct.DefaultOpaqueRendererMethod.html "struct bevy::pbr::DefaultOpaqueRendererMethod")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#232)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DefaultSpatialScale](../audio/struct.DefaultSpatialScale.html "struct bevy::audio::DefaultSpatialScale")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#66)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DenoiseCas](../anti_alias/contrast_adaptive_sharpening/struct.DenoiseCas.html "struct bevy::anti_alias::contrast_adaptive_sharpening::DenoiseCas")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#88)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Dir2](struct.Dir2.html "struct bevy::prelude::Dir2")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#399)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#1053)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Dir4](../math/struct.Dir4.html "struct bevy::math::Dir4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/direction.rs.html#803)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Dir3A](struct.Dir3A.html "struct bevy::prelude::Dir3A")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#354)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DirectlyHovered](../picking/hover/struct.DirectlyHovered.html "struct bevy::picking::hover::DirectlyHovered")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/tuple_struct.rs.html#268)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DynamicTupleStruct](../reflect/tuple_struct/struct.DynamicTupleStruct.html "struct bevy::reflect::tuple_struct::DynamicTupleStruct")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#36)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DynamicWorldRoot](struct.DynamicWorldRoot.html "struct bevy::prelude::DynamicWorldRoot")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_set.rs.html#22)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EntityHashSet](../ecs/entity/struct.EntityHashSet.html "struct bevy::ecs::entity::EntityHashSet")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_set.rs.html#29)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EntityIndexSet](../ecs/entity/struct.EntityIndexSet.html "struct bevy::ecs::entity::EntityIndexSet")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/float_ord.rs.html#22)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [FloatOrd](../math/struct.FloatOrd.html "struct bevy::math::FloatOrd")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#728)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [FontFeatureTag](../text/struct.FontFeatureTag.html "struct bevy::text::FontFeatureTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#913)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [FontVariationTag](../text/struct.FontVariationTag.html "struct bevy::text::FontVariationTag")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#596)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [FontWeight](struct.FontWeight.html "struct bevy::prelude::FontWeight")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#659)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [FontWidth](struct.FontWidth.html "struct bevy::prelude::FontWidth")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#247)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Frustum](../camera/primitives/struct.Frustum.html "struct bevy::camera::primitives::Frustum")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/global_transform.rs.html#53)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [GlobalTransform](struct.GlobalTransform.html "struct bevy::prelude::GlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2448)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [GlobalZIndex](struct.GlobalZIndex.html "struct bevy::prelude::GlobalZIndex")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#344)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [GltfMaterialName](../gltf/struct.GltfMaterialName.html "struct bevy::gltf::GltfMaterialName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#319)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [GltfMeshName](../gltf/struct.GltfMeshName.html "struct bevy::gltf::GltfMeshName")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#294)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [GltfSceneName](../gltf/struct.GltfSceneName.html "struct bevy::gltf::GltfSceneName")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#60)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [HashedStr](../ecs/name/struct.HashedStr.html "struct bevy::ecs::name::HashedStr")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/hover.rs.html#336)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Hovered](../picking/hover/struct.Hovered.html "struct bevy::picking::hover::Hovered")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#436)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [IgnoreScroll](struct.IgnoreScroll.html "struct bevy::prelude::IgnoreScroll")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#106)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [InheritableThemeTextColor](../feathers/theme/struct.InheritableThemeTextColor.html "struct bevy::feathers::theme::InheritableThemeTextColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#162)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [InheritedVisibility](struct.InheritedVisibility.html "struct bevy::prelude::InheritedVisibility")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#173)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [InputFocusVisible](../input_focus/struct.InputFocusVisible.html "struct bevy::input_focus::InputFocusVisible")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#50)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [InstanceId](../world_serialization/struct.InstanceId.html "struct bevy::world_serialization::InstanceId")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/resource.rs.html#121)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [IsResource](../ecs/resource/struct.IsResource.html "struct bevy::ecs::resource::IsResource")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#336)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [JointIndex](../mesh/skinning/struct.JointIndex.html "struct bevy::mesh::skinning::JointIndex")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#158)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MainEntity](../render/sync_world/struct.MainEntity.html "struct bevy::render::sync_world::MainEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#142)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MainPassResolutionOverride](../camera/struct.MainPassResolutionOverride.html "struct bevy::camera::MainPassResolutionOverride")

[Source](https://docs.rs/bevy_a11y/0.19.0/x86_64-unknown-linux-gnu/src/bevy_a11y/lib.rs.html#156)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ManageAccessibilityUpdates](../a11y/struct.ManageAccessibilityUpdates.html "struct bevy::a11y::ManageAccessibilityUpdates")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#976)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ManualTextureViewHandle](../camera/struct.ManualTextureViewHandle.html "struct bevy::camera::ManualTextureViewHandle")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#276)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MaterialBindGroupIndex](../pbr/struct.MaterialBindGroupIndex.html "struct bevy::pbr::MaterialBindGroupIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#294)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MaterialBindGroupSlot](../pbr/struct.MaterialBindGroupSlot.html "struct bevy::pbr::MaterialBindGroupSlot")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#41)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Mesh2d](struct.Mesh2d.html "struct bevy::prelude::Mesh2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#445)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Mesh2dWireframe](../sprite_render/struct.Mesh2dWireframe.html "struct bevy::sprite_render::Mesh2dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#98)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Mesh3d](struct.Mesh3d.html "struct bevy::prelude::Mesh3d")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#935)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Mesh3dWireframe](../pbr/wireframe/struct.Mesh3dWireframe.html "struct bevy::pbr::wireframe::Mesh3dWireframe")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/components.rs.html#154)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MeshTag](../mesh/struct.MeshTag.html "struct bevy::mesh::MeshTag")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/meshlet/mod.rs.html#230)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MeshletMesh3d](../pbr/experimental/meshlet/struct.MeshletMesh3d.html "struct bevy::pbr::experimental::meshlet::MeshletMesh3d")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#805)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MipBias](../render/camera/struct.MipBias.html "struct bevy::render::camera::MipBias")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/name.rs.html#43)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Name](struct.Name.html "struct bevy::prelude::Name")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#105)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [NormalizedWindowRef](../window/struct.NormalizedWindowRef.html "struct bevy::window::NormalizedWindowRef")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/observer/distributed_storage.rs.html#501)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ObservedBy](../ecs/observer/struct.ObservedBy.html "struct bevy::ecs::observer::ObservedBy")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2456)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [OuterColor](struct.OuterColor.html "struct bevy::prelude::OuterColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/cursor.rs.html#47)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [OverrideCursor](../feathers/cursor/struct.OverrideCursor.html "struct bevy::feathers::cursor::OverrideCursor")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#84)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [PanGesture](../input/gestures/struct.PanGesture.html "struct bevy::input::gestures::PanGesture")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#367)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ParsedPath](../reflect/struct.ParsedPath.html "struct bevy::reflect::ParsedPath")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#25)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [PinchGesture](../input/gestures/struct.PinchGesture.html "struct bevy::input::gestures::PinchGesture")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/scene/types.rs.html#19)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [RaytracingMesh3d](../solari/scene/struct.RaytracingMesh3d.html "struct bevy::solari::scene::RaytracingMesh3d")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#399)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [RemovedComponentEntity](../ecs/lifecycle/struct.RemovedComponentEntity.html "struct bevy::ecs::lifecycle::RemovedComponentEntity")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#129)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [RenderEntity](../render/sync_world/struct.RenderEntity.html "struct bevy::render::sync_world::RenderEntity")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/render_layers.rs.html#18)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [RenderLayers](../camera/visibility/struct.RenderLayers.html "struct bevy::camera::visibility::RenderLayers")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/schedule.rs.html#120)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [RootNonCameraView](../core_pipeline/schedule/struct.RootNonCameraView.html "struct bevy::core_pipeline::schedule::RootNonCameraView")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#47)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [RotationGesture](../input/gestures/struct.RotationGesture.html "struct bevy::input::gestures::RotationGesture")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#78)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Screenshot](../render/view/window/screenshot/struct.Screenshot.html "struct bevy::render::view::window::screenshot::Screenshot")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#417)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ScrollPosition](struct.ScrollPosition.html "struct bevy::prelude::ScrollPosition")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#113)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SimplifiedMesh](../picking/mesh_picking/ray_cast/struct.SimplifiedMesh.html "struct bevy::picking::mesh_picking::ray_cast::SimplifiedMesh")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#147)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SliderBaseColor](../feathers/controls/struct.SliderBaseColor.html "struct bevy::feathers::controls::SliderBaseColor")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#233)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SliderPrecision](../ui_widgets/struct.SliderPrecision.html "struct bevy::ui_widgets::SliderPrecision")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#214)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SliderStep](../ui_widgets/struct.SliderStep.html "struct bevy::ui_widgets::SliderStep")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#120)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SliderValue](../ui_widgets/struct.SliderValue.html "struct bevy::ui_widgets::SliderValue")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#203)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SpatialScale](../audio/struct.SpatialScale.html "struct bevy::audio::SpatialScale")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1137)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [StrikethroughColor](struct.StrikethroughColor.html "struct bevy::prelude::StrikethroughColor")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#60)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TabIndex](../input_focus/tab_navigation/struct.TabIndex.html "struct bevy::input_focus::tab_navigation::TabIndex")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#97)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Text](struct.Text.html "struct bevy::prelude::Text")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#85)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Text2d](struct.Text2d.html "struct bevy::prelude::Text2d")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1088)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TextBackgroundColor](struct.TextBackgroundColor.html "struct bevy::prelude::TextBackgroundColor")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1064)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TextColor](struct.TextColor.html "struct bevy::prelude::TextColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text_input_layout.rs.html#32)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TextScroll](../ui/widget/struct.TextScroll.html "struct bevy::ui::widget::TextScroll")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#190)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TextSpan](struct.TextSpan.html "struct bevy::prelude::TextSpan")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#90)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ThemeBackgroundColor](../feathers/theme/struct.ThemeBackgroundColor.html "struct bevy::feathers::theme::ThemeBackgroundColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#99)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ThemeBorderColor](../feathers/theme/struct.ThemeBorderColor.html "struct bevy::feathers::theme::ThemeBorderColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#118)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ThemeTextColor](../feathers/theme/struct.ThemeTextColor.html "struct bevy::feathers::theme::ThemeTextColor")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#22)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ThemeToken](../feathers/theme/struct.ThemeToken.html "struct bevy::feathers::theme::ThemeToken")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#288)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ThreadedAnimationGraphs](struct.ThreadedAnimationGraphs.html "struct bevy::prelude::ThreadedAnimationGraphs")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#46)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TilemapChunkMeshCache](../sprite_render/struct.TilemapChunkMeshCache.html "struct bevy::sprite_render::TilemapChunkMeshCache")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#130)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [TilemapChunkTileData](../sprite_render/struct.TilemapChunkTileData.html "struct bevy::sprite_render::TilemapChunkTileData")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#199)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [UiGlobalTransform](struct.UiGlobalTransform.html "struct bevy::prelude::UiGlobalTransform")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/lib.rs.html#124)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [UiScale](struct.UiScale.html "struct bevy::prelude::UiScale")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2936)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [UiTargetCamera](struct.UiTargetCamera.html "struct bevy::prelude::UiTargetCamera")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#59)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [UiTheme](../feathers/theme/struct.UiTheme.html "struct bevy::feathers::theme::UiTheme")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1159)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [UnderlineColor](struct.UnderlineColor.html "struct bevy::prelude::UnderlineColor")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#224)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ViewVisibility](struct.ViewVisibility.html "struct bevy::prelude::ViewVisibility")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#208)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [VisibilityClass](../camera/visibility/struct.VisibilityClass.html "struct bevy::camera::visibility::VisibilityClass")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#64)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [WeightsCurveSample](struct.WeightsCurveSample.html "struct bevy::prelude::WeightsCurveSample")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/components.rs.html#18)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [WorldAssetRoot](struct.WorldAssetRoot.html "struct bevy::prelude::WorldAssetRoot")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2438)

### impl [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [ZIndex](struct.ZIndex.html "struct bevy::prelude::ZIndex")

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#96)

### impl<C> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Inherited](../app/struct.Inherited.html "struct bevy::app::Inherited")<C>

where C: [Component](trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Inherited](../app/struct.Inherited.html "struct bevy::app::Inherited")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#70)

### impl<C> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Propagate](../app/struct.Propagate.html "struct bevy::app::Propagate")<C>

where C: [Component](trait.Component.html "trait bevy::prelude::Component") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Propagate](../app/struct.Propagate.html "struct bevy::app::Propagate")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#78)

### impl<C> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [PropagateOver](../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>

where [PropagateOver](../app/struct.PropagateOver.html "struct bevy::app::PropagateOver")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> C>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#83)

### impl<C> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [PropagateStop](../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>

where [PropagateStop](../app/struct.PropagateStop.html "struct bevy::app::PropagateStop")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> C>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#152)

### impl<C> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SampleDerivativeWrapper](derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>

where [SampleDerivativeWrapper](derivatives/struct.SampleDerivativeWrapper.html "struct bevy::prelude::derivatives::SampleDerivativeWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/derivatives/mod.rs.html#185)

### impl<C> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [SampleTwoDerivativesWrapper](derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>

where [SampleTwoDerivativesWrapper](derivatives/struct.SampleTwoDerivativesWrapper.html "struct bevy::prelude::derivatives::SampleTwoDerivativesWrapper")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/morph.rs.html#19)

### impl<C> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [WeightsCurve](struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>

where [WeightsCurve](struct.WeightsCurve.html "struct bevy::prelude::WeightsCurve")<C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/ui_material.rs.html#172)

### impl<M> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MaterialNode](struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>

where M: [UiMaterial](trait.UiMaterial.html "trait bevy::prelude::UiMaterial") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [MaterialNode](struct.MaterialNode.html "struct bevy::prelude::MaterialNode")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/material.rs.html#202)

### impl<M> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MeshMaterial2d](struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>

where M: [Material2d](../sprite_render/trait.Material2d.html "trait bevy::sprite_render::Material2d") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial2d](struct.MeshMaterial2d.html "struct bevy::prelude::MeshMaterial2d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/mesh_material.rs.html#39)

### impl<M> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [MeshMaterial3d](struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>

where M: [Material](trait.Material.html "trait bevy::prelude::Material") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [MeshMaterial3d](struct.MeshMaterial3d.html "struct bevy::prelude::MeshMaterial3d")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Handle](enum.Handle.html "enum bevy::prelude::Handle")<M>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#229)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DespawnOnEnter](struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DespawnOnEnter](struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#148)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DespawnOnExit](struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DespawnOnExit](struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#476)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DisableOnEnter](struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DisableOnEnter](struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#395)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [DisableOnExit](struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [DisableOnExit](struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#723)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EnableOnEnter](struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [EnableOnEnter](struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#642)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EnableOnExit](struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [EnableOnExit](struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#131)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [PreviousState](struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [PreviousState](struct.PreviousState.html "struct bevy::prelude::PreviousState")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/resources.rs.html#55)

### impl<S> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [State](struct.State.html "struct bevy::prelude::State")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [State](struct.State.html "struct bevy::prelude::State")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#248)

### impl<Source> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [AudioPlayer](struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>

where [AudioPlayer](struct.AudioPlayer.html "struct bevy::prelude::AudioPlayer")<Source>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Source: [Asset](trait.Asset.html "trait bevy::prelude::Asset") + [Decodable](trait.Decodable.html "trait bevy::prelude::Decodable") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [Handle](enum.Handle.html "enum bevy::prelude::Handle")<Source>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/intern.rs.html#45)

### impl<T> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>

where T: [Internable](../ecs/intern/trait.Internable.html "trait bevy::ecs::intern::Internable") + 'static + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), [Interned](../ecs/intern/struct.Interned.html "struct bevy::ecs::intern::Interned")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [&'static T](https://doc.rust-lang.org/nightly/std/primitive.reference.html): [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#22)

### impl<T> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [VirtualKeyboard](../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>

where T: [AsRef](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html "trait core::convert::AsRef")<[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\> + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [VirtualKeyboard](../feathers/controls/struct.VirtualKeyboard.html "struct bevy::feathers::controls::VirtualKeyboard")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [PhantomData](https://doc.rust-lang.org/nightly/core/marker/struct.PhantomData.html "struct core::marker::PhantomData")<[fn](https://doc.rust-lang.org/nightly/std/primitive.fn.html)() -> T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#159)

### impl<V, W> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [Sum](../math/struct.Sum.html "struct bevy::math::Sum")<V, W>

where [Sum](../math/struct.Sum.html "struct bevy::math::Sum")<V, W>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, W: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash_map.rs.html#19)

### impl<V> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EntityHashMap](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>

where [EntityHashMap](../ecs/entity/struct.EntityHashMap.html "struct bevy::ecs::entity::EntityHashMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Entity](struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/index_map.rs.html#32)

### impl<V> [TupleStruct](trait.TupleStruct.html "trait bevy::prelude::TupleStruct") for [EntityIndexMap](../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>

where [EntityIndexMap](../ecs/entity/struct.EntityIndexMap.html "struct bevy::ecs::entity::EntityIndexMap")<V>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), V: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [IndexMap](https://docs.rs/indexmap/2.14.0/x86_64-unknown-linux-gnu/indexmap/map/struct.IndexMap.html "struct indexmap::map::IndexMap")<[Entity](struct.Entity.html "struct bevy::prelude::Entity"), V, [EntityHash](../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

{"TupleStructFieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/tuple\_struct/struct.TupleStructFieldIter.html\\" title=\\"struct bevy::reflect::tuple\_struct::TupleStructFieldIter\\">TupleStructFieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &amp;'a (dyn <a class=\\"trait\\" href=\\"trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static);</div>"}
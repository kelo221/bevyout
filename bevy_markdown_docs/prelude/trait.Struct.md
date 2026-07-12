[bevy](../index.html)::[prelude](index.html)

# Trait Struct 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#51)

```rust
pub trait Struct: PartialReflect {
    // Required methods
    fn field(&self, name: &str) -> Option<&(dyn PartialReflect + 'static)>;
    fn field_mut(
        &mut self,
        name: &str,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn field_at(&self, index: usize) -> Option<&(dyn PartialReflect + 'static)>;
    fn field_at_mut(
        &mut self,
        index: usize,
    ) -> Option<&mut (dyn PartialReflect + 'static)>;
    fn name_at(&self, index: usize) -> Option<&str>;
    fn index_of_name(&self, name: &str) -> Option<usize>;
    fn field_len(&self) -> usize;
    fn iter_fields(&self) -> FieldIter<'_> ⓘ;

    // Provided methods
    fn to_dynamic_struct(&self) -> DynamicStruct { ... }
    fn get_represented_struct_info(&self) -> Option<&'static StructInfo> { ... }
}
```

A trait used to power [struct-like](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) operations via [reflection](../reflect/index.html "mod bevy::reflect").

This trait uses the [`Reflect`](trait.Reflect.html "trait bevy::prelude::Reflect") trait to allow implementors to have their fields be dynamically addressed by both name and index.

When using [`#[derive(Reflect)]`](derive.Reflect.html "derive bevy::prelude::Reflect") on a standard struct, this trait will be automatically implemented. This goes for [unit structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#unit-like-structs-without-any-fields) as well.

## Example

```rust
use bevy_reflect::{PartialReflect, Reflect, structs::Struct};

#[derive(Reflect)]
struct Foo {
    bar: u32,
}

let foo = Foo { bar: 123 };

assert_eq!(foo.field_len(), 1);
assert_eq!(foo.name_at(0), Some("bar"));

let field: &dyn PartialReflect = foo.field("bar").unwrap();
assert_eq!(field.try_downcast_ref::<u32>(), Some(&123));
```

## Required Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#54)

#### fn [field](#tymethod.field)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field named `name` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#58)

#### fn [field\_mut](#tymethod.field_mut)( &mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field named `name` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#62)

#### fn [field\_at](#tymethod.field_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&(dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a reference to the value of the field with index `index` as a `&dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#66)

#### fn [field\_at\_mut](#tymethod.field_at_mut)( &mut self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html), ) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&mut (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static)>

Gets a mutable reference to the value of the field with index `index` as a `&mut dyn PartialReflect`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#69)

#### fn [name\_at](#tymethod.name_at)(&self, index: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)\>

Gets the name of the field with index `index`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#72)

#### fn [index\_of\_name](#tymethod.index_of_name)(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\>

Gets the index of the field with the given name.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#75)

#### fn [field\_len](#tymethod.field_len)(&self) -> [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

Returns the number of fields in the struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#78)

#### fn [iter\_fields](#tymethod.iter_fields)(&self) -> [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'\_> [ⓘ](#)

Returns an iterator over the values of the reflectable fields for this struct.

## Provided Methods

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#81)

#### fn [to\_dynamic\_struct](#method.to_dynamic_struct)(&self) -> [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

Creates a new [`DynamicStruct`](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct") from this struct.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#91)

#### fn [get\_represented\_struct\_info](#method.get_represented_struct_info)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'static [StructInfo](../reflect/structs/struct.StructInfo.html "struct bevy::reflect::structs::StructInfo")\>

Will return `None` if [`TypeInfo`](../reflect/enum.TypeInfo.html "enum bevy::reflect::TypeInfo") is not available.

## Trait Implementations

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#284)

### impl [GetField](trait.GetField.html "trait bevy::prelude::GetField") for dyn [Struct](trait.Struct.html "trait bevy::prelude::Struct")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#285)

#### fn [get\_field](trait.GetField.html#tymethod.get_field)<T>(&self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#290)

#### fn [get\_field\_mut](trait.GetField.html#tymethod.get_field_mut)<T>(&mut self, name: &[str](https://doc.rust-lang.org/nightly/std/primitive.str.html)) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[&mut T](https://doc.rust-lang.org/nightly/std/primitive.reference.html)\>

where T: [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect"),

Gets a mutable reference to the value of the field named `name`, downcast to `T`.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#96)

### impl<'a> [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator") for &'a (dyn [Struct](trait.Struct.html "trait bevy::prelude::Struct") + 'static)

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#97)

#### type [Item](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.Item) = (&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html), &'a (dyn [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + 'static))

The type of the elements being iterated over.

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#98)

#### type [IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter) = [FieldIter](../reflect/structs/struct.FieldIter.html "struct bevy::reflect::structs::FieldIter")<'a>

Which kind of iterator are we turning this into?

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#100)

#### fn [into\_iter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)(self) -> <&'a (dyn [Struct](trait.Struct.html "trait bevy::prelude::Struct") + 'static) as [IntoIterator](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html "trait core::iter::traits::collect::IntoIterator")\>::[IntoIter](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#associatedtype.IntoIter "type core::iter::traits::collect::IntoIterator::IntoIter")

Creates an iterator from a value. [Read more](https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html#tymethod.into_iter)

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#62)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Aabb](../camera/primitives/struct.Aabb.html "struct bevy::camera::primitives::Aabb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#42)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Aabb2d](../math/bounding/struct.Aabb2d.html "struct bevy::math::bounding::Aabb2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#48)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Aabb3d](../math/bounding/struct.Aabb3d.html "struct bevy::math::bounding::Aabb3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#112)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AabbCast2d](../math/bounding/struct.AabbCast2d.html "struct bevy::math::bounding::AabbCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#109)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AabbCast3d](../math/bounding/struct.AabbCast3d.html "struct bevy::math::bounding::AabbCast3d")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#43)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AabbGizmoConfigGroup](struct.AabbGizmoConfigGroup.html "struct bevy::prelude::AabbGizmoConfigGroup")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#210)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AccumulatedMouseMotion](../input/mouse/struct.AccumulatedMouseMotion.html "struct bevy::input::mouse::AccumulatedMouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#231)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AccumulatedMouseScroll](../input/mouse/struct.AccumulatedMouseScroll.html "struct bevy::input::mouse::AccumulatedMouseScroll")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#208)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AcquireFocus](../input_focus/struct.AcquireFocus.html "struct bevy::input_focus::AcquireFocus")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#80)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Activate](../ui_widgets/struct.Activate.html "struct bevy::ui_widgets::Activate")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#33)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ActivateOnPress](../ui_widgets/struct.ActivateOnPress.html "struct bevy::ui_widgets::ActivateOnPress")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#507)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ActiveAnimation](../animation/struct.ActiveAnimation.html "struct bevy::animation::ActiveAnimation")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#333)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Add](struct.Add.html "struct bevy::prelude::Add")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#408-415)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Affine2](../math/struct.Affine2.html "struct bevy::math::Affine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#416-423)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Affine3](../math/struct.Affine3.html "struct bevy::math::Affine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#424-431)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Affine3A](../math/struct.Affine3A.html "struct bevy::math::Affine3A")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#9)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AmbientLight](struct.AmbientLight.html "struct bevy::prelude::AmbientLight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#113)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AngularColorStop](struct.AngularColorStop.html "struct bevy::prelude::AngularColorStop")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#103)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimationClip](struct.AnimationClip.html "struct bevy::prelude::AnimationClip")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#112)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimationGraph](struct.AnimationGraph.html "struct bevy::prelude::AnimationGraph")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#169)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimationGraphNode](struct.AnimationGraphNode.html "struct bevy::prelude::AnimationGraphNode")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/lib.rs.html#730)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimationPlayer](struct.AnimationPlayer.html "struct bevy::prelude::AnimationPlayer")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#54)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimationTransition](struct.AnimationTransition.html "struct bevy::prelude::AnimationTransition")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/transition.rs.html#31)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimationTransitions](struct.AnimationTransitions.html "struct bevy::prelude::AnimationTransitions")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#955)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Annulus](struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#745)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnnulusMeshBuilder](../mesh/struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#117)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Arc2d](struct.Arc2d.html "struct bevy::prelude::Arc2d")

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/assets.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AssetIndex](../asset/struct.AssetIndex.html "struct bevy::asset::AssetIndex")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#286)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AtmosphereSettings](../pbr/struct.AtmosphereSettings.html "struct bevy::pbr::AtmosphereSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/auto_directional_navigation.rs.html#105)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AutoDirectionalNavigation](../ui/auto_directional_navigation/struct.AutoDirectionalNavigation.html "struct bevy::ui::auto_directional_navigation::AutoDirectionalNavigation")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/settings.rs.html#27)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AutoExposure](../post_process/auto_exposure/struct.AutoExposure.html "struct bevy::post_process::auto_exposure::AutoExposure")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/auto_exposure/compensation_curve.rs.html#20)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AutoExposureCompensationCurve](../post_process/auto_exposure/struct.AutoExposureCompensationCurve.html "struct bevy::post_process::auto_exposure::AutoExposureCompensationCurve")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/autofocus.rs.html#20)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AutoFocus](../input_focus/struct.AutoFocus.html "struct bevy::input_focus::AutoFocus")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#90)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AutoNavigationConfig](../input_focus/directional_navigation/struct.AutoNavigationConfig.html "struct bevy::input_focus::directional_navigation::AutoNavigationConfig")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#984)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AxisSettings](../input/gamepad/struct.AxisSettings.html "struct bevy::input::gamepad::AxisSettings")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#287-294)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BVec2](struct.BVec2.html "struct bevy::prelude::BVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#295-303)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BVec3](struct.BVec3.html "struct bevy::prelude::BVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#304-313)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BVec4](struct.BVec4.html "struct bevy::prelude::BVec4")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#30)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Bloom](../post_process/bloom/struct.Bloom.html "struct bevy::post_process::bloom::Bloom")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/bloom/settings.rs.html#199)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BloomPrefilter](../post_process/bloom/struct.BloomPrefilter.html "struct bevy::post_process::bloom::BloomPrefilter")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2249)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BorderColor](struct.BorderColor.html "struct bevy::prelude::BorderColor")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2519)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BorderRadius](struct.BorderRadius.html "struct bevy::prelude::BorderRadius")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/border_rect.rs.html#8)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BorderRect](struct.BorderRect.html "struct bevy::prelude::BorderRect")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded2d/mod.rs.html#478)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BoundingCircle](../math/bounding/struct.BoundingCircle.html "struct bevy::math::bounding::BoundingCircle")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#150)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BoundingCircleCast](../math/bounding/struct.BoundingCircleCast.html "struct bevy::math::bounding::BoundingCircleCast")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/bounded3d/mod.rs.html#504)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BoundingSphere](../math/bounding/struct.BoundingSphere.html "struct bevy::math::bounding::BoundingSphere")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#154)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [BoundingSphereCast](../math/bounding/struct.BoundingSphereCast.html "struct bevy::math::bounding::BoundingSphereCast")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/button.rs.html#6)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for bevy::prelude::[Button](struct.Button.html "struct bevy::prelude::Button")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/button.rs.html#27)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for bevy::ui\_widgets::[Button](../ui_widgets/struct.Button.html "struct bevy::ui_widgets::Button")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1412)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ButtonAxisSettings](../input/gamepad/struct.ButtonAxisSettings.html "struct bevy::input::gamepad::ButtonAxisSettings")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#820)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ButtonSettings](../input/gamepad/struct.ButtonSettings.html "struct bevy::input::gamepad::ButtonSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2407)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CalculatedClip](struct.CalculatedClip.html "struct bevy::prelude::CalculatedClip")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#374)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Camera](struct.Camera.html "struct bevy::prelude::Camera")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#9)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Camera2d](struct.Camera2d.html "struct bevy::prelude::Camera2d")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#22)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Camera3d](struct.Camera3d.html "struct bevy::prelude::Camera3d")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#178)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Cancel](struct.Cancel.html "struct bevy::prelude::Cancel")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2183)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Capsule2d](struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1121)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Capsule2dMeshBuilder](../mesh/struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#856)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Capsule3d](struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Capsule3dMeshBuilder](../mesh/struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#179)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Cascade](../light/cascade/struct.Cascade.html "struct bevy::light::cascade::Cascade")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#24)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CascadeShadowConfig](../light/struct.CascadeShadowConfig.html "struct bevy::light::CascadeShadowConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cascade.rs.html#167)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Cascades](../light/struct.Cascades.html "struct bevy::light::Cascades")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#443)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CascadesFrusta](../camera/primitives/struct.CascadesFrusta.html "struct bevy::camera::primitives::CascadesFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#460)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CascadesVisibleEntities](../camera/visibility/struct.CascadesVisibleEntities.html "struct bevy::camera::visibility::CascadesVisibleEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#49)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Checkable](../ui/struct.Checkable.html "struct bevy::ui::Checkable")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#36)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Checkbox](../ui_widgets/struct.Checkbox.html "struct bevy::ui_widgets::Checkbox")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#54)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Checked](../ui/struct.Checked.html "struct bevy::ui::Checked")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/chromatic_aberration.rs.html#43)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ChromaticAberration](../post_process/effect_stack/struct.ChromaticAberration.html "struct bevy::post_process::effect_stack::ChromaticAberration")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#29)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Circle](struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#22)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CircleMeshBuilder](../mesh/struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#285)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CircularSector](struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#128)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CircularSectorMeshBuilder](../mesh/struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#437)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CircularSegment](struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#266)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CircularSegmentMeshBuilder](../mesh/struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#309)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Click](struct.Click.html "struct bevy::prelude::Click")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#95)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ClusterZConfig](../light/cluster/struct.ClusterZConfig.html "struct bevy::light::cluster::ClusterZConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/cluster/mod.rs.html#229)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ClusteredDecal](../light/struct.ClusteredDecal.html "struct bevy::light::ClusteredDecal")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#399)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorGrading](../render/view/struct.ColorGrading.html "struct bevy::render::view::ColorGrading")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#428)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorGradingGlobal](../render/view/struct.ColorGradingGlobal.html "struct bevy::render::view::ColorGradingGlobal")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/mod.rs.html#494)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorGradingSection](../render/view/struct.ColorGradingSection.html "struct bevy::render::view::ColorGradingSection")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/color_material.rs.html#36)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorMaterial](struct.ColorMaterial.html "struct bevy::prelude::ColorMaterial")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#187)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorSlider](../feathers/controls/struct.ColorSlider.html "struct bevy::feathers::controls::ColorSlider")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#10)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorStop](struct.ColorStop.html "struct bevy::prelude::ColorStop")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#40)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorSwatchFg](../feathers/controls/struct.ColorSwatchFg.html "struct bevy::feathers::controls::ColorSwatchFg")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#136)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ComponentTicks](../ecs/change_detection/struct.ComponentTicks.html "struct bevy::ecs::change_detection::ComponentTicks")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#217)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ComputedCameraValues](../camera/struct.ComputedCameraValues.html "struct bevy::camera::ComputedCameraValues")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#26)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ComputedNode](struct.ComputedNode.html "struct bevy::prelude::ComputedNode")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#37)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ComputedTextBlock](../text/struct.ComputedTextBlock.html "struct bevy::text::ComputedTextBlock")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3036)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ComputedUiRenderTargetInfo](struct.ComputedUiRenderTargetInfo.html "struct bevy::prelude::ComputedUiRenderTargetInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#3014)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ComputedUiTargetCamera](struct.ComputedUiTargetCamera.html "struct bevy::prelude::ComputedUiTargetCamera")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#927)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Cone](struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#20)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ConeMeshBuilder](../mesh/struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#410)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ConicGradient](struct.ConicGradient.html "struct bevy::prelude::ConicGradient")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1010)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ConicalFrustum](struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#7)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ConicalFrustumMeshBuilder](../mesh/struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/contact_shadows.rs.html#34)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ContactShadows](../pbr/struct.ContactShadows.html "struct bevy::pbr::ContactShadows")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/measurement.rs.html#139)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ContentSize](../ui/struct.ContentSize.html "struct bevy::ui::ContentSize")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/contrast_adaptive_sharpening/mod.rs.html#37)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ContrastAdaptiveSharpening](../anti_alias/contrast_adaptive_sharpening/struct.ContrastAdaptiveSharpening.html "struct bevy::anti_alias::contrast_adaptive_sharpening::ContrastAdaptiveSharpening")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1950)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ConvexPolygon](struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#413)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ConvexPolygonMeshBuilder](../mesh/struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#392)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubemapFrusta](../camera/primitives/struct.CubemapFrusta.html "struct bevy::camera::primitives::CubemapFrusta")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#435)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubemapVisibleEntities](../camera/visibility/struct.CubemapVisibleEntities.html "struct bevy::camera::visibility::CubemapVisibleEntities")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#113)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicRotationCurve](../animation/gltf_curves/struct.CubicRotationCurve.html "struct bevy::animation::gltf_curves::CubicRotationCurve")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#684)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Cuboid](struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#7)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CuboidMeshBuilder](../mesh/struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#209)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CursorEntered](struct.CursorEntered.html "struct bevy::prelude::CursorEntered")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#226)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CursorLeft](struct.CursorLeft.html "struct bevy::prelude::CursorLeft")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#184)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CursorMoved](struct.CursorMoved.html "struct bevy::prelude::CursorMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#744)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CursorOptions](../window/struct.CursorOptions.html "struct bevy::window::CursorOptions")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#15)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CustomCursorImage](../window/struct.CustomCursorImage.html "struct bevy::window::CustomCursorImage")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/cursor/custom_cursor.rs.html#55)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CustomCursorUrl](../window/struct.CustomCursorUrl.html "struct bevy::window::CustomCursorUrl")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#109)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CustomProjection](../camera/struct.CustomProjection.html "struct bevy::camera::CustomProjection")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#777)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Cylinder](struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#20)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CylinderMeshBuilder](../mesh/struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#433-440)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DAffine2](../math/struct.DAffine2.html "struct bevy::math::DAffine2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#441-448)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DAffine3](../math/struct.DAffine3.html "struct bevy::math::DAffine3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#380-387)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DMat2](../math/struct.DMat2.html "struct bevy::math::DMat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#388-396)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DMat3](../math/struct.DMat3.html "struct bevy::math::DMat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#397-406)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DMat4](../math/struct.DMat4.html "struct bevy::math::DMat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#460-469)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DQuat](../math/struct.DQuat.html "struct bevy::math::DQuat")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#315-322)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DVec2](../math/struct.DVec2.html "struct bevy::math::DVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#323-331)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DVec3](../math/struct.DVec3.html "struct bevy::math::DVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#332-341)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DVec4](../math/struct.DVec4.html "struct bevy::math::DVec4")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#84)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DefaultGizmoConfigGroup](struct.DefaultGizmoConfigGroup.html "struct bevy::prelude::DefaultGizmoConfigGroup")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#172)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DefaultQueryFilters](../ecs/entity_disabling/struct.DefaultQueryFilters.html "struct bevy::ecs::entity_disabling::DefaultQueryFilters")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#82)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DeferredPrepass](../core_pipeline/prepass/struct.DeferredPrepass.html "struct bevy::core_pipeline::prepass::DeferredPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#93)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DeferredPrepassDoubleBuffer](../core_pipeline/prepass/struct.DeferredPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#133)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DelayedCommandQueue](../time/struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/dof/mod.rs.html#75)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DepthOfField](../post_process/dof/struct.DepthOfField.html "struct bevy::post_process::dof::DepthOfField")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#62)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DepthPrepass](../core_pipeline/prepass/struct.DepthPrepass.html "struct bevy::core_pipeline::prepass::DepthPrepass")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#87)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DepthPrepassDoubleBuffer](../core_pipeline/prepass/struct.DepthPrepassDoubleBuffer.html "struct bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#388)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Despawn](struct.Despawn.html "struct bevy::prelude::Despawn")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#61)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DirectionalLight](struct.DirectionalLight.html "struct bevy::prelude::DirectionalLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#191)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DirectionalLightShadowMap](../light/struct.DirectionalLightShadowMap.html "struct bevy::light::DirectionalLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/directional_light.rs.html#173)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DirectionalLightTexture](../light/struct.DirectionalLightTexture.html "struct bevy::light::DirectionalLightTexture")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#251)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DirectionalNavigationMap](../input_focus/directional_navigation/struct.DirectionalNavigationMap.html "struct bevy::input_focus::directional_navigation::DirectionalNavigationMap")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity_disabling.rs.html#131)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Disabled](../ecs/entity_disabling/struct.Disabled.html "struct bevy::ecs::entity_disabling::Disabled")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#361)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Discard](struct.Discard.html "struct bevy::prelude::Discard")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/fog.rs.html#51)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DistanceFog](struct.DistanceFog.html "struct bevy::prelude::DistanceFog")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gestures.rs.html#66)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DoubleTapGesture](../input/gestures/struct.DoubleTapGesture.html "struct bevy::input::gestures::DoubleTapGesture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#348)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Drag](struct.Drag.html "struct bevy::prelude::Drag")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#421)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DragDrop](struct.DragDrop.html "struct bevy::prelude::DragDrop")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#370)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DragEnd](struct.DragEnd.html "struct bevy::prelude::DragEnd")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#385)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DragEnter](struct.DragEnter.html "struct bevy::prelude::DragEnter")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#433)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DragEntry](struct.DragEntry.html "struct bevy::prelude::DragEntry")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#409)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DragLeave](struct.DragLeave.html "struct bevy::prelude::DragLeave")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#397)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DragOver](struct.DragOver.html "struct bevy::prelude::DragOver")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#338)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DragStart](struct.DragStart.html "struct bevy::prelude::DragStart")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#329)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DynamicSkinnedMeshBounds](../camera/visibility/struct.DynamicSkinnedMeshBounds.html "struct bevy::camera::visibility::DynamicSkinnedMeshBounds")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/structs.rs.html#411)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DynamicStruct](../reflect/structs/struct.DynamicStruct.html "struct bevy::reflect::structs::DynamicStruct")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#804)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Ellipse](struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#556)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [EllipseMeshBuilder](../mesh/struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1432)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [EnabledButtons](../window/struct.EnabledButtons.html "struct bevy::window::EnabledButtons")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#223)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Enter](struct.Enter.html "struct bevy::prelude::Enter")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/hash.rs.html#8)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [EntityHash](../ecs/entity/struct.EntityHash.html "struct bevy::ecs::entity::EntityHash")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#105)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [EnvironmentMapLight](struct.EnvironmentMapLight.html "struct bevy::prelude::EnvironmentMapLight")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#90)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ErasedGizmoConfigGroup](../gizmos/config/struct.ErasedGizmoConfigGroup.html "struct bevy::gizmos::config::ErasedGizmoConfigGroup")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#59)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersButton](../feathers/controls/struct.FeathersButton.html "struct bevy::feathers::controls::FeathersButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/checkbox.rs.html#48)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersCheckbox](../feathers/controls/struct.FeathersCheckbox.html "struct bevy::feathers::controls::FeathersCheckbox")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_slider.rs.html#162)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersColorSlider](../feathers/controls/struct.FeathersColorSlider.html "struct bevy::feathers::controls::FeathersColorSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#27)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersColorSwatch](../feathers/controls/struct.FeathersColorSwatch.html "struct bevy::feathers::controls::FeathersColorSwatch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/disclosure_toggle.rs.html#33)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersDisclosureToggle](../feathers/controls/struct.FeathersDisclosureToggle.html "struct bevy::feathers::controls::FeathersDisclosureToggle")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#106)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersListRow](../feathers/controls/struct.FeathersListRow.html "struct bevy::feathers::controls::FeathersListRow")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/listview.rs.html#36)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersListView](../feathers/controls/struct.FeathersListView.html "struct bevy::feathers::controls::FeathersListView")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#48)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersMenu](../feathers/controls/struct.FeathersMenu.html "struct bevy::feathers::controls::FeathersMenu")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#141)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersMenuButton](../feathers/controls/struct.FeathersMenuButton.html "struct bevy::feathers::controls::FeathersMenuButton")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#439)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersMenuDivider](../feathers/controls/struct.FeathersMenuDivider.html "struct bevy::feathers::controls::FeathersMenuDivider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#250)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersMenuItem](../feathers/controls/struct.FeathersMenuItem.html "struct bevy::feathers::controls::FeathersMenuItem")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/menu.rs.html#195)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersMenuPopup](../feathers/controls/struct.FeathersMenuPopup.html "struct bevy::feathers::controls::FeathersMenuPopup")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#55)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersNumberInput](../feathers/controls/struct.FeathersNumberInput.html "struct bevy::feathers::controls::FeathersNumberInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/radio.rs.html#47)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersRadio](../feathers/controls/struct.FeathersRadio.html "struct bevy::feathers::controls::FeathersRadio")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/scrollbar.rs.html#22)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersScrollbar](../feathers/controls/struct.FeathersScrollbar.html "struct bevy::feathers::controls::FeathersScrollbar")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/slider.rs.html#50)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersSlider](../feathers/controls/struct.FeathersSlider.html "struct bevy::feathers::controls::FeathersSlider")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#85)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersTextInput](../feathers/controls/struct.FeathersTextInput.html "struct bevy::feathers::controls::FeathersTextInput")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/text_input.rs.html#38)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersTextInputContainer](../feathers/controls/struct.FeathersTextInputContainer.html "struct bevy::feathers::controls::FeathersTextInputContainer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/toggle_switch.rs.html#42)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersToggleSwitch](../feathers/controls/struct.FeathersToggleSwitch.html "struct bevy::feathers::controls::FeathersToggleSwitch")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/button.rs.html#126)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FeathersToolButton](../feathers/controls/struct.FeathersToolButton.html "struct bevy::feathers::controls::FeathersToolButton")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#68)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Fixed](struct.Fixed.html "struct bevy::prelude::Fixed")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#33)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FocusGained](../input_focus/struct.FocusGained.html "struct bevy::input_focus::FocusGained")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#23)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FocusIndicator](../feathers/focus/struct.FocusIndicator.html "struct bevy::feathers::focus::FocusIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/gained_and_lost.rs.html#50)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FocusLost](../input_focus/struct.FocusLost.html "struct bevy::input_focus::FocusLost")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/focus.rs.html#30)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FocusWithinIndicator](../feathers/focus/struct.FocusWithinIndicator.html "struct bevy::feathers::focus::FocusWithinIndicator")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#469)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FocusableArea](../input_focus/directional_navigation/struct.FocusableArea.html "struct bevy::input_focus::directional_navigation::FocusableArea")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#75)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FogVolume](../light/struct.FogVolume.html "struct bevy::light::FogVolume")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#839)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FontFeatures](../text/struct.FontFeatures.html "struct bevy::text::FontFeatures")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#960)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FontVariations](../text/struct.FontVariations.html "struct bevy::text::FontVariations")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/decal/forward.rs.html#62)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ForwardDecal](../pbr/decal/struct.ForwardDecal.html "struct bevy::pbr::decal::ForwardDecal")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#108)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FpsOverlayConfig](../dev_tools/fps_overlay/struct.FpsOverlayConfig.html "struct bevy::dev_tools::fps_overlay::FpsOverlayConfig")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/fps_overlay.rs.html#139)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FrameTimeGraphConfig](../dev_tools/fps_overlay/struct.FrameTimeGraphConfig.html "struct bevy::dev_tools::fps_overlay::FrameTimeGraphConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#78)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FrustumGizmoConfigGroup](struct.FrustumGizmoConfigGroup.html "struct bevy::prelude::FrustumGizmoConfigGroup")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/fxaa/mod.rs.html#53)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Fxaa](../anti_alias/fxaa/struct.Fxaa.html "struct bevy::anti_alias::fxaa::Fxaa")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#371)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Gamepad](struct.Gamepad.html "struct bevy::prelude::Gamepad")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#258)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GamepadAxisChangedEvent](../input/gamepad/struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#222)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GamepadButtonChangedEvent](../input/gamepad/struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#190)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GamepadButtonStateChangedEvent](../input/gamepad/struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#151)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GamepadConnectionEvent](../input/gamepad/struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#1688)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GamepadRumbleIntensity](../input/gamepad/struct.GamepadRumbleIntensity.html "struct bevy::input::gamepad::GamepadRumbleIntensity")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#736)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GamepadSettings](struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#261)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GeneratedEnvironmentMapLight](struct.GeneratedEnvironmentMapLight.html "struct bevy::prelude::GeneratedEnvironmentMapLight")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/experimental/ghost_hierarchy.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GhostNode](../ui/experimental/struct.GhostNode.html "struct bevy::ui::experimental::GhostNode")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/retained.rs.html#64)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Gizmo](struct.Gizmo.html "struct bevy::prelude::Gizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#206)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GizmoConfig](struct.GizmoConfig.html "struct bevy::prelude::GizmoConfig")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#97)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GizmoConfigStore](struct.GizmoConfigStore.html "struct bevy::prelude::GizmoConfigStore")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/config.rs.html#246)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GizmoLineConfig](struct.GizmoLineConfig.html "struct bevy::prelude::GizmoLineConfig")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/ambient_light.rs.html#60)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GlobalAmbientLight](struct.GlobalAmbientLight.html "struct bevy::prelude::GlobalAmbientLight")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#296)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GlobalRenderDebugOverlay](../dev_tools/render_debug/struct.GlobalRenderDebugOverlay.html "struct bevy::dev_tools::render_debug::GlobalRenderDebugOverlay")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#107)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GlobalUiDebugOptions](struct.GlobalUiDebugOptions.html "struct bevy::prelude::GlobalUiDebugOptions")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/volume.rs.html#8)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GlobalVolume](struct.GlobalVolume.html "struct bevy::prelude::GlobalVolume")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/globals.rs.html#42)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GlobalsUniform](../render/globals/struct.GlobalsUniform.html "struct bevy::render::globals::GlobalsUniform")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#266)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GltfExtras](struct.GltfExtras.html "struct bevy::prelude::GltfExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#334)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GltfMaterialExtras](../gltf/struct.GltfMaterialExtras.html "struct bevy::gltf::GltfMaterialExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#309)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GltfMeshExtras](../gltf/struct.GltfMeshExtras.html "struct bevy::gltf::GltfMeshExtras")

[Source](https://docs.rs/bevy_gltf/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gltf/assets.rs.html#284)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GltfSceneExtras](../gltf/struct.GltfSceneExtras.html "struct bevy::gltf::GltfSceneExtras")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#32)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GlyphAtlasInfo](../text/struct.GlyphAtlasInfo.html "struct bevy::text::GlyphAtlasInfo")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#51)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GlyphAtlasLocation](../text/struct.GlyphAtlasLocation.html "struct bevy::text::GlyphAtlasLocation")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/atmosphere/mod.rs.html#357)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GpuAtmosphereSettings](../pbr/struct.GpuAtmosphereSettings.html "struct bevy::pbr::GpuAtmosphereSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2020)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GridPlacement](struct.GridPlacement.html "struct bevy::prelude::GridPlacement")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1609)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GridTrack](struct.GridTrack.html "struct bevy::prelude::GridTrack")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/half_space.rs.html#36)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [HalfSpace](struct.HalfSpace.html "struct bevy::prelude::HalfSpace")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/components.rs.html#87)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Hdr](../camera/struct.Hdr.html "struct bevy::camera::Hdr")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#133)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [HitData](../picking/backend/struct.HitData.html "struct bevy::picking::backend::HitData")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsla.rs.html#18)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Hsla](struct.Hsla.html "struct bevy::prelude::Hsla")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hsva.rs.html#18)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Hsva](struct.Hsva.html "struct bevy::prelude::Hsva")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/hwba.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Hwba](struct.Hwba.html "struct bevy::prelude::Hwba")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#48-55)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I8Vec2](../math/struct.I8Vec2.html "struct bevy::math::I8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#57-65)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I8Vec3](../math/struct.I8Vec3.html "struct bevy::math::I8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#67-76)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I8Vec4](../math/struct.I8Vec4.html "struct bevy::math::I8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#78-85)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I16Vec2](../math/struct.I16Vec2.html "struct bevy::math::I16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#87-95)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I16Vec3](../math/struct.I16Vec3.html "struct bevy::math::I16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#97-106)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I16Vec4](../math/struct.I16Vec4.html "struct bevy::math::I16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#108-115)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I64Vec2](../math/struct.I64Vec2.html "struct bevy::math::I64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#117-125)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I64Vec3](../math/struct.I64Vec3.html "struct bevy::math::I64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#127-136)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [I64Vec4](../math/struct.I64Vec4.html "struct bevy::math::I64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/irect.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [IRect](struct.IRect.html "struct bevy::prelude::IRect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#20-27)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [IVec2](struct.IVec2.html "struct bevy::prelude::IVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#28-36)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [IVec3](struct.IVec3.html "struct bevy::prelude::IVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#37-46)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [IVec4](struct.IVec4.html "struct bevy::prelude::IVec4")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#15)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ImageNode](struct.ImageNode.html "struct bevy::prelude::ImageNode")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#192)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ImageNodeSize](../ui/widget/struct.ImageNodeSize.html "struct bevy::ui::widget::ImageNodeSize")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#983)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ImageRenderTarget](../camera/struct.ImageRenderTarget.html "struct bevy::camera::ImageRenderTarget")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/image.rs.html#830)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ImageSamplerDescriptor](../image/struct.ImageSamplerDescriptor.html "struct bevy::image::ImageSamplerDescriptor")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#89)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [InfiniteGrid](../dev_tools/infinite_grid/struct.InfiniteGrid.html "struct bevy::dev_tools::infinite_grid::InfiniteGrid")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/infinite_grid.rs.html#105)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [InfiniteGridSettings](../dev_tools/infinite_grid/struct.InfiniteGridSettings.html "struct bevy::dev_tools::infinite_grid::InfiniteGridSettings")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#180)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [InfinitePlane3d](struct.InfinitePlane3d.html "struct bevy::prelude::InfinitePlane3d")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/font_styles.rs.html#19)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [InheritableFont](../feathers/font_styles/struct.InheritableFont.html "struct bevy::feathers::font_styles::InheritableFont")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#100)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [InputFocus](../input_focus/struct.InputFocus.html "struct bevy::input_focus::InputFocus")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#346)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Insert](struct.Insert.html "struct bevy::prelude::Insert")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [InteractionDisabled](../ui/struct.InteractionDisabled.html "struct bevy::ui::InteractionDisabled")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#1098)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [InternalWindowState](../window/struct.InternalWindowState.html "struct bevy::window::InternalWindowState")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/interval.rs.html#23)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Interval](struct.Interval.html "struct bevy::prelude::Interval")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#329)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [IrradianceVolume](../light/struct.IrradianceVolume.html "struct bevy::light::IrradianceVolume")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2978)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [IsDefaultUiCamera](struct.IsDefaultUiCamera.html "struct bevy::prelude::IsDefaultUiCamera")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#90)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Isometry2d](struct.Isometry2d.html "struct bevy::prelude::Isometry2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/isometry.rs.html#368)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Isometry3d](struct.Isometry3d.html "struct bevy::prelude::Isometry3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#53)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [JointAabb](../mesh/skinning/struct.JointAabb.html "struct bevy::mesh::skinning::JointAabb")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#152)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [KeyboardFocusLost](../input/keyboard/struct.KeyboardFocusLost.html "struct bevy::input::keyboard::KeyboardFocusLost")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/keyboard.rs.html#103)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [KeyboardInput](../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/laba.rs.html#17)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Laba](struct.Laba.html "struct bevy::prelude::Laba")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/label.rs.html#5)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Label](struct.Label.html "struct bevy::prelude::Label")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2903)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LayoutConfig](struct.LayoutConfig.html "struct bevy::prelude::LayoutConfig")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/lcha.rs.html#17)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Lcha](struct.Lcha.html "struct bevy::prelude::Lcha")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#273)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Leave](struct.Leave.html "struct bevy::prelude::Leave")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/lens_distortion.rs.html#22)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LensDistortion](../post_process/effect_stack/struct.LensDistortion.html "struct bevy::post_process::effect_stack::LensDistortion")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#166)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LightGizmoConfigGroup](struct.LightGizmoConfigGroup.html "struct bevy::prelude::LightGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#71)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LightProbe](struct.LightProbe.html "struct bevy::prelude::LightProbe")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/lightmap/mod.rs.html#87)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Lightmap](../pbr/struct.Lightmap.html "struct bevy::pbr::Lightmap")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1234)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Line2d](struct.Line2d.html "struct bevy::prelude::Line2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#357)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Line3d](struct.Line3d.html "struct bevy::prelude::Line3d")

[Source](https://docs.rs/bevy_gizmos_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos_render/lib.rs.html#622)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LineGizmoEntities](../gizmos_render/struct.LineGizmoEntities.html "struct bevy::gizmos_render::LineGizmoEntities")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#227)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LinearGradient](struct.LinearGradient.html "struct bevy::prelude::LinearGradient")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/linear_rgba.rs.html#18)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LinearRgba](struct.LinearRgba.html "struct bevy::prelude::LinearRgba")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/list.rs.html#39)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ListItem](../ui_widgets/struct.ListItem.html "struct bevy::ui_widgets::ListItem")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#210)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Location](../picking/pointer/struct.Location.html "struct bevy::picking::pointer::Location")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#343-350)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Mat2](struct.Mat2.html "struct bevy::prelude::Mat2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#351-359)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Mat3](struct.Mat3.html "struct bevy::prelude::Mat3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#369-378)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Mat4](struct.Mat4.html "struct bevy::prelude::Mat4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#360-368)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Mat3A](struct.Mat3A.html "struct bevy::prelude::Mat3A")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/material_bind_groups.rs.html#259)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MaterialBindingId](../pbr/struct.MaterialBindingId.html "struct bevy::pbr::MaterialBindingId")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#414)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MenuButton](../ui_widgets/struct.MenuButton.html "struct bevy::ui_widgets::MenuButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#79)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MenuEvent](../ui_widgets/struct.MenuEvent.html "struct bevy::ui_widgets::MenuEvent")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#133)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MenuItem](../ui_widgets/struct.MenuItem.html "struct bevy::ui_widgets::MenuItem")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/menu.rs.html#123)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MenuPopup](../ui_widgets/struct.MenuPopup.html "struct bevy::ui_widgets::MenuPopup")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/mesh.rs.html#225)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Mesh](struct.Mesh.html "struct bevy::prelude::Mesh")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#33)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MeshPickingCamera](struct.MeshPickingCamera.html "struct bevy::prelude::MeshPickingCamera")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/mod.rs.html#38)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MeshPickingSettings](struct.MeshPickingSettings.html "struct bevy::prelude::MeshPickingSettings")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#24)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Monitor](../window/struct.Monitor.html "struct bevy::window::Monitor")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#133)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MorphAttributes](../mesh/morph/struct.MorphAttributes.html "struct bevy::mesh::morph::MorphAttributes")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/morph.rs.html#79)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MorphWeights](struct.MorphWeights.html "struct bevy::prelude::MorphWeights")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/motion_blur/mod.rs.html#73)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MotionBlur](../post_process/motion_blur/struct.MotionBlur.html "struct bevy::post_process::motion_blur::MotionBlur")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#76)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MotionVectorPrepass](../core_pipeline/prepass/struct.MotionVectorPrepass.html "struct bevy::core_pipeline::prepass::MotionVectorPrepass")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#34)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MouseButtonInput](../input/mouse/struct.MouseButtonInput.html "struct bevy::input::mouse::MouseButtonInput")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#99)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MouseMotion](../input/mouse/struct.MouseMotion.html "struct bevy::input::mouse::MouseMotion")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/mouse.rs.html#160)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MouseWheel](../input/mouse/struct.MouseWheel.html "struct bevy::input::mouse::MouseWheel")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#323)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Move](struct.Move.html "struct bevy::prelude::Move")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/directional_navigation.rs.html#187)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NavNeighbors](../input_focus/directional_navigation/struct.NavNeighbors.html "struct bevy::input_focus::directional_navigation::NavNeighbors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#550)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NoAutoAabb](../camera/visibility/struct.NoAutoAabb.html "struct bevy::camera::visibility::NoAutoAabb")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/background_motion_vectors.rs.html#55)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NoBackgroundMotionVectors](../core_pipeline/prepass/struct.NoBackgroundMotionVectors.html "struct bevy::core_pipeline::prepass::NoBackgroundMotionVectors")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#316)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NoFrustumCulling](../camera/visibility/struct.NoFrustumCulling.html "struct bevy::camera::visibility::NoFrustumCulling")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#868)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NoWireframe](../pbr/wireframe/struct.NoWireframe.html "struct bevy::pbr::wireframe::NoWireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#418)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NoWireframe2d](../sprite_render/struct.NoWireframe2d.html "struct bevy::sprite_render::NoWireframe2d")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#471)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Node](struct.Node.html "struct bevy::prelude::Node")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/prepass/mod.rs.html#68)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NormalPrepass](../core_pipeline/prepass/struct.NormalPrepass.html "struct bevy::core_pipeline::prepass::NormalPrepass")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#256)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NotShadowCaster](../light/struct.NotShadowCaster.html "struct bevy::light::NotShadowCaster")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#264)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [NotShadowReceiver](../light/struct.NotShadowReceiver.html "struct bevy::light::NotShadowReceiver")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/occlusion_culling/mod.rs.html#70)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [OcclusionCulling](../render/occlusion_culling/struct.OcclusionCulling.html "struct bevy::render::occlusion_culling::OcclusionCulling")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/path/mod.rs.html#298)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [OffsetAccess](../reflect/struct.OffsetAccess.html "struct bevy::reflect::OffsetAccess")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklaba.rs.html#17)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Oklaba](struct.Oklaba.html "struct bevy::prelude::Oklaba")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/oklcha.rs.html#17)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Oklcha](struct.Oklcha.html "struct bevy::prelude::Oklcha")

[Source](https://docs.rs/bevy_core_pipeline/0.19.0/x86_64-unknown-linux-gnu/src/bevy_core_pipeline/oit/mod.rs.html#39)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [OrderIndependentTransparencySettings](../core_pipeline/oit/struct.OrderIndependentTransparencySettings.html "struct bevy::core_pipeline::oit::OrderIndependentTransparencySettings")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#578)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [OrthographicProjection](struct.OrthographicProjection.html "struct bevy::prelude::OrthographicProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#240)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Out](struct.Out.html "struct bevy::prelude::Out")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2315)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Outline](struct.Outline.html "struct bevy::prelude::Outline")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#190)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Over](struct.Over.html "struct bevy::prelude::Over")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1235)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Overflow](struct.Overflow.html "struct bevy::prelude::Overflow")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1381)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [OverflowClipMargin](struct.OverflowClipMargin.html "struct bevy::prelude::OverflowClipMargin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2416)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [OverrideClip](struct.OverrideClip.html "struct bevy::prelude::OverrideClip")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/pathtracer/mod.rs.html#63)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Pathtracer](../solari/pathtracer/struct.Pathtracer.html "struct bevy::solari::pathtracer::Pathtracer")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/projection.rs.html#281)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PerspectiveProjection](struct.PerspectiveProjection.html "struct bevy::prelude::PerspectiveProjection")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#196)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Pickable](struct.Pickable.html "struct bevy::prelude::Pickable")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/lib.rs.html#296)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PickingSettings](../picking/struct.PickingSettings.html "struct bevy::picking::PickingSettings")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1192)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Plane2d](struct.Plane2d.html "struct bevy::prelude::Plane2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#96)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Plane3d](struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#7)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PlaneMeshBuilder](../mesh/struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#33)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PlaybackSettings](struct.PlaybackSettings.html "struct bevy::prelude::PlaybackSettings")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#38)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointLight](struct.PointLight.html "struct bevy::prelude::PointLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#177)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointLightShadowMap](../light/struct.PointLightShadowMap.html "struct bevy::light::PointLightShadowMap")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/point_light.rs.html#159)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointLightTexture](../light/struct.PointLightTexture.html "struct bevy::light::PointLightTexture")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#91)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointerHits](../picking/backend/struct.PointerHits.html "struct bevy::picking::backend::PointerHits")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#278)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointerInput](../picking/pointer/struct.PointerInput.html "struct bevy::picking::pointer::PointerInput")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/input.rs.html#42)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointerInputSettings](../picking/input/struct.PointerInputSettings.html "struct bevy::picking::input::PointerInputSettings")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#71)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointerInteraction](../picking/pointer/struct.PointerInteraction.html "struct bevy::picking::pointer::PointerInteraction")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#178)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointerLocation](../picking/pointer/struct.PointerLocation.html "struct bevy::picking::pointer::PointerLocation")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/pointer.rs.html#114)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PointerPress](../picking/pointer/struct.PointerPress.html "struct bevy::picking::pointer::PointerPress")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1894)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Polygon](struct.Polygon.html "struct bevy::prelude::Polygon")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1566)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Polyline2d](struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#701)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Polyline2dMeshBuilder](../mesh/struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#624)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Polyline3d](struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#84)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Popover](../ui_widgets/popover/struct.Popover.html "struct bevy::ui_widgets::popover::Popover")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/popover.rs.html#69)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PopoverPlacement](../ui_widgets/popover/struct.PopoverPlacement.html "struct bevy::ui_widgets::popover::PopoverPlacement")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/glyph.rs.html#13)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PositionedGlyph](../text/struct.PositionedGlyph.html "struct bevy::text::PositionedGlyph")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text_edit.rs.html#16)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PreeditCursor](../text/struct.PreeditCursor.html "struct bevy::text::PreeditCursor")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#286)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Press](struct.Press.html "struct bevy::prelude::Press")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/interaction_states.rs.html#44)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Pressed](../ui/struct.Pressed.html "struct bevy::ui::Pressed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#53)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PrimaryMonitor](../window/struct.PrimaryMonitor.html "struct bevy::window::PrimaryMonitor")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#53)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PrimaryWindow](../window/struct.PrimaryWindow.html "struct bevy::window::PrimaryWindow")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#450-459)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Quat](struct.Quat.html "struct bevy::prelude::Quat")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/gradients.rs.html#361)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RadialGradient](struct.RadialGradient.html "struct bevy::prelude::RadialGradient")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#58)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RadioButton](../ui_widgets/struct.RadioButton.html "struct bevy::ui_widgets::RadioButton")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/radio.rs.html#40)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RadioGroup](../ui_widgets/struct.RadioGroup.html "struct bevy::ui_widgets::RadioGroup")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#118)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RawGamepadAxisChangedEvent](../input/gamepad/struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/gamepad.rs.html#86)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RawGamepadButtonChangedEvent](../input/gamepad/struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#17)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Ray2d](struct.Ray2d.html "struct bevy::prelude::Ray2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/ray.rs.html#74)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Ray3d](struct.Ray3d.html "struct bevy::prelude::Ray3d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast2d.rs.html#12)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RayCast2d](../math/bounding/struct.RayCast2d.html "struct bevy::math::bounding::RayCast2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/bounding/raycast3d.rs.html#12)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RayCast3d](../math/bounding/struct.RayCast3d.html "struct bevy::math::bounding::RayCast3d")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/mod.rs.html#106)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RayCastBackfaces](struct.RayCastBackfaces.html "struct bevy::prelude::RayCastBackfaces")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/backend.rs.html#245)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RayId](../picking/backend/ray/struct.RayId.html "struct bevy::picking::backend::ray::RayId")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/mesh_picking/ray_cast/intersections.rs.html#8)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RayMeshHit](../picking/mesh_picking/ray_cast/struct.RayMeshHit.html "struct bevy::picking::mesh_picking::ray_cast::RayMeshHit")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/gpu_readback.rs.html#114)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ReadbackComplete](../render/gpu_readback/struct.ReadbackComplete.html "struct bevy::render::gpu_readback::ReadbackComplete")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/real.rs.html#44)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Real](struct.Real.html "struct bevy::prelude::Real")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/rect.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Rect](struct.Rect.html "struct bevy::prelude::Rect")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/rect_light.rs.html#18)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RectLight](struct.RectLight.html "struct bevy::prelude::RectLight")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1801)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Rectangle](struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1041)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RectangleMeshBuilder](../mesh/struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#2036)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RegularPolygon](struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#482)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RegularPolygonMeshBuilder](../mesh/struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/focus.rs.html#78)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RelativeCursorPosition](../ui/struct.RelativeCursorPosition.html "struct bevy::ui::RelativeCursorPosition")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#298)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Release](struct.Release.html "struct bevy::prelude::Release")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lifecycle.rs.html#376)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Remove](struct.Remove.html "struct bevy::prelude::Remove")

[Source](https://docs.rs/bevy_dev_tools/0.19.0/x86_64-unknown-linux-gnu/src/bevy_dev_tools/render_debug.rs.html#273)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RenderDebugOverlay](../dev_tools/render_debug/struct.RenderDebugOverlay.html "struct bevy::dev_tools::render_debug::RenderDebugOverlay")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#63)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RenderShadowMapVisibleEntities](../render/view/struct.RenderShadowMapVisibleEntities.html "struct bevy::render::view::RenderShadowMapVisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#196)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RenderTargetInfo](../camera/struct.RenderTargetInfo.html "struct bevy::camera::RenderTargetInfo")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/visibility/mod.rs.html#87)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RenderVisibleEntitiesClass](../render/view/struct.RenderVisibleEntitiesClass.html "struct bevy::render::view::RenderVisibleEntitiesClass")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#1823)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RepeatedGridTrack](struct.RepeatedGridTrack.html "struct bevy::prelude::RepeatedGridTrack")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#53)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RequestRedraw](../window/struct.RequestRedraw.html "struct bevy::window::RequestRedraw")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2802)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ResolvedBorderRadius](struct.ResolvedBorderRadius.html "struct bevy::prelude::ResolvedBorderRadius")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1055)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Rhombus](struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#878)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RhombusMeshBuilder](../mesh/struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rotation2d.rs.html#44)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Rot2](struct.Rot2.html "struct bevy::prelude::Rot2")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#501)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RunGeometry](../text/struct.RunGeometry.html "struct bevy::text::RunGeometry")

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene_component.rs.html#22)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SceneComponentInfo](../scene/struct.SceneComponentInfo.html "struct bevy::scene::SceneComponentInfo")

[Source](https://docs.rs/bevy_remote/0.19.0/x86_64-unknown-linux-gnu/src/bevy_remote/schemas/mod.rs.html#18)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SchemaTypesMetadata](../remote/schemas/struct.SchemaTypesMetadata.html "struct bevy::remote::schemas::SchemaTypesMetadata")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssao/mod.rs.html#111)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ScreenSpaceAmbientOcclusion](../pbr/struct.ScreenSpaceAmbientOcclusion.html "struct bevy::pbr::ScreenSpaceAmbientOcclusion")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/ssr/mod.rs.html#78)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ScreenSpaceReflections](../pbr/struct.ScreenSpaceReflections.html "struct bevy::pbr::ScreenSpaceReflections")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/transmission/mod.rs.html#65)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ScreenSpaceTransmission](../pbr/struct.ScreenSpaceTransmission.html "struct bevy::pbr::ScreenSpaceTransmission")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/view/window/screenshot.rs.html#47)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ScreenshotCaptured](../render/view/window/screenshot/struct.ScreenshotCaptured.html "struct bevy::render::view::window::screenshot::ScreenshotCaptured")

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#455)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Scroll](struct.Scroll.html "struct bevy::prelude::Scroll")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollarea.rs.html#16)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ScrollArea](../ui_widgets/struct.ScrollArea.html "struct bevy::ui_widgets::ScrollArea")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#67)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Scrollbar](../ui_widgets/struct.Scrollbar.html "struct bevy::ui_widgets::Scrollbar")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#130)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ScrollbarDragState](../ui_widgets/struct.ScrollbarDragState.html "struct bevy::ui_widgets::ScrollbarDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/scrollbar.rs.html#100)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ScrollbarThumb](../ui_widgets/struct.ScrollbarThumb.html "struct bevy::ui_widgets::ScrollbarThumb")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1254)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Segment2d](struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#376)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Segment3d](struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/text_input.rs.html#406)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SelectAllOnFocus](../ui_widgets/struct.SelectAllOnFocus.html "struct bevy::ui_widgets::SelectAllOnFocus")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#181)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SetChecked](../ui_widgets/struct.SetChecked.html "struct bevy::ui_widgets::SetChecked")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#673)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SetSliderValue](../ui_widgets/struct.SetSliderValue.html "struct bevy::ui_widgets::SetSliderValue")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#854)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ShadowLodOrigin](../camera/struct.ShadowLodOrigin.html "struct bevy::camera::ShadowLodOrigin")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_node.rs.html#2868)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ShadowStyle](struct.ShadowStyle.html "struct bevy::prelude::ShadowStyle")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/aabb.rs.html#61)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ShowAabbGizmo](struct.ShowAabbGizmo.html "struct bevy::prelude::ShowAabbGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/frustum.rs.html#96)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ShowFrustumGizmo](struct.ShowFrustumGizmo.html "struct bevy::prelude::ShowFrustumGizmo")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/gizmos.rs.html#210)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ShowLightGizmo](struct.ShowLightGizmo.html "struct bevy::prelude::ShowLightGizmo")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#76)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ShowSkinnedMeshBoundsGizmo](struct.ShowSkinnedMeshBoundsGizmo.html "struct bevy::prelude::ShowSkinnedMeshBoundsGizmo")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#16)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SkinnedMesh](../mesh/skinning/struct.SkinnedMesh.html "struct bevy::mesh::skinning::SkinnedMesh")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/skinning.rs.html#88)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SkinnedMeshBounds](../mesh/skinning/struct.SkinnedMeshBounds.html "struct bevy::mesh::skinning::SkinnedMeshBounds")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/skinned_mesh_bounds.rs.html#52)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SkinnedMeshBoundsGizmoConfigGroup](struct.SkinnedMeshBoundsGizmoConfigGroup.html "struct bevy::prelude::SkinnedMeshBoundsGizmoConfigGroup")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/probe.rs.html#227)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Skybox](../core_pipeline/struct.Skybox.html "struct bevy::core_pipeline::Skybox")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#103)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Slider](../ui_widgets/struct.Slider.html "struct bevy::ui_widgets::Slider")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#245)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SliderDragState](../ui_widgets/struct.SliderDragState.html "struct bevy::ui_widgets::SliderDragState")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#127)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SliderRange](../ui_widgets/struct.SliderRange.html "struct bevy::ui_widgets::SliderRange")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/slider.rs.html#113)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SliderThumb](../ui_widgets/struct.SliderThumb.html "struct bevy::ui_widgets::SliderThumb")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/smaa/mod.rs.html#84)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Smaa](../anti_alias/smaa/struct.Smaa.html "struct bevy::anti_alias::smaa::Smaa")

[Source](https://docs.rs/bevy_solari/0.19.0/x86_64-unknown-linux-gnu/src/bevy_solari/realtime/mod.rs.html#85)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SolariLighting](../solari/realtime/struct.SolariLighting.html "struct bevy::solari::realtime::SolariLighting")

[Source](https://docs.rs/bevy_audio/0.19.0/x86_64-unknown-linux-gnu/src/bevy_audio/audio.rs.html#170)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SpatialListener](struct.SpatialListener.html "struct bevy::prelude::SpatialListener")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#23)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for bevy::prelude::[Sphere](struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#196)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for bevy::camera::primitives::[Sphere](../camera/primitives/struct.Sphere.html "struct bevy::camera::primitives::Sphere")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#51)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SphereMeshBuilder](../mesh/struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#22)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SpotLight](struct.SpotLight.html "struct bevy::prelude::SpotLight")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/spot_light.rs.html#204)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SpotLightTexture](../light/struct.SpotLightTexture.html "struct bevy::light::SpotLightTexture")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite.rs.html#15)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Sprite](struct.Sprite.html "struct bevy::prelude::Sprite")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/sprite_mesh/sprite_material.rs.html#34)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SpriteMaterial](struct.SpriteMaterial.html "struct bevy::prelude::SpriteMaterial")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/sprite_mesh.rs.html#16)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SpriteMesh](struct.SpriteMesh.html "struct bevy::prelude::SpriteMesh")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#34)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SpritePickingCamera](struct.SpritePickingCamera.html "struct bevy::prelude::SpritePickingCamera")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/picking_backend.rs.html#51)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SpritePickingSettings](struct.SpritePickingSettings.html "struct bevy::prelude::SpritePickingSettings")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/srgba.rs.html#20)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Srgba](struct.Srgba.html "struct bevy::prelude::Srgba")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/pbr_material.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [StandardMaterial](struct.StandardMaterial.html "struct bevy::prelude::StandardMaterial")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/stopwatch.rs.html#31)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Stopwatch](../time/struct.Stopwatch.html "struct bevy::time::Stopwatch")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1132)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Strikethrough](struct.Strikethrough.html "struct bevy::prelude::Strikethrough")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#174)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SubCameraView](../camera/struct.SubCameraView.html "struct bevy::camera::SubCameraView")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#121)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SyncToRenderWorld](../render/sync_world/struct.SyncToRenderWorld.html "struct bevy::render::sync_world::SyncToRenderWorld")

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/tab_navigation.rs.html#69)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TabGroup](../input_focus/tab_navigation/struct.TabGroup.html "struct bevy::input_focus::tab_navigation::TabGroup")

[Source](https://docs.rs/bevy_anti_alias/0.19.0/x86_64-unknown-linux-gnu/src/bevy_anti_alias/taa/mod.rs.html#111)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TemporalAntiAliasing](../anti_alias/taa/struct.TemporalAntiAliasing.html "struct bevy::anti_alias::taa::TemporalAntiAliasing")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/camera.rs.html#780)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TemporalJitter](../render/camera/struct.TemporalJitter.html "struct bevy::render::camera::TemporalJitter")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/sync_world.rs.html#190)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TemporaryRenderEntity](../render/sync_world/struct.TemporaryRenderEntity.html "struct bevy::render::sync_world::TemporaryRenderEntity")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1433)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Tetrahedron](struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#8)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TetrahedronMeshBuilder](../mesh/struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/text2d.rs.html#141)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Text2dShadow](../sprite/struct.Text2dShadow.html "struct bevy::sprite::Text2dShadow")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/bounds.rs.html#13)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextBounds](../text/struct.TextBounds.html "struct bevy::text::TextBounds")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextEntity](../text/struct.TextEntity.html "struct bevy::text::TextEntity")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#374)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextFont](struct.TextFont.html "struct bevy::prelude::TextFont")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#130)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextLayout](struct.TextLayout.html "struct bevy::prelude::TextLayout")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/pipeline.rs.html#461)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextLayoutInfo](../text/struct.TextLayoutInfo.html "struct bevy::text::TextLayoutInfo")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#32)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextNodeFlags](../ui/struct.TextNodeFlags.html "struct bevy::ui::TextNodeFlags")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/text.rs.html#144)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextShadow](struct.TextShadow.html "struct bevy::prelude::TextShadow")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#211)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextureAtlas](struct.TextureAtlas.html "struct bevy::prelude::TextureAtlas")

[Source](https://docs.rs/bevy_image/0.19.0/x86_64-unknown-linux-gnu/src/bevy_image/texture_atlas.rs.html#95)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextureAtlasLayout](struct.TextureAtlasLayout.html "struct bevy::prelude::TextureAtlasLayout")

[Source](https://docs.rs/bevy_sprite/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite/texture_slice/slicer.rs.html#13)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TextureSlicer](struct.TextureSlicer.html "struct bevy::prelude::TextureSlicer")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#50)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ThemeProps](../feathers/theme/struct.ThemeProps.html "struct bevy::feathers::theme::ThemeProps")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/theme.rs.html#125)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ThemedText](../feathers/theme/struct.ThemedText.html "struct bevy::feathers::theme::ThemedText")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/graph.rs.html#298)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ThreadedAnimationGraph](struct.ThreadedAnimationGraph.html "struct bevy::prelude::ThreadedAnimationGraph")

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/tick.rs.html#15)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Tick](../ecs/change_detection/struct.Tick.html "struct bevy::ecs::change_detection::Tick")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#94)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TileData](../sprite_render/struct.TileData.html "struct bevy::sprite_render::TileData")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/tilemap_chunk/mod.rs.html#52)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TilemapChunk](../sprite_render/struct.TilemapChunk.html "struct bevy::sprite_render::TilemapChunk")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/timer.rs.html#31)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Timer](struct.Timer.html "struct bevy::prelude::Timer")

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/checkbox.rs.html#209)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ToggleChecked](../ui_widgets/struct.ToggleChecked.html "struct bevy::ui_widgets::ToggleChecked")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1124)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Torus](struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#8)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TorusMeshBuilder](../mesh/struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#45)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TouchInput](struct.TouchInput.html "struct bevy::prelude::TouchInput")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#79)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Transform](struct.Transform.html "struct bevy::prelude::Transform")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#95)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TransformGizmoCamera](struct.TransformGizmoCamera.html "struct bevy::prelude::TransformGizmoCamera")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#85)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TransformGizmoFocus](struct.TransformGizmoFocus.html "struct bevy::prelude::TransformGizmoFocus")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#136)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TransformGizmoSettings](struct.TransformGizmoSettings.html "struct bevy::prelude::TransformGizmoSettings")

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/transform_gizmo.rs.html#179)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TransformGizmoState](struct.TransformGizmoState.html "struct bevy::prelude::TransformGizmoState")

[Source](https://docs.rs/bevy_transform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_transform/components/transform.rs.html#666)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TransformTreeChanged](struct.TransformTreeChanged.html "struct bevy::prelude::TransformTreeChanged")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/lib.rs.html#274)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [TransmittedShadowReceiver](../light/struct.TransmittedShadowReceiver.html "struct bevy::light::TransmittedShadowReceiver")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim2.rs.html#1627)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Triangle2d](struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#964)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Triangle2dMeshBuilder](../mesh/struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/dim3.rs.html#1236)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Triangle3d](struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#7)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Triangle3dMeshBuilder](../mesh/struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#166-173)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U8Vec2](../math/struct.U8Vec2.html "struct bevy::math::U8Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#174-182)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U8Vec3](../math/struct.U8Vec3.html "struct bevy::math::U8Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#183-192)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U8Vec4](../math/struct.U8Vec4.html "struct bevy::math::U8Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#194-201)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U16Vec2](../math/struct.U16Vec2.html "struct bevy::math::U16Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#202-210)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U16Vec3](../math/struct.U16Vec3.html "struct bevy::math::U16Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#211-220)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U16Vec4](../math/struct.U16Vec4.html "struct bevy::math::U16Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#222-229)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U64Vec2](../math/struct.U64Vec2.html "struct bevy::math::U64Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#230-238)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U64Vec3](../math/struct.U64Vec3.html "struct bevy::math::U64Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#239-248)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [U64Vec4](../math/struct.U64Vec4.html "struct bevy::math::U64Vec4")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/rects/urect.rs.html#21)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [URect](struct.URect.html "struct bevy::prelude::URect")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#138-145)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UVec2](struct.UVec2.html "struct bevy::prelude::UVec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#146-154)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UVec3](struct.UVec3.html "struct bevy::prelude::UVec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#155-164)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UVec4](struct.UVec4.html "struct bevy::prelude::UVec4")

[Source](https://docs.rs/bevy_ui_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_render/debug_overlay.rs.html#39)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UiDebugOptions](struct.UiDebugOptions.html "struct bevy::prelude::UiDebugOptions")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#40)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UiPickingCamera](struct.UiPickingCamera.html "struct bevy::prelude::UiPickingCamera")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/picking_backend.rs.html#45)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UiPickingSettings](struct.UiPickingSettings.html "struct bevy::prelude::UiPickingSettings")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#993)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UiPosition](struct.UiPosition.html "struct bevy::prelude::UiPosition")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/geometry.rs.html#625)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UiRect](struct.UiRect.html "struct bevy::prelude::UiRect")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/stack.rs.html#25)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UiStack](../ui/struct.UiStack.html "struct bevy::ui::UiStack")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#122)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UiTransform](struct.UiTransform.html "struct bevy::prelude::UiTransform")

[Source](https://docs.rs/bevy_text/0.19.0/x86_64-unknown-linux-gnu/src/bevy_text/text.rs.html#1154)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Underline](struct.Underline.html "struct bevy::prelude::Underline")

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/number_input.rs.html#170)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UpdateNumberInput](../feathers/controls/struct.UpdateNumberInput.html "struct bevy::feathers::controls::UpdateNumberInput")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/ui_transform.rs.html#15)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Val2](struct.Val2.html "struct bevy::prelude::Val2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#250-257)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Vec2](struct.Vec2.html "struct bevy::prelude::Vec2")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#258-266)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Vec3](struct.Vec3.html "struct bevy::prelude::Vec3")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#276-285)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Vec4](struct.Vec4.html "struct bevy::prelude::Vec4")

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/impls/glam.rs.html#267-275)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Vec3A](struct.Vec3A.html "struct bevy::prelude::Vec3A")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/monitor.rs.html#72)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [VideoMode](../window/struct.VideoMode.html "struct bevy::window::VideoMode")

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/primitives/view_frustum.rs.html#18)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ViewFrustum](struct.ViewFrustum.html "struct bevy::prelude::ViewFrustum")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/camera.rs.html#60)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Viewport](../camera/struct.Viewport.html "struct bevy::camera::Viewport")

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/viewport.rs.html#36)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ViewportNode](struct.ViewportNode.html "struct bevy::prelude::ViewportNode")

[Source](https://docs.rs/bevy_post_process/0.19.0/x86_64-unknown-linux-gnu/src/bevy_post_process/effect_stack/vignette.rs.html#28)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Vignette](../post_process/effect_stack/struct.Vignette.html "struct bevy::post_process::effect_stack::Vignette")

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/virt.rs.html#74)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Virtual](struct.Virtual.html "struct bevy::prelude::Virtual")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/range.rs.html#78)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [VisibilityRange](../camera/visibility/struct.VisibilityRange.html "struct bevy::camera::visibility::VisibilityRange")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#342)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [VisibleEntities](../camera/visibility/struct.VisibleEntities.html "struct bevy::camera::visibility::VisibleEntities")

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/visibility/mod.rs.html#408)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [VisibleMeshEntities](../camera/visibility/struct.VisibleMeshEntities.html "struct bevy::camera::visibility::VisibleMeshEntities")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#23)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [VolumetricFog](../light/struct.VolumetricFog.html "struct bevy::light::VolumetricFog")

[Source](https://docs.rs/bevy_light/0.19.0/x86_64-unknown-linux-gnu/src/bevy_light/volumetric.rs.html#14)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [VolumetricLight](../light/struct.VolumetricLight.html "struct bevy::light::VolumetricLight")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#155)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Window](struct.Window.html "struct bevy::prelude::Window")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#357)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowBackendScaleFactorChanged](../window/struct.WindowBackendScaleFactorChanged.html "struct bevy::window::WindowBackendScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#95)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowCloseRequested](../window/struct.WindowCloseRequested.html "struct bevy::window::WindowCloseRequested")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#113)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowClosed](../window/struct.WindowClosed.html "struct bevy::window::WindowClosed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#134)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowClosing](../window/struct.WindowClosing.html "struct bevy::window::WindowClosing")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#69)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowCreated](../window/struct.WindowCreated.html "struct bevy::window::WindowCreated")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#154)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowDestroyed](../window/struct.WindowDestroyed.html "struct bevy::window::WindowDestroyed")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#292)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowFocused](../window/struct.WindowFocused.html "struct bevy::window::WindowFocused")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#412)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowMoved](struct.WindowMoved.html "struct bevy::prelude::WindowMoved")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#319)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowOccluded](../window/struct.WindowOccluded.html "struct bevy::window::WindowOccluded")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#675)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowResizeConstraints](struct.WindowResizeConstraints.html "struct bevy::prelude::WindowResizeConstraints")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#31)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowResized](../window/struct.WindowResized.html "struct bevy::window::WindowResized")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/window.rs.html#889)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowResolution](../window/struct.WindowResolution.html "struct bevy::window::WindowResolution")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#338)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowScaleFactorChanged](../window/struct.WindowScaleFactorChanged.html "struct bevy::window::WindowScaleFactorChanged")

[Source](https://docs.rs/bevy_window/0.19.0/x86_64-unknown-linux-gnu/src/bevy_window/event.rs.html#434)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WindowThemeChanged](../window/struct.WindowThemeChanged.html "struct bevy::window::WindowThemeChanged")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#199)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Wireframe](../pbr/wireframe/struct.Wireframe.html "struct bevy::pbr::wireframe::Wireframe")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#163)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Wireframe2d](../sprite_render/struct.Wireframe2d.html "struct bevy::sprite_render::Wireframe2d")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#403)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Wireframe2dColor](../sprite_render/struct.Wireframe2dColor.html "struct bevy::sprite_render::Wireframe2dColor")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#422)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Wireframe2dConfig](../sprite_render/struct.Wireframe2dConfig.html "struct bevy::sprite_render::Wireframe2dConfig")

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/mesh2d/wireframe2d.rs.html#434)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Wireframe2dMaterial](../sprite_render/struct.Wireframe2dMaterial.html "struct bevy::sprite_render::Wireframe2dMaterial")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#843)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WireframeColor](../pbr/wireframe/struct.WireframeColor.html "struct bevy::pbr::wireframe::WireframeColor")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#883)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WireframeConfig](../pbr/wireframe/struct.WireframeConfig.html "struct bevy::pbr::wireframe::WireframeConfig")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#852)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WireframeLineWidth](../pbr/wireframe/struct.WireframeLineWidth.html "struct bevy::pbr::wireframe::WireframeLineWidth")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/wireframe.rs.html#910)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WireframeMaterial](../pbr/wireframe/struct.WireframeMaterial.html "struct bevy::pbr::wireframe::WireframeMaterial")

[Source](https://docs.rs/bevy_world_serialization/0.19.0/x86_64-unknown-linux-gnu/src/bevy_world_serialization/world_asset_spawner.rs.html#31)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WorldInstanceReady](../world_serialization/struct.WorldInstanceReady.html "struct bevy::world_serialization::WorldInstanceReady")

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/xyza.rs.html#17)

### impl [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Xyza](struct.Xyza.html "struct bevy::prelude::Xyza")

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#303)

### impl<A> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimatableCurveEvaluator](struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>

where A: [Animatable](trait.Animatable.html "trait bevy::prelude::Animatable") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [AnimatableCurveEvaluator](struct.AnimatableCurveEvaluator.html "struct bevy::prelude::AnimatableCurveEvaluator")<A>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), BasicAnimationCurveEvaluator<A>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [AnimatableProperty](trait.AnimatableProperty.html "trait bevy::prelude::AnimatableProperty")<Property = A>>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/extended_material.rs.html#142)

### impl<B, E> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ExtendedMaterial](../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>

where B: [Material](trait.Material.html "trait bevy::prelude::Material") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, E: [MaterialExtension](../pbr/trait.MaterialExtension.html "trait bevy::pbr::MaterialExtension") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, [ExtendedMaterial](../pbr/struct.ExtendedMaterial.html "struct bevy::pbr::ExtendedMaterial")<B, E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/gizmos.rs.html#284)

### impl<Config, Clear> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_picking/0.19.0/x86_64-unknown-linux-gnu/src/bevy_picking/events.rs.html#71)

### impl<E> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Pointer](struct.Pointer.html "struct bevy::prelude::Pointer")<E>

where E: [Debug](https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html "trait core::fmt::Debug") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Reflect](trait.Reflect.html "trait bevy::prelude::Reflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Pointer](struct.Pointer.html "struct bevy::prelude::Pointer")<E>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_input_focus/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input_focus/lib.rs.html#189)

### impl<M> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FocusedInput](../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>

where M: [Message](trait.Message.html "trait bevy::prelude::Message") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [FocusedInput](../input_focus/struct.FocusedInput.html "struct bevy::input_focus::FocusedInput")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/mod.rs.html#117)

### impl<M> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MessageId](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>

where M: [Message](trait.Message.html "trait bevy::prelude::Message") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [MessageId](../ecs/message/struct.MessageId.html "struct bevy::ecs::message::MessageId")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/message/messages.rs.html#94)

### impl<M> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Messages](struct.Messages.html "struct bevy::prelude::Messages")<M>

where M: [Message](trait.Message.html "trait bevy::prelude::Message") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [Messages](struct.Messages.html "struct bevy::prelude::Messages")<M>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), MessageSequence<M>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#286)

### impl<P, C> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimatableCurve](struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>

where [AnimatableCurve](struct.AnimatableCurve.html "struct bevy::prelude::AnimatableCurve")<P, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), P: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#434)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicBSpline](struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBSpline](struct.CubicBSpline.html "struct bevy::prelude::CubicBSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#54)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicBezier](struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicBezier](struct.CubicBezier.html "struct bevy::prelude::CubicBezier")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html)\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#272)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicCardinalSpline](struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCardinalSpline](struct.CubicCardinalSpline.html "struct bevy::prelude::CubicCardinalSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1169)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicCurve](struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicCurve](struct.CubicCurve.html "struct bevy::prelude::CubicCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[CubicSegment](struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#144)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicHermite](struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicHermite](struct.CubicHermite.html "struct bevy::prelude::CubicHermite")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[(P, P)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#611)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicNurbs](struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicNurbs](struct.CubicNurbs.html "struct bevy::prelude::CubicNurbs")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#946)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicSegment](struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [CubicSegment](struct.CubicSegment.html "struct bevy::prelude::CubicSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#837)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LinearSpline](../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [LinearSpline](../math/cubic_splines/struct.LinearSpline.html "struct bevy::math::cubic_splines::LinearSpline")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<P>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1470)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RationalCurve](struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalCurve](struct.RationalCurve.html "struct bevy::prelude::RationalCurve")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<[RationalSegment](struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/cubic_splines/mod.rs.html#1328)

### impl<P> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RationalSegment](struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>

where P: [VectorSpace](../math/trait.VectorSpace.html "trait bevy::math::VectorSpace") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [RationalSegment](struct.RationalSegment.html "struct bevy::prelude::RationalSegment")<P>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [\[P; 4\]](https://doc.rust-lang.org/nightly/std/primitive.array.html): [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#482)

### impl<S, T, C, D> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ZipCurve](struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>

where [ZipCurve](struct.ZipCurve.html "struct bevy::prelude::ZipCurve")<S, T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), S: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#188)

### impl<S, T, C, F> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MapCurve](struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>

where [MapCurve](struct.MapCurve.html "struct bevy::prelude::MapCurve")<S, T, C, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, S: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#66)

### impl<S> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DespawnWhen](struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [DespawnWhen](struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#313)

### impl<S> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [DisableWhen](struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [DisableWhen](struct.DisableWhen.html "struct bevy::prelude::DisableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state_scoped.rs.html#560)

### impl<S> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [EnableWhen](struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>

where S: [States](trait.States.html "trait bevy::prelude::States") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [EnableWhen](struct.EnableWhen.html "struct bevy::prelude::EnableWhen")<S>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [Box](struct.Box.html "struct bevy::prelude::Box")<dyn [Fn](https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html "trait core::ops::function::Fn")(&[StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent")<S>) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync")\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#524)

### impl<T, C, D> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ChainCurve](struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>

where [ChainCurve](struct.ChainCurve.html "struct bevy::prelude::ChainCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#787)

### impl<T, C, D> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ContinuationCurve](struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>

where [ContinuationCurve](struct.ContinuationCurve.html "struct bevy::prelude::ContinuationCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#417)

### impl<T, C, D> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CurveReparamCurve](struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>

where [CurveReparamCurve](struct.CurveReparamCurve.html "struct bevy::prelude::CurveReparamCurve")<T, C, D>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection, D: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#285)

### impl<T, C, F> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ReparamCurve](struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>

where [ReparamCurve](struct.ReparamCurve.html "struct bevy::prelude::ReparamCurve")<T, C, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), C: [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#677)

### impl<T, C> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ForeverCurve](struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>

where [ForeverCurve](struct.ForeverCurve.html "struct bevy::prelude::ForeverCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#451)

### impl<T, C> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [GraphCurve](struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>

where [GraphCurve](struct.GraphCurve.html "struct bevy::prelude::GraphCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#381)

### impl<T, C> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [LinearReparamCurve](struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>

where [LinearReparamCurve](struct.LinearReparamCurve.html "struct bevy::prelude::LinearReparamCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#732)

### impl<T, C> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [PingPongCurve](struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>

where [PingPongCurve](struct.PingPongCurve.html "struct bevy::prelude::PingPongCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#617)

### impl<T, C> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [RepeatCurve](struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>

where [RepeatCurve](struct.RepeatCurve.html "struct bevy::prelude::RepeatCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#575)

### impl<T, C> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ReverseCurve](struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>

where [ReverseCurve](struct.ReverseCurve.html "struct bevy::prelude::ReverseCurve")<T, C>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), C: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#86)

### impl<T, F> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [FunctionCurve](struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>

where [FunctionCurve](struct.FunctionCurve.html "struct bevy::prelude::FunctionCurve")<T, F>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#26)

### impl<T, I> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SampleCurve](struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>

where [SampleCurve](struct.SampleCurve.html "struct bevy::prelude::SampleCurve")<T, I>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [EvenCore](struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#186)

### impl<T, I> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UnevenSampleCurve](struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>

where [UnevenSampleCurve](struct.UnevenSampleCurve.html "struct bevy::prelude::UnevenSampleCurve")<T, I>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [UnevenCore](struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [PartialReflect](trait.PartialReflect.html "trait bevy::prelude::PartialReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"),

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/animation_curves.rs.html#722)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [AnimatableKeyframeCurve](struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>

where [AnimatableKeyframeCurve](struct.AnimatableKeyframeCurve.html "struct bevy::prelude::AnimatableKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/axis.rs.html#16)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Axis](struct.Axis.html "struct bevy::prelude::Axis")<T>

where [Axis](struct.Axis.html "struct bevy::prelude::Axis")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<T, [f32](https://doc.rust-lang.org/nightly/std/primitive.f32.html)\>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/button_input.rs.html#124)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ButtonInput](struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>

where T: [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync") + 'static + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [ButtonInput](struct.ButtonInput.html "struct bevy::prelude::ButtonInput")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), [HashSet](../platform/collections/struct.HashSet.html "struct bevy::platform::collections::HashSet")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#467)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ChunkedUnevenCore](cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>

where [ChunkedUnevenCore](cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_color/0.19.0/x86_64-unknown-linux-gnu/src/bevy_color/color_gradient.rs.html#11)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ColorCurve](../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>

where [ColorCurve](../color/struct.ColorCurve.html "struct bevy::color::ColorCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [EvenCore](struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/adaptors.rs.html#46)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ConstantCurve](struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>

where [ConstantCurve](struct.ConstantCurve.html "struct bevy::prelude::ConstantCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#50)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [CubicKeyframeCurve](../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>

where [CubicKeyframeCurve](../animation/gltf_curves/struct.CubicKeyframeCurve.html "struct bevy::animation::gltf_curves::CubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/easing.rs.html#298)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [EasingCurve](struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>

where [EasingCurve](struct.EasingCurve.html "struct bevy::prelude::EasingCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#122)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [EvenCore](struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>

where [EvenCore](struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/maybe_location.rs.html#20)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [MaybeLocation](../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>

where [MaybeLocation](../ecs/change_detection/struct.MaybeLocation.html "struct bevy::ecs::change_detection::MaybeLocation")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#139)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SampleAutoCurve](struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>

where [SampleAutoCurve](struct.SampleAutoCurve.html "struct bevy::prelude::SampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [EvenCore](struct.EvenCore.html "struct bevy::prelude::EvenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#12)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [SteppedKeyframeCurve](../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>

where [SteppedKeyframeCurve](../animation/gltf_curves/struct.SteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::SteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/time.rs.html#191)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [Time](struct.Time.html "struct bevy::prelude::Time")<T>

where T: [Default](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, [Time](struct.Time.html "struct bevy::prelude::Time")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/cores.rs.html#326)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UnevenCore](struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>

where [UnevenCore](struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [Vec](struct.Vec.html "struct bevy::prelude::Vec")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/curve/sample_curves.rs.html#314)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [UnevenSampleAutoCurve](struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>

where [UnevenSampleAutoCurve](struct.UnevenSampleAutoCurve.html "struct bevy::prelude::UnevenSampleAutoCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [UnevenCore](struct.UnevenCore.html "struct bevy::prelude::UnevenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_ui_widgets/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui_widgets/lib.rs.html#88)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [ValueChange](../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>

where [ValueChange](../ui_widgets/struct.ValueChange.html "struct bevy::ui_widgets::ValueChange")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/virtual_keyboard.rs.html#93)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [VirtualKeyPressed](../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>

where [VirtualKeyPressed](../feathers/controls/struct.VirtualKeyPressed.html "struct bevy::feathers::controls::VirtualKeyPressed")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#285)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WideCubicKeyframeCurve](../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>

where [WideCubicKeyframeCurve](../animation/gltf_curves/struct.WideCubicKeyframeCurve.html "struct bevy::animation::gltf_curves::WideCubicKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#174)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WideLinearKeyframeCurve](../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>

where [WideLinearKeyframeCurve](../animation/gltf_curves/struct.WideLinearKeyframeCurve.html "struct bevy::animation::gltf_curves::WideLinearKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_animation/0.19.0/x86_64-unknown-linux-gnu/src/bevy_animation/gltf_curves.rs.html#228)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WideSteppedKeyframeCurve](../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>

where [WideSteppedKeyframeCurve](../animation/gltf_curves/struct.WideSteppedKeyframeCurve.html "struct bevy::animation::gltf_curves::WideSteppedKeyframeCurve")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath"), [ChunkedUnevenCore](cores/struct.ChunkedUnevenCore.html "struct bevy::prelude::cores::ChunkedUnevenCore")<T>: [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#602)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WithDerivative](../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>

where [WithDerivative](../math/struct.WithDerivative.html "struct bevy::math::WithDerivative")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, <T as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

[Source](https://docs.rs/bevy_math/0.19.0/x86_64-unknown-linux-gnu/src/bevy_math/common_traits.rs.html#617)

### impl<T> [Struct](trait.Struct.html "trait bevy::prelude::Struct") for [WithTwoDerivatives](../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>

where [WithTwoDerivatives](../math/struct.WithTwoDerivatives.html "struct bevy::math::WithTwoDerivatives")<T>: [Any](https://doc.rust-lang.org/nightly/core/any/trait.Any.html "trait core::any::Any") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"), T: [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + MaybeTyped + RegisterForReflection, <T as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection, <<T as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent") as [HasTangent](../math/trait.HasTangent.html "trait bevy::math::HasTangent")\>::[Tangent](../math/trait.HasTangent.html#associatedtype.Tangent "type bevy::math::HasTangent::Tangent"): [FromReflect](trait.FromReflect.html "trait bevy::prelude::FromReflect") + [TypePath](trait.TypePath.html "trait bevy::prelude::TypePath") + MaybeTyped + RegisterForReflection,

{"FieldIter<'\_>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"../reflect/structs/struct.FieldIter.html\\" title=\\"struct bevy::reflect::structs::FieldIter\\">FieldIter</a>&lt;'a&gt;</div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = (&amp;'a <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.str.html\\">str</a>, &amp;'a (dyn <a class=\\"trait\\" href=\\"trait.PartialReflect.html\\" title=\\"trait bevy::prelude::PartialReflect\\">PartialReflect</a> + 'static));</div>"}
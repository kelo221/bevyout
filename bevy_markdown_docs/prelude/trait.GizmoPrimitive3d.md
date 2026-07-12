[bevy](../index.html)::[prelude](index.html)

# Trait GizmoPrimitive3d 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#21)

```rust
pub trait GizmoPrimitive3d<P>where
    P: Primitive3d,{
    type Output<'a>
       where Self: 'a;

    // Required method
    fn primitive_3d(
        &mut self,
        primitive: &P,
        isometry: impl Into<Isometry3d>,
        color: impl Into<Color>,
    ) -> Self::Output<'_>;
}
```

A trait for rendering 3D geometric primitives (`P`) with [`GizmoBuffer`](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer").

## Required Associated Types

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#23)

#### type [Output](#associatedtype.Output)<'a> where Self: 'a

The output of `primitive_3d`. This is a builder to set non-default values.

## Required Methods

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#28-33)

#### fn [primitive\_3d](#tymethod.primitive_3d)( &mut self, primitive: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry3d](struct.Isometry3d.html "struct bevy::prelude::Isometry3d")\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](enum.Color.html "enum bevy::prelude::Color")\>, ) -> Self::[Output](trait.GizmoPrimitive3d.html#associatedtype.Output "type bevy::prelude::GizmoPrimitive3d::Output")<'\_>

Renders a 3D primitive with its associated details.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#477-480)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Capsule3d](struct.Capsule3d.html "struct bevy::prelude::Capsule3d")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#482)

#### type [Output](#associatedtype.Output)<'a> = [Capsule3dBuilder](../gizmos/primitives/dim3/struct.Capsule3dBuilder.html "struct bevy::gizmos::primitives::dim3::Capsule3dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#617-620)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Cone](struct.Cone.html "struct bevy::prelude::Cone")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#622)

#### type [Output](#associatedtype.Output)<'a> = [Cone3dBuilder](../gizmos/primitives/dim3/struct.Cone3dBuilder.html "struct bevy::gizmos::primitives::dim3::Cone3dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#716-719)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[ConicalFrustum](struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#721)

#### type [Output](#associatedtype.Output)<'a> = [ConicalFrustum3dBuilder](../gizmos/primitives/dim3/struct.ConicalFrustum3dBuilder.html "struct bevy::gizmos::primitives::dim3::ConicalFrustum3dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#296-299)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Cuboid](struct.Cuboid.html "struct bevy::prelude::Cuboid")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#301)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#391-394)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Cylinder](struct.Cylinder.html "struct bevy::prelude::Cylinder")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#396)

#### type [Output](#associatedtype.Output)<'a> = [Cylinder3dBuilder](../gizmos/primitives/dim3/struct.Cylinder3dBuilder.html "struct bevy::gizmos::primitives::dim3::Cylinder3dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#38-41)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Dir3](struct.Dir3.html "struct bevy::prelude::Dir3")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#43)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#176-179)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Line3d](struct.Line3d.html "struct bevy::prelude::Line3d")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#181)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#124-127)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Plane3d](struct.Plane3d.html "struct bevy::prelude::Plane3d")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#129)

#### type [Output](#associatedtype.Output)<'a> = [Plane3dBuilder](../gizmos/primitives/dim3/struct.Plane3dBuilder.html "struct bevy::gizmos::primitives::dim3::Plane3dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#238-241)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Polyline3d](struct.Polyline3d.html "struct bevy::prelude::Polyline3d")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#243)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#211-214)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Segment3d](struct.Segment3d.html "struct bevy::prelude::Segment3d")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#216)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#63-66)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Sphere](struct.Sphere.html "struct bevy::prelude::Sphere")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#68)

#### type [Output](#associatedtype.Output)<'a> = [SphereBuilder](../gizmos/circles/struct.SphereBuilder.html "struct bevy::gizmos::circles::SphereBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#900-903)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Tetrahedron](struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#905)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#820-823)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Torus](struct.Torus.html "struct bevy::prelude::Torus")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#825)

#### type [Output](#associatedtype.Output)<'a> = [Torus3dBuilder](../gizmos/primitives/dim3/struct.Torus3dBuilder.html "struct bevy::gizmos::primitives::dim3::Torus3dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#268-271)

### impl<Config, Clear> [GizmoPrimitive3d](trait.GizmoPrimitive3d.html "trait bevy::prelude::GizmoPrimitive3d")<[Triangle3d](struct.Triangle3d.html "struct bevy::prelude::Triangle3d")\> for [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim3.rs.html#273)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a
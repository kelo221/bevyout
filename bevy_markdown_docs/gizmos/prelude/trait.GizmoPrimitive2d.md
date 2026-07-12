[bevy](../../index.html)::[gizmos](../index.html)::[prelude](index.html)

# Trait GizmoPrimitive2d 

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#26)

```rust
pub trait GizmoPrimitive2d<P>where
    P: Primitive2d,{
    type Output<'a>
       where Self: 'a;

    // Required method
    fn primitive_2d(
        &mut self,
        primitive: &P,
        isometry: impl Into<Isometry2d>,
        color: impl Into<Color>,
    ) -> Self::Output<'_>;
}
```

A trait for rendering 2D geometric primitives (`P`) with [`GizmoBuffer`](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer").

## Required Associated Types

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#28)

#### type [Output](#associatedtype.Output)<'a> where Self: 'a

The output of `primitive_2d`. This is a builder to set non-default values.

## Required Methods

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#33-38)

#### fn [primitive\_2d](#tymethod.primitive_2d)( &mut self, primitive: [&P](https://doc.rust-lang.org/nightly/std/primitive.reference.html), isometry: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Isometry2d](../../prelude/struct.Isometry2d.html "struct bevy::prelude::Isometry2d")\>, color: impl [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Color](../../prelude/enum.Color.html "enum bevy::prelude::Color")\>, ) -> Self::[Output](../../prelude/trait.GizmoPrimitive2d.html#associatedtype.Output "type bevy::prelude::GizmoPrimitive2d::Output")<'\_>

Renders a 2D primitive with its associated details.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#274-277)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Annulus](../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#279)

#### type [Output](#associatedtype.Output)<'a> = [Annulus2dBuilder](../primitives/dim2/struct.Annulus2dBuilder.html "struct bevy::gizmos::primitives::dim2::Annulus2dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#71-74)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Arc2d](../../prelude/struct.Arc2d.html "struct bevy::prelude::Arc2d")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#76)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#368-371)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Capsule2d](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#373)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#105-108)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Circle](../../prelude/struct.Circle.html "struct bevy::prelude::Circle")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#110)

#### type [Output](#associatedtype.Output)<'a> = [Ellipse2dBuilder](../circles/struct.Ellipse2dBuilder.html "struct bevy::gizmos::circles::Ellipse2dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#127-130)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[CircularSector](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#132)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#170-173)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[CircularSegment](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#175)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#43-46)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Dir2](../../prelude/struct.Dir2.html "struct bevy::prelude::Dir2")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#48)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#212-215)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Ellipse](../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#217)

#### type [Output](#associatedtype.Output)<'a> = [Ellipse2dBuilder](../circles/struct.Ellipse2dBuilder.html "struct bevy::gizmos::circles::Ellipse2dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#461-464)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Line2d](../../prelude/struct.Line2d.html "struct bevy::prelude::Line2d")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#466)

#### type [Output](#associatedtype.Output)<'a> = [Line2dBuilder](../primitives/dim2/struct.Line2dBuilder.html "struct bevy::gizmos::primitives::dim2::Line2dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#519-522)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Plane2d](../../prelude/struct.Plane2d.html "struct bevy::prelude::Plane2d")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#524)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#752-755)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Polygon](../../prelude/struct.Polygon.html "struct bevy::prelude::Polygon")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#757)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#651-654)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Polyline2d](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#656)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#716-719)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Rectangle](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#721)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#787-790)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[RegularPolygon](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#792)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#334-337)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Rhombus](../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#339)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#598-601)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Segment2d](../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#603)

#### type [Output](#associatedtype.Output)<'a> = [Segment2dBuilder](../primitives/dim2/struct.Segment2dBuilder.html "struct bevy::gizmos::primitives::dim2::Segment2dBuilder")<'a, Config, Clear> where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#686-689)

### impl<Config, Clear> [GizmoPrimitive2d](../../prelude/trait.GizmoPrimitive2d.html "trait bevy::prelude::GizmoPrimitive2d")<[Triangle2d](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")\> for [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>

where Config: [GizmoConfigGroup](../../prelude/trait.GizmoConfigGroup.html "trait bevy::prelude::GizmoConfigGroup"), Clear: 'static + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + [Sync](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync"),

[Source](https://docs.rs/bevy_gizmos/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gizmos/primitives/dim2.rs.html#691)

#### type [Output](#associatedtype.Output)<'a> = [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) where [GizmoBuffer](../gizmos/struct.GizmoBuffer.html "struct bevy::gizmos::gizmos::GizmoBuffer")<Config, Clear>: 'a
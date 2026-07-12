[bevy](../../index.html)::[mesh](../index.html)::[prelude](index.html)

# Trait Meshable 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/mod.rs.html#34)

```rust
pub trait Meshable {
    type Output: MeshBuilder;

    // Required method
    fn mesh(&self) -> Self::Output;
}
```

A trait for shapes that can be turned into a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

## Required Associated Types

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/mod.rs.html#36)

#### type [Output](#associatedtype.Output): [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder")

The output of [`Self::mesh`](../../prelude/trait.Meshable.html#tymethod.mesh "method bevy::prelude::Meshable::mesh"). This will be a [`MeshBuilder`](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

## Required Methods

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/mod.rs.html#39)

#### fn [mesh](#tymethod.mesh)(&self) -> Self::[Output](../../prelude/trait.Meshable.html#associatedtype.Output "type bevy::prelude::Meshable::Output")

Creates a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") for a shape.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#860)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Annulus](../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#861)

#### type [Output](#associatedtype.Output) = [AnnulusMeshBuilder](../struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1264)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Capsule2d](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1265)

#### type [Output](#associatedtype.Output) = [Capsule2dMeshBuilder](../struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#421)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Capsule3d](../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#422)

#### type [Output](#associatedtype.Output) = [Capsule3dMeshBuilder](../struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#80)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Circle](../../prelude/struct.Circle.html "struct bevy::prelude::Circle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#81)

#### type [Output](#associatedtype.Output) = [CircleMeshBuilder](../struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#242)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [CircularSector](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#243)

#### type [Output](#associatedtype.Output) = [CircularSectorMeshBuilder](../struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#389)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [CircularSegment](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#390)

#### type [Output](#associatedtype.Output) = [CircularSegmentMeshBuilder](../struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#174)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Cone](../../prelude/struct.Cone.html "struct bevy::prelude::Cone")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#175)

#### type [Output](#associatedtype.Output) = [ConeMeshBuilder](../struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#169)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [ConicalFrustum](../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#170)

#### type [Output](#associatedtype.Output) = [ConicalFrustumMeshBuilder](../struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#419)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [ConvexPolygon](../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#420)

#### type [Output](#associatedtype.Output) = [ConvexPolygonMeshBuilder](../struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#85)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Cuboid](../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#86)

#### type [Output](#associatedtype.Output) = [CuboidMeshBuilder](../struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#207)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Cylinder](../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#208)

#### type [Output](#associatedtype.Output) = [CylinderMeshBuilder](../struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#643)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Ellipse](../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#644)

#### type [Output](#associatedtype.Output) = [EllipseMeshBuilder](../struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#190)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Plane3d](../../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#191)

#### type [Output](#associatedtype.Output) = [PlaneMeshBuilder](../struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#728)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Polyline2d](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#729)

#### type [Output](#associatedtype.Output) = [Polyline2dMeshBuilder](../struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/polyline3d.rs.html#29)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Polyline3d](../../prelude/struct.Polyline3d.html "struct bevy::prelude::Polyline3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/polyline3d.rs.html#30)

#### type [Output](#associatedtype.Output) = Polyline3dMeshBuilder

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1104)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Rectangle](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1105)

#### type [Output](#associatedtype.Output) = [RectangleMeshBuilder](../struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#520)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [RegularPolygon](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#521)

#### type [Output](#associatedtype.Output) = [RegularPolygonMeshBuilder](../struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#947)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Rhombus](../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#948)

#### type [Output](#associatedtype.Output) = [RhombusMeshBuilder](../struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#685)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Segment2d](../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#686)

#### type [Output](#associatedtype.Output) = [Segment2dMeshBuilder](../struct.Segment2dMeshBuilder.html "struct bevy::mesh::Segment2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/segment3d.rs.html#24)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Segment3d](../../prelude/struct.Segment3d.html "struct bevy::prelude::Segment3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/segment3d.rs.html#25)

#### type [Output](#associatedtype.Output) = Segment3dMeshBuilder

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#253)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Sphere](../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#254)

#### type [Output](#associatedtype.Output) = [SphereMeshBuilder](../struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#54)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Tetrahedron](../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#55)

#### type [Output](#associatedtype.Output) = [TetrahedronMeshBuilder](../struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#161)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Torus](../../prelude/struct.Torus.html "struct bevy::prelude::Torus")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#162)

#### type [Output](#associatedtype.Output) = [TorusMeshBuilder](../struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#979)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Triangle2d](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#980)

#### type [Output](#associatedtype.Output) = [Triangle2dMeshBuilder](../struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#35)

### impl [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Triangle3d](../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#36)

#### type [Output](#associatedtype.Output) = [Triangle3dMeshBuilder](../struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/extrusion.rs.html#88-91)

### impl<P> [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Extrusion](../../prelude/struct.Extrusion.html "struct bevy::prelude::Extrusion")<P>

where P: [Primitive2d](../../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable"), <P as [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable")\>::[Output](../../prelude/trait.Meshable.html#associatedtype.Output "type bevy::prelude::Meshable::Output"): [Extrudable](../trait.Extrudable.html "trait bevy::mesh::Extrudable"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/extrusion.rs.html#93)

#### type [Output](#associatedtype.Output) = [ExtrusionBuilder](../struct.ExtrusionBuilder.html "struct bevy::mesh::ExtrusionBuilder")<P>

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1524-1526)

### impl<P> [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") for [Ring](../../prelude/struct.Ring.html "struct bevy::prelude::Ring")<P>

where P: [Primitive2d](../../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1528)

#### type [Output](#associatedtype.Output) = [RingMeshBuilder](../struct.RingMeshBuilder.html "struct bevy::mesh::RingMeshBuilder")<P>
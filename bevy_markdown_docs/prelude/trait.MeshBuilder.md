[bevy](../index.html)::[prelude](index.html)

# Trait MeshBuilder 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/mod.rs.html#43)

```rust
pub trait MeshBuilder {
    // Required method
    fn build(&self) -> Mesh;
}
```

A trait used to build [`Mesh`](struct.Mesh.html "struct bevy::prelude::Mesh")es from a configuration

## Required Methods

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/mod.rs.html#45)

#### fn [build](#tymethod.build)(&self) -> [Mesh](struct.Mesh.html "struct bevy::prelude::Mesh")

Builds a [`Mesh`](struct.Mesh.html "struct bevy::prelude::Mesh") based on the configuration in `self`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#783)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [AnnulusMeshBuilder](../mesh/struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1162)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Capsule2dMeshBuilder](../mesh/struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#96)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Capsule3dMeshBuilder](../mesh/struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#61)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CircleMeshBuilder](../mesh/struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#177)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CircularSectorMeshBuilder](../mesh/struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#315)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CircularSegmentMeshBuilder](../mesh/struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#71)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ConeMeshBuilder](../mesh/struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#63)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ConicalFrustumMeshBuilder](../mesh/struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#429)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ConvexPolygonMeshBuilder](../mesh/struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#22)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CuboidMeshBuilder](../mesh/struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#96)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CylinderMeshBuilder](../mesh/struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#595)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [EllipseMeshBuilder](../mesh/struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#141)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [PlaneMeshBuilder](../mesh/struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#707)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Polyline2dMeshBuilder](../mesh/struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1072)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RectangleMeshBuilder](../mesh/struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#531)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RegularPolygonMeshBuilder](../mesh/struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#915)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RhombusMeshBuilder](../mesh/struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#674)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Segment2dMeshBuilder](../mesh/struct.Segment2dMeshBuilder.html "struct bevy::mesh::Segment2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#238)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [SphereMeshBuilder](../mesh/struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#14)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [TetrahedronMeshBuilder](../mesh/struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#79)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [TorusMeshBuilder](../mesh/struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#987)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Triangle2dMeshBuilder](../mesh/struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#13)

### impl [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Triangle3dMeshBuilder](../mesh/struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/extrusion.rs.html#175-178)

### impl<P> [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ExtrusionBuilder](../mesh/struct.ExtrusionBuilder.html "struct bevy::mesh::ExtrusionBuilder")<P>

where P: [Primitive2d](trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Meshable](trait.Meshable.html "trait bevy::prelude::Meshable"), <P as [Meshable](trait.Meshable.html "trait bevy::prelude::Meshable")\>::[Output](trait.Meshable.html#associatedtype.Output "type bevy::prelude::Meshable::Output"): [Extrudable](../mesh/trait.Extrudable.html "trait bevy::mesh::Extrudable"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1374-1376)

### impl<P> [MeshBuilder](trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RingMeshBuilder](../mesh/struct.RingMeshBuilder.html "struct bevy::mesh::RingMeshBuilder")<P>

where P: [Primitive2d](trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Meshable](trait.Meshable.html "trait bevy::prelude::Meshable"),
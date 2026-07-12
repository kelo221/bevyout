[bevy](../../index.html)::[mesh](../index.html)::[prelude](index.html)

# Trait MeshBuilder 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/mod.rs.html#43)

```rust
pub trait MeshBuilder {
    // Required method
    fn build(&self) -> Mesh;
}
```

A trait used to build [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")es from a configuration

## Required Methods

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/mod.rs.html#45)

#### fn [build](#tymethod.build)(&self) -> [Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")

Builds a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") based on the configuration in `self`.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#783)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [AnnulusMeshBuilder](../struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1162)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Capsule2dMeshBuilder](../struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/capsule.rs.html#96)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Capsule3dMeshBuilder](../struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#61)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CircleMeshBuilder](../struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#177)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CircularSectorMeshBuilder](../struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#315)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CircularSegmentMeshBuilder](../struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cone.rs.html#71)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ConeMeshBuilder](../struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/conical_frustum.rs.html#63)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ConicalFrustumMeshBuilder](../struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#429)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ConvexPolygonMeshBuilder](../struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cuboid.rs.html#22)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CuboidMeshBuilder](../struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/cylinder.rs.html#96)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [CylinderMeshBuilder](../struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#595)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [EllipseMeshBuilder](../struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/plane.rs.html#141)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [PlaneMeshBuilder](../struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#707)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Polyline2dMeshBuilder](../struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1072)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RectangleMeshBuilder](../struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#531)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RegularPolygonMeshBuilder](../struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#915)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RhombusMeshBuilder](../struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#674)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Segment2dMeshBuilder](../struct.Segment2dMeshBuilder.html "struct bevy::mesh::Segment2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/sphere.rs.html#238)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [SphereMeshBuilder](../struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/tetrahedron.rs.html#14)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [TetrahedronMeshBuilder](../struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/torus.rs.html#79)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [TorusMeshBuilder](../struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#987)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Triangle2dMeshBuilder](../struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim3/triangle3d.rs.html#13)

### impl [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [Triangle3dMeshBuilder](../struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/extrusion.rs.html#175-178)

### impl<P> [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [ExtrusionBuilder](../struct.ExtrusionBuilder.html "struct bevy::mesh::ExtrusionBuilder")<P>

where P: [Primitive2d](../../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable"), <P as [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable")\>::[Output](../../prelude/trait.Meshable.html#associatedtype.Output "type bevy::prelude::Meshable::Output"): [Extrudable](../trait.Extrudable.html "trait bevy::mesh::Extrudable"),

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1374-1376)

### impl<P> [MeshBuilder](../../prelude/trait.MeshBuilder.html "trait bevy::prelude::MeshBuilder") for [RingMeshBuilder](../struct.RingMeshBuilder.html "struct bevy::mesh::RingMeshBuilder")<P>

where P: [Primitive2d](../../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Meshable](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable"),
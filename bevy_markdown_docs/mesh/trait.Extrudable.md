[bevy](../index.html)::[mesh](index.html)

# Trait Extrudable 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/extrusion.rs.html#83)

```rust
pub trait Extrudable: MeshBuilder {
    // Required method
    fn perimeter(&self) -> Vec<PerimeterSegment>;
}
```

A trait required for implementing `Meshable` for `Extrusion<T>`.

### Warning

By implementing this trait you guarantee that the `primitive_topology` of the mesh returned by this builder is [`PrimitiveTopology::TriangleList`](enum.PrimitiveTopology.html#variant.TriangleList "variant bevy::mesh::PrimitiveTopology::TriangleList") and that your mesh has a [`Mesh::ATTRIBUTE_POSITION`](../prelude/struct.Mesh.html#associatedconstant.ATTRIBUTE_POSITION "associated constant bevy::prelude::Mesh::ATTRIBUTE_POSITION") attribute.

## Required Methods

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/extrusion.rs.html#85)

#### fn [perimeter](#tymethod.perimeter)(&self) -> [Vec](../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[PerimeterSegment](enum.PerimeterSegment.html "enum bevy::mesh::PerimeterSegment")\>

A list of the indices each representing a part of the perimeter of the mesh.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#842)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [AnnulusMeshBuilder](struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1238)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [Capsule2dMeshBuilder](struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#70)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [CircleMeshBuilder](struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#224)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [CircularSectorMeshBuilder](struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#371)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [CircularSegmentMeshBuilder](struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#467)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [ConvexPolygonMeshBuilder](struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#633)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [EllipseMeshBuilder](struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1096)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [RectangleMeshBuilder](struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#541)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [RegularPolygonMeshBuilder](struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#939)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [RhombusMeshBuilder](struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1019)

### impl [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [Triangle2dMeshBuilder](struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/primitives/dim2.rs.html#1458-1461)

### impl<P> [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable") for [RingMeshBuilder](struct.RingMeshBuilder.html "struct bevy::mesh::RingMeshBuilder")<P>

where P: [Primitive2d](../prelude/trait.Primitive2d.html "trait bevy::prelude::Primitive2d") + [Meshable](../prelude/trait.Meshable.html "trait bevy::prelude::Meshable"), <P as [Meshable](../prelude/trait.Meshable.html "trait bevy::prelude::Meshable")\>::[Output](../prelude/trait.Meshable.html#associatedtype.Output "type bevy::prelude::Meshable::Output"): [Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable"),
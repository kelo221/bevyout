[bevy](../../index.html)::[mesh](../index.html)

# Module primitives 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/lib.rs.html#14)

Mesh generation for [primitive shapes](../../math/primitives/index.html "mod bevy::math::primitives").

Primitives that support meshing implement the [`Meshable`](../../prelude/trait.Meshable.html "trait bevy::prelude::Meshable") trait. Calling [`mesh`](../../prelude/trait.Meshable.html#tymethod.mesh "method bevy::prelude::Meshable::mesh") will return either a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") or a builder that can be used to specify shape-specific configuration for creating the [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

```rust
// Create circle mesh with default configuration
let circle = meshes.add(Circle { radius: 25.0 });

// Specify number of vertices
let circle = meshes.add(Circle { radius: 25.0 }.mesh().resolution(64));
```

## Structs

[AnnulusMeshBuilder](struct.AnnulusMeshBuilder.html "struct bevy::mesh::primitives::AnnulusMeshBuilder")

A builder for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Annulus`](../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus") shape.

[Capsule2dMeshBuilder](struct.Capsule2dMeshBuilder.html "struct bevy::mesh::primitives::Capsule2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Capsule2d`](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d") shape.

[Capsule3dMeshBuilder](struct.Capsule3dMeshBuilder.html "struct bevy::mesh::primitives::Capsule3dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Capsule3d`](../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d") shape.

[CircleMeshBuilder](struct.CircleMeshBuilder.html "struct bevy::mesh::primitives::CircleMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Circle`](../../prelude/struct.Circle.html "struct bevy::prelude::Circle") shape.

[CircularSectorMeshBuilder](struct.CircularSectorMeshBuilder.html "struct bevy::mesh::primitives::CircularSectorMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`CircularSector`](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector") shape.

[CircularSegmentMeshBuilder](struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::primitives::CircularSegmentMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`CircularSegment`](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment") shape.

[ConeMeshBuilder](struct.ConeMeshBuilder.html "struct bevy::mesh::primitives::ConeMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cone`](../../prelude/struct.Cone.html "struct bevy::prelude::Cone") shape.

[ConicalFrustumMeshBuilder](struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::primitives::ConicalFrustumMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`ConicalFrustum`](../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum") shape.

[ConvexPolygonMeshBuilder](struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::primitives::ConvexPolygonMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`ConvexPolygon`](../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon") shape.

[CuboidMeshBuilder](struct.CuboidMeshBuilder.html "struct bevy::mesh::primitives::CuboidMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cuboid`](../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid") shape.

[CylinderMeshBuilder](struct.CylinderMeshBuilder.html "struct bevy::mesh::primitives::CylinderMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cylinder`](../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder") shape.

[EllipseMeshBuilder](struct.EllipseMeshBuilder.html "struct bevy::mesh::primitives::EllipseMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Ellipse`](../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse") shape.

[ExtrusionBuilder](struct.ExtrusionBuilder.html "struct bevy::mesh::primitives::ExtrusionBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Extrusion`](../../prelude/struct.Extrusion.html "struct bevy::prelude::Extrusion") shape.

[PlaneMeshBuilder](struct.PlaneMeshBuilder.html "struct bevy::mesh::primitives::PlaneMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Plane3d`](../../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d") shape.

[Polyline2dMeshBuilder](struct.Polyline2dMeshBuilder.html "struct bevy::mesh::primitives::Polyline2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Polyline2d`](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d") shape.

[RectangleMeshBuilder](struct.RectangleMeshBuilder.html "struct bevy::mesh::primitives::RectangleMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Rectangle`](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle") shape.

[RegularPolygonMeshBuilder](struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::primitives::RegularPolygonMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`RegularPolygon`](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon") shape.

[RhombusMeshBuilder](struct.RhombusMeshBuilder.html "struct bevy::mesh::primitives::RhombusMeshBuilder")

A builder for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Rhombus`](../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus") shape.

[RingMeshBuilder](struct.RingMeshBuilder.html "struct bevy::mesh::primitives::RingMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Ring`](../../prelude/struct.Ring.html "struct bevy::prelude::Ring") shape.

[Segment2dMeshBuilder](struct.Segment2dMeshBuilder.html "struct bevy::mesh::primitives::Segment2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Segment2d`](../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d").

[SphereMeshBuilder](struct.SphereMeshBuilder.html "struct bevy::mesh::primitives::SphereMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Sphere`](../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere") shape.

[TetrahedronMeshBuilder](struct.TetrahedronMeshBuilder.html "struct bevy::mesh::primitives::TetrahedronMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Tetrahedron`](../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron") shape.

[TorusMeshBuilder](struct.TorusMeshBuilder.html "struct bevy::mesh::primitives::TorusMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Torus`](../../prelude/struct.Torus.html "struct bevy::prelude::Torus") shape.

[Triangle2dMeshBuilder](struct.Triangle2dMeshBuilder.html "struct bevy::mesh::primitives::Triangle2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Triangle2d`](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d") shape.

[Triangle3dMeshBuilder](struct.Triangle3dMeshBuilder.html "struct bevy::mesh::primitives::Triangle3dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Triangle3d`](../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d") shape.

## Enums

[CapsuleUvProfile](enum.CapsuleUvProfile.html "enum bevy::mesh::primitives::CapsuleUvProfile")

Manner in which UV coordinates are distributed vertically.

[CircularMeshUvMode](enum.CircularMeshUvMode.html "enum bevy::mesh::primitives::CircularMeshUvMode")

Specifies how to generate UV-mappings for the [`CircularSector`](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector") and [`CircularSegment`](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment") shapes.

[ConeAnchor](enum.ConeAnchor.html "enum bevy::mesh::primitives::ConeAnchor")

Anchoring options for [`ConeMeshBuilder`](../struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[CylinderAnchor](enum.CylinderAnchor.html "enum bevy::mesh::primitives::CylinderAnchor")

Anchoring options for [`CylinderMeshBuilder`](../struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[IcosphereError](enum.IcosphereError.html "enum bevy::mesh::primitives::IcosphereError")

An error when creating an icosphere [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") from a [`SphereMeshBuilder`](../struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder").

[PerimeterSegment](enum.PerimeterSegment.html "enum bevy::mesh::primitives::PerimeterSegment")

A type representing a segment of the perimeter of an extrudable mesh.

[SphereKind](enum.SphereKind.html "enum bevy::mesh::primitives::SphereKind")

A type of sphere mesh.

## Traits

[Extrudable](trait.Extrudable.html "trait bevy::mesh::primitives::Extrudable")

A trait required for implementing `Meshable` for `Extrusion<T>`.

[MeshBuilder](trait.MeshBuilder.html "trait bevy::mesh::primitives::MeshBuilder")

A trait used to build [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")es from a configuration

[Meshable](trait.Meshable.html "trait bevy::mesh::primitives::Meshable")

A trait for shapes that can be turned into a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").
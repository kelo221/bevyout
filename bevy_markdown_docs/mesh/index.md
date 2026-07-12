[bevy](../index.html)

# Crate mesh 

[Source](https://docs.rs/bevy_mesh/0.19.0/x86_64-unknown-linux-gnu/src/bevy_mesh/lib.rs.html#1-122)

## Modules

[morph](morph/index.html "mod bevy::mesh::morph")`morph`

[prelude](prelude/index.html "mod bevy::mesh::prelude")

The mesh prelude.

[primitives](primitives/index.html "mod bevy::mesh::primitives")

Mesh generation for [primitive shapes](../math/primitives/index.html "mod bevy::math::primitives").

[skinning](skinning/index.html "mod bevy::mesh::skinning")

## Structs

[AnnulusMeshBuilder](struct.AnnulusMeshBuilder.html "struct bevy::mesh::AnnulusMeshBuilder")

A builder for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Annulus`](../prelude/struct.Annulus.html "struct bevy::prelude::Annulus") shape.

[BaseMeshPipelineKey](struct.BaseMeshPipelineKey.html "struct bevy::mesh::BaseMeshPipelineKey")

Our base mesh pipeline key bits start from the highest bit and go downward. The PBR mesh pipeline key bits start from the lowest bit and go upward. This allows the PBR bits in the downstream crate `bevy_pbr` to coexist in the same field without any shifts.

[Capsule2dMeshBuilder](struct.Capsule2dMeshBuilder.html "struct bevy::mesh::Capsule2dMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Capsule2d`](../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d") shape.

[Capsule3dMeshBuilder](struct.Capsule3dMeshBuilder.html "struct bevy::mesh::Capsule3dMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Capsule3d`](../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d") shape.

[CircleMeshBuilder](struct.CircleMeshBuilder.html "struct bevy::mesh::CircleMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Circle`](../prelude/struct.Circle.html "struct bevy::prelude::Circle") shape.

[CircularSectorMeshBuilder](struct.CircularSectorMeshBuilder.html "struct bevy::mesh::CircularSectorMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`CircularSector`](../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector") shape.

[CircularSegmentMeshBuilder](struct.CircularSegmentMeshBuilder.html "struct bevy::mesh::CircularSegmentMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`CircularSegment`](../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment") shape.

[ConeMeshBuilder](struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cone`](../prelude/struct.Cone.html "struct bevy::prelude::Cone") shape.

[ConicalFrustumMeshBuilder](struct.ConicalFrustumMeshBuilder.html "struct bevy::mesh::ConicalFrustumMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`ConicalFrustum`](../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum") shape.

[ConvexPolygonMeshBuilder](struct.ConvexPolygonMeshBuilder.html "struct bevy::mesh::ConvexPolygonMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`ConvexPolygon`](../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon") shape.

[CuboidMeshBuilder](struct.CuboidMeshBuilder.html "struct bevy::mesh::CuboidMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cuboid`](../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid") shape.

[CylinderMeshBuilder](struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cylinder`](../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder") shape.

[EllipseMeshBuilder](struct.EllipseMeshBuilder.html "struct bevy::mesh::EllipseMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Ellipse`](../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse") shape.

[ExtrusionBuilder](struct.ExtrusionBuilder.html "struct bevy::mesh::ExtrusionBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Extrusion`](../prelude/struct.Extrusion.html "struct bevy::prelude::Extrusion") shape.

[Mesh](struct.Mesh.html "struct bevy::mesh::Mesh")

A 3D object made out of vertices representing triangles, lines, or points, with “attribute” values for each vertex.

[Mesh2d](struct.Mesh2d.html "struct bevy::mesh::Mesh2d")

A component for 2D meshes. Requires a [`MeshMaterial2d`](https://docs.rs/bevy/latest/bevy/prelude/struct.MeshMaterial2d.html) to be rendered, commonly using a [`ColorMaterial`](https://docs.rs/bevy/latest/bevy/prelude/struct.ColorMaterial.html).

[Mesh2dTemplate](struct.Mesh2dTemplate.html "struct bevy::mesh::Mesh2dTemplate")

[Mesh3d](struct.Mesh3d.html "struct bevy::mesh::Mesh3d")

A component for 3D meshes. Requires a [`MeshMaterial3d`](https://docs.rs/bevy/latest/bevy/pbr/struct.MeshMaterial3d.html) to be rendered, commonly using a [`StandardMaterial`](https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html).

[Mesh3dTemplate](struct.Mesh3dTemplate.html "struct bevy::mesh::Mesh3dTemplate")

[MeshDeserializer](struct.MeshDeserializer.html "struct bevy::mesh::MeshDeserializer")`serialize`

Use to specify extra options when deserializing a [`SerializedMesh`](struct.SerializedMesh.html "struct bevy::mesh::SerializedMesh") into a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

[MeshPlugin](struct.MeshPlugin.html "struct bevy::mesh::MeshPlugin")

Adds [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") as an asset.

[MeshTag](struct.MeshTag.html "struct bevy::mesh::MeshTag")

A component that stores an arbitrary index used to identify the mesh instance when rendering.

[MeshVertexAttribute](struct.MeshVertexAttribute.html "struct bevy::mesh::MeshVertexAttribute")

[MeshVertexAttributeId](struct.MeshVertexAttributeId.html "struct bevy::mesh::MeshVertexAttributeId")

[MeshVertexBufferLayout](struct.MeshVertexBufferLayout.html "struct bevy::mesh::MeshVertexBufferLayout")

[MeshVertexBufferLayoutRef](struct.MeshVertexBufferLayoutRef.html "struct bevy::mesh::MeshVertexBufferLayoutRef")

Describes the layout of the mesh vertices in GPU memory.

[MeshVertexBufferLayouts](struct.MeshVertexBufferLayouts.html "struct bevy::mesh::MeshVertexBufferLayouts")

Stores the single copy of each mesh vertex buffer layout.

[MissingVertexAttributeError](struct.MissingVertexAttributeError.html "struct bevy::mesh::MissingVertexAttributeError")

[PlaneMeshBuilder](struct.PlaneMeshBuilder.html "struct bevy::mesh::PlaneMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Plane3d`](../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d") shape.

[Polyline2dMeshBuilder](struct.Polyline2dMeshBuilder.html "struct bevy::mesh::Polyline2dMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Polyline2d`](../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d") shape.

[RectangleMeshBuilder](struct.RectangleMeshBuilder.html "struct bevy::mesh::RectangleMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Rectangle`](../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle") shape.

[RegularPolygonMeshBuilder](struct.RegularPolygonMeshBuilder.html "struct bevy::mesh::RegularPolygonMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`RegularPolygon`](../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon") shape.

[RhombusMeshBuilder](struct.RhombusMeshBuilder.html "struct bevy::mesh::RhombusMeshBuilder")

A builder for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Rhombus`](../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus") shape.

[RingMeshBuilder](struct.RingMeshBuilder.html "struct bevy::mesh::RingMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Ring`](../prelude/struct.Ring.html "struct bevy::prelude::Ring") shape.

[Segment2dMeshBuilder](struct.Segment2dMeshBuilder.html "struct bevy::mesh::Segment2dMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Segment2d`](../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d").

[SerializedMesh](struct.SerializedMesh.html "struct bevy::mesh::SerializedMesh")`serialize`

A version of [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") suitable for serializing for short-term transfer.

[SphereMeshBuilder](struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Sphere`](../prelude/struct.Sphere.html "struct bevy::prelude::Sphere") shape.

[TetrahedronMeshBuilder](struct.TetrahedronMeshBuilder.html "struct bevy::mesh::TetrahedronMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Tetrahedron`](../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron") shape.

[TorusMeshBuilder](struct.TorusMeshBuilder.html "struct bevy::mesh::TorusMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Torus`](../prelude/struct.Torus.html "struct bevy::prelude::Torus") shape.

[Triangle2dMeshBuilder](struct.Triangle2dMeshBuilder.html "struct bevy::mesh::Triangle2dMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Triangle2d`](../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d") shape.

[Triangle3dMeshBuilder](struct.Triangle3dMeshBuilder.html "struct bevy::mesh::Triangle3dMeshBuilder")

A builder used for creating a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Triangle3d`](../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d") shape.

[VertexAttributeDescriptor](struct.VertexAttributeDescriptor.html "struct bevy::mesh::VertexAttributeDescriptor")

[VertexBufferLayout](struct.VertexBufferLayout.html "struct bevy::mesh::VertexBufferLayout")

Describes how the vertex buffer is interpreted.

## Enums

[CapsuleUvProfile](enum.CapsuleUvProfile.html "enum bevy::mesh::CapsuleUvProfile")

Manner in which UV coordinates are distributed vertically.

[CircularMeshUvMode](enum.CircularMeshUvMode.html "enum bevy::mesh::CircularMeshUvMode")

Specifies how to generate UV-mappings for the [`CircularSector`](../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector") and [`CircularSegment`](../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment") shapes.

[ConeAnchor](enum.ConeAnchor.html "enum bevy::mesh::ConeAnchor")

Anchoring options for [`ConeMeshBuilder`](struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[CylinderAnchor](enum.CylinderAnchor.html "enum bevy::mesh::CylinderAnchor")

Anchoring options for [`CylinderMeshBuilder`](struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[GenerateTangentsError](enum.GenerateTangentsError.html "enum bevy::mesh::GenerateTangentsError")

Failed to generate tangents for the mesh.

[IcosphereError](enum.IcosphereError.html "enum bevy::mesh::IcosphereError")

An error when creating an icosphere [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") from a [`SphereMeshBuilder`](struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder").

[Indices](enum.Indices.html "enum bevy::mesh::Indices")

An array of indices into the [`VertexAttributeValues`](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues") for a mesh.

[MeshAccessError](enum.MeshAccessError.html "enum bevy::mesh::MeshAccessError")

Error from accessing mesh vertex attributes or indices

[MeshMergeDuplicateVerticesError](enum.MeshMergeDuplicateVerticesError.html "enum bevy::mesh::MeshMergeDuplicateVerticesError")

Error that can occur when calling [`Mesh::merge_duplicate_vertices`](../prelude/struct.Mesh.html#method.merge_duplicate_vertices "method bevy::prelude::Mesh::merge_duplicate_vertices")

[MeshMergeError](enum.MeshMergeError.html "enum bevy::mesh::MeshMergeError")

Error that can occur when calling [`Mesh::merge`](../prelude/struct.Mesh.html#method.merge "method bevy::prelude::Mesh::merge").

[MeshTrianglesError](enum.MeshTrianglesError.html "enum bevy::mesh::MeshTrianglesError")

An error that occurred while trying to extract a collection of triangles from a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

[MeshWindingInvertError](enum.MeshWindingInvertError.html "enum bevy::mesh::MeshWindingInvertError")

An error that occurred while trying to invert the winding of a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

[PerimeterSegment](enum.PerimeterSegment.html "enum bevy::mesh::PerimeterSegment")

A type representing a segment of the perimeter of an extrudable mesh.

[PrimitiveTopology](enum.PrimitiveTopology.html "enum bevy::mesh::PrimitiveTopology")

Primitive type the input mesh is composed of.

[SphereKind](enum.SphereKind.html "enum bevy::mesh::SphereKind")

A type of sphere mesh.

[UvChannel](enum.UvChannel.html "enum bevy::mesh::UvChannel")

An enum to define which UV attribute to use for a texture.

[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues")

Contains an array where each entry describes a property of a single vertex. Matches the [`VertexFormats`](enum.VertexFormat.html "enum bevy::mesh::VertexFormat").

[VertexFormat](enum.VertexFormat.html "enum bevy::mesh::VertexFormat")

Vertex Format for a [`VertexAttribute`](../render/render_resource/struct.VertexAttribute.html "struct bevy::render::render_resource::VertexAttribute") (input).

## Constants

[INDEX\_BUFFER\_ASSET\_INDEX](constant.INDEX_BUFFER_ASSET_INDEX.html "constant bevy::mesh::INDEX_BUFFER_ASSET_INDEX")

[VERTEX\_ATTRIBUTE\_BUFFER\_ID](constant.VERTEX_ATTRIBUTE_BUFFER_ID.html "constant bevy::mesh::VERTEX_ATTRIBUTE_BUFFER_ID")

## Traits

[Extrudable](trait.Extrudable.html "trait bevy::mesh::Extrudable")

A trait required for implementing `Meshable` for `Extrusion<T>`.

[MeshBuilder](trait.MeshBuilder.html "trait bevy::mesh::MeshBuilder")

A trait used to build [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")es from a configuration

[Meshable](trait.Meshable.html "trait bevy::mesh::Meshable")

A trait for shapes that can be turned into a [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

## Functions

[mark\_3d\_meshes\_as\_changed\_if\_their\_assets\_changed](fn.mark_3d_meshes_as_changed_if_their_assets_changed.html "fn bevy::mesh::mark_3d_meshes_as_changed_if_their_assets_changed")

A system that marks a [`Mesh3d`](../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") as changed if the associated [`Mesh`](../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") asset has changed.

[triangle\_area\_normal](fn.triangle_area_normal.html "fn bevy::mesh::triangle_area_normal")

Compute a vector whose direction is the normal of the triangle formed by points a, b, c, and whose magnitude is double the area of the triangle. This is useful for computing smooth normals where the contributing normals are proportionate to the areas of the triangles as [discussed here](https://iquilezles.org/articles/normals/).

[triangle\_normal](fn.triangle_normal.html "fn bevy::mesh::triangle_normal")

Compute the normal of a face made of three points: a, b, and c.
[bevy](../../index.html)::[render](../index.html)

# Module mesh 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/lib.rs.html#52)

## Modules

[allocator](allocator/index.html "mod bevy::render::mesh::allocator")

Manages mesh vertex and index buffers.

[morph](morph/index.html "mod bevy::render::mesh::morph")`morph`

## Structs

[AnnulusMeshBuilder](struct.AnnulusMeshBuilder.html "struct bevy::render::mesh::AnnulusMeshBuilder")

A builder for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Annulus`](../../prelude/struct.Annulus.html "struct bevy::prelude::Annulus") shape.

[BaseMeshPipelineKey](struct.BaseMeshPipelineKey.html "struct bevy::render::mesh::BaseMeshPipelineKey")

Our base mesh pipeline key bits start from the highest bit and go downward. The PBR mesh pipeline key bits start from the lowest bit and go upward. This allows the PBR bits in the downstream crate `bevy_pbr` to coexist in the same field without any shifts.

[Capsule2dMeshBuilder](struct.Capsule2dMeshBuilder.html "struct bevy::render::mesh::Capsule2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Capsule2d`](../../prelude/struct.Capsule2d.html "struct bevy::prelude::Capsule2d") shape.

[Capsule3dMeshBuilder](struct.Capsule3dMeshBuilder.html "struct bevy::render::mesh::Capsule3dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Capsule3d`](../../prelude/struct.Capsule3d.html "struct bevy::prelude::Capsule3d") shape.

[CircleMeshBuilder](struct.CircleMeshBuilder.html "struct bevy::render::mesh::CircleMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Circle`](../../prelude/struct.Circle.html "struct bevy::prelude::Circle") shape.

[CircularSectorMeshBuilder](struct.CircularSectorMeshBuilder.html "struct bevy::render::mesh::CircularSectorMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`CircularSector`](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector") shape.

[CircularSegmentMeshBuilder](struct.CircularSegmentMeshBuilder.html "struct bevy::render::mesh::CircularSegmentMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`CircularSegment`](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment") shape.

[ConeMeshBuilder](struct.ConeMeshBuilder.html "struct bevy::render::mesh::ConeMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cone`](../../prelude/struct.Cone.html "struct bevy::prelude::Cone") shape.

[ConicalFrustumMeshBuilder](struct.ConicalFrustumMeshBuilder.html "struct bevy::render::mesh::ConicalFrustumMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`ConicalFrustum`](../../prelude/struct.ConicalFrustum.html "struct bevy::prelude::ConicalFrustum") shape.

[ConvexPolygonMeshBuilder](struct.ConvexPolygonMeshBuilder.html "struct bevy::render::mesh::ConvexPolygonMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`ConvexPolygon`](../../prelude/struct.ConvexPolygon.html "struct bevy::prelude::ConvexPolygon") shape.

[CuboidMeshBuilder](struct.CuboidMeshBuilder.html "struct bevy::render::mesh::CuboidMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cuboid`](../../prelude/struct.Cuboid.html "struct bevy::prelude::Cuboid") shape.

[CylinderMeshBuilder](struct.CylinderMeshBuilder.html "struct bevy::render::mesh::CylinderMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Cylinder`](../../prelude/struct.Cylinder.html "struct bevy::prelude::Cylinder") shape.

[EllipseMeshBuilder](struct.EllipseMeshBuilder.html "struct bevy::render::mesh::EllipseMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Ellipse`](../../prelude/struct.Ellipse.html "struct bevy::prelude::Ellipse") shape.

[ExtrusionBuilder](struct.ExtrusionBuilder.html "struct bevy::render::mesh::ExtrusionBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Extrusion`](../../prelude/struct.Extrusion.html "struct bevy::prelude::Extrusion") shape.

[Mesh](struct.Mesh.html "struct bevy::render::mesh::Mesh")

A 3D object made out of vertices representing triangles, lines, or points, with “attribute” values for each vertex.

[Mesh2d](struct.Mesh2d.html "struct bevy::render::mesh::Mesh2d")

A component for 2D meshes. Requires a [`MeshMaterial2d`](https://docs.rs/bevy/latest/bevy/prelude/struct.MeshMaterial2d.html) to be rendered, commonly using a [`ColorMaterial`](https://docs.rs/bevy/latest/bevy/prelude/struct.ColorMaterial.html).

[Mesh2dTemplate](struct.Mesh2dTemplate.html "struct bevy::render::mesh::Mesh2dTemplate")

[Mesh3d](struct.Mesh3d.html "struct bevy::render::mesh::Mesh3d")

A component for 3D meshes. Requires a [`MeshMaterial3d`](https://docs.rs/bevy/latest/bevy/pbr/struct.MeshMaterial3d.html) to be rendered, commonly using a [`StandardMaterial`](https://docs.rs/bevy/latest/bevy/pbr/struct.StandardMaterial.html).

[Mesh3dTemplate](struct.Mesh3dTemplate.html "struct bevy::render::mesh::Mesh3dTemplate")

[MeshDeserializer](struct.MeshDeserializer.html "struct bevy::render::mesh::MeshDeserializer")`serialize`

Use to specify extra options when deserializing a [`SerializedMesh`](../../mesh/struct.SerializedMesh.html "struct bevy::mesh::SerializedMesh") into a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

[MeshPlugin](struct.MeshPlugin.html "struct bevy::render::mesh::MeshPlugin")

Adds [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") as an asset.

[MeshRenderAssetPlugin](struct.MeshRenderAssetPlugin.html "struct bevy::render::mesh::MeshRenderAssetPlugin")

Makes sure that [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")es are extracted and prepared for the GPU. Does _not_ add the [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") as an asset. Use [`MeshPlugin`](../../mesh/struct.MeshPlugin.html "struct bevy::mesh::MeshPlugin") for that.

[MeshTag](struct.MeshTag.html "struct bevy::render::mesh::MeshTag")

A component that stores an arbitrary index used to identify the mesh instance when rendering.

[MeshVertexAttribute](struct.MeshVertexAttribute.html "struct bevy::render::mesh::MeshVertexAttribute")

[MeshVertexAttributeId](struct.MeshVertexAttributeId.html "struct bevy::render::mesh::MeshVertexAttributeId")

[MeshVertexBufferLayout](struct.MeshVertexBufferLayout.html "struct bevy::render::mesh::MeshVertexBufferLayout")

[MeshVertexBufferLayoutRef](struct.MeshVertexBufferLayoutRef.html "struct bevy::render::mesh::MeshVertexBufferLayoutRef")

Describes the layout of the mesh vertices in GPU memory.

[MeshVertexBufferLayouts](struct.MeshVertexBufferLayouts.html "struct bevy::render::mesh::MeshVertexBufferLayouts")

Stores the single copy of each mesh vertex buffer layout.

[MissingVertexAttributeError](struct.MissingVertexAttributeError.html "struct bevy::render::mesh::MissingVertexAttributeError")

[PlaneMeshBuilder](struct.PlaneMeshBuilder.html "struct bevy::render::mesh::PlaneMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Plane3d`](../../prelude/struct.Plane3d.html "struct bevy::prelude::Plane3d") shape.

[Polyline2dMeshBuilder](struct.Polyline2dMeshBuilder.html "struct bevy::render::mesh::Polyline2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Polyline2d`](../../prelude/struct.Polyline2d.html "struct bevy::prelude::Polyline2d") shape.

[RectangleMeshBuilder](struct.RectangleMeshBuilder.html "struct bevy::render::mesh::RectangleMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Rectangle`](../../prelude/struct.Rectangle.html "struct bevy::prelude::Rectangle") shape.

[RegularPolygonMeshBuilder](struct.RegularPolygonMeshBuilder.html "struct bevy::render::mesh::RegularPolygonMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`RegularPolygon`](../../prelude/struct.RegularPolygon.html "struct bevy::prelude::RegularPolygon") shape.

[RenderMesh](struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh")

The render world representation of a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

[RhombusMeshBuilder](struct.RhombusMeshBuilder.html "struct bevy::render::mesh::RhombusMeshBuilder")

A builder for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Rhombus`](../../prelude/struct.Rhombus.html "struct bevy::prelude::Rhombus") shape.

[RingMeshBuilder](struct.RingMeshBuilder.html "struct bevy::render::mesh::RingMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Ring`](../../prelude/struct.Ring.html "struct bevy::prelude::Ring") shape.

[Segment2dMeshBuilder](struct.Segment2dMeshBuilder.html "struct bevy::render::mesh::Segment2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Segment2d`](../../prelude/struct.Segment2d.html "struct bevy::prelude::Segment2d").

[SerializedMesh](struct.SerializedMesh.html "struct bevy::render::mesh::SerializedMesh")`serialize`

A version of [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") suitable for serializing for short-term transfer.

[SphereMeshBuilder](struct.SphereMeshBuilder.html "struct bevy::render::mesh::SphereMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with an [`Sphere`](../../prelude/struct.Sphere.html "struct bevy::prelude::Sphere") shape.

[TetrahedronMeshBuilder](struct.TetrahedronMeshBuilder.html "struct bevy::render::mesh::TetrahedronMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Tetrahedron`](../../prelude/struct.Tetrahedron.html "struct bevy::prelude::Tetrahedron") shape.

[TorusMeshBuilder](struct.TorusMeshBuilder.html "struct bevy::render::mesh::TorusMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Torus`](../../prelude/struct.Torus.html "struct bevy::prelude::Torus") shape.

[Triangle2dMeshBuilder](struct.Triangle2dMeshBuilder.html "struct bevy::render::mesh::Triangle2dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Triangle2d`](../../prelude/struct.Triangle2d.html "struct bevy::prelude::Triangle2d") shape.

[Triangle3dMeshBuilder](struct.Triangle3dMeshBuilder.html "struct bevy::render::mesh::Triangle3dMeshBuilder")

A builder used for creating a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") with a [`Triangle3d`](../../prelude/struct.Triangle3d.html "struct bevy::prelude::Triangle3d") shape.

[VertexAttributeDescriptor](struct.VertexAttributeDescriptor.html "struct bevy::render::mesh::VertexAttributeDescriptor")

[VertexBufferLayout](struct.VertexBufferLayout.html "struct bevy::render::mesh::VertexBufferLayout")

Describes how the vertex buffer is interpreted.

## Enums

[CapsuleUvProfile](enum.CapsuleUvProfile.html "enum bevy::render::mesh::CapsuleUvProfile")

Manner in which UV coordinates are distributed vertically.

[CircularMeshUvMode](enum.CircularMeshUvMode.html "enum bevy::render::mesh::CircularMeshUvMode")

Specifies how to generate UV-mappings for the [`CircularSector`](../../prelude/struct.CircularSector.html "struct bevy::prelude::CircularSector") and [`CircularSegment`](../../prelude/struct.CircularSegment.html "struct bevy::prelude::CircularSegment") shapes.

[ConeAnchor](enum.ConeAnchor.html "enum bevy::render::mesh::ConeAnchor")

Anchoring options for [`ConeMeshBuilder`](../../mesh/struct.ConeMeshBuilder.html "struct bevy::mesh::ConeMeshBuilder")

[CylinderAnchor](enum.CylinderAnchor.html "enum bevy::render::mesh::CylinderAnchor")

Anchoring options for [`CylinderMeshBuilder`](../../mesh/struct.CylinderMeshBuilder.html "struct bevy::mesh::CylinderMeshBuilder")

[GenerateTangentsError](enum.GenerateTangentsError.html "enum bevy::render::mesh::GenerateTangentsError")

Failed to generate tangents for the mesh.

[IcosphereError](enum.IcosphereError.html "enum bevy::render::mesh::IcosphereError")

An error when creating an icosphere [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") from a [`SphereMeshBuilder`](../../mesh/struct.SphereMeshBuilder.html "struct bevy::mesh::SphereMeshBuilder").

[Indices](enum.Indices.html "enum bevy::render::mesh::Indices")

An array of indices into the [`VertexAttributeValues`](../../mesh/enum.VertexAttributeValues.html "enum bevy::mesh::VertexAttributeValues") for a mesh.

[MeshAccessError](enum.MeshAccessError.html "enum bevy::render::mesh::MeshAccessError")

Error from accessing mesh vertex attributes or indices

[MeshMergeDuplicateVerticesError](enum.MeshMergeDuplicateVerticesError.html "enum bevy::render::mesh::MeshMergeDuplicateVerticesError")

Error that can occur when calling [`Mesh::merge_duplicate_vertices`](../../prelude/struct.Mesh.html#method.merge_duplicate_vertices "method bevy::prelude::Mesh::merge_duplicate_vertices")

[MeshMergeError](enum.MeshMergeError.html "enum bevy::render::mesh::MeshMergeError")

Error that can occur when calling [`Mesh::merge`](../../prelude/struct.Mesh.html#method.merge "method bevy::prelude::Mesh::merge").

[MeshTrianglesError](enum.MeshTrianglesError.html "enum bevy::render::mesh::MeshTrianglesError")

An error that occurred while trying to extract a collection of triangles from a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

[MeshWindingInvertError](enum.MeshWindingInvertError.html "enum bevy::render::mesh::MeshWindingInvertError")

An error that occurred while trying to invert the winding of a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

[PerimeterSegment](enum.PerimeterSegment.html "enum bevy::render::mesh::PerimeterSegment")

A type representing a segment of the perimeter of an extrudable mesh.

[PrimitiveTopology](enum.PrimitiveTopology.html "enum bevy::render::mesh::PrimitiveTopology")

Primitive type the input mesh is composed of.

[RenderMeshBufferInfo](enum.RenderMeshBufferInfo.html "enum bevy::render::mesh::RenderMeshBufferInfo")

The index/vertex buffer info of a [`RenderMesh`](struct.RenderMesh.html "struct bevy::render::mesh::RenderMesh").

[SphereKind](enum.SphereKind.html "enum bevy::render::mesh::SphereKind")

A type of sphere mesh.

[UvChannel](enum.UvChannel.html "enum bevy::render::mesh::UvChannel")

An enum to define which UV attribute to use for a texture.

[VertexAttributeValues](enum.VertexAttributeValues.html "enum bevy::render::mesh::VertexAttributeValues")

Contains an array where each entry describes a property of a single vertex. Matches the [`VertexFormats`](../../mesh/enum.VertexFormat.html "enum bevy::mesh::VertexFormat").

[VertexFormat](enum.VertexFormat.html "enum bevy::render::mesh::VertexFormat")

Vertex Format for a [`VertexAttribute`](../render_resource/struct.VertexAttribute.html "struct bevy::render::render_resource::VertexAttribute") (input).

## Constants

[INDEX\_BUFFER\_ASSET\_INDEX](constant.INDEX_BUFFER_ASSET_INDEX.html "constant bevy::render::mesh::INDEX_BUFFER_ASSET_INDEX")

[VERTEX\_ATTRIBUTE\_BUFFER\_ID](constant.VERTEX_ATTRIBUTE_BUFFER_ID.html "constant bevy::render::mesh::VERTEX_ATTRIBUTE_BUFFER_ID")

## Traits

[Extrudable](trait.Extrudable.html "trait bevy::render::mesh::Extrudable")

A trait required for implementing `Meshable` for `Extrusion<T>`.

[MeshBuilder](trait.MeshBuilder.html "trait bevy::render::mesh::MeshBuilder")

A trait used to build [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")es from a configuration

[Meshable](trait.Meshable.html "trait bevy::render::mesh::Meshable")

A trait for shapes that can be turned into a [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh").

## Functions

[mark\_3d\_meshes\_as\_changed\_if\_their\_assets\_changed](fn.mark_3d_meshes_as_changed_if_their_assets_changed.html "fn bevy::render::mesh::mark_3d_meshes_as_changed_if_their_assets_changed")

A system that marks a [`Mesh3d`](../../prelude/struct.Mesh3d.html "struct bevy::prelude::Mesh3d") as changed if the associated [`Mesh`](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh") asset has changed.

[triangle\_area\_normal](fn.triangle_area_normal.html "fn bevy::render::mesh::triangle_area_normal")

Compute a vector whose direction is the normal of the triangle formed by points a, b, c, and whose magnitude is double the area of the triangle. This is useful for computing smooth normals where the contributing normals are proportionate to the areas of the triangles as [discussed here](https://iquilezles.org/articles/normals/).

[triangle\_normal](fn.triangle_normal.html "fn bevy::render::mesh::triangle_normal")

Compute the normal of a face made of three points: a, b, and c.
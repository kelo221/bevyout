[bevy](../../index.html)::[camera](../index.html)::[primitives](index.html)

# Trait MeshAabb 

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#13)

```rust
pub trait MeshAabb {
    // Required method
    fn compute_aabb(&self) -> Option<Aabb>;
}
```

## Required Methods

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#18)

#### fn [compute\_aabb](#tymethod.compute_aabb)(&self) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<[Aabb](struct.Aabb.html "struct bevy::camera::primitives::Aabb")\>

Compute the Axis-Aligned Bounding Box of the mesh vertices in model space

Returns `None` if `self` doesn’t have [`Mesh::ATTRIBUTE_POSITION`](../../prelude/struct.Mesh.html#associatedconstant.ATTRIBUTE_POSITION "associated constant bevy::prelude::Mesh::ATTRIBUTE_POSITION") of type [`VertexAttributeValues::Float32x3`](../../mesh/enum.VertexAttributeValues.html#variant.Float32x3 "variant bevy::mesh::VertexAttributeValues::Float32x3"), or if `self` doesn’t have any vertices.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_camera/0.19.0/x86_64-unknown-linux-gnu/src/bevy_camera/primitives.rs.html#21)

### impl [MeshAabb](trait.MeshAabb.html "trait bevy::camera::primitives::MeshAabb") for [Mesh](../../prelude/struct.Mesh.html "struct bevy::prelude::Mesh")
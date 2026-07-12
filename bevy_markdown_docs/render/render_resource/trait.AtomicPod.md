[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait AtomicPod 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#29)

```rust
pub trait AtomicPod:
    Pod
    + Default
    + Send
    + Sync
    + 'static {
    type Blob: AtomicPodBlob;

    // Required methods
    fn read_from_blob(blob: &Self::Blob) -> Self;
    fn write_to_blob(&self, blob: &Self::Blob);
}
```

Data that can be converted to an array of [`std::sync::atomic::AtomicU32`](../../platform/sync/atomic/type.AtomicU32.html "type bevy::platform::sync::atomic::AtomicU32") values.

That array is known as the _blob_ ([`Self::Blob`](trait.AtomicPod.html#associatedtype.Blob "associated type bevy::render::render_resource::AtomicPod::Blob")). The trait provides methods to copy data into and out of the blob type.

Note that, while implementing this trait isn’t unsafe, it can be tedious, and in any case implementing [`AtomicPodBlob`](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") _is_ unsafe. Therefore, you should almost always use the `impl_atomic_pod!` macro to produce implementations of this trait.

## Required Associated Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#37)

#### type [Blob](#associatedtype.Blob): [AtomicPodBlob](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob")

The _blob_ type that allows shared mutation.

This type must be an array of [`std::sync::atomic::AtomicU32`](../../platform/sync/atomic/type.AtomicU32.html "type bevy::platform::sync::atomic::AtomicU32")s. Because the renderer can’t guarantee that, the [`AtomicPodBlob`](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") trait is unsafe. However, the [`crate::impl_atomic_pod`](../macro.impl_atomic_pod.html "macro bevy::render::impl_atomic_pod") macro can automatically generate safe implementations of [`AtomicPodBlob`](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") for you.

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#41)

#### fn [read\_from\_blob](#tymethod.read_from_blob)(blob: &Self::[Blob](trait.AtomicPod.html#associatedtype.Blob "type bevy::render::render_resource::AtomicPod::Blob")) -> Self

Produces a value of this type from the blob, typically by reading its fields one after another atomically.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#48)

#### fn [write\_to\_blob](#tymethod.write_to_blob)(&self, blob: &Self::[Blob](trait.AtomicPod.html#associatedtype.Blob "type bevy::render::render_resource::AtomicPod::Blob"))

Copies the `self` value to the blob, typically by writing its fields one after another atomically.

Note that, because we’re using atomics, the `blob` parameter doesn’t need a mutable reference.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#211)

### impl [AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod") for [()](https://doc.rust-lang.org/nightly/std/primitive.unit.html)

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#211)

#### type [Blob](#associatedtype.Blob) = [AtomicPodUnitBlob](struct.AtomicPodUnitBlob.html "struct bevy::render::render_resource::AtomicPodUnitBlob")

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#211)

#### fn [read\_from\_blob](#tymethod.read_from_blob)(blob: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod")\>::[Blob](trait.AtomicPod.html#associatedtype.Blob "type bevy::render::render_resource::AtomicPod::Blob"))

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#211)

#### fn [write\_to\_blob](#tymethod.write_to_blob)(&self, blob: &<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html) as [AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod")\>::[Blob](trait.AtomicPod.html#associatedtype.Blob "type bevy::render::render_resource::AtomicPod::Blob"))

## Implementors

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#652)

### impl [AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod") for [MeshCullingData](../../pbr/struct.MeshCullingData.html "struct bevy::pbr::MeshCullingData")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#652)

#### type [Blob](#associatedtype.Blob) = [MeshCullingDataBlob](../../pbr/struct.MeshCullingDataBlob.html "struct bevy::pbr::MeshCullingDataBlob")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#627)

### impl [AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod") for [MeshInputUniform](../../pbr/struct.MeshInputUniform.html "struct bevy::pbr::MeshInputUniform")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#627)

#### type [Blob](#associatedtype.Blob) = [MeshInputUniformBlob](../../pbr/struct.MeshInputUniformBlob.html "struct bevy::pbr::MeshInputUniformBlob")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#901-905)

### impl [AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod") for [RenderMeshInstanceGpuFlat](../../pbr/struct.RenderMeshInstanceGpuFlat.html "struct bevy::pbr::RenderMeshInstanceGpuFlat")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#901-905)

#### type [Blob](#associatedtype.Blob) = [RenderMeshInstanceGpuFlatBlob](../../pbr/struct.RenderMeshInstanceGpuFlatBlob.html "struct bevy::pbr::RenderMeshInstanceGpuFlatBlob")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#847-863)

### impl [AtomicPod](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod") for [RenderMeshInstanceSharedFlat](../../pbr/struct.RenderMeshInstanceSharedFlat.html "struct bevy::pbr::RenderMeshInstanceSharedFlat")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#847-863)

#### type [Blob](#associatedtype.Blob) = [RenderMeshInstanceSharedFlatBlob](../../pbr/struct.RenderMeshInstanceSharedFlatBlob.html "struct bevy::pbr::RenderMeshInstanceSharedFlatBlob")
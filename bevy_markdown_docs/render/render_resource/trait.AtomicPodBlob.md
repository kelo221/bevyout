[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait AtomicPodBlob 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#71)

```rust
pub unsafe trait AtomicPodBlob:
    Default
    + Send
    + Sync
    + 'static { }
```

Describes a type that has the same bit pattern as another type, but is made up entirely of an array of [`std::sync::atomic::AtomicU32`](../../platform/sync/atomic/type.AtomicU32.html "type bevy::platform::sync::atomic::AtomicU32") values.

This trait enables values of whatever type this mirrors to be written from multiple threads. It’s memory-safe because the type must be POD. However, this doesn’t protect against data races; it’s possible for safe code to see partially-updated values, which might be incorrect. Therefore, use this type with caution.

The [`crate::impl_atomic_pod`](../macro.impl_atomic_pod.html "macro bevy::render::impl_atomic_pod") macro that generates an implementation of [`AtomicPod`](trait.AtomicPod.html "trait bevy::render::render_resource::AtomicPod") automatically generates a blob type that implements [`AtomicPodBlob`](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob"). This is the preferred way to implement this trait and doesn’t require any unsafe code.

## Safety

This trait must only be implemented by types that are `#[repr(transparent)]` wrappers around `[AtomicU32; N]` for some N (where N may legally be 0). That’s because values implementing this trait are read as a `&[u8]` when uploading to the GPU.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#211)

### impl [AtomicPodBlob](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") for [AtomicPodUnitBlob](struct.AtomicPodUnitBlob.html "struct bevy::render::render_resource::AtomicPodUnitBlob")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#652)

### impl [AtomicPodBlob](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") for [MeshCullingDataBlob](../../pbr/struct.MeshCullingDataBlob.html "struct bevy::pbr::MeshCullingDataBlob")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#627)

### impl [AtomicPodBlob](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") for [MeshInputUniformBlob](../../pbr/struct.MeshInputUniformBlob.html "struct bevy::pbr::MeshInputUniformBlob")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#901-905)

### impl [AtomicPodBlob](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") for [RenderMeshInstanceGpuFlatBlob](../../pbr/struct.RenderMeshInstanceGpuFlatBlob.html "struct bevy::pbr::RenderMeshInstanceGpuFlatBlob")

[Source](https://docs.rs/bevy_pbr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_pbr/render/mesh.rs.html#847-863)

### impl [AtomicPodBlob](trait.AtomicPodBlob.html "trait bevy::render::render_resource::AtomicPodBlob") for [RenderMeshInstanceSharedFlatBlob](../../pbr/struct.RenderMeshInstanceSharedFlatBlob.html "struct bevy::pbr::RenderMeshInstanceSharedFlatBlob")
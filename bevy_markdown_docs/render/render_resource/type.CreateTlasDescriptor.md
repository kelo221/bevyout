[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias CreateTlasDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/tlas.rs.html#10)

```rust
pub type CreateTlasDescriptor<'a> = CreateTlasDescriptor<Option<&'a str>>;
```

Descriptor to create top level acceleration structures.

## Aliased Type

```rust
#[repr(C)]pub struct CreateTlasDescriptor<'a> {
    pub label: Option<&'a str>,
    pub max_instances: u32,
    pub flags: AccelerationStructureFlags,
    pub update_mode: AccelerationStructureUpdateMode,
}
```

## Fields

`label: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

Label for the top level acceleration structure.

`max_instances: [u32](https://doc.rust-lang.org/nightly/std/primitive.u32.html)`

Number of instances that can be stored in the acceleration structure.

`flags: [AccelerationStructureFlags](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/struct.AccelerationStructureFlags.html "struct wgpu_types::ray_tracing::AccelerationStructureFlags")`

Flags for the bottom level acceleration structure.

`update_mode: [AccelerationStructureUpdateMode](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/enum.AccelerationStructureUpdateMode.html "enum wgpu_types::ray_tracing::AccelerationStructureUpdateMode")`

Update mode for the bottom level acceleration structure.
[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Type Alias CreateBlasDescriptor 

[Source](https://docs.rs/wgpu/29.0.3/x86_64-unknown-linux-gnu/src/wgpu/api/blas.rs.html#32)

```rust
pub type CreateBlasDescriptor<'a> = CreateBlasDescriptor<Option<&'a str>>;
```

Descriptor to create bottom level acceleration structures.

## Aliased Type

```rust
#[repr(C)]pub struct CreateBlasDescriptor<'a> {
    pub label: Option<&'a str>,
    pub flags: AccelerationStructureFlags,
    pub update_mode: AccelerationStructureUpdateMode,
}
```

## Fields

`label: [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<&'a [str](https://doc.rust-lang.org/nightly/std/primitive.str.html)>`

Label for the bottom level acceleration structure.

`flags: [AccelerationStructureFlags](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/struct.AccelerationStructureFlags.html "struct wgpu_types::ray_tracing::AccelerationStructureFlags")`

Flags for the bottom level acceleration structure.

`update_mode: [AccelerationStructureUpdateMode](https://docs.rs/wgpu-types/29.0.3/x86_64-unknown-linux-gnu/wgpu_types/ray_tracing/enum.AccelerationStructureUpdateMode.html "enum wgpu_types::ray_tracing::AccelerationStructureUpdateMode")`

Update mode for the bottom level acceleration structure.
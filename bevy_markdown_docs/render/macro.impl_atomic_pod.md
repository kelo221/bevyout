[bevy](../index.html)::[render](index.html)

# Macro impl\_atomic\_pod 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/atomic_pod.rs.html#118)

```rust
macro_rules! impl_atomic_pod {
    (
        $pod_ty: ty,
        $blob_ty: ident
        $(, field($field_name: ident : $field_ty: ty, $getter: ident $(, $($setter: ident)?)?))*
        $(,)?
    ) => { ... };
}
```

A macro that generates a _blob_ type that allows a POD type to be updated in shared memory.

An example of use of this macro:

```rust
#[derive(Clone, Copy, Default, Pod, Zeroable)]
#[repr(C)]
struct Foo {
    a: u32,
    b: u32,
}
impl_atomic_pod!(
    Foo,
    FooBlob,
    field(a: u32, a, set_a),
    field(b: u32, b, set_b),
);
```

The first argument to this macro is the name of the type that you wish to be updatable in shared memory. The second argument is the name of a “blob” type: conventionally, it matches the name of the type with `Blob` appended.

Afterward follow optional _field getter and setter_ declarations. These declarations direct the [`crate::impl_atomic_pod`](macro.impl_atomic_pod.html "macro bevy::render::impl_atomic_pod") macro to create convenience accessor and mutation methods that allow fields of the blob value to be accessed and mutated. The first argument of `field` is the name of the field, a `:`, and the type of the field. The second argument is the name that this macro should assign the accessor method, and the third, optional, argument is the name that this macro should give the mutator method.

This macro generates (1) the `struct` corresponding to the blob type; (2) the implementation of `AtomicPod` for the POD type; (3) the unsafe implementation of `AtomicPodBlob`; (4) an inherent implementation of `AtomicPodBlob` that contains accessor and mutator methods as directed.

The POD type must have a size that’s a multiple of 4 bytes, as must the types of any fields that are named in `field` declarations.
[bevy](../../index.html)::[app](../index.html)::[hotpatch](index.html)

# Trait HotFunction 

[Source](https://docs.rs/subsecond/0.7.9/x86_64-unknown-linux-gnu/src/subsecond/lib.rs.html#852)

```rust
pub trait HotFunction<Args, Marker> {
    type Return;
    type Real;

    // Required methods
    fn call_it(&mut self, args: Args) -> Self::Return;
    unsafe fn call_as_ptr(&mut self, _args: Args) -> Self::Return;
}
```

Available on **crate feature `hotpatching`** only.

A trait that enables types to be hot-patched.

This trait is only implemented for FnMut types which naturally includes function pointers and closures that can be re-ran. FnOnce closures are currently not supported since the hot-patching system we use implies that the function can be called multiple times.

## Required Associated Types

[Source](https://docs.rs/subsecond/0.7.9/x86_64-unknown-linux-gnu/src/subsecond/lib.rs.html#854)

#### type [Return](#associatedtype.Return)

The return type of the function.

[Source](https://docs.rs/subsecond/0.7.9/x86_64-unknown-linux-gnu/src/subsecond/lib.rs.html#858)

#### type [Real](#associatedtype.Real)

The real function type. This is meant to be a function pointer. When we call `call_as_ptr`, we will transmute the function to this type and call it.

## Required Methods

[Source](https://docs.rs/subsecond/0.7.9/x86_64-unknown-linux-gnu/src/subsecond/lib.rs.html#866)

#### fn [call\_it](#tymethod.call_it)(&mut self, args: Args) -> Self::[Return](trait.HotFunction.html#associatedtype.Return "type bevy::app::hotpatch::HotFunction::Return")

Call the HotFunction with the given arguments.

##### Why

“rust-call” isn’t stable, so we wrap the underlying call with our own, giving it a stable vtable entry. This is more important than it seems since this function becomes “real” and can be hot-patched.

[Source](https://docs.rs/subsecond/0.7.9/x86_64-unknown-linux-gnu/src/subsecond/lib.rs.html#874)

#### unsafe fn [call\_as\_ptr](#tymethod.call_as_ptr)(&mut self, \_args: Args) -> Self::[Return](trait.HotFunction.html#associatedtype.Return "type bevy::app::hotpatch::HotFunction::Return")

Call the HotFunction as if it were a function pointer.

##### Safety

This is only safe if the underlying type is a function (function pointer or virtual/fat pointer). Using this will use the JumpTable to find the patched function and call it.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
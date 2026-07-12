[bevy](../../index.html)::[app](../index.html)

# Module hotpatch 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/lib.rs.html#41)

Available on **crate feature `hotpatching`** only.

Utilities for hotpatching code.

## Structs

[HotPatchPlugin](struct.HotPatchPlugin.html "struct bevy::app::hotpatch::HotPatchPlugin")

Plugin connecting to Dioxus CLI to enable hot patching.

## Traits

[HotFunction](trait.HotFunction.html "trait bevy::app::hotpatch::HotFunction")

A trait that enables types to be hot-patched.

## Functions

[call](fn.call.html "fn bevy::app::hotpatch::call")

Call a given function with hot-reloading enabled. If the function’s code changes, `call` will use the new version of the function. If code _above_ the function changes, this will emit a panic that forces an unwind to the next [`call`](fn.call.html "fn bevy::app::hotpatch::call") instance.
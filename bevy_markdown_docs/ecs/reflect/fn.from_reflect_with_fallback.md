[bevy](../../index.html)::[ecs](../index.html)::[reflect](index.html)

# Function from\_reflect\_with\_fallback 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/reflect/mod.rs.html#108-112)

```rust
pub fn from_reflect_with_fallback<T>(
    reflected: &(dyn PartialReflect + 'static),
    world: &mut World,
    registry: &TypeRegistry,
) -> Twhere
    T: Reflect + TypePath,
```

Available on **crate feature `bevy_reflect`** only.

Creates a `T` from a `&dyn PartialReflect`.

This will try the following strategies, in this order:

*   use the reflected `FromReflect`, if it’s present and doesn’t fail;
*   use the reflected `Default`, if it’s present, and then call `apply` on the result;
*   use the reflected `FromWorld`, just like the `Default`.

The first one that is present and doesn’t fail will be used.

## Panics

If any strategy produces a `Box<dyn Reflect>` that doesn’t store a value of type `T` this method will panic.

If none of the strategies succeed, this method will panic.
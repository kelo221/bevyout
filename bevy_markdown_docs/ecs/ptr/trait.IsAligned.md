[bevy](../../index.html)::[ecs](../index.html)::[ptr](index.html)

# Trait IsAligned 

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#34)

```rust
pub trait IsAligned: Sealed { }
```

Trait that is only implemented for [`Aligned`](struct.Aligned.html "struct bevy::ecs::ptr::Aligned") and [`Unaligned`](struct.Unaligned.html "struct bevy::ecs::ptr::Unaligned") to work around the lack of ability to have const generics of an enum.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#75)

### impl [IsAligned](trait.IsAligned.html "trait bevy::ecs::ptr::IsAligned") for [Aligned](struct.Aligned.html "struct bevy::ecs::ptr::Aligned")

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#113)

### impl [IsAligned](trait.IsAligned.html "trait bevy::ecs::ptr::IsAligned") for [Unaligned](struct.Unaligned.html "struct bevy::ecs::ptr::Unaligned")
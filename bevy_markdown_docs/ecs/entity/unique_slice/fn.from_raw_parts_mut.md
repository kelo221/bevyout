[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Function from\_raw\_parts\_mut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#889-892)

```rust
pub const unsafe fn from_raw_parts_mut<'a, T>(
    data: *mut T,
    len: usize,
) -> &'a mut UniqueEntityEquivalentSlice<T>where
    T: EntityEquivalent,
```

Performs the same functionality as [`from_raw_parts`](fn.from_raw_parts.html "fn bevy::ecs::entity::unique_slice::from_raw_parts"), except that a mutable slice is returned.

Equivalent to [`slice::from_raw_parts_mut`](https://doc.rust-lang.org/nightly/core/slice/raw/fn.from_raw_parts_mut.html "fn core::slice::raw::from_raw_parts_mut").

## Safety

[`slice::from_raw_parts_mut`](https://doc.rust-lang.org/nightly/core/slice/raw/fn.from_raw_parts_mut.html "fn core::slice::raw::from_raw_parts_mut") must be safe to call with `data` and `len`. Additionally, all elements in the resulting slice must be unique.
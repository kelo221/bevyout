[bevy](../../../index.html)::[ecs](../../index.html)::[entity](../index.html)::[unique\_slice](index.html)

# Function from\_raw\_parts 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/entity/unique_slice.rs.html#873-876)

```rust
pub const unsafe fn from_raw_parts<'a, T>(
    data: *const T,
    len: usize,
) -> &'a UniqueEntityEquivalentSlice<T>where
    T: EntityEquivalent,
```

Forms a slice from a pointer and a length.

Equivalent to [`slice::from_raw_parts`](https://doc.rust-lang.org/nightly/core/slice/raw/fn.from_raw_parts.html "fn core::slice::raw::from_raw_parts").

## Safety

[`slice::from_raw_parts`](https://doc.rust-lang.org/nightly/core/slice/raw/fn.from_raw_parts.html "fn core::slice::raw::from_raw_parts") must be safe to call with `data` and `len`. Additionally, all elements in the resulting slice must be unique.
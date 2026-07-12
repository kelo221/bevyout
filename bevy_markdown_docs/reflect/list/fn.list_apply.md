[bevy](../../index.html)::[reflect](../index.html)::[list](index.html)

# Function list\_apply 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/list.rs.html#437)

```rust
pub fn list_apply<L>(a: &mut L, b: &(dyn PartialReflect + 'static))where
    L: List,
```

Applies the elements of `b` to the corresponding elements of `a`.

If the length of `b` is greater than that of `a`, the excess elements of `b` are cloned and appended to `a`.

## Panics

This function panics if `b` is not a list.
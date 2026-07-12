[bevy](../../index.html)::[reflect](../index.html)::[erased\_serde](index.html)

# Macro serialize\_trait\_object 

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/macros.rs.html#26)

```rust
macro_rules! serialize_trait_object {
    ($($path:tt)+) => { ... };
}
```

Implement `serde::Serialize` for a trait object that has `erased_serde::Serialize` as a supertrait.

```rust
use erased_serde::serialize_trait_object;

trait Event: erased_serde::Serialize {
    /* ... */
}

erased_serde::serialize_trait_object!(Event);
```

The macro supports traits that have type parameters and/or `where` clauses.

```rust
trait Difficult<T>: erased_serde::Serialize where T: Copy {
    /* ... */
}

serialize_trait_object!(<T> Difficult<T> where T: Copy);
```
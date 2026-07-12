[bevy](../../index.html)::[reflect](../index.html)::[erased\_serde](index.html)

# Function serialize 

[Source](https://docs.rs/erased-serde/0.4.10/x86_64-unknown-linux-gnu/src/erased_serde/ser.rs.html#674-677)

```rust
pub fn serialize<T, S>(
    value: &T,
    serializer: S,
) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>where
    T: Serialize + ?Sized,
    S: Serializer,
```

Serialize the given type-erased serializable value.

This can be used to implement `serde::Serialize` for trait objects that have `erased_serde::Serialize` as a supertrait.

```rust
trait Event: erased_serde::Serialize {
    /* ... */
}

impl<'a> serde::Serialize for dyn Event + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}
```

Since this is reasonably common, the `serialize_trait_object!` macro generates such a Serialize impl.

```rust
use erased_serde::serialize_trait_object;

serialize_trait_object!(Event);
```
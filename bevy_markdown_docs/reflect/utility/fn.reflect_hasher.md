[bevy](../../index.html)::[reflect](../index.html)::[utility](index.html)

# Function reflect\_hasher 

[Source](https://docs.rs/bevy_reflect/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect/utility.rs.html#306)

```rust
pub fn reflect_hasher() -> FoldHasher<'static>
```

Deterministic fixed state hasher to be used by implementors of [`Reflect::reflect_hash`](../../prelude/trait.Reflect.html "trait bevy::prelude::Reflect").

Hashes should be deterministic across processes so hashes can be used as checksums for saved scenes, rollback snapshots etc. This function returns such a hasher.
[bevy](../index.html)::[utils](index.html)

# Trait PreHashMapExt 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#17)

```rust
pub trait PreHashMapExt<K, V> {
    // Required method
    fn get_or_insert_with<F>(&mut self, key: &Hashed<K>, func: F) -> &mut V
       where F: FnOnce() -> V;
}
```

Extension methods intended to add functionality to [`PreHashMap`](type.PreHashMap.html "type bevy::utils::PreHashMap").

## Required Methods

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#21)

#### fn [get\_or\_insert\_with](#tymethod.get_or_insert_with)<F>(&mut self, key: &[Hashed](../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<K>, func: F) -> [&mut V](https://doc.rust-lang.org/nightly/std/primitive.reference.html)

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")() -> V,

Tries to get or insert the value for the given `key` using the pre-computed hash first. If the [`PreHashMap`](type.PreHashMap.html "type bevy::utils::PreHashMap") does not already contain the `key`, it will clone it and insert the value returned by `func`.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/map.rs.html#24)

### impl<K, V> [PreHashMapExt](trait.PreHashMapExt.html "trait bevy::utils::PreHashMapExt")<K, V> for [HashMap](../platform/collections/struct.HashMap.html "struct bevy::platform::collections::HashMap")<[Hashed](../platform/hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<K>, V, [PassHash](../platform/hash/struct.PassHash.html "struct bevy::platform::hash::PassHash")\>

where K: [Hash](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash") + [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + [PartialEq](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq") + [Clone](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html "trait core::clone::Clone"),
[bevy](../../index.html)::[platform](../index.html)::[collections](index.html)

# Trait Equivalent 

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#77)

```rust
pub trait Equivalent<K>where
    K: ?Sized,{
    // Required method
    fn equivalent(&self, key: &K) -> bool;
}
```

Key equivalence trait.

This trait allows hash table lookup to be customized. It has one blanket implementation that uses the regular solution with `Borrow` and `Eq`, just like `HashMap` does, so that you can pass `&str` to lookup into a map with `String` keys and so on.

## Contract

The implementor **must** hash like `K`, if it is hashable.

## Required Methods

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#79)

#### fn [equivalent](#tymethod.equivalent)(&self, key: [&K](https://doc.rust-lang.org/nightly/std/primitive.reference.html)) -> [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html)

Compare self to `key` and return `true` if they are equal.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#188)

### impl [Equivalent](trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<[Hashed](../hash/struct.Hashed.html "struct bevy::platform::hash::Hashed")<[InnerSceneEntityReference](../../ecs/template/struct.InnerSceneEntityReference.html "struct bevy::ecs::template::InnerSceneEntityReference")\>> for [SceneEntityReference](../../ecs/template/struct.SceneEntityReference.html "struct bevy::ecs::template::SceneEntityReference")

[Source](https://docs.rs/equivalent/1.0.2/x86_64-unknown-linux-gnu/src/equivalent/lib.rs.html#82-85)

### impl<Q, K> [Equivalent](trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<K> for Q

where Q: [Eq](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"), K: [Borrow](https://doc.rust-lang.org/nightly/core/borrow/trait.Borrow.html "trait core::borrow::Borrow")<Q> + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/handle.rs.html#407)

### impl<T> [Equivalent](trait.Equivalent.html "trait bevy::platform::collections::Equivalent")<[Handle](../../prelude/enum.Handle.html "enum bevy::prelude::Handle")<T>> for [AssetId](../../prelude/enum.AssetId.html "enum bevy::prelude::AssetId")<T>

where T: [Asset](../../prelude/trait.Asset.html "trait bevy::prelude::Asset"),
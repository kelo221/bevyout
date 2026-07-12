[bevy](../../index.html)::[platform](../index.html)::[prelude](index.html)

# Trait ToString 

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2875)

```rust
pub trait ToString {
    // Required method
    fn to_string(&self) -> String;
}
```

A trait for converting a value to a `String`.

This trait is automatically implemented for any type which implements the [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") trait. As such, `ToString` shouldn’t be implemented directly: [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") should be implemented instead, and you get the `ToString` implementation for free.

## Required Methods

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2889)

#### fn [to\_string](#tymethod.to_string)(&self) -> [String](../../prelude/struct.String.html "struct bevy::prelude::String")

Converts the given value to a `String`.

##### Examples

```rust
let i = 5;
let five = String::from("5");

assert_eq!(five, i.to_string());
```

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/src/gltf_json/mesh.rs.html#346)

### impl [ToString](../../prelude/trait.ToString.html "trait bevy::prelude::ToString") for [Checked](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/validation/enum.Checked.html "enum gltf_json::validation::Checked")<[Semantic](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/mesh/enum.Semantic.html "enum gltf_json::mesh::Semantic")\>

[Source](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/src/gltf_json/mesh.rs.html#329)

### impl [ToString](../../prelude/trait.ToString.html "trait bevy::prelude::ToString") for [Semantic](https://docs.rs/gltf-json/1.4.1/x86_64-unknown-linux-gnu/gltf_json/mesh/enum.Semantic.html "enum gltf_json::mesh::Semantic")

1.0.0 · [Source](https://doc.rust-lang.org/nightly/src/alloc/string.rs.html#2900)

### impl<T> [ToString](../../prelude/trait.ToString.html "trait bevy::prelude::ToString") for T

where T: [Display](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html "trait core::fmt::Display") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Available on **non-`no_global_oom_handling`** only.

#### Panics

In this implementation, the `to_string` method panics if the `Display` implementation returns an error. This indicates an incorrect `Display` implementation since `fmt::Write for String` never returns an error itself.
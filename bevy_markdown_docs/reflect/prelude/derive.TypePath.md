[bevy](../../index.html)::[reflect](../index.html)::[prelude](index.html)

# Derive Macro TypePath 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#480)

```rust
#[derive(TypePath)]
{
    // Attributes available to this derive:
    #[type_path]
    #[type_name]
}
```

Derives the `TypePath` trait, providing a stable alternative to \[`std::any::type_name`\].

## Container Attributes

### `#[type_path = "my_crate::foo"]`

Optionally specifies a custom module path to use instead of \[`module_path`\].

This path does not include the final identifier.

### `#[type_name = "RenamedType"]`

Optionally specifies a new terminating identifier for `TypePath`.

To use this attribute, `#[type_path = "..."]` must also be specified.
[bevy](../index.html)::[prelude](index.html)

# Derive Macro FromReflect 

[Source](https://docs.rs/bevy_reflect_derive/0.19.0/x86_64-unknown-linux-gnu/src/bevy_reflect_derive/lib.rs.html#434)

```rust
#[derive(FromReflect)]
{
    // Attributes available to this derive:
    #[reflect]
}
```

Derives the `FromReflect` trait.

## Field Attributes

### `#[reflect(ignore)]`

The `#[reflect(ignore)]` attribute is shared with the [`#[derive(Reflect)]`](derive.Reflect.html "derive bevy::prelude::Reflect") macro and has much of the same functionality in that it denotes that a field will be ignored by the reflection API.

The only major difference is that using it with this derive requires that the field implements \[`Default`\]. Without this requirement, there would be no way for `FromReflect` to automatically construct missing fields that have been ignored.

### `#[reflect(default)]`

If a field cannot be read, this attribute specifies a default value to be used in its place.

By default, this attribute denotes that the field’s type implements \[`Default`\]. However, it can also take in a path string to a user-defined function that will return the default value. This takes the form: `#[reflect(default = "path::to::my_function")]` where `my_function` is a parameterless function that must return some default value for the type.

Specifying a custom default can be used to give different fields their own specialized defaults, or to remove the `Default` requirement on fields marked with `#[reflect(ignore)]`. Additionally, either form of this attribute can be used to fill in fields that are simply missing, such as when converting a partially-constructed dynamic type to a concrete one.
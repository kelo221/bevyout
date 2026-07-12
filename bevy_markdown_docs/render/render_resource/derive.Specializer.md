[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Derive Macro Specializer 

[Source](https://docs.rs/bevy_render_macros/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render_macros/lib.rs.html#103)

```rust
#[derive(Specializer)]
{
    // Attributes available to this derive:
    #[specialize]
    #[key]
    #[base_descriptor]
}
```

Derive macro generating an impl of the trait `Specializer`

This only works for structs whose members all implement `Specializer`
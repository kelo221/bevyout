[bevy](../../index.html)::[ecs](../index.html)::[template](index.html)

# Function template 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/template.rs.html#495)

```rust
pub fn template<F, O>(func: F) -> FnTemplate<F, O>where
    F: Fn(&mut TemplateContext<'_, '_>) -> Result<O, BevyError>,
```

Returns a “free floating” template for a given `func`. This prevents the need to define a custom type for one-off templates.
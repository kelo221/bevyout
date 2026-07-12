[bevy](../../index.html)::[feathers](../index.html)::[controls](index.html)

# Function color\_swatch\_bundle 

[Source](https://docs.rs/bevy_feathers/0.19.0/x86_64-unknown-linux-gnu/src/bevy_feathers/controls/color_swatch.rs.html#77)

```rust
pub fn color_swatch_bundle<B>(overrides: B) -> impl Bundlewhere
    B: Bundle,
```

👎Deprecated since 0.19.0:

Use the color\_swatch() BSN function

Template function to spawn a color swatch.

## Arguments

*   `overrides` - a bundle of components that are merged in with the normal swatch components.
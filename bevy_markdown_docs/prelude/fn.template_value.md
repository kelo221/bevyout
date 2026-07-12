[bevy](../index.html)::[prelude](index.html)

# Function template\_value 

[Source](https://docs.rs/bevy_scene/0.19.0/x86_64-unknown-linux-gnu/src/bevy_scene/scene.rs.html#288-290)

```rust
pub fn template_value<T>(
    value: T,
) -> TemplatePatch<impl FnOnce(&mut T, &mut ResolveContext<'_>), T>where
    T: Template,
```

Returns a [`Scene`](trait.Scene.html "trait bevy::prelude::Scene") that completely overwrites the current value of a [`Template`](trait.Template.html "trait bevy::prelude::Template") `T` with the given `value`. The `value` is cloned each time the [`Template`](trait.Template.html "trait bevy::prelude::Template") is built.
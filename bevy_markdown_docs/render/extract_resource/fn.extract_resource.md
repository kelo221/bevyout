[bevy](../../index.html)::[render](../index.html)::[extract\_resource](index.html)

# Function extract\_resource 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/extract_resource.rs.html#57-61)

```rust
pub fn extract_resource<R, F>(
    commands: Commands<'_, '_>,
    main_resource: Extract<'_, '_, Option<Res<'_, <R as ExtractResource<F>>::Source>>>,
    target_resource: Option<ResMut<'_, R>>,
)where
    R: ExtractResource<F, Mutability = Mutable>,
```

This system extracts the resource of the corresponding [`Resource`](../../prelude/trait.Resource.html "trait bevy::prelude::Resource") type
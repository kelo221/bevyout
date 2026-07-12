[bevy](../../index.html)::[ui](../index.html)::[widget](index.html)

# Function update\_image\_content\_size\_system 

[Source](https://docs.rs/bevy_ui/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ui/widget/image.rs.html#319-331)

```rust
pub fn update_image_content_size_system(
    textures: Res<'_, Assets<Image>>,
    atlases: Res<'_, Assets<TextureAtlasLayout>>,
    query: Query<'_, '_, (&mut ContentSize, Ref<'_, ImageNode>, &mut ImageNodeSize, Ref<'_, ComputedUiRenderTargetInfo>), (With<Node>, Without<Text>)>,
)
```

Updates content size of the node based on the image provided
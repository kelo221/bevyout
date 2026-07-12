[bevy](../index.html)::[sprite\_render](index.html)

# Type Alias DrawSprite 

[Source](https://docs.rs/bevy_sprite_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_sprite_render/render/mod.rs.html#884)

```rust
pub type DrawSprite = (SetItemPipeline, SetSpriteViewBindGroup<0>, SetSpriteTextureBindGroup<1>, DrawSpriteBatch);
```

[`RenderCommand`](../render/render_phase/trait.RenderCommand.html "trait bevy::render::render_phase::RenderCommand") for sprite rendering.
[bevy](../../index.html)::[render](../index.html)::[render\_resource](index.html)

# Trait IntoBindingArray 

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#199)

```rust
pub trait IntoBindingArray<'b, const N: usize> {
    // Required method
    fn into_array(self) -> [BindingResource<'b>; N];
}
```

## Required Methods

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#200)

#### fn [into\_array](#tymethod.into_array)(self) -> \[[BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")<'b>; [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#216-223)

### impl<'b, T> [IntoBindingArray](trait.IntoBindingArray.html "trait bevy::render::render_resource::IntoBindingArray")<'b, 1> for [(T₁, T₂, …, Tₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where T: [IntoBinding](trait.IntoBinding.html "trait bevy::render::render_resource::IntoBinding")<'b>,

This trait is implemented for tuples up to 32 items long.

[Source](https://docs.rs/bevy_render/0.19.0/x86_64-unknown-linux-gnu/src/bevy_render/render_resource/bind_group_entries.rs.html#216-223)

#### fn [into\_array](#tymethod.into_array)(self) -> \[[BindingResource](enum.BindingResource.html "enum bevy::render::render_resource::BindingResource")<'b>; [1](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

## Implementors
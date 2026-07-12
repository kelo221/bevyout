[bevy](../../index.html)::[material](../index.html)::[bind\_group\_layout\_entries](index.html)

# Trait IntoBindGroupLayoutEntryBuilderArray 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#241)

```rust
pub trait IntoBindGroupLayoutEntryBuilderArray<const N: usize> {
    // Required method
    fn into_array(self) -> [BindGroupLayoutEntryBuilder; N];
}
```

## Required Methods

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#242)

#### fn [into\_array](#tymethod.into_array)(self) -> \[[BindGroupLayoutEntryBuilder](struct.BindGroupLayoutEntryBuilder.html "struct bevy::material::bind_group_layout_entries::BindGroupLayoutEntryBuilder"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#256-263)

### impl<T> [IntoBindGroupLayoutEntryBuilderArray](trait.IntoBindGroupLayoutEntryBuilderArray.html "trait bevy::material::bind_group_layout_entries::IntoBindGroupLayoutEntryBuilderArray")<1> for [(T₁, T₂, …, Tₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where T: [IntoBindGroupLayoutEntryBuilder](trait.IntoBindGroupLayoutEntryBuilder.html "trait bevy::material::bind_group_layout_entries::IntoBindGroupLayoutEntryBuilder"),

This trait is implemented for tuples up to 32 items long.

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#256-263)

#### fn [into\_array](#tymethod.into_array)(self) -> \[[BindGroupLayoutEntryBuilder](struct.BindGroupLayoutEntryBuilder.html "struct bevy::material::bind_group_layout_entries::BindGroupLayoutEntryBuilder"); [1](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#281)

### impl<const N: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)\> [IntoBindGroupLayoutEntryBuilderArray](trait.IntoBindGroupLayoutEntryBuilderArray.html "trait bevy::material::bind_group_layout_entries::IntoBindGroupLayoutEntryBuilderArray")<N> for \[[BindGroupLayoutEntry](../../render/render_resource/struct.BindGroupLayoutEntry.html "struct bevy::render::render_resource::BindGroupLayoutEntry"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#282)

#### fn [into\_array](#tymethod.into_array)(self) -> \[[BindGroupLayoutEntryBuilder](struct.BindGroupLayoutEntryBuilder.html "struct bevy::material::bind_group_layout_entries::BindGroupLayoutEntryBuilder"); [N](https://doc.rust-lang.org/nightly/std/primitive.array.html)\]

## Implementors
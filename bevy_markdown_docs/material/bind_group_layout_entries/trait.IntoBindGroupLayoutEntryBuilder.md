[bevy](../../index.html)::[material](../index.html)::[bind\_group\_layout\_entries](index.html)

# Trait IntoBindGroupLayoutEntryBuilder 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#208)

```rust
pub trait IntoBindGroupLayoutEntryBuilder {
    // Required method
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder;
}
```

## Required Methods

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#209)

#### fn [into\_bind\_group\_layout\_entry\_builder](#tymethod.into_bind_group_layout_entry_builder)(self) -> [BindGroupLayoutEntryBuilder](struct.BindGroupLayoutEntryBuilder.html "struct bevy::material::bind_group_layout_entries::BindGroupLayoutEntryBuilder")

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#222)

### impl [IntoBindGroupLayoutEntryBuilder](trait.IntoBindGroupLayoutEntryBuilder.html "trait bevy::material::bind_group_layout_entries::IntoBindGroupLayoutEntryBuilder") for [BindGroupLayoutEntry](../../render/render_resource/struct.BindGroupLayoutEntry.html "struct bevy::render::render_resource::BindGroupLayoutEntry")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#235)

### impl [IntoBindGroupLayoutEntryBuilder](trait.IntoBindGroupLayoutEntryBuilder.html "trait bevy::material::bind_group_layout_entries::IntoBindGroupLayoutEntryBuilder") for [BindGroupLayoutEntryBuilder](struct.BindGroupLayoutEntryBuilder.html "struct bevy::material::bind_group_layout_entries::BindGroupLayoutEntryBuilder")

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/bind_group_layout_entries.rs.html#212)

### impl [IntoBindGroupLayoutEntryBuilder](trait.IntoBindGroupLayoutEntryBuilder.html "trait bevy::material::bind_group_layout_entries::IntoBindGroupLayoutEntryBuilder") for [BindingType](../../render/render_resource/enum.BindingType.html "enum bevy::render::render_resource::BindingType")
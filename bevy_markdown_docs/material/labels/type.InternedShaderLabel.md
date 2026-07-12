[bevy](../../index.html)::[material](../index.html)::[labels](index.html)

# Type Alias InternedShaderLabel 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#15)

```rust
pub type InternedShaderLabel = Interned<dyn ShaderLabel>;
```

A shorthand for `Interned<dyn RenderSubGraph>`.

## Aliased Type

```rust
pub struct InternedShaderLabel(pub &'static dyn ShaderLabel);
```

## Tuple Fields

`0: &'static dyn [ShaderLabel](trait.ShaderLabel.html "trait bevy::material::labels::ShaderLabel")`
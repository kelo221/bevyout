[bevy](../../index.html)::[material](../index.html)::[labels](index.html)

# Type Alias InternedDrawFunctionLabel 

[Source](https://docs.rs/bevy_material/0.19.0/x86_64-unknown-linux-gnu/src/bevy_material/labels.rs.html#28)

```rust
pub type InternedDrawFunctionLabel = Interned<dyn DrawFunctionLabel>;
```

## Aliased Type

```rust
pub struct InternedDrawFunctionLabel(pub &'static dyn DrawFunctionLabel);
```

## Tuple Fields

`0: &'static dyn [DrawFunctionLabel](trait.DrawFunctionLabel.html "trait bevy::material::labels::DrawFunctionLabel")`
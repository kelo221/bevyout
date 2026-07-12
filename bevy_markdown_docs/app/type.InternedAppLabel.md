[bevy](../index.html)::[app](index.html)

# Type Alias InternedAppLabel 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/app.rs.html#51)

```rust
pub type InternedAppLabel = Interned<dyn AppLabel>;
```

A shorthand for `Interned<dyn AppLabel>`.

## Aliased Type

```rust
pub struct InternedAppLabel(pub &'static dyn AppLabel);
```

## Tuple Fields

`0: &'static dyn [AppLabel](trait.AppLabel.html "trait bevy::app::AppLabel")`
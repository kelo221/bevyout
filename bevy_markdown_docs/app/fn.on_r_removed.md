[bevy](../index.html)::[app](index.html)

# Function on\_r\_removed 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#212-216)

```rust
pub fn on_r_removed<C, F, R>(
    event: On<'_, '_, Remove, R>,
    commands: Commands<'_, '_>,
    query: Query<'_, '_, (), (With<Inherited<C>>, Without<Propagate<C>>, F)>,
)where
    C: Component + Clone + PartialEq,
    F: QueryFilter + 'static,
    R: Relationship,
```

Remove [`Inherited::<C>`](struct.Inherited.html "struct bevy::app::Inherited") when an entity loses its `R` relationship
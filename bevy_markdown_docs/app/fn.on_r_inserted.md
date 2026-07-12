[bevy](../index.html)::[app](index.html)

# Function on\_r\_inserted 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#191-200)

```rust
pub fn on_r_inserted<C, F, R>(
    event: On<'_, '_, Insert, R>,
    commands: Commands<'_, '_>,
    query: Query<'_, '_, (&R, Has<Inherited<C>>), (Without<Propagate<C>>, F)>,
    relations: Query<'_, '_, &Inherited<C>, Without<PropagateStop<C>>>,
)where
    C: Component + Clone + PartialEq,
    F: QueryFilter + 'static,
    R: Relationship,
```

Add/remove [`Inherited::<C>`](struct.Inherited.html "struct bevy::app::Inherited") when an entity gains or changes its `R` relationship
[bevy](../index.html)::[app](index.html)

# Function update\_source 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#158-165)

```rust
pub fn update_source<C, F, R>(
    commands: Commands<'_, '_>,
    changed: Query<'_, '_, (Entity, &Propagate<C>), (Or<(Changed<Propagate<C>>, Without<Inherited<C>>)>,)>,
    removed: RemovedComponents<'_, '_, Propagate<C>>,
    relationship: Query<'_, '_, &R>,
    relations: Query<'_, '_, &Inherited<C>, Without<PropagateStop<C>>>,
    sources: Query<'_, '_, (), With<Propagate<C>>>,
)where
    C: Component + Clone + PartialEq,
    F: QueryFilter,
    R: Relationship,
```

add/remove `Inherited::<C>` for entities with a direct `Propagate::<C>`
[bevy](../index.html)::[app](index.html)

# Function propagate\_output 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#305-313)

```rust
pub fn propagate_output<C, F>(
    commands: Commands<'_, '_>,
    changed: Query<'_, '_, (Entity, &Inherited<C>, Option<&C>), (Changed<Inherited<C>>, Without<PropagateOver<C>>, F)>,
    inherited_removed: RemovedComponents<'_, '_, Inherited<C>>,
    without_propagation_components: Query<'_, '_, (), (Without<PropagateOver<C>>, Without<Inherited<C>>)>,
)where
    C: Component + Clone + PartialEq,
    F: QueryFilter,
```

add/remove `C` on entities with `Inherited::<C>`
[bevy](../index.html)::[app](index.html)

# Function update\_removed\_limit 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#223-227)

```rust
pub fn update_removed_limit<C, F, R>(
    inherited: Query<'_, '_, &mut Inherited<C>>,
    removed_skip: RemovedComponents<'_, '_, PropagateOver<C>>,
    removed_stop: RemovedComponents<'_, '_, PropagateStop<C>>,
)where
    C: Component + Clone + PartialEq,
    F: QueryFilter,
    R: Relationship,
```

When `PropagateOver` or `PropagateStop` is removed, update the `Inherited::<C>` to trigger propagation
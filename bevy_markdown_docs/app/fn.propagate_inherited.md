[bevy](../index.html)::[app](index.html)

# Function propagate\_inherited 

[Source](https://docs.rs/bevy_app/0.19.0/x86_64-unknown-linux-gnu/src/bevy_app/propagate.rs.html#241-257)

```rust
pub fn propagate_inherited<C, F, R>(
    commands: Commands<'_, '_>,
    changed: Query<'_, '_, (&Inherited<C>, &<R as Relationship>::RelationshipTarget), (Changed<Inherited<C>>, Without<PropagateStop<C>>, F)>,
    recurse: Query<'_, '_, (Option<&<R as Relationship>::RelationshipTarget>, Option<&Inherited<C>>, Option<&PropagateStop<C>>), (Without<Propagate<C>>, F)>,
    removed: RemovedComponents<'_, '_, Inherited<C>>,
    to_process: Local<'_, Vec<(Entity, Option<Inherited<C>>)>>,
)where
    C: Component + Clone + PartialEq,
    F: QueryFilter,
    R: Relationship,
```

add/remove `Inherited::<C>` for targets of entities with modified `Inherited::<C>`
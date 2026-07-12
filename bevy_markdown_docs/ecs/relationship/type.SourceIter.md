[bevy](../../index.html)::[ecs](../index.html)::[relationship](index.html)

# Type Alias SourceIter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/relationship/mod.rs.html#265)

```rust
pub type SourceIter<'w, R> = <<R as RelationshipTarget>::Collection as RelationshipSourceCollection>::SourceIter<'w>;
```

The iterator type for the source entities in a [`RelationshipTarget`](../../prelude/trait.RelationshipTarget.html "trait bevy::prelude::RelationshipTarget") collection, as defined in the [`RelationshipSourceCollection`](trait.RelationshipSourceCollection.html "trait bevy::ecs::relationship::RelationshipSourceCollection") trait.
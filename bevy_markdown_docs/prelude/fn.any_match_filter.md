[bevy](../index.html)::[prelude](index.html)

# Function any\_match\_filter 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1206)

```rust
pub fn any_match_filter<F>(query: Query<'_, '_, (), F>) -> boolwhere
    F: QueryFilter,
```

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities that match the given [`QueryFilter`](../ecs/query/trait.QueryFilter.html "trait bevy::ecs::query::QueryFilter").

For a simple `With<T>` filter, this is equivalent to [`any_with_component::<T>()`](fn.any_with_component.html "fn bevy::prelude::any_with_component").

To skip a system with a [`Query`](struct.Query.html "struct bevy::prelude::Query") parameter if the query is empty, you may instead use [`Populated`](struct.Populated.html "struct bevy::prelude::Populated"), if the query may match multiple entities, or [`Single`](struct.Single.html "struct bevy::prelude::Single"), if it will only match one.
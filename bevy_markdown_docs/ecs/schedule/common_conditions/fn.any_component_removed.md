[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)::[common\_conditions](index.html)

# Function any\_component\_removed 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1189)

```rust
pub fn any_component_removed<T>(removals: RemovedComponents<'_, '_, T>) -> boolwhere
    T: Component,
```

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entity with a component of the given type removed.
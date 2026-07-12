[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Function assert\_is\_read\_only\_system 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#374-379)

```rust
pub fn assert_is_read_only_system<In, Out, Marker, S>(system: S)where
    In: SystemInput,
    Out: 'static,
    S: IntoSystem<In, Out, Marker>,
    <S as IntoSystem<In, Out, Marker>>::System: ReadOnlySystem,
```

Ensure that a given function is a [read-only system](../../prelude/trait.ReadOnlySystem.html "trait bevy::prelude::ReadOnlySystem").

This should be used when writing doc examples, to confirm that systems used in an example are valid systems.

## Examples

The following example will fail to compile since the system accesses a component mutably.

[ⓘ](# "This example deliberately fails to compile")

```rust
fn my_system(query: Query<&mut Transform>) {
    // ...
}

assert_is_read_only_system(my_system);
```
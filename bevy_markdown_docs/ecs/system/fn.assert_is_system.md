[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Function assert\_is\_system 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#341-343)

```rust
pub fn assert_is_system<In, Out, Marker>(
    system: impl IntoSystem<In, Out, Marker>,
)where
    In: SystemInput,
    Out: 'static,
```

Ensure that a given function is a [system](../../prelude/trait.System.html "trait bevy::prelude::System").

This should be used when writing doc examples, to confirm that systems used in an example are valid systems.

## Examples

The following example will panic when run since the system’s parameters mutably access the same component multiple times.

[ⓘ](# "This example panics")

```rust
fn my_system(query1: Query<&mut Transform>, query2: Query<&mut Transform>) {
    // ...
}

assert_is_system(my_system);
```
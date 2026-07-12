[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Function assert\_system\_does\_not\_conflict 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/mod.rs.html#389)

```rust
pub fn assert_system_does_not_conflict<Out, Params, S>(sys: S)where
    S: IntoSystem<(), Out, Params>,
```

Ensures that the provided system doesn’t conflict with itself.

This function will panic if the provided system conflict with itself.

Note: this will run the system on an empty world.
[bevy](../../index.html)::[ecs](../index.html)::[system](index.html)

# Type Alias ScheduleSystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/schedule_system.rs.html#199)

```rust
pub type ScheduleSystem = Box<dyn System<Out = (), In = ()>>;
```

Type alias for a `BoxedSystem` that a `Schedule` can store.

## Aliased Type

```rust
pub struct ScheduleSystem(/* private fields */);
```
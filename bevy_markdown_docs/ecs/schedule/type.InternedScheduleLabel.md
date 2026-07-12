[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias InternedScheduleLabel 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#181)

```rust
pub type InternedScheduleLabel = Interned<dyn ScheduleLabel>;
```

A shorthand for `Interned<dyn ScheduleLabel>`.

## Aliased Type

```rust
pub struct InternedScheduleLabel(pub &'static dyn ScheduleLabel);
```

## Tuple Fields

`0: &'static dyn [ScheduleLabel](trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel")`
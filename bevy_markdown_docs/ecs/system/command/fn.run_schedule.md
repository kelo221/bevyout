[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[command](index.html)

# Function run\_schedule 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/command.rs.html#270)

```rust
pub fn run_schedule(label: impl ScheduleLabel) -> impl Command
```

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the schedule corresponding to the given [`ScheduleLabel`](../../schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel").
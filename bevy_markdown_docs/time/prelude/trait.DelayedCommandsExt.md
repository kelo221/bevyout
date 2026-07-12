[bevy](../../index.html)::[time](../index.html)::[prelude](index.html)

# Trait DelayedCommandsExt 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#63)

```rust
pub trait DelayedCommandsExt<'w> {
    // Required method
    fn delayed(&mut self) -> DelayedCommands<'w, '_>;
}
```

Extension trait for [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") that provides delayed command functionality.

## Required Methods

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#107)

#### fn [delayed](#tymethod.delayed)(&mut self) -> [DelayedCommands](../struct.DelayedCommands.html "struct bevy::time::DelayedCommands")<'w, '\_>

Returns a [`DelayedCommands`](../struct.DelayedCommands.html "struct bevy::time::DelayedCommands") instance that can be used to queue commands to be submitted at a later point in time.

When dropped, the [`DelayedCommands`](../struct.DelayedCommands.html "struct bevy::time::DelayedCommands") submits spawn commands that will spawn [`DelayedCommandQueue`](../struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue") entities. The entities are checked by the [`check_delayed_command_queues`](../fn.check_delayed_command_queues.html "fn bevy::time::check_delayed_command_queues") system, and their queues are submitted when the specified time has elapsed.

##### Usage

```rust
fn my_system(mut commands: Commands) {
    // Spawn an entity after one second
    commands.delayed().secs(1.0).spawn_empty();
}
```

Entity allocation happens immediately even if the spawn command is delayed. This allows you to queue delayed commands on an entity that hasn’t been spawned yet.

```rust
fn my_system(mut commands: Commands) {
    let mut delayed = commands.delayed();
    // spawn an entity after 1 second, then despawn it a second later
    let entity = delayed.secs(1.0).spawn_empty().id();
    delayed.secs(2.0).entity(entity).despawn();
}
```

##### Timing

Delayed commands are currently checked against the default clock in the [`PreUpdate`](../../prelude/struct.PreUpdate.html "struct bevy::prelude::PreUpdate") schedule. There’s currently no way to specify different clocks for different delayed commands - this is a limitation of the system and if you need this behavior you’ll likely have to implement your own delay system.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/delayed_commands.rs.html#110)

### impl<'w, 's> [DelayedCommandsExt](../../prelude/trait.DelayedCommandsExt.html "trait bevy::prelude::DelayedCommandsExt")<'w> for [Commands](../../prelude/struct.Commands.html "struct bevy::prelude::Commands")<'w, 's>
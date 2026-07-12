[bevy](../index.html)::[prelude](index.html)

# Trait CommandsStatesExt 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/commands.rs.html#7)

```rust
pub trait CommandsStatesExt {
    // Required methods
    fn set_state<S>(&mut self, state: S)
       where S: FreelyMutableState;
    fn set_state_if_neq<S>(&mut self, state: S)
       where S: FreelyMutableState;
}
```

Extension trait for [`Commands`](struct.Commands.html "struct bevy::prelude::Commands") adding `bevy_state` helpers.

## Required Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/commands.rs.html#15)

#### fn [set\_state](#tymethod.set_state)<S>(&mut self, state: S)

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"),

Sets the next state the app should move to.

Internally this schedules a command that updates the [`NextState<S>`](enum.NextState.html "enum bevy::prelude::NextState") resource with `state`.

Note that commands introduce sync points to the ECS schedule, so modifying `NextState` directly may be more efficient depending on your use-case.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/commands.rs.html#24)

#### fn [set\_state\_if\_neq](#tymethod.set_state_if_neq)<S>(&mut self, state: S)

where S: [FreelyMutableState](../state/state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState"),

Sets the next state the app should move to, skipping any state transitions if the next state is the same as the current state.

Internally this schedules a command that updates the [`NextState<S>`](enum.NextState.html "enum bevy::prelude::NextState") resource with `state`.

Note that commands introduce sync points to the ECS schedule, so modifying `NextState` directly may be more efficient depending on your use-case.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/commands.rs.html#27)

### impl [CommandsStatesExt](trait.CommandsStatesExt.html "trait bevy::prelude::CommandsStatesExt") for [Commands](struct.Commands.html "struct bevy::prelude::Commands")<'\_, '\_>
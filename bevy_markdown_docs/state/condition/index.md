[bevy](../../index.html)::[state](../index.html)

# Module condition 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/lib.rs.html#59)

Provides definitions for the runtime conditions that interact with the state system

## Functions

[in\_state](fn.in_state.html "fn bevy::state::condition::in_state")

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the state machine is currently in `state`.

[state\_changed](fn.state_changed.html "fn bevy::state::condition::state_changed")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine changed state.

[state\_exists](fn.state_exists.html "fn bevy::state::condition::state_exists")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine exists.
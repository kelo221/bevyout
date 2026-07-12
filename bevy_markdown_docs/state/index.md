[bevy](../index.html)

# Crate state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/lib.rs.html#1-155)

In Bevy, states are app-wide interdependent, finite state machines that are generally used to model the large scale structure of your program: whether a game is paused, if the player is in combat, if assets are loaded and so on.

This module provides 3 distinct types of state, all of which implement the [`States`](../prelude/trait.States.html "trait bevy::prelude::States") trait:

*   Standard [`States`](../prelude/trait.States.html "trait bevy::prelude::States") can only be changed by manually setting the [`NextState<S>`](../prelude/enum.NextState.html "enum bevy::prelude::NextState") resource. These states are the baseline on which the other state types are built, and can be used on their own for many simple patterns. See the [states example](https://github.com/bevyengine/bevy/blob/latest/examples/state/states.rs) for a simple use case.
*   [`SubStates`](../prelude/trait.SubStates.html "trait bevy::prelude::SubStates") are children of other states - they can be changed manually using [`NextState<S>`](../prelude/enum.NextState.html "enum bevy::prelude::NextState"), but are removed from the [`World`](../prelude/struct.World.html "struct bevy::prelude::World") if the source states aren’t in the right state. See the [sub\_states example](https://github.com/bevyengine/bevy/blob/latest/examples/state/sub_states.rs) for a simple use case based on the derive macro, or read the trait docs for more complex scenarios.
*   [`ComputedStates`](../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates") are fully derived from other states - they provide a [`compute`](../prelude/trait.ComputedStates.html#tymethod.compute "associated function bevy::prelude::ComputedStates::compute") method that takes in the source states and returns their derived value. They are particularly useful for situations where a simplified view of the source states is necessary - such as having an `InAMenu` computed state, derived from a source state that defines multiple distinct menus. See the [computed state example](https://github.com/bevyengine/bevy/blob/latest/examples/state/computed_states.rs) to see usage samples for these states.

Most of the utilities around state involve running systems during transitions between states, or determining whether to run certain systems, though they can be used more directly as well. This makes it easier to transition between menus, add loading screens, pause games, and more.

Specifically, Bevy provides the following utilities:

*   3 Transition Schedules - [`OnEnter<S>`](../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter"), [`OnExit<S>`](../prelude/struct.OnExit.html "struct bevy::prelude::OnExit") and [`OnTransition<S>`](../prelude/struct.OnTransition.html "struct bevy::prelude::OnTransition") - which are used to trigger systems specifically during matching transitions.
*   A [`StateTransitionEvent<S>`](../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") that gets fired when a given state changes.
*   The [`in_state<S>`](../prelude/fn.in_state.html "fn bevy::prelude::in_state") and [`state_changed<S>`](../prelude/fn.state_changed.html "fn bevy::prelude::state_changed") run conditions - which are used to determine whether a system should run based on the current state.

Bevy also provides functionality for managing the lifetime of entities in the context of game states, using the [`state_scoped`](state_scoped/index.html "mod bevy::state::state_scoped") module. Specifically, the marker components [`DespawnOnEnter<S>`](../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter") and [`DespawnOnExit<S>`](../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit") are provided for despawning entities on state transition. This, especially in combination with system scheduling, enables a flexible and expressive way to manage spawning and despawning entities.

## Modules

[app](app/index.html "mod bevy::state::app")`bevy_app`

Provides [`App`](../prelude/struct.App.html "struct bevy::prelude::App") and [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with state installation methods

[commands](commands/index.html "mod bevy::state::commands")

Provides extension methods for [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands").

[condition](condition/index.html "mod bevy::state::condition")

Provides definitions for the runtime conditions that interact with the state system

[prelude](prelude/index.html "mod bevy::state::prelude")

The state prelude.

[reflect](reflect/index.html "mod bevy::state::reflect")`bevy_reflect`

Provides definitions for the basic traits required by the state system

[state](state/index.html "mod bevy::state::state")

Provides definitions for the basic traits required by the state system

[state\_scoped](state_scoped/index.html "mod bevy::state::state_scoped")

Provides tools for managing the lifetime of entities based on state transitions.

[state\_scoped\_events](state_scoped_events/index.html "mod bevy::state::state_scoped_events")`bevy_app`

Provides [`App`](../prelude/struct.App.html "struct bevy::prelude::App") and [`SubApp`](../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with methods for registering state-scoped events.
[bevy](../../index.html)::[state](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/lib.rs.html#77)

The state prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[DespawnOnEnter](struct.DespawnOnEnter.html "struct bevy::state::prelude::DespawnOnEnter")

Entities marked with this component will be despawned upon entering the given state.

[DespawnOnExit](struct.DespawnOnExit.html "struct bevy::state::prelude::DespawnOnExit")

Entities marked with this component will be despawned upon exiting the given state.

[DespawnWhen](struct.DespawnWhen.html "struct bevy::state::prelude::DespawnWhen")

Entities marked with this component will be despawned when a [`StateTransitionEvent<S>`](../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[DisableOnEnter](struct.DisableOnEnter.html "struct bevy::state::prelude::DisableOnEnter")

Entities marked with this component will be disabled upon entering the given state.

[DisableOnExit](struct.DisableOnExit.html "struct bevy::state::prelude::DisableOnExit")

Entities marked with this component will be disabled upon exiting the given state.

[DisableWhen](struct.DisableWhen.html "struct bevy::state::prelude::DisableWhen")

Entities marked with this component will be disabled when a [`StateTransitionEvent<S>`](../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[EnableOnEnter](struct.EnableOnEnter.html "struct bevy::state::prelude::EnableOnEnter")

Entities marked with this component will be enabled upon entering the given state.

[EnableOnExit](struct.EnableOnExit.html "struct bevy::state::prelude::EnableOnExit")

Entities marked with this component will be enabled upon exiting the given state.

[EnableWhen](struct.EnableWhen.html "struct bevy::state::prelude::EnableWhen")

Entities marked with this component will be enabled when a [`StateTransitionEvent<S>`](../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[EnterSchedules](struct.EnterSchedules.html "struct bevy::state::prelude::EnterSchedules")

System set that runs enter schedule(s) for state `S`.

[ExitSchedules](struct.ExitSchedules.html "struct bevy::state::prelude::ExitSchedules")

System set that runs exit schedule(s) for state `S`.

[OnEnter](struct.OnEnter.html "struct bevy::state::prelude::OnEnter")

The label of a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") enters the provided state.

[OnExit](struct.OnExit.html "struct bevy::state::prelude::OnExit")

The label of a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") exits the provided state.

[OnTransition](struct.OnTransition.html "struct bevy::state::prelude::OnTransition")

The label of a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") exits AND enters the provided `exited` and `entered` states.

[PreviousState](struct.PreviousState.html "struct bevy::state::prelude::PreviousState")

The previous state of [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State").

[ReflectFreelyMutableState](struct.ReflectFreelyMutableState.html "struct bevy::state::prelude::ReflectFreelyMutableState")

A struct used to operate on the reflected [`FreelyMutableState`](../state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") trait of a type.

[ReflectState](struct.ReflectState.html "struct bevy::state::prelude::ReflectState")

A struct used to operate on the reflected [`States`](../../prelude/trait.States.html "trait bevy::prelude::States") trait of a type.

[State](struct.State.html "struct bevy::state::prelude::State")

A finite-state machine whose transitions have associated schedules ([`OnEnter(state)`](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter") and [`OnExit(state)`](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit")).

[StateTransition](struct.StateTransition.html "struct bevy::state::prelude::StateTransition")

Runs [state transitions](../../prelude/trait.States.html "trait bevy::prelude::States").

[StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::state::prelude::StateTransitionEvent")

A [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") sent when any state transition of `S` happens. This includes identity transitions, where `exited` and `entered` have the same value.

[TransitionSchedules](struct.TransitionSchedules.html "struct bevy::state::prelude::TransitionSchedules")

System set that runs transition schedule(s) for state `S`.

## Enums

[NextState](enum.NextState.html "enum bevy::state::prelude::NextState")

The next state of [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State").

## Traits

[AppExtStates](trait.AppExtStates.html "trait bevy::state::prelude::AppExtStates")

State installation methods for [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") and [`SubApp`](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp").

[CommandsStatesExt](trait.CommandsStatesExt.html "trait bevy::state::prelude::CommandsStatesExt")

Extension trait for [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") adding `bevy_state` helpers.

[ComputedStates](trait.ComputedStates.html "trait bevy::state::prelude::ComputedStates")

A state whose value is automatically computed based on the values of other [`States`](../../prelude/trait.States.html "trait bevy::prelude::States").

[StateScopedMessagesAppExt](trait.StateScopedMessagesAppExt.html "trait bevy::state::prelude::StateScopedMessagesAppExt")

Extension trait for [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") adding methods for registering state scoped messages.

[StateSet](trait.StateSet.html "trait bevy::state::prelude::StateSet")

A [`States`](../../prelude/trait.States.html "trait bevy::prelude::States") type or tuple of types which implement [`States`](../../prelude/trait.States.html "trait bevy::prelude::States").

[States](trait.States.html "trait bevy::state::prelude::States")

Types that can define world-wide states in a finite-state machine.

[SubStates](trait.SubStates.html "trait bevy::state::prelude::SubStates")

A sub-state is a state that exists only when the source state meet certain conditions, but unlike [`ComputedStates`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates") - while they exist they can be manually modified.

## Functions

[in\_state](fn.in_state.html "fn bevy::state::prelude::in_state")

Generates a [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the state machine is currently in `state`.

[last\_transition](fn.last_transition.html "fn bevy::state::prelude::last_transition")

Returns the latest state transition event of type `S`, if any are available.

[state\_changed](fn.state_changed.html "fn bevy::state::prelude::state_changed")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine changed state.

[state\_exists](fn.state_exists.html "fn bevy::state::prelude::state_exists")

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine exists.

## Derive Macros

[States](derive.States.html "derive bevy::state::prelude::States")

Implements the `States` trait for a type - see the trait docs for an example usage.

[SubStates](derive.SubStates.html "derive bevy::state::prelude::SubStates")

Implements the `SubStates` trait for a type - see the trait docs for an example usage.
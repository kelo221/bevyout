[bevy](../../index.html)::[state](../index.html)

# Module state 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/lib.rs.html#61)

Provides definitions for the basic traits required by the state system

## Structs

[EnterSchedules](struct.EnterSchedules.html "struct bevy::state::state::EnterSchedules")

System set that runs enter schedule(s) for state `S`.

[ExitSchedules](struct.ExitSchedules.html "struct bevy::state::state::ExitSchedules")

System set that runs exit schedule(s) for state `S`.

[OnEnter](struct.OnEnter.html "struct bevy::state::state::OnEnter")

The label of a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") enters the provided state.

[OnExit](struct.OnExit.html "struct bevy::state::state::OnExit")

The label of a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") exits the provided state.

[OnTransition](struct.OnTransition.html "struct bevy::state::state::OnTransition")

The label of a [`Schedule`](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule") that **only** runs whenever [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State") exits AND enters the provided `exited` and `entered` states.

[PreviousState](struct.PreviousState.html "struct bevy::state::state::PreviousState")

The previous state of [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State").

[State](struct.State.html "struct bevy::state::state::State")

A finite-state machine whose transitions have associated schedules ([`OnEnter(state)`](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter") and [`OnExit(state)`](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit")).

[StateTransition](struct.StateTransition.html "struct bevy::state::state::StateTransition")

Runs [state transitions](../../prelude/trait.States.html "trait bevy::prelude::States").

[StateTransitionEvent](struct.StateTransitionEvent.html "struct bevy::state::state::StateTransitionEvent")

A [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") sent when any state transition of `S` happens. This includes identity transitions, where `exited` and `entered` have the same value.

[TransitionSchedules](struct.TransitionSchedules.html "struct bevy::state::state::TransitionSchedules")

System set that runs transition schedule(s) for state `S`.

## Enums

[NextState](enum.NextState.html "enum bevy::state::state::NextState")

The next state of [`State<S>`](../../prelude/struct.State.html "struct bevy::prelude::State").

[StateTransitionSystems](enum.StateTransitionSystems.html "enum bevy::state::state::StateTransitionSystems")

Applies state transitions and runs transitions schedules in order.

## Traits

[ComputedStates](trait.ComputedStates.html "trait bevy::state::state::ComputedStates")

A state whose value is automatically computed based on the values of other [`States`](../../prelude/trait.States.html "trait bevy::prelude::States").

[FreelyMutableState](trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState")

This trait allows a state to be mutated directly using the [`NextState<S>`](../../prelude/enum.NextState.html "enum bevy::prelude::NextState") resource.

[StateSet](trait.StateSet.html "trait bevy::state::state::StateSet")

A [`States`](../../prelude/trait.States.html "trait bevy::prelude::States") type or tuple of types which implement [`States`](../../prelude/trait.States.html "trait bevy::prelude::States").

[States](trait.States.html "trait bevy::state::state::States")

Types that can define world-wide states in a finite-state machine.

[SubStates](trait.SubStates.html "trait bevy::state::state::SubStates")

A sub-state is a state that exists only when the source state meet certain conditions, but unlike [`ComputedStates`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates") - while they exist they can be manually modified.

## Functions

[last\_transition](fn.last_transition.html "fn bevy::state::state::last_transition")

Returns the latest state transition event of type `S`, if any are available.

[setup\_state\_transitions\_in\_world](fn.setup_state_transitions_in_world.html "fn bevy::state::state::setup_state_transitions_in_world")

Sets up the schedules and systems for handling state transitions within a [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

## Derive Macros

[States](derive.States.html "derive bevy::state::state::States")

Implements the `States` trait for a type - see the trait docs for an example usage.

[SubStates](derive.SubStates.html "derive bevy::state::state::SubStates")

Implements the `SubStates` trait for a type - see the trait docs for an example usage.
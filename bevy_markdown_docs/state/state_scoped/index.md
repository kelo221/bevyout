[bevy](../../index.html)::[state](../index.html)

# Module state\_scoped 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/lib.rs.html#64)

Provides tools for managing the lifetime of entities based on state transitions.

## Structs

[DespawnOnEnter](struct.DespawnOnEnter.html "struct bevy::state::state_scoped::DespawnOnEnter")

Entities marked with this component will be despawned upon entering the given state.

[DespawnOnExit](struct.DespawnOnExit.html "struct bevy::state::state_scoped::DespawnOnExit")

Entities marked with this component will be despawned upon exiting the given state.

[DespawnWhen](struct.DespawnWhen.html "struct bevy::state::state_scoped::DespawnWhen")

Entities marked with this component will be despawned when a [`StateTransitionEvent<S>`](../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[DisableOnEnter](struct.DisableOnEnter.html "struct bevy::state::state_scoped::DisableOnEnter")

Entities marked with this component will be disabled upon entering the given state.

[DisableOnExit](struct.DisableOnExit.html "struct bevy::state::state_scoped::DisableOnExit")

Entities marked with this component will be disabled upon exiting the given state.

[DisableWhen](struct.DisableWhen.html "struct bevy::state::state_scoped::DisableWhen")

Entities marked with this component will be disabled when a [`StateTransitionEvent<S>`](../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

[EnableOnEnter](struct.EnableOnEnter.html "struct bevy::state::state_scoped::EnableOnEnter")

Entities marked with this component will be enabled upon entering the given state.

[EnableOnExit](struct.EnableOnExit.html "struct bevy::state::state_scoped::EnableOnExit")

Entities marked with this component will be enabled upon exiting the given state.

[EnableWhen](struct.EnableWhen.html "struct bevy::state::state_scoped::EnableWhen")

Entities marked with this component will be enabled when a [`StateTransitionEvent<S>`](../../prelude/struct.StateTransitionEvent.html "struct bevy::prelude::StateTransitionEvent") matching the given predicate is sent.

## Functions

[despawn\_entities\_on\_enter\_state](fn.despawn_entities_on_enter_state.html "fn bevy::state::state_scoped::despawn_entities_on_enter_state")

Despawns entities marked with [`DespawnOnEnter<S>`](../../prelude/struct.DespawnOnEnter.html "struct bevy::prelude::DespawnOnEnter") when their state matches the world state.

[despawn\_entities\_on\_exit\_state](fn.despawn_entities_on_exit_state.html "fn bevy::state::state_scoped::despawn_entities_on_exit_state")

Despawns entities marked with [`DespawnOnExit<S>`](../../prelude/struct.DespawnOnExit.html "struct bevy::prelude::DespawnOnExit") when their state no longer matches the world state.

[despawn\_entities\_when\_state](fn.despawn_entities_when_state.html "fn bevy::state::state_scoped::despawn_entities_when_state")

Despawns entities marked with [`DespawnWhen<S>`](../../prelude/struct.DespawnWhen.html "struct bevy::prelude::DespawnWhen") when the state transition message matches their predicate.

[disable\_entities\_on\_enter\_state](fn.disable_entities_on_enter_state.html "fn bevy::state::state_scoped::disable_entities_on_enter_state")

Disables entities marked with [`DisableOnEnter<S>`](../../prelude/struct.DisableOnEnter.html "struct bevy::prelude::DisableOnEnter") when their state matches the world state.

[disable\_entities\_on\_exit\_state](fn.disable_entities_on_exit_state.html "fn bevy::state::state_scoped::disable_entities_on_exit_state")

Disables entities marked with [`DisableOnExit<S>`](../../prelude/struct.DisableOnExit.html "struct bevy::prelude::DisableOnExit") when their state no longer matches the world state.

[disable\_entities\_when\_state](fn.disable_entities_when_state.html "fn bevy::state::state_scoped::disable_entities_when_state")

Disable entities marked with [`DisableWhen<S>`](../../prelude/struct.DisableWhen.html "struct bevy::prelude::DisableWhen") when the state transition message matches their predicate.

[enable\_entities\_on\_enter\_state](fn.enable_entities_on_enter_state.html "fn bevy::state::state_scoped::enable_entities_on_enter_state")

Enables entities marked with [`EnableOnEnter<S>`](../../prelude/struct.EnableOnEnter.html "struct bevy::prelude::EnableOnEnter") when their state matches the world state.

[enable\_entities\_on\_exit\_state](fn.enable_entities_on_exit_state.html "fn bevy::state::state_scoped::enable_entities_on_exit_state")

Enables entities marked with [`EnableOnExit<S>`](../../prelude/struct.EnableOnExit.html "struct bevy::prelude::EnableOnExit") when their state no longer matches the world state.

[enable\_entities\_when\_state](fn.enable_entities_when_state.html "fn bevy::state::state_scoped::enable_entities_when_state")

Enable entities marked with [`EnableWhen<S>`](../../prelude/struct.EnableWhen.html "struct bevy::prelude::EnableWhen") when the state transition message matches their predicate.
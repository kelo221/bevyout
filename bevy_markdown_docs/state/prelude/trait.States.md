[bevy](../../index.html)::[state](../index.html)::[prelude](index.html)

# Trait States 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/states.rs.html#64)

```rust
pub trait States:
    'static
    + Send
    + Sync
    + Clone
    + PartialEq
    + Eq
    + Hash
    + Debug {
    const DEPENDENCY_DEPTH: usize = 1;
}
```

Types that can define world-wide states in a finite-state machine.

The [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html "trait core::default::Default") trait defines the starting state. Multiple states can be defined for the same world, allowing you to classify the state of the world across orthogonal dimensions. You can access the current state of type `T` with the [`State<T>`](../../prelude/struct.State.html "struct bevy::prelude::State") resource, and the queued state with the [`NextState<T>`](../../prelude/enum.NextState.html "enum bevy::prelude::NextState") resource.

State transitions typically occur in the [`OnEnter<T::Variant>`](../../prelude/struct.OnEnter.html "struct bevy::prelude::OnEnter") and [`OnExit<T::Variant>`](../../prelude/struct.OnExit.html "struct bevy::prelude::OnExit") schedules, which can be run by triggering the [`StateTransition`](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition") schedule.

Types used as [`ComputedStates`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates") do not need to and should not derive [`States`](../../prelude/trait.States.html "trait bevy::prelude::States"). [`ComputedStates`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates") should not be manually mutated: functionality provided by the [`States`](../../prelude/trait.States.html "trait bevy::prelude::States") derive and the associated [`FreelyMutableState`](../state/trait.FreelyMutableState.html "trait bevy::state::state::FreelyMutableState") trait.

## Example

```rust
use bevy_state::prelude::*;
use bevy_ecs::prelude::IntoScheduleConfigs;
use bevy_ecs::system::{ResMut, ScheduleSystem};


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    MainMenu,
    SettingsMenu,
    InGame,
}

fn handle_escape_pressed(mut next_state: ResMut<NextState<GameState>>) {
    if escape_pressed {
        next_state.set(GameState::SettingsMenu);
    }
}

fn open_settings_menu() {
    // Show the settings menu...
}


app.init_state::<GameState>();
app.add_systems(Update, handle_escape_pressed.run_if(in_state(GameState::MainMenu)));
app.add_systems(OnEnter(GameState::SettingsMenu), open_settings_menu);
```

## Provided Associated Constants

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/states.rs.html#68)

#### const [DEPENDENCY\_DEPTH](#associatedconstant.DEPENDENCY_DEPTH): [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html) = 1

How many other states this state depends on. Used to help order transitions and de-duplicate [`ComputedStates`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates"), as well as prevent cyclical `ComputedState` dependencies.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/computed_states.rs.html#95)

### impl<S> [States](../../prelude/trait.States.html "trait bevy::prelude::States") for S

where S: [ComputedStates](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates"),

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/computed_states.rs.html#96)

#### const [DEPENDENCY\_DEPTH](#associatedconstant.DEPENDENCY_DEPTH): [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)
[bevy](../index.html)::[prelude](index.html)

# Trait SubStates 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/sub_states.rs.html#148)

```rust
pub trait SubStates: States + FreelyMutableState {
    type SourceStates: StateSet;

    // Required method
    fn should_exist(sources: Self::SourceStates) -> Option<Self>;

    // Provided method
    fn register_sub_state_systems(schedule: &mut Schedule) { ... }
}
```

A sub-state is a state that exists only when the source state meet certain conditions, but unlike [`ComputedStates`](trait.ComputedStates.html "trait bevy::prelude::ComputedStates") - while they exist they can be manually modified.

The default approach to creating [`SubStates`](trait.SubStates.html "trait bevy::prelude::SubStates") is using the derive macro, and defining a single source state and value to determine its existence.

```rust
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Menu,
    InGame
}


#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(AppState = AppState::InGame)]
enum GamePhase {
    #[default]
    Setup,
    Battle,
    Conclusion
}
```

you can then add it to an App, and from there you use the state as normal:

```rust
App::new()
        .init_state::<AppState>()
        .add_sub_state::<GamePhase>();
```

In more complex situations, the recommendation is to use an intermediary computed state, like so:

```rust
/// Computed States require some state to derive from
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Menu,
    InGame { paused: bool }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct InGame;

impl ComputedStates for InGame {
    /// We set the source state to be the state, or set of states,
    /// we want to depend on. Any of the states can be wrapped in an Option.
    type SourceStates = Option<AppState>;

    /// We then define the compute function, which takes in the AppState
    fn compute(sources: Option<AppState>) -> Option<Self> {
        match sources {
            /// When we are in game, we want to return the InGame state
            Some(AppState::InGame { .. }) => Some(InGame),
            /// Otherwise, we don't want the `State<InGame>` resource to exist,
            /// so we return None.
            _ => None
        }
    }
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(InGame = InGame)]
enum GamePhase {
    #[default]
    Setup,
    Battle,
    Conclusion
}
```

However, you can also manually implement them. If you do so, you’ll also need to manually implement the `States` & `FreelyMutableState` traits.

```rust
/// Computed States require some state to derive from
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
enum AppState {
    #[default]
    Menu,
    InGame { paused: bool }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum GamePhase {
    Setup,
    Battle,
    Conclusion
}

impl SubStates for GamePhase {
    /// We set the source state to be the state, or set of states,
    /// we want to depend on. Any of the states can be wrapped in an Option.
    type SourceStates = Option<AppState>;

    /// We then define the compute function, which takes in the [`Self::SourceStates`]
    fn should_exist(sources: Option<AppState>) -> Option<Self> {
        match sources {
            /// When we are in game, we want a GamePhase state to exist.
            /// We can set the initial value here or overwrite it through [`NextState`].
            Some(AppState::InGame { .. }) => Some(Self::Setup),
            /// If we don't want the `State<GamePhase>` resource to exist we return [`None`].
            _ => None
        }
    }
}

impl States for GamePhase {
    const DEPENDENCY_DEPTH : usize = <GamePhase as SubStates>::SourceStates::SET_DEPENDENCY_DEPTH + 1;
}

impl FreelyMutableState for GamePhase {}
```

## Required Associated Types

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/sub_states.rs.html#154)

#### type [SourceStates](#associatedtype.SourceStates): [StateSet](trait.StateSet.html "trait bevy::prelude::StateSet")

The set of states from which the [`Self`](trait.SubStates.html "trait bevy::prelude::SubStates") is derived.

This can either be a single type that implements [`States`](trait.States.html "trait bevy::prelude::States"), or a tuple containing multiple types that implement [`States`](trait.States.html "trait bevy::prelude::States"), or any combination of types implementing [`States`](trait.States.html "trait bevy::prelude::States") and Options of types implementing [`States`](trait.States.html "trait bevy::prelude::States").

## Required Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/sub_states.rs.html#167)

#### fn [should\_exist](#tymethod.should_exist)(sources: Self::[SourceStates](trait.SubStates.html#associatedtype.SourceStates "type bevy::prelude::SubStates::SourceStates")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self>

This function gets called whenever one of the [`SourceStates`](trait.SubStates.html#associatedtype.SourceStates "associated type bevy::prelude::SubStates::SourceStates") changes. The result is used to determine the existence of [`State<Self>`](struct.State.html "struct bevy::prelude::State").

If the result is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), the [`State<Self>`](struct.State.html "struct bevy::prelude::State") resource will be removed from the world, otherwise if the [`State<Self>`](struct.State.html "struct bevy::prelude::State") resource doesn’t exist it will be created from the returned [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some") as the initial state.

Value within [`Some`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.Some "variant core::option::Option::Some") is ignored if the state already exists in the world and only symbolizes that the state should still exist.

Initial value can also be overwritten by [`NextState`](enum.NextState.html "enum bevy::prelude::NextState").

## Provided Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/sub_states.rs.html#172)

#### fn [register\_sub\_state\_systems](#method.register_sub_state_systems)(schedule: &mut [Schedule](struct.Schedule.html "struct bevy::prelude::Schedule"))

This function sets up systems that compute the state whenever one of the [`SourceStates`](trait.SubStates.html#associatedtype.SourceStates "associated type bevy::prelude::SubStates::SourceStates") change. It is called by `App::add_computed_state`, but can be called manually if `App` is not used.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
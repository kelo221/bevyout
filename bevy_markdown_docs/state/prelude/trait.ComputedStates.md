[bevy](../../index.html)::[state](../index.html)::[prelude](index.html)

# Trait ComputedStates 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/computed_states.rs.html#68)

```rust
pub trait ComputedStates:
    'static
    + Send
    + Sync
    + Clone
    + PartialEq
    + Eq
    + Hash
    + Debug {
    type SourceStates: StateSet;

    const ALLOW_SAME_STATE_TRANSITIONS: bool = true;

    // Required method
    fn compute(sources: Self::SourceStates) -> Option<Self>;

    // Provided method
    fn register_computed_state_systems(schedule: &mut Schedule) { ... }
}
```

A state whose value is automatically computed based on the values of other [`States`](../../prelude/trait.States.html "trait bevy::prelude::States").

A **computed state** is a state that is deterministically derived from a set of `SourceStates`. The [`StateSet`](../../prelude/trait.StateSet.html "trait bevy::prelude::StateSet") is passed into the `compute` method whenever one of them changes, and the result becomes the state’s value.

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
    /// We set the source state to be the state, or a tuple of states,
    /// we want to depend on. You can also wrap each state in an Option,
    /// if you want the computed state to execute even if the state doesn't
    /// currently exist in the world.
    type SourceStates = AppState;

    /// We then define the compute function, which takes in
    /// your SourceStates
    fn compute(sources: AppState) -> Option<Self> {
        match sources {
            /// When we are in game, we want to return the InGame state
            AppState::InGame { .. } => Some(InGame),
            /// Otherwise, we don't want the `State<InGame>` resource to exist,
            /// so we return None.
            _ => None
        }
    }
}
```

you can then add it to an App, and from there you use the state as normal

```rust
App::new()
    .init_state::<AppState>()
    .add_computed_state::<InGame>();
```

## Provided Associated Constants

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/computed_states.rs.html#79)

#### const [ALLOW\_SAME\_STATE\_TRANSITIONS](#associatedconstant.ALLOW_SAME_STATE_TRANSITIONS): [bool](https://doc.rust-lang.org/nightly/std/primitive.bool.html) = true

Whether state transition schedules should be run when the state changes to the same value. Default is `true`.

## Required Associated Types

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/computed_states.rs.html#76)

#### type [SourceStates](#associatedtype.SourceStates): [StateSet](../../prelude/trait.StateSet.html "trait bevy::prelude::StateSet")

The set of states from which the [`Self`](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates") is derived.

This can either be a single type that implements [`States`](../../prelude/trait.States.html "trait bevy::prelude::States"), an Option of a type that implements [`States`](../../prelude/trait.States.html "trait bevy::prelude::States"), or a tuple containing multiple types that implement [`States`](../../prelude/trait.States.html "trait bevy::prelude::States") or Optional versions of them.

For example, `(MapState, EnemyState)` is valid, as is `(MapState, Option<EnemyState>)`

## Required Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/computed_states.rs.html#85)

#### fn [compute](#tymethod.compute)(sources: Self::[SourceStates](../../prelude/trait.ComputedStates.html#associatedtype.SourceStates "type bevy::prelude::ComputedStates::SourceStates")) -> [Option](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option")<Self>

Computes the next value of [`State<Self>`](../../prelude/struct.State.html "struct bevy::prelude::State"). This function gets called whenever one of the [`SourceStates`](../../prelude/trait.ComputedStates.html#associatedtype.SourceStates "associated type bevy::prelude::ComputedStates::SourceStates") changes.

If the result is [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None "variant core::option::Option::None"), the [`State<Self>`](../../prelude/struct.State.html "struct bevy::prelude::State") resource will be removed from the world.

## Provided Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/computed_states.rs.html#90)

#### fn [register\_computed\_state\_systems](#method.register_computed_state_systems)(schedule: &mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"))

This function sets up systems that compute the state whenever one of the [`SourceStates`](../../prelude/trait.ComputedStates.html#associatedtype.SourceStates "associated type bevy::prelude::ComputedStates::SourceStates") change. It is called by `App::add_computed_state`, but can be called manually if `App` is not used.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
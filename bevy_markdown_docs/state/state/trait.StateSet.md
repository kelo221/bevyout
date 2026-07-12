[bevy](../../index.html)::[state](../index.html)::[state](index.html)

# Trait StateSet 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#30)

```rust
pub trait StateSet: StateSetSealed {
    const SET_DEPENDENCY_DEPTH: usize;

    // Required methods
    fn register_computed_state_systems_in_schedule<T>(schedule: &mut Schedule)
       where T: ComputedStates<SourceStates = Self>;
    fn register_sub_state_systems_in_schedule<T>(schedule: &mut Schedule)
       where T: SubStates<SourceStates = Self>;
}
```

A [`States`](../../prelude/trait.States.html "trait bevy::prelude::States") type or tuple of types which implement [`States`](../../prelude/trait.States.html "trait bevy::prelude::States").

This trait is used to allow implementors of [`States`](../../prelude/trait.States.html "trait bevy::prelude::States"), as well as tuples containing exclusively implementors of [`States`](../../prelude/trait.States.html "trait bevy::prelude::States"), to be used as [`ComputedStates::SourceStates`](../../prelude/trait.ComputedStates.html#associatedtype.SourceStates "associated type bevy::prelude::ComputedStates::SourceStates").

It is sealed, and auto implemented for all [`States`](../../prelude/trait.States.html "trait bevy::prelude::States") types and tuples containing them.

## Required Associated Constants

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#36)

#### const [SET\_DEPENDENCY\_DEPTH](#associatedconstant.SET_DEPENDENCY_DEPTH): [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

The total [`DEPENDENCY_DEPTH`](../../prelude/trait.States.html#associatedconstant.DEPENDENCY_DEPTH "associated constant bevy::prelude::States::DEPENDENCY_DEPTH") of all the states that are part of this [`StateSet`](../../prelude/trait.StateSet.html "trait bevy::prelude::StateSet"), added together.

Used to de-duplicate computed state executions and prevent cyclic computed states.

## Required Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#40-42)

#### fn [register\_computed\_state\_systems\_in\_schedule](#tymethod.register_computed_state_systems_in_schedule)<T>(schedule: &mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"))

where T: [ComputedStates](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates")<SourceStates = Self>,

Sets up the systems needed to compute `T` whenever any `State` in this `StateSet` is changed.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#46-48)

#### fn [register\_sub\_state\_systems\_in\_schedule](#tymethod.register_sub_state_systems_in_schedule)<T>(schedule: &mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"))

where T: [SubStates](../../prelude/trait.SubStates.html "trait bevy::prelude::SubStates")<SourceStates = Self>,

Sets up the systems needed to compute whether `T` exists whenever any `State` in this `StateSet` is changed.

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementations on Foreign Types

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#384-392)

### impl<S> [StateSet](../../prelude/trait.StateSet.html "trait bevy::prelude::StateSet") for [(S₁, S₂, …, Sₙ)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html#trait-implementations-1)

where S: InnerStateSet,

This trait is implemented for tuples up to 15 items long.

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#384-392)

#### const [SET\_DEPENDENCY\_DEPTH](#associatedconstant.SET_DEPENDENCY_DEPTH): [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html)

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#384-392)

#### fn [register\_computed\_state\_systems\_in\_schedule](#tymethod.register_computed_state_systems_in_schedule)<T>(schedule: &mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"))

where T: [ComputedStates](../../prelude/trait.ComputedStates.html "trait bevy::prelude::ComputedStates")<SourceStates = [(S,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#384-392)

#### fn [register\_sub\_state\_systems\_in\_schedule](#tymethod.register_sub_state_systems_in_schedule)<T>(schedule: &mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"))

where T: [SubStates](../../prelude/trait.SubStates.html "trait bevy::prelude::SubStates")<SourceStates = [(S,)](https://doc.rust-lang.org/nightly/std/primitive.tuple.html)\>,

## Implementors

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#91)

### impl<S> [StateSet](../../prelude/trait.StateSet.html "trait bevy::prelude::StateSet") for S

where S: InnerStateSet,

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/state_set.rs.html#92)

#### const [SET\_DEPENDENCY\_DEPTH](#associatedconstant.SET_DEPENDENCY_DEPTH): [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html) = S::DEPENDENCY\_DEPTH
[bevy](../../index.html)::[state](../index.html)::[state](index.html)

# Trait FreelyMutableState 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/freely_mutable_state.rs.html#15)

```rust
pub trait FreelyMutableState: States {
    // Provided method
    fn register_state(schedule: &mut Schedule) { ... }
}
```

This trait allows a state to be mutated directly using the [`NextState<S>`](../../prelude/enum.NextState.html "enum bevy::prelude::NextState") resource.

While ordinary states are freely mutable (and implement this trait as part of their derive macro), computed states are not: instead, they can _only_ change when the states that drive them do.

## Provided Methods

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/state/freely_mutable_state.rs.html#17)

#### fn [register\_state](#method.register_state)(schedule: &mut [Schedule](../../prelude/struct.Schedule.html "struct bevy::prelude::Schedule"))

This function registers all the necessary systems to apply state changes and run transition schedules

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors
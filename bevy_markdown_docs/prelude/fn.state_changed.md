[bevy](../index.html)::[prelude](index.html)

# Function state\_changed 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/condition.rs.html#165)

```rust
pub fn state_changed<S>(current_state: Option<Res<'_, State<S>>>) -> boolwhere
    S: States,
```

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine changed state.

To do things on transitions to/from specific states, use their respective OnEnter/OnExit schedules. Use this run condition if you want to detect any change, regardless of the value.

Returns false if the state does not exist or the state has not changed.

## Example

```rust
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum GameState {
    #[default]
    Playing,
    Paused,
}

app
    .init_state::<GameState>()
    .add_systems(Update,
        // `state_changed` will only return true if the
        // given states value has just been updated or
        // the state has just been added
        my_system.run_if(state_changed::<GameState>),
    );

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// `GameState` has just been added so `my_system` will run
app.update();
assert_eq!(app.world().resource::<Counter>().0, 1);

// `GameState` has not been updated so `my_system` will not run
app.update();
assert_eq!(app.world().resource::<Counter>().0, 1);

app.insert_state(GameState::Paused);

// Now that `GameState` has been updated `my_system` will run
app.update();
assert_eq!(app.world().resource::<Counter>().0, 2);
```
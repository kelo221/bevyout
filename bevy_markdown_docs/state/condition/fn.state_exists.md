[bevy](../../index.html)::[state](../index.html)::[condition](index.html)

# Function state\_exists 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/condition.rs.html#47)

```rust
pub fn state_exists<S>(current_state: Option<Res<'_, State<S>>>) -> boolwhere
    S: States,
```

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the state machine exists.

## Example

```rust
#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
enum GameState {
    #[default]
    Playing,
    Paused,
}

app.add_systems(Update,
    // `state_exists` will only return true if the
    // given state exists
    my_system.run_if(state_exists::<GameState>),
);

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// `GameState` does not yet exist so `my_system` won't run
app.update();
assert_eq!(app.world().resource::<Counter>().0, 0);

app.init_state::<GameState>();

// `GameState` now exists so `my_system` will run
app.update();
assert_eq!(app.world().resource::<Counter>().0, 1);
```
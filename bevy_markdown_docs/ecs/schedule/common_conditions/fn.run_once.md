[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)::[common\_conditions](index.html)

# Function run\_once 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#690)

```rust
pub fn run_once(has_run: Local<'_, bool>) -> bool
```

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` on the first time the condition is run and false every time after.

## Example

```rust
app.add_systems(
    // `run_once` will only return true the first time it's evaluated
    my_system.run_if(run_once),
);

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// This is the first time the condition will be evaluated so `my_system` will run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);

// This is the seconds time the condition will be evaluated so `my_system` won't run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
```
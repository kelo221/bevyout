[bevy](../index.html)::[prelude](index.html)

# Function condition\_changed 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1287-1290)

```rust
pub fn condition_changed<Marker, CIn, C>(
    condition: C,
) -> impl SystemCondition<(), CIn>where
    CIn: SystemInput,
    C: SystemCondition<Marker, CIn>,
```

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the passed one changes.

The first time this is called, the passed condition is assumed to have been previously false.

## Example

```rust
app.add_systems(
    my_system.run_if(condition_changed(resource_exists::<MyResource>)),
);

#[derive(Resource)]
struct MyResource;

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// `MyResource` is initially there, the inner condition is true, the system runs once
world.insert_resource(MyResource);
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);

// We remove `MyResource`, the inner condition is now false, the system runs one more time.
world.remove_resource::<MyResource>();
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 2);
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 2);
```
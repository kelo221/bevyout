[bevy](../index.html)::[prelude](index.html)

# Function condition\_changed\_to 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1343-1349)

```rust
pub fn condition_changed_to<Marker, CIn, C>(
    to: bool,
    condition: C,
) -> impl SystemCondition<(), CIn>where
    CIn: SystemInput,
    C: SystemCondition<Marker, CIn>,
```

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition") that returns true when the result of the passed one went from false to true since the last time this was called.

The first time this is called, the passed condition is assumed to have been previously false.

## Example

```rust
app.add_systems(
    my_system.run_if(condition_changed_to(true, resource_exists::<MyResource>)),
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

// We remove `MyResource`, the inner condition is now false, the system doesn't run.
world.remove_resource::<MyResource>();
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);

// We reinsert `MyResource` again, so the system will run one more time
world.insert_resource(MyResource);
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 2);
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 2);
```
[bevy](../index.html)::[prelude](index.html)

# Function resource\_exists\_and\_equals 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#812-814)

```rust
pub fn resource_exists_and_equals<T>(value: T) -> impl FnMut(Option<Res<'_, T>>)where
    T: Resource + PartialEq,
```

Generates a [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying closure that returns `true` if the resource exists and is equal to `value`.

The condition will return `false` if the resource does not exist.

## Example

```rust
app.add_systems(
    // `resource_exists_and_equals` will only return true
    // if the given resource exists and equals the given value
    my_system.run_if(resource_exists_and_equals(Counter(0))),
);

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// `Counter` hasn't been added so `my_system` can't run
app.run(&mut world);
world.init_resource::<Counter>();

// `Counter` is `0` so `my_system` can run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);

// `Counter` is no longer `0` so `my_system` won't run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
```
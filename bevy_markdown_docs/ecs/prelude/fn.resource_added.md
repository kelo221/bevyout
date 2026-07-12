[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Function resource\_added 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#853-855)

```rust
pub fn resource_added<T>(res: Option<Res<'_, T>>) -> boolwhere
    T: Resource,
```

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added since the condition was last checked.

## Example

```rust
app.add_systems(
    // `resource_added` will only return true if the
    // given resource was just added
    my_system.run_if(resource_added::<Counter>),
);

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

world.init_resource::<Counter>();

// `Counter` was just added so `my_system` will run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);

// `Counter` was not just added so `my_system` will not run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
```
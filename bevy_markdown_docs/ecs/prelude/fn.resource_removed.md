[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Function resource\_removed 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1083-1085)

```rust
pub fn resource_removed<T>(
    res: Option<Res<'_, T>>,
    existed: Local<'_, bool>,
) -> boolwhere
    T: Resource,
```

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been removed since the condition was last checked.

## Example

```rust
app.add_systems(
    // `resource_removed` will only return true if the
    // given resource was just removed
    my_system.run_if(resource_removed::<MyResource>),
);

#[derive(Resource, Default)]
struct MyResource;

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

world.init_resource::<MyResource>();

// `MyResource` hasn't just been removed so `my_system` won't run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 0);

world.remove_resource::<MyResource>();

// `MyResource` was just removed so `my_system` will run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
```
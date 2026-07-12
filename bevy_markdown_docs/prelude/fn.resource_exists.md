[bevy](../index.html)::[prelude](index.html)

# Function resource\_exists 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#730-732)

```rust
pub fn resource_exists<T>(res: Option<Res<'_, T>>) -> boolwhere
    T: Resource,
```

A [`SystemCondition`](trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource exists.

To skip a system with a [`Res`](struct.Res.html "struct bevy::prelude::Res") or [`ResMut`](struct.ResMut.html "struct bevy::prelude::ResMut") parameter if the resource does not exist, you may instead wrap the parameter in [`If`](struct.If.html "struct bevy::prelude::If"), like `If<Res<T>>` or `If<ResMut<T>>`.

## Example

```rust
app.add_systems(
    // `resource_exists` will only return true if the given resource exists in the world
    my_system.run_if(resource_exists::<Counter>),
);

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// `Counter` hasn't been added so `my_system` won't run
app.run(&mut world);
world.init_resource::<Counter>();

// `Counter` has now been added so `my_system` can run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
```
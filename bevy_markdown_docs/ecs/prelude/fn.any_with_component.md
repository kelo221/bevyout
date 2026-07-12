[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Function any\_with\_component 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1183)

```rust
pub fn any_with_component<T>(query: Query<'_, '_, (), With<T>>) -> boolwhere
    T: Component,
```

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any entities with the given component type.

This is equivalent to [`any_match_filter::<With<T>>()`](../../prelude/fn.any_match_filter.html "fn bevy::prelude::any_match_filter")

To skip a system with a [`Query`](../../prelude/struct.Query.html "struct bevy::prelude::Query") parameter if the query is empty, you may instead use [`Populated`](../../prelude/struct.Populated.html "struct bevy::prelude::Populated"), if the query may match multiple entities, or [`Single`](../../prelude/struct.Single.html "struct bevy::prelude::Single"), if it will only match one.

## Example

```rust
app.add_systems(
    my_system.run_if(any_with_component::<MyComponent>),
);

#[derive(Component)]
struct MyComponent;

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// No entities exist yet with a `MyComponent` component so `my_system` won't run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 0);

world.spawn(MyComponent);

// An entities with `MyComponent` now exists so `my_system` will run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
```
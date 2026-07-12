[bevy](../../../index.html)::[ecs](../../index.html)::[schedule](../index.html)::[common\_conditions](index.html)

# Function resource\_changed 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#909-911)

```rust
pub fn resource_changed<T>(res: Res<'_, T>) -> boolwhere
    T: Resource,
```

A [`SystemCondition`](../../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if the resource of the given type has been added or mutably dereferenced since the condition was last checked.

**Note** that simply _mutably dereferencing_ a resource is considered a change ([`DerefMut`](https://doc.rust-lang.org/nightly/core/ops/deref/trait.DerefMut.html "trait core::ops::deref::DerefMut")). Bevy does not compare resources to their previous values.

## Panics

The condition will panic if the resource does not exist.

## Example

```rust
app.add_systems(
    // `resource_changed` will only return true if the
    // given resource was just changed (or added)
    my_system.run_if(
        resource_changed::<Counter>
        // By default detecting changes will also trigger if the resource was
        // just added, this won't work with my example so I will add a second
        // condition to make sure the resource wasn't just added
        .and(not(resource_added::<Counter>))
    ),
);

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// `Counter` hasn't been changed so `my_system` won't run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 0);

world.resource_mut::<Counter>().0 = 50;

// `Counter` was just changed so `my_system` will run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 51);
```
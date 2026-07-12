[bevy](../../index.html)::[ecs](../index.html)::[prelude](index.html)

# Function on\_message 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1136)

```rust
pub fn on_message<M>(reader: MessageReader<'_, '_, M>) -> boolwhere
    M: Message,
```

A [`SystemCondition`](../../prelude/trait.SystemCondition.html "trait bevy::prelude::SystemCondition")\-satisfying system that returns `true` if there are any new messages of the given type since it was last called.

To skip a system based on messages that it reads, use [`PopulatedMessageReader`](../../prelude/struct.PopulatedMessageReader.html "struct bevy::prelude::PopulatedMessageReader") instead.

## Example

```rust
app.add_systems(
    my_system.run_if(on_message::<MyMessage>),
);

#[derive(Message)]
struct MyMessage;

fn my_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

// No new `MyMessage` messages have been pushed so `my_system` won't run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 0);

world.resource_mut::<Messages<MyMessage>>().write(MyMessage);

// A `MyMessage` message has been pushed so `my_system` will run
app.run(&mut world);
assert_eq!(world.resource::<Counter>().0, 1);
```
[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[entity\_command](index.html)

# Trait EntityCommand 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#86)

```rust
pub trait EntityCommand: Send + 'static {
    type Out: EntityCommandOutput;

    // Required method
    fn apply(self, entity: EntityWorldMut<'_>) -> Self::Out;

    // Provided method
    fn with_entity(self, entity: Entity) -> impl Command
       where Self: Sized { ... }
}
```

A command which gets executed for a given [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

Should be used with [`EntityCommands::queue`](../../../prelude/struct.EntityCommands.html#method.queue "method bevy::prelude::EntityCommands::queue").

The `Out` generic parameter is the returned “output” of the command.

## Examples

```rust
use bevy_ecs::system::EntityCommand;

#[derive(Resource, Default)]
struct Counter(i64);

/// A `Command` which names an entity based on a global counter.
fn count_name(mut entity: EntityWorldMut) {
    // Get the current value of the counter, and increment it for next time.
    let i = {
        let mut counter = entity.resource_mut::<Counter>();
        let i = counter.0;
        counter.0 += 1;
        i
    };
    // Name the entity after the value of the counter.
    entity.insert(Name::new(format!("Entity #{i}")));
}

// App creation boilerplate omitted...

fn setup(mut commands: Commands) {
    commands.spawn_empty().queue(count_name);
    commands.spawn_empty().queue(count_name);
}

fn assert_names(named: Query<&Name>) {
    // We use a HashSet because we do not care about the order.
    let names: HashSet<_> = named.iter().map(Name::as_str).collect();
    assert_eq!(names, HashSet::from_iter(["Entity #0", "Entity #1"]));
}
```

## Required Associated Types

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#88)

#### type [Out](#associatedtype.Out): [EntityCommandOutput](../../error/trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput")

The return type of [`apply`](../../../prelude/trait.EntityCommand.html#tymethod.apply "method bevy::prelude::EntityCommand::apply").

## Required Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#91)

#### fn [apply](#tymethod.apply)(self, entity: [EntityWorldMut](../../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> Self::[Out](../../../prelude/trait.EntityCommand.html#associatedtype.Out "type bevy::prelude::EntityCommand::Out")

Executes this command for the given [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

## Provided Methods

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#96-98)

#### fn [with\_entity](#method.with_entity)(self, entity: [Entity](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity")) -> impl [Command](../../../prelude/trait.Command.html "trait bevy::prelude::Command")

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Passes in a specific entity to an [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand"), resulting in a [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that internally runs the [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") on that entity.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#118-121)

### impl<Out, F> [EntityCommand](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") for F

where F: [FnOnce](https://doc.rust-lang.org/nightly/core/ops/function/trait.FnOnce.html "trait core::ops::function::FnOnce")([EntityWorldMut](../../../prelude/struct.EntityWorldMut.html "struct bevy::prelude::EntityWorldMut")<'\_>) -> Out + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'static, Out: [EntityCommandOutput](../../error/trait.EntityCommandOutput.html "trait bevy::ecs::error::EntityCommandOutput"),

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/entity_command.rs.html#123)

#### type [Out](#associatedtype.Out) = Out
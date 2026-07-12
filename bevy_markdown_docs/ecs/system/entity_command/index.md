[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)

# Module entity\_command 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#2)

Contains the definition of the [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") trait, as well as the blanket implementation of the trait for closures.

It also contains functions that return closures for use with [`EntityCommands`](../../../prelude/struct.EntityCommands.html "struct bevy::prelude::EntityCommands").

## Enums

[EntityCommandError](enum.EntityCommandError.html "enum bevy::ecs::system::entity_command::EntityCommandError")

An error that occurs when running an [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") on a specific entity.

## Traits

[EntityCommand](trait.EntityCommand.html "trait bevy::ecs::system::entity_command::EntityCommand")

A command which gets executed for a given [`Entity`](../../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

## Functions

[clear](fn.clear.html "fn bevy::ecs::system::entity_command::clear")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes all components from an entity.

[clone\_components](fn.clone_components.html "fn bevy::ecs::system::entity_command::clone_components")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that clones the specified components of an entity and inserts them into another entity.

[clone\_with\_opt\_in](fn.clone_with_opt_in.html "fn bevy::ecs::system::entity_command::clone_with_opt_in")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that clones parts of an entity onto another entity, configured through [`EntityClonerBuilder`](../../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

[clone\_with\_opt\_out](fn.clone_with_opt_out.html "fn bevy::ecs::system::entity_command::clone_with_opt_out")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that clones parts of an entity onto another entity, configured through [`EntityClonerBuilder`](../../entity/struct.EntityClonerBuilder.html "struct bevy::ecs::entity::EntityClonerBuilder").

[despawn](fn.despawn.html "fn bevy::ecs::system::entity_command::despawn")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that despawns an entity.

[insert](fn.insert.html "fn bevy::ecs::system::entity_command::insert")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds the components in a [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to an entity.

[insert\_by\_id](fn.insert_by_id.html "fn bevy::ecs::system::entity_command::insert_by_id")⚠

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds a dynamic component to an entity.

[insert\_from\_world](fn.insert_from_world.html "fn bevy::ecs::system::entity_command::insert_from_world")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds a component to an entity using the component’s [`FromWorld`](../../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") implementation.

[insert\_with](fn.insert_with.html "fn bevy::ecs::system::entity_command::insert_with")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that adds a component to an entity using some function that returns the component.

[log\_components](fn.log_components.html "fn bevy::ecs::system::entity_command::log_components")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that logs the components of an entity.

[move\_components](fn.move_components.html "fn bevy::ecs::system::entity_command::move_components")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") moves the specified components of this entity into another entity.

[observe](fn.observe.html "fn bevy::ecs::system::entity_command::observe")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that creates an [`Observer`](../../../prelude/struct.Observer.html "struct bevy::prelude::Observer") watching for an [`EntityEvent`](../../../prelude/trait.EntityEvent.html "trait bevy::prelude::EntityEvent") of type `E` whose [`event_target`](../../../prelude/trait.EntityEvent.html#tymethod.event_target "method bevy::prelude::EntityEvent::event_target") targets this entity.

[remove](fn.remove.html "fn bevy::ecs::system::entity_command::remove")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes the components in a [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") from an entity.

[remove\_by\_id](fn.remove_by_id.html "fn bevy::ecs::system::entity_command::remove_by_id")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes a dynamic component from an entity.

[remove\_with\_requires](fn.remove_with_requires.html "fn bevy::ecs::system::entity_command::remove_with_requires")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes the components in a [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") from an entity, as well as the required components for each component removed.

[retain](fn.retain.html "fn bevy::ecs::system::entity_command::retain")

An [`EntityCommand`](../../../prelude/trait.EntityCommand.html "trait bevy::prelude::EntityCommand") that removes all components from an entity, except for those in the given [`Bundle`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle").
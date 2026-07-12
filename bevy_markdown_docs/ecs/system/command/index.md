[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)

# Module command 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/commands/mod.rs.html#1)

Contains the definition of the [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") trait, as well as the blanket implementation of the trait for closures.

It also contains functions that return closures for use with [`Commands`](../../../prelude/struct.Commands.html "struct bevy::prelude::Commands").

## Traits

[Command](trait.Command.html "trait bevy::ecs::system::command::Command")

A [`World`](../../../prelude/struct.World.html "struct bevy::prelude::World") mutation.

## Functions

[init\_resource](fn.init_resource.html "fn bevy::ecs::system::command::init_resource")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that inserts a [`Resource`](../../../prelude/trait.Resource.html "trait bevy::prelude::Resource") into the world using a value created with the [`FromWorld`](../../../prelude/trait.FromWorld.html "trait bevy::prelude::FromWorld") trait.

[insert\_batch](fn.insert_batch.html "fn bevy::ecs::system::command::insert_batch")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that consumes an iterator to add a series of [`Bundles`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to a set of entities.

[insert\_resource](fn.insert_resource.html "fn bevy::ecs::system::command::insert_resource")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that inserts a [`Resource`](../../../prelude/trait.Resource.html "trait bevy::prelude::Resource") into the world.

[remove\_resource](fn.remove_resource.html "fn bevy::ecs::system::command::remove_resource")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that removes a [`Resource`](../../../prelude/trait.Resource.html "trait bevy::prelude::Resource") from the world.

[run\_schedule](fn.run_schedule.html "fn bevy::ecs::system::command::run_schedule")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the schedule corresponding to the given [`ScheduleLabel`](../../schedule/trait.ScheduleLabel.html "trait bevy::ecs::schedule::ScheduleLabel").

[run\_system](fn.run_system.html "fn bevy::ecs::system::command::run_system")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the system corresponding to the given [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId").

[run\_system\_cached](fn.run_system_cached.html "fn bevy::ecs::system::command::run_system_cached")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the given system, caching its [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`CachedSystemId`](../struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource.

[run\_system\_cached\_with](fn.run_system_cached_with.html "fn bevy::ecs::system::command::run_system_cached_with")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the given system with the given input value, caching its [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId") in a [`CachedSystemId`](../struct.CachedSystemId.html "struct bevy::ecs::system::CachedSystemId") resource.

[run\_system\_with](fn.run_system_with.html "fn bevy::ecs::system::command::run_system_with")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that runs the system corresponding to the given [`SystemId`](../struct.SystemId.html "struct bevy::ecs::system::SystemId") and provides the given input value.

[spawn\_batch](fn.spawn_batch.html "fn bevy::ecs::system::command::spawn_batch")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that consumes an iterator of [`Bundles`](../../../prelude/trait.Bundle.html "trait bevy::prelude::Bundle") to spawn a series of entities.

[trigger](fn.trigger.html "fn bevy::ecs::system::command::trigger")

Triggers the given [`Event`](../../../prelude/trait.Event.html "trait bevy::prelude::Event"), which will run any [`Observer`](../../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

[trigger\_with](fn.trigger_with.html "fn bevy::ecs::system::command::trigger_with")

Triggers the given [`Event`](../../../prelude/trait.Event.html "trait bevy::prelude::Event") using the given [`Trigger`](../../event/trait.Trigger.html "trait bevy::ecs::event::Trigger"), which will run any [`Observer`](../../../prelude/struct.Observer.html "struct bevy::prelude::Observer")s watching for it.

[unregister\_system](fn.unregister_system.html "fn bevy::ecs::system::command::unregister_system")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that removes a system previously registered with [`Commands::register_system`](../../../prelude/struct.Commands.html#method.register_system "method bevy::prelude::Commands::register_system") or [`World::register_system`](../../../prelude/struct.World.html#method.register_system "method bevy::prelude::World::register_system").

[unregister\_system\_cached](fn.unregister_system_cached.html "fn bevy::ecs::system::command::unregister_system_cached")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that removes a system previously registered with one of the following:

[write\_message](fn.write_message.html "fn bevy::ecs::system::command::write_message")

A [`Command`](../../../prelude/trait.Command.html "trait bevy::prelude::Command") that writes an arbitrary [`Message`](../../../prelude/trait.Message.html "trait bevy::prelude::Message").
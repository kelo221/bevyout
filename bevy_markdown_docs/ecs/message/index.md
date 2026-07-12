[bevy](../../index.html)::[ecs](../index.html)

# Module message 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/lib.rs.html#42)

[`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message") functionality.

## Structs

[MessageCursor](struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor")

Stores the state for a [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader") or [`MessageMutator`](../../prelude/struct.MessageMutator.html "struct bevy::prelude::MessageMutator").

[MessageId](struct.MessageId.html "struct bevy::ecs::message::MessageId")

A [`MessageId`](struct.MessageId.html "struct bevy::ecs::message::MessageId") uniquely identifies a message stored in a specific [`World`](../../prelude/struct.World.html "struct bevy::prelude::World").

[MessageIterator](struct.MessageIterator.html "struct bevy::ecs::message::MessageIterator")

An iterator that yields any unread messages from a [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader") or [`MessageCursor`](struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor").

[MessageIteratorWithId](struct.MessageIteratorWithId.html "struct bevy::ecs::message::MessageIteratorWithId")

An iterator that yields any unread messages (and their IDs) from a [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader") or [`MessageCursor`](struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor").

[MessageMutIterator](struct.MessageMutIterator.html "struct bevy::ecs::message::MessageMutIterator")

An iterator that yields any unread messages from an [`MessageMutator`](../../prelude/struct.MessageMutator.html "struct bevy::prelude::MessageMutator") or [`MessageCursor`](struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor").

[MessageMutIteratorWithId](struct.MessageMutIteratorWithId.html "struct bevy::ecs::message::MessageMutIteratorWithId")

An iterator that yields any unread messages (and their IDs) from an [`MessageMutator`](../../prelude/struct.MessageMutator.html "struct bevy::prelude::MessageMutator") or [`MessageCursor`](struct.MessageCursor.html "struct bevy::ecs::message::MessageCursor").

[MessageMutParIter](struct.MessageMutParIter.html "struct bevy::ecs::message::MessageMutParIter")`multi_threaded`

A parallel iterator over `Message`s.

[MessageMutator](struct.MessageMutator.html "struct bevy::ecs::message::MessageMutator")

Reads and writes [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T`, keeping track of which messages have already been read.

[MessageParIter](struct.MessageParIter.html "struct bevy::ecs::message::MessageParIter")`multi_threaded`

A parallel iterator over `Message`s.

[MessageReader](struct.MessageReader.html "struct bevy::ecs::message::MessageReader")

Reads [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T` in order and tracks which messages have already been read.

[MessageRegistry](struct.MessageRegistry.html "struct bevy::ecs::message::MessageRegistry")

A registry of all of the [`Messages`](../../prelude/struct.Messages.html "struct bevy::prelude::Messages") in the [`World`](../../prelude/struct.World.html "struct bevy::prelude::World"), used by [`message_update_system`](fn.message_update_system.html "fn bevy::ecs::message::message_update_system") to update all of the messages.

[MessageWriter](struct.MessageWriter.html "struct bevy::ecs::message::MessageWriter")

Writes [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T`.

[Messages](struct.Messages.html "struct bevy::ecs::message::Messages")

A message collection that represents the messages that occurred within the last two [`Messages::update`](../../prelude/struct.Messages.html#method.update "method bevy::prelude::Messages::update") calls. Messages can be written to using a [`MessageWriter`](../../prelude/struct.MessageWriter.html "struct bevy::prelude::MessageWriter") and are typically cheaply read using a [`MessageReader`](../../prelude/struct.MessageReader.html "struct bevy::prelude::MessageReader").

[PopulatedMessageReader](struct.PopulatedMessageReader.html "struct bevy::ecs::message::PopulatedMessageReader")

Reads [`Message`](../../prelude/trait.Message.html "trait bevy::prelude::Message")s of type `T` in order and tracks which messages have already been read. Skips the system if there no messages.

[WriteBatchIds](struct.WriteBatchIds.html "struct bevy::ecs::message::WriteBatchIds")

[`Iterator`](https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html "trait core::iter::traits::iterator::Iterator") over written [`MessageIds`](struct.MessageId.html "struct bevy::ecs::message::MessageId") from a batch.

## Enums

[ShouldUpdateMessages](enum.ShouldUpdateMessages.html "enum bevy::ecs::message::ShouldUpdateMessages")

Controls whether or not the messages in an [`MessageRegistry`](struct.MessageRegistry.html "struct bevy::ecs::message::MessageRegistry") should be updated.

## Traits

[Message](trait.Message.html "trait bevy::ecs::message::Message")

A buffered message for pull-based event handling.

## Functions

[message\_update\_condition](fn.message_update_condition.html "fn bevy::ecs::message::message_update_condition")

A run condition for [`message_update_system`](fn.message_update_system.html "fn bevy::ecs::message::message_update_system").

[message\_update\_system](fn.message_update_system.html "fn bevy::ecs::message::message_update_system")

A system that calls [`Messages::update`](../../prelude/struct.Messages.html#method.update "method bevy::prelude::Messages::update") on all registered [`Messages`](../../prelude/struct.Messages.html "struct bevy::prelude::Messages") in the world.

[signal\_message\_update\_system](fn.signal_message_update_system.html "fn bevy::ecs::message::signal_message_update_system")

Signals the [`message_update_system`](fn.message_update_system.html "fn bevy::ecs::message::message_update_system") to run after `FixedUpdate` systems.

## Derive Macros

[Message](derive.Message.html "derive bevy::ecs::message::Message")

Implement the `Message` trait.
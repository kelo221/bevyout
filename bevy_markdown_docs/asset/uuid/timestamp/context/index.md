[bevy](../../../../index.html)::[asset](../../../index.html)::[uuid](../../index.html)::[timestamp](../index.html)

# Module context 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/timestamp.rs.html#478)

Default implementations for the [`ClockSequence`](../../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") trait.

## Structs

[NoContext](struct.NoContext.html "struct bevy::asset::uuid::timestamp::context::NoContext")

An empty counter that will always return the value `0`.

[ThreadLocalContext](struct.ThreadLocalContext.html "struct bevy::asset::uuid::timestamp::context::ThreadLocalContext")

A wrapper for a context that uses thread-local storage.
[bevy](../../../index.html)::[asset](../../index.html)::[uuid](../index.html)

# Module timestamp 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/lib.rs.html#234)

Generating UUIDs from timestamps.

Timestamps are used in a few UUID versions as a source of decentralized uniqueness (as in versions 1 and 6), and as a way to enable sorting (as in versions 6 and 7). Timestamps aren’t encoded the same way by all UUID versions so this module provides a single [`Timestamp`](../struct.Timestamp.html "struct bevy::asset::uuid::Timestamp") type that can convert between them.

## Timestamp representations in UUIDs

Versions 1 and 6 UUIDs use a bespoke timestamp that consists of the number of 100ns ticks since `1582-10-15 00:00:00`, along with a counter value to avoid duplicates.

Version 7 UUIDs use a more standard timestamp that consists of the number of millisecond ticks since the Unix epoch (`1970-01-01 00:00:00`).

## References

*   [UUID Version 1 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.1)
*   [UUID Version 7 in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-5.7)
*   [Timestamp Considerations in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-6.1)

## Modules

[context](context/index.html "mod bevy::asset::uuid::timestamp::context")

Default implementations for the [`ClockSequence`](../trait.ClockSequence.html "trait bevy::asset::uuid::ClockSequence") trait.

## Structs

[Timestamp](struct.Timestamp.html "struct bevy::asset::uuid::timestamp::Timestamp")

A timestamp that can be encoded into a UUID.

## Constants

[UUID\_TICKS\_BETWEEN\_EPOCHS](constant.UUID_TICKS_BETWEEN_EPOCHS.html "constant bevy::asset::uuid::timestamp::UUID_TICKS_BETWEEN_EPOCHS")

The number of 100 nanosecond ticks between the RFC 9562 epoch (`1582-10-15 00:00:00`) and the Unix epoch (`1970-01-01 00:00:00`).

## Traits

[ClockSequence](trait.ClockSequence.html "trait bevy::asset::uuid::timestamp::ClockSequence")

A counter that can be used by versions 1 and 6 UUIDs to support the uniqueness of timestamps.
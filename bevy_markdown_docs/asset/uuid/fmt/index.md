[bevy](../../../index.html)::[asset](../../index.html)::[uuid](../index.html)

# Module fmt 

[Source](https://docs.rs/uuid/1.23.2/x86_64-unknown-linux-gnu/src/uuid/lib.rs.html#233)

Adapters for alternative string formats.

## Structs

[Braced](struct.Braced.html "struct bevy::asset::uuid::fmt::Braced")

Format a [`Uuid`](../struct.Uuid.html "struct bevy::asset::uuid::Uuid") as a braced hyphenated string, like `{67e55044-10b1-426f-9247-bb680e5fe0c8}`.

[Hyphenated](struct.Hyphenated.html "struct bevy::asset::uuid::fmt::Hyphenated")

Format a [`Uuid`](../struct.Uuid.html "struct bevy::asset::uuid::Uuid") as a hyphenated string, like `67e55044-10b1-426f-9247-bb680e5fe0c8`.

[Simple](struct.Simple.html "struct bevy::asset::uuid::fmt::Simple")

Format a [`Uuid`](../struct.Uuid.html "struct bevy::asset::uuid::Uuid") as a simple string, like `67e5504410b1426f9247bb680e5fe0c8`.

[Urn](struct.Urn.html "struct bevy::asset::uuid::fmt::Urn")

Format a [`Uuid`](../struct.Uuid.html "struct bevy::asset::uuid::Uuid") as a URN string, like `urn:uuid:67e55044-10b1-426f-9247-bb680e5fe0c8`.
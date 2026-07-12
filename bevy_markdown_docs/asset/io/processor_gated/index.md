[bevy](../../../index.html)::[asset](../../index.html)::[io](../index.html)

# Module processor\_gated 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#14)

## Structs

[TransactionLockedReader](struct.TransactionLockedReader.html "struct bevy::asset::io::processor_gated::TransactionLockedReader")

An [`AsyncRead`](../../../tasks/futures_lite/trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") impl that will hold its asset’s transaction lock until [`TransactionLockedReader`](struct.TransactionLockedReader.html "struct bevy::asset::io::processor_gated::TransactionLockedReader") is dropped.
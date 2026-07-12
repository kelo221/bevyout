[bevy](../../index.html)::[platform](../index.html)

# Module cell 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/lib.rs.html#24)

Provides cell primitives.

This is a drop-in replacement for `std::cell::SyncCell`/`std::cell::SyncUnsafeCell`.

## Structs

[SyncCell](struct.SyncCell.html "struct bevy::platform::cell::SyncCell")

See [`Exclusive`](https://github.com/rust-lang/rust/issues/98407) for stdlib’s upcoming implementation, which should replace this one entirely.

[SyncUnsafeCell](struct.SyncUnsafeCell.html "struct bevy::platform::cell::SyncUnsafeCell")

[`UnsafeCell`](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html "struct core::cell::UnsafeCell"), but [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html "trait core::marker::Sync").
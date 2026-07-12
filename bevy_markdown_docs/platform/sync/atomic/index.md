[bevy](../../../index.html)::[platform](../../index.html)::[sync](../index.html)

# Module atomic 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/sync/mod.rs.html#29)

Provides various atomic alternatives to language primitives.

Certain platforms lack complete atomic support, requiring the use of a fallback such as `portable-atomic`. Using these types will ensure the correct atomic provider is used without the need for feature gates in your own code.

## Enums

[Ordering](enum.Ordering.html "enum bevy::platform::sync::atomic::Ordering")

Atomic memory orderings

## Type Aliases

[AtomicBool](type.AtomicBool.html "type bevy::platform::sync::atomic::AtomicBool")`target_has_atomic_load_store=8`

A boolean type which can be safely shared between threads.

[AtomicI8](type.AtomicI8.html "type bevy::platform::sync::atomic::AtomicI8")

An integer type which can be safely shared between threads.

[AtomicI16](type.AtomicI16.html "type bevy::platform::sync::atomic::AtomicI16")

An integer type which can be safely shared between threads.

[AtomicI32](type.AtomicI32.html "type bevy::platform::sync::atomic::AtomicI32")

An integer type which can be safely shared between threads.

[AtomicI64](type.AtomicI64.html "type bevy::platform::sync::atomic::AtomicI64")

An integer type which can be safely shared between threads.

[AtomicIsize](type.AtomicIsize.html "type bevy::platform::sync::atomic::AtomicIsize")

An integer type which can be safely shared between threads.

[AtomicPtr](type.AtomicPtr.html "type bevy::platform::sync::atomic::AtomicPtr")`target_has_atomic_load_store=ptr`

A raw pointer type which can be safely shared between threads.

[AtomicU8](type.AtomicU8.html "type bevy::platform::sync::atomic::AtomicU8")

An integer type which can be safely shared between threads.

[AtomicU16](type.AtomicU16.html "type bevy::platform::sync::atomic::AtomicU16")

An integer type which can be safely shared between threads.

[AtomicU32](type.AtomicU32.html "type bevy::platform::sync::atomic::AtomicU32")

An integer type which can be safely shared between threads.

[AtomicU64](type.AtomicU64.html "type bevy::platform::sync::atomic::AtomicU64")

An integer type which can be safely shared between threads.

[AtomicUsize](type.AtomicUsize.html "type bevy::platform::sync::atomic::AtomicUsize")

An integer type which can be safely shared between threads.
[bevy](../../index.html)::[platform](../index.html)

# Module sync 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/lib.rs.html#28)

Provides various synchronization alternatives to language primitives.

Currently missing from this module are the following items:

*   `Condvar`
*   `WaitTimeoutResult`
*   `mpsc`

Otherwise, this is a drop-in replacement for `std::sync`.

## Modules

[atomic](atomic/index.html "mod bevy::platform::sync::atomic")

Provides various atomic alternatives to language primitives.

## Structs

[Arc](struct.Arc.html "struct bevy::platform::sync::Arc")

A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.

[Barrier](struct.Barrier.html "struct bevy::platform::sync::Barrier")

A barrier enables multiple threads to synchronize the beginning of some computation.

[BarrierWaitResult](struct.BarrierWaitResult.html "struct bevy::platform::sync::BarrierWaitResult")

A `BarrierWaitResult` is returned by [`Barrier::wait()`](struct.Barrier.html#method.wait "method bevy::platform::sync::Barrier::wait") when all threads in the [`Barrier`](struct.Barrier.html "struct bevy::platform::sync::Barrier") have rendezvoused.

[LazyLock](struct.LazyLock.html "struct bevy::platform::sync::LazyLock")

A value which is initialized on the first access.

[Mutex](struct.Mutex.html "struct bevy::platform::sync::Mutex")

A mutual exclusion primitive useful for protecting shared data

[MutexGuard](struct.MutexGuard.html "struct bevy::platform::sync::MutexGuard")

An RAII implementation of a “scoped lock” of a mutex. When this structure is dropped (falls out of scope), the lock will be unlocked.

[Once](struct.Once.html "struct bevy::platform::sync::Once")

A low-level synchronization primitive for one-time global execution.

[OnceLock](struct.OnceLock.html "struct bevy::platform::sync::OnceLock")

A synchronization primitive which can nominally be written to only once.

[OnceState](struct.OnceState.html "struct bevy::platform::sync::OnceState")

State yielded to [`Once::call_once_force()`](struct.Once.html#method.call_once_force "method bevy::platform::sync::Once::call_once_force")’s closure parameter. The state can be used to query the poison status of the [`Once`](struct.Once.html "struct bevy::platform::sync::Once").

[PoisonError](struct.PoisonError.html "struct bevy::platform::sync::PoisonError")

A type of error which can be returned whenever a lock is acquired.

[RwLock](struct.RwLock.html "struct bevy::platform::sync::RwLock")

A reader-writer lock

[RwLockReadGuard](struct.RwLockReadGuard.html "struct bevy::platform::sync::RwLockReadGuard")

RAII structure used to release the shared read access of a lock when dropped.

[RwLockWriteGuard](struct.RwLockWriteGuard.html "struct bevy::platform::sync::RwLockWriteGuard")

RAII structure used to release the exclusive write access of a lock when dropped.

[Weak](struct.Weak.html "struct bevy::platform::sync::Weak")

`Weak` is a version of [`Arc`](struct.Arc.html "struct bevy::platform::sync::Arc") that holds a non-owning reference to the managed allocation.

## Enums

[TryLockError](enum.TryLockError.html "enum bevy::platform::sync::TryLockError")

An enumeration of possible errors associated with a [`TryLockResult`](type.TryLockResult.html "type bevy::platform::sync::TryLockResult") which can occur while trying to acquire a lock, from the [`try_lock`](struct.Mutex.html#method.try_lock "method bevy::platform::sync::Mutex::try_lock") method on a [`Mutex`](struct.Mutex.html "struct bevy::platform::sync::Mutex") or the [`try_read`](struct.RwLock.html#method.try_read "method bevy::platform::sync::RwLock::try_read") and [`try_write`](struct.RwLock.html#method.try_write "method bevy::platform::sync::RwLock::try_write") methods on an [`RwLock`](struct.RwLock.html "struct bevy::platform::sync::RwLock").

## Type Aliases

[LockResult](type.LockResult.html "type bevy::platform::sync::LockResult")

A type alias for the result of a lock method which can be poisoned.

[TryLockResult](type.TryLockResult.html "type bevy::platform::sync::TryLockResult")

A type alias for the result of a nonblocking locking method.
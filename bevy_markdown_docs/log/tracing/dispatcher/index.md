[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)

# Module dispatcher 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/lib.rs.html#979)

Dispatches trace events to [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber")s.

The _dispatcher_ is the component of the tracing system which is responsible for forwarding trace data from the instrumentation points that generate it to the subscriber that collects it.

## Using the Trace Dispatcher

Every thread in a program using `tracing` has a _default subscriber_. When events occur, or spans are created, they are dispatched to the thread’s current subscriber.

### Setting the Default Subscriber

By default, the current subscriber is an empty implementation that does nothing. To use a subscriber implementation, it must be set as the default. There are two methods for doing so: [`with_default`](fn.with_default.html "fn bevy::log::tracing::dispatcher::with_default") and [`set_global_default`](fn.set_global_default.html "fn bevy::log::tracing::dispatcher::set_global_default"). `with_default` sets the default subscriber for the duration of a scope, while `set_global_default` sets a default subscriber for the entire process.

To use either of these functions, we must first wrap our subscriber in a [`Dispatch`](../struct.Dispatch.html "struct bevy::log::tracing::Dispatch"), a cloneable, type-erased reference to a subscriber. For example:

```rust
use dispatcher::Dispatch;

let my_subscriber = FooSubscriber::new();
let my_dispatch = Dispatch::new(my_subscriber);
```

Then, we can use [`with_default`](fn.with_default.html "fn bevy::log::tracing::dispatcher::with_default") to set our `Dispatch` as the default for the duration of a block:

```rust
// no default subscriber

dispatcher::with_default(&my_dispatch, || {
    // my_subscriber is the default
});

// no default subscriber again
```

It’s important to note that `with_default` will not propagate the current thread’s default subscriber to any threads spawned within the `with_default` block. To propagate the default subscriber to new threads, either use `with_default` from the new thread, or use `set_global_default`.

As an alternative to `with_default`, we can use [`set_global_default`](fn.set_global_default.html "fn bevy::log::tracing::dispatcher::set_global_default") to set a `Dispatch` as the default for all threads, for the lifetime of the program. For example:

```rust
// no default subscriber

dispatcher::set_global_default(my_dispatch)
    // `set_global_default` will return an error if the global default
    // subscriber has already been set.
    .expect("global default was already set!");

// `my_subscriber` is now the default
```

**Note**: The thread-local scoped dispatcher (`with_default`)
requires the Rust standard library. `no_std` users should
use [`set_global_default`](fn.set_global_default.html)
instead.

### Accessing the Default Subscriber

A thread’s current default subscriber can be accessed using the [`get_default`](fn.get_default.html "fn bevy::log::tracing::dispatcher::get_default") function, which executes a closure with a reference to the currently default `Dispatch`. This is used primarily by `tracing` instrumentation.

## Structs

[DefaultGuard](struct.DefaultGuard.html "struct bevy::log::tracing::dispatcher::DefaultGuard")`std`

A guard that resets the current default dispatcher to the prior default dispatcher when dropped.

[Dispatch](struct.Dispatch.html "struct bevy::log::tracing::dispatcher::Dispatch")

`Dispatch` trace data to a [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

[SetGlobalDefaultError](struct.SetGlobalDefaultError.html "struct bevy::log::tracing::dispatcher::SetGlobalDefaultError")

Returned if setting the global dispatcher fails.

[WeakDispatch](struct.WeakDispatch.html "struct bevy::log::tracing::dispatcher::WeakDispatch")

`WeakDispatch` is a version of [`Dispatch`](../struct.Dispatch.html "struct bevy::log::tracing::Dispatch") that holds a non-owning reference to a [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

## Functions

[get\_default](fn.get_default.html "fn bevy::log::tracing::dispatcher::get_default")`std`

Executes a closure with a reference to this thread’s current [dispatcher](../struct.Dispatch.html "struct bevy::log::tracing::Dispatch").

[set\_default](fn.set_default.html "fn bevy::log::tracing::dispatcher::set_default")`std`

Sets the dispatch as the default dispatch for the duration of the lifetime of the returned DefaultGuard

[set\_global\_default](fn.set_global_default.html "fn bevy::log::tracing::dispatcher::set_global_default")

Sets this dispatch as the global default for the duration of the entire program. Will be used as a fallback if no thread-local dispatch has been set in a thread (using `with_default`.)

[with\_default](fn.with_default.html "fn bevy::log::tracing::dispatcher::with_default")`std`

Sets this dispatch as the default for the duration of a closure.
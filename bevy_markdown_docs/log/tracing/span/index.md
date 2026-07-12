[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)

# Module span 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/lib.rs.html#984)

Spans represent periods of time in which a program was executing in a particular context.

A span consists of [fields](../field/index.html "mod bevy::log::tracing::field"), user-defined key-value pairs of arbitrary data that describe the context the span represents, and a set of fixed attributes that describe all `tracing` spans and events. Attributes describing spans include:

*   An [`Id`](../struct.Id.html "struct bevy::log::tracing::Id") assigned by the subscriber that uniquely identifies it in relation to other spans.
*   The span’s [parent](#span-relationships) in the trace tree.
*   [Metadata](../struct.Metadata.html "struct bevy::log::tracing::Metadata") that describes static characteristics of all spans originating from that callsite, such as its name, source code location, [verbosity level](../../struct.Level.html "struct bevy::log::Level"), and the names of its fields.

## Creating Spans

Spans are created using the [`span!`](../macro.span.html "macro bevy::log::tracing::span") macro. This macro is invoked with the following arguments, in order:

*   The [`target`](../struct.Metadata.html#method.target "method bevy::log::tracing::Metadata::target") and/or [`parent`](#span-relationships) attributes, if the user wishes to override their default values.
*   The span’s [verbosity level](../../struct.Level.html "struct bevy::log::Level")
*   A string literal providing the span’s name.
*   Finally, zero or more arbitrary key/value fields.

For example:

```rust
use tracing::{span, Level};

/// Construct a new span at the `INFO` level named "my_span", with a single
/// field named answer , with the value `42`.
let my_span = span!(Level::INFO, "my_span", answer = 42);
```

The documentation for the [`span!`](../macro.span.html "macro bevy::log::tracing::span") macro provides additional examples of the various options that exist when creating spans.

The [`trace_span!`](../../../prelude/macro.trace_span.html "macro bevy::prelude::trace_span"), [`debug_span!`](../../../prelude/macro.debug_span.html "macro bevy::prelude::debug_span"), [`info_span!`](../../../prelude/macro.info_span.html "macro bevy::prelude::info_span"), [`warn_span!`](../../../prelude/macro.warn_span.html "macro bevy::prelude::warn_span"), and [`error_span!`](../../../prelude/macro.error_span.html "macro bevy::prelude::error_span") exist as shorthand for constructing spans at various verbosity levels.

### Recording Span Creation

The [`Attributes`](struct.Attributes.html "struct bevy::log::tracing::span::Attributes") type contains data associated with a span, and is provided to the [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") when a new span is created. It contains the span’s metadata, the ID of [the span’s parent](#span-relationships) if one was explicitly set, and any fields whose values were recorded when the span was constructed. The subscriber, which is responsible for recording `tracing` data, can then store or record these values.

## The Span Lifecycle

### Entering a Span

A thread of execution is said to _enter_ a span when it begins executing, and _exit_ the span when it switches to another context. Spans may be entered through the [`enter`](../struct.Span.html#method.enter "method bevy::log::tracing::Span::enter"), [`entered`](../struct.Span.html#method.entered "method bevy::log::tracing::Span::entered"), and [`in_scope`](../struct.Span.html#method.in_scope "method bevy::log::tracing::Span::in_scope") methods.

The [`enter`](../struct.Span.html#method.enter "method bevy::log::tracing::Span::enter") method enters a span, returning a [guard](struct.Entered.html "struct bevy::log::tracing::span::Entered") that exits the span when dropped

```rust
let my_var: u64 = 5;
let my_span = span!(Level::TRACE, "my_span", my_var);

// `my_span` exists but has not been entered.

// Enter `my_span`...
let _enter = my_span.enter();

// Perform some work inside of the context of `my_span`...
// Dropping the `_enter` guard will exit the span.
```

     **Warning**: In asynchronous code that uses async/await syntax,
     `Span::enter` may produce incorrect traces if the returned drop
     guard is held across an await point. See
     [the method documentation](struct.Span.html#in-asynchronous-code)
     for details.
 

The [`entered`](../struct.Span.html#method.entered "method bevy::log::tracing::Span::entered") method is analogous to [`enter`](../struct.Span.html#method.enter "method bevy::log::tracing::Span::enter"), but moves the span into the returned guard, rather than borrowing it. This allows creating and entering a span in a single expression:

```rust
// Create a span and enter it, returning a guard:
let span = span!(Level::INFO, "my_span").entered();

// We are now inside the span! Like `enter()`, the guard returned by
// `entered()` will exit the span when it is dropped...

// ...but, it can also be exited explicitly, returning the `Span`
// struct:
let span = span.exit();
```

Finally, [`in_scope`](../struct.Span.html#method.in_scope "method bevy::log::tracing::Span::in_scope") takes a closure or function pointer and executes it inside the span:

```rust
let my_var: u64 = 5;
let my_span = span!(Level::TRACE, "my_span", my_var = &my_var);

my_span.in_scope(|| {
    // perform some work in the context of `my_span`...
});

// Perform some work outside of the context of `my_span`...

my_span.in_scope(|| {
    // Perform some more work in the context of `my_span`.
});
```

     **Note**: Since entering a span takes `&self`, and
     `Span`s are `Clone`, `Send`, and
     `Sync`, it is entirely valid for multiple threads to enter the
     same span concurrently.
 

### Span Relationships

Spans form a tree structure — unless it is a root span, all spans have a _parent_, and may have one or more _children_. When a new span is created, the current span becomes the new span’s parent. The total execution time of a span consists of the time spent in that span and in the entire subtree represented by its children. Thus, a parent span always lasts for at least as long as the longest-executing span in its subtree.

```rust
// this span is considered the "root" of a new trace tree:
span!(Level::INFO, "root").in_scope(|| {
    // since we are now inside "root", this span is considered a child
    // of "root":
    span!(Level::DEBUG, "outer_child").in_scope(|| {
        // this span is a child of "outer_child", which is in turn a
        // child of "root":
        span!(Level::TRACE, "inner_child").in_scope(|| {
            // and so on...
        });
    });
    // another span created here would also be a child of "root".
});
```

In addition, the parent of a span may be explicitly specified in the `span!` macro. For example:

```rust
// Create, but do not enter, a span called "foo".
let foo = span!(Level::INFO, "foo");

// Create and enter a span called "bar".
let bar = span!(Level::INFO, "bar");
let _enter = bar.enter();

// Although we have currently entered "bar", "baz"'s parent span
// will be "foo".
let baz = span!(parent: &foo, Level::INFO, "baz");
```

A child span should typically be considered _part_ of its parent. For example, if a subscriber is recording the length of time spent in various spans, it should generally include the time spent in a span’s children as part of that span’s duration.

In addition to having zero or one parent, a span may also _follow from_ any number of other spans. This indicates a causal relationship between the span and the spans that it follows from, but a follower is _not_ typically considered part of the duration of the span it follows. Unlike the parent, a span may record that it follows from another span after it is created, using the [`follows_from`](../struct.Span.html#method.follows_from "method bevy::log::tracing::Span::follows_from") method.

As an example, consider a listener task in a server. As the listener accepts incoming connections, it spawns new tasks that handle those connections. We might want to have a span representing the listener, and instrument each spawned handler task with its own span. We would want our instrumentation to record that the handler tasks were spawned as a result of the listener task. However, we might not consider the handler tasks to be _part_ of the time spent in the listener task, so we would not consider those spans children of the listener span. Instead, we would record that the handler tasks follow from the listener, recording the causal relationship but treating the spans as separate durations.

### Closing Spans

Execution may enter and exit a span multiple times before that span is _closed_. Consider, for example, a future which has an associated span and enters that span every time it is polled:

```rust
struct MyFuture {
   // data
   span: tracing::Span,
}

impl Future for MyFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let _enter = self.span.enter();
        // Do actual future work...
    }
}
```

If this future was spawned on an executor, it might yield one or more times before `poll` returns [`Poll::Ready`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Ready "variant core::task::poll::Poll::Ready"). If the future were to yield, then the executor would move on to poll the next future, which may _also_ enter an associated span or series of spans. Therefore, it is valid for a span to be entered repeatedly before it completes. Only the time when that span or one of its children was the current span is considered to be time spent in that span. A span which is not executing and has not yet been closed is said to be _idle_.

Because spans may be entered and exited multiple times before they close, [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber")s have separate trait methods which are called to notify them of span exits and when span handles are dropped. When execution exits a span, [`exit`](../trait.Subscriber.html#tymethod.exit "method bevy::log::tracing::Subscriber::exit") will always be called with that span’s ID to notify the subscriber that the span has been exited. When span handles are dropped, the [`drop_span`](../trait.Subscriber.html#method.drop_span "method bevy::log::tracing::Subscriber::drop_span") method is called with that span’s ID. The subscriber may use this to determine whether or not the span will be entered again.

If there is only a single handle with the capacity to exit a span, dropping that handle “closes” the span, since the capacity to enter it no longer exists. For example:

```rust
{
    span!(Level::TRACE, "my_span").in_scope(|| {
        // perform some work in the context of `my_span`...
    }); // --> Subscriber::exit(my_span)

    // The handle to `my_span` only lives inside of this block; when it is
    // dropped, the subscriber will be informed via `drop_span`.

} // --> Subscriber::drop_span(my_span)
```

However, if multiple handles exist, the span can still be re-entered even if one or more is dropped. For determining when _all_ handles to a span have been dropped, `Subscriber`s have a [`clone_span`](../trait.Subscriber.html#method.clone_span "method bevy::log::tracing::Subscriber::clone_span") method, which is called every time a span handle is cloned. Combined with `drop_span`, this may be used to track the number of handles to a given span — if `drop_span` has been called one more time than the number of calls to `clone_span` for a given ID, then no more handles to the span with that ID exist. The subscriber may then treat it as closed.

## When to use spans

As a rule of thumb, spans should be used to represent discrete units of work (e.g., a given request’s lifetime in a server) or periods of time spent in a given context (e.g., time spent interacting with an instance of an external system, such as a database).

Which scopes in a program correspond to new spans depend somewhat on user intent. For example, consider the case of a loop in a program. Should we construct one span and perform the entire loop inside of that span, like:

```rust
let span = span!(Level::TRACE, "my_loop");
let _enter = span.enter();
for i in 0..n {
    // ...
}
```

Or, should we create a new span for each iteration of the loop, as in:

```rust
for i in 0..n {
    let span = span!(Level::TRACE, "my_loop", iteration = i);
    let _enter = span.enter();
    // ...
}
```

Depending on the circumstances, we might want to do either, or both. For example, if we want to know how long was spent in the loop overall, we would create a single span around the entire loop; whereas if we wanted to know how much time was spent in each individual iteration, we would enter a new span on every iteration.

## Structs

[Attributes](struct.Attributes.html "struct bevy::log::tracing::span::Attributes")

Attributes provided to a `Subscriber` describing a new span when it is created.

[Entered](struct.Entered.html "struct bevy::log::tracing::span::Entered")

A guard representing a span which has been entered and is currently executing.

[EnteredSpan](struct.EnteredSpan.html "struct bevy::log::tracing::span::EnteredSpan")

An owned version of [`Entered`](struct.Entered.html "struct bevy::log::tracing::span::Entered"), a guard representing a span which has been entered and is currently executing.

[Id](struct.Id.html "struct bevy::log::tracing::span::Id")

Identifies a span within the context of a subscriber.

[Record](struct.Record.html "struct bevy::log::tracing::span::Record")

A set of fields recorded by a span.

[Span](struct.Span.html "struct bevy::log::tracing::span::Span")

A handle representing a span, with the capability to enter the span if it exists.

## Traits

[AsId](trait.AsId.html "trait bevy::log::tracing::span::AsId")

Trait implemented by types which have a span `Id`.
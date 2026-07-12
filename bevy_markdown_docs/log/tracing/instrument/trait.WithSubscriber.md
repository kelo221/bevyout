[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[instrument](index.html)

# Trait WithSubscriber 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#136)

```rust
pub trait WithSubscriber: Sized {
    // Provided methods
    fn with_subscriber<S>(self, subscriber: S) -> WithDispatch<Self> ⓘ
       where S: Into<Dispatch> { ... }
    fn with_current_subscriber(self) -> WithDispatch<Self> ⓘ { ... }
}
```

Extension trait allowing futures to be instrumented with a `tracing` [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber").

## Provided Methods

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#176-178)

#### fn [with\_subscriber](#method.with_subscriber)<S>(self, subscriber: S) -> [WithDispatch](struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

where S: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,

Attaches the provided [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper.

The attached [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") will be set as the [default](../dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") when the returned [`Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future") is polled.

##### Examples

```rust
use tracing::instrument::WithSubscriber;

// Set the default `Subscriber`
let _default = tracing::subscriber::set_default(MySubscriber::default());

tracing::info!("this event will be recorded by the default `Subscriber`");

// Create a different `Subscriber` and attach it to a future.
let other_subscriber = MyOtherSubscriber::default();
let future = async {
    tracing::info!("this event will be recorded by the other `Subscriber`");
    // ...
};

future
    // Attach the other `Subscriber` to the future before awaiting it
    .with_subscriber(other_subscriber)
    .await;

// Once the future has completed, we return to the default `Subscriber`.
tracing::info!("this event will be recorded by the default `Subscriber`");
```

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#228)

#### fn [with\_current\_subscriber](#method.with_current_subscriber)(self) -> [WithDispatch](struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch")<Self> [ⓘ](#)

Attaches the current [default](../dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") [`Subscriber`](../trait.Subscriber.html "trait bevy::log::tracing::Subscriber") to this type, returning a [`WithDispatch`](struct.WithDispatch.html "struct bevy::log::tracing::instrument::WithDispatch") wrapper.

The attached `Subscriber` will be set as the [default](../dispatcher/index.html#setting-the-default-subscriber "mod bevy::log::tracing::dispatcher") when the returned [`Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future") is polled.

This can be used to propagate the current dispatcher context when spawning a new future that may run on a different thread.

##### Examples

```rust
use tracing::instrument::WithSubscriber;

// Using `set_default` (rather than `set_global_default`) sets the
// default `Subscriber` for *this* thread only.
let _default = tracing::subscriber::set_default(MySubscriber::default());

let future = async {
    // ...
};

// If a multi-threaded async runtime is in use, this spawned task may
// run on a different thread, in a different default `Subscriber`'s context.
tokio::spawn(future);

// However, calling `with_current_subscriber` on the future before
// spawning it, ensures that the current thread's default `Subscriber` is
// propagated to the spawned task, regardless of where it executes:
tokio::spawn(future.with_current_subscriber());
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#393)

### impl<T> [WithSubscriber](trait.WithSubscriber.html "trait bevy::log::tracing::instrument::WithSubscriber") for T

{"WithDispatch<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.WithDispatch.html\\" title=\\"struct bevy::log::tracing::instrument::WithDispatch\\">WithDispatch</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}
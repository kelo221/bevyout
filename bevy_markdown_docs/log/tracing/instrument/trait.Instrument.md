[bevy](../../../index.html)::[log](../../index.html)::[tracing](../index.html)::[instrument](index.html)

# Trait Instrument 

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#20)

```rust
pub trait Instrument: Sized {
    // Provided methods
    fn instrument(self, span: Span) -> Instrumented<Self> ⓘ { ... }
    fn in_current_span(self) -> Instrumented<Self> ⓘ { ... }
}
```

Attaches spans to a [`std::future::Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future").

Extension trait allowing futures to be instrumented with a `tracing` [span](../struct.Span.html "struct bevy::log::tracing::Span").

## Provided Methods

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#86)

#### fn [instrument](#method.instrument)(self, span: [Span](../struct.Span.html "struct bevy::log::tracing::Span")) -> [Instrumented](struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the provided [`Span`](../struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper.

The attached [`Span`](../struct.Span.html "struct bevy::log::tracing::Span") will be [entered](../struct.Span.html#method.enter "method bevy::log::tracing::Span::enter") every time the instrumented [`Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future") is polled or [`Drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop")ped.

##### Examples

Instrumenting a future:

```rust
use tracing::Instrument;

let my_future = async {
    // ...
};

my_future
    .instrument(tracing::info_span!("my_future"))
    .await
```

The [`Span::or_current`](../struct.Span.html#method.or_current "method bevy::log::tracing::Span::or_current") combinator can be used in combination with `instrument` to ensure that the [current span](../struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") is attached to the future if the span passed to `instrument` is [disabled](../struct.Span.html#method.is_disabled "method bevy::log::tracing::Span::is_disabled"):

```rust
use tracing::Instrument;

let my_future = async {
    // ...
};

let outer_span = tracing::info_span!("outer").entered();

// If the "my_future" span is enabled, then the spawned task will
// be within both "my_future" *and* "outer", since "outer" is
// "my_future"'s parent. However, if "my_future" is disabled,
// the spawned task will *not* be in any span.
tokio::spawn(
    my_future
        .instrument(tracing::debug_span!("my_future"))
);

// Using `Span::or_current` ensures the spawned task is instrumented
// with the current span, if the new span passed to `instrument` is
// not enabled. This means that if the "my_future"  span is disabled,
// the spawned task will still be instrumented with the "outer" span:
tokio::spawn(
   my_future
        .instrument(tracing::debug_span!("my_future").or_current())
);
```

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#128)

#### fn [in\_current\_span](#method.in_current_span)(self) -> [Instrumented](struct.Instrumented.html "struct bevy::log::tracing::instrument::Instrumented")<Self> [ⓘ](#)

Instruments this type with the [current](../struct.Span.html#method.current "associated function bevy::log::tracing::Span::current") [`Span`](../struct.Span.html "struct bevy::log::tracing::Span"), returning an `Instrumented` wrapper.

The attached [`Span`](../struct.Span.html "struct bevy::log::tracing::Span") will be [entered](../struct.Span.html#method.enter "method bevy::log::tracing::Span::enter") every time the instrumented [`Future`](../../../tasks/futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future") is polled or [`Drop`](https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html "trait core::ops::drop::Drop")ped.

This can be used to propagate the current span when spawning a new future.

##### Examples

```rust
use tracing::Instrument;

let span = tracing::info_span!("my_span");
let _enter = span.enter();

// ...

let future = async {
    tracing::debug!("this event will occur inside `my_span`");
    // ...
};
tokio::spawn(future.in_current_span());
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing/0.1.44/x86_64-unknown-linux-gnu/src/tracing/instrument.rs.html#325)

### impl<T> [Instrument](../trait.Instrument.html "trait bevy::log::tracing::Instrument") for T

{"Instrumented<Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Instrumented.html\\" title=\\"struct bevy::log::tracing::instrument::Instrumented\\">Instrumented</a>&lt;T&gt;<div class=\\"where\\">where\\n T: <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;T as <a class=\\"trait\\" href=\\"../../../tasks/futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../../../tasks/futures\_lite/trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}
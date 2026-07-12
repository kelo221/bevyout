[bevy](../../../index.html)::[log](../../index.html)::[tracing\_subscriber](../index.html)::[util](index.html)

# Trait SubscriberInitExt 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/util.rs.html#26)

```rust
pub trait SubscriberInitExt: Into<Dispatch> {
    // Provided methods
    fn set_default(self) -> DefaultGuard { ... }
    fn try_init(self) -> Result<(), TryInitError> { ... }
    fn init(self) { ... }
}
```

Extension trait adding utility methods for subscriber initialization.

This trait provides extension methods to make configuring and setting a [default subscriber](https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html#setting-the-default-subscriber) more ergonomic. It is automatically implemented for all types that can be converted into a [trace dispatcher](https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html). Since `Dispatch` implements `From<T>` for all `T: Subscriber`, all `Subscriber` implementations will implement this extension trait as well. Types which can be converted into `Subscriber`s, such as builders that construct a `Subscriber`, may implement `Into<Dispatch>`, and will also receive an implementation of this trait.

## Provided Methods

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/util.rs.html#41)

#### fn [set\_default](#method.set_default)(self) -> [DefaultGuard](../../tracing/dispatcher/struct.DefaultGuard.html "struct bevy::log::tracing::dispatcher::DefaultGuard")

Available on **crate feature `std`** only.

Sets `self` as the [default subscriber](https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html#setting-the-default-subscriber) in the current scope, returning a guard that will unset it when dropped.

If the “tracing-log” feature flag is enabled, this will also initialize a [`log`](https://crates.io/log) compatibility layer. This allows the subscriber to consume `log::Record`s as though they were `tracing` `Event`s.

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/util.rs.html#61)

#### fn [try\_init](#method.try_init)(self) -> [Result](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result")<[()](https://doc.rust-lang.org/nightly/std/primitive.unit.html), [TryInitError](struct.TryInitError.html "struct bevy::log::tracing_subscriber::util::TryInitError")\>

Attempts to set `self` as the [global default subscriber](https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html#setting-the-default-subscriber) in the current scope, returning an error if one is already set.

If the “tracing-log” feature flag is enabled, this will also attempt to initialize a [`log`](https://crates.io/log) compatibility layer. This allows the subscriber to consume `log::Record`s as though they were `tracing` `Event`s.

This method returns an error if a global default subscriber has already been set, or if a `log` logger has already been set (when the “tracing-log” feature is enabled).

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/util.rs.html#92)

#### fn [init](#method.init)(self)

Attempts to set `self` as the [global default subscriber](https://docs.rs/tracing/0.1.21/tracing/dispatcher/index.html#setting-the-default-subscriber) in the current scope, panicking if this fails.

If the “tracing-log” feature flag is enabled, this will also attempt to initialize a [`log`](https://crates.io/log) compatibility layer. This allows the subscriber to consume `log::Record`s as though they were `tracing` `Event`s.

This method panics if a global default subscriber has already been set, or if a `log` logger has already been set (when the “tracing-log” feature is enabled).

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/util.rs.html#98)

### impl<T> [SubscriberInitExt](../prelude/trait._.html "trait bevy::log::tracing_subscriber::prelude::_") for T

where T: [Into](https://doc.rust-lang.org/nightly/core/convert/trait.Into.html "trait core::convert::Into")<[Dispatch](../../tracing/struct.Dispatch.html "struct bevy::log::tracing::Dispatch")\>,
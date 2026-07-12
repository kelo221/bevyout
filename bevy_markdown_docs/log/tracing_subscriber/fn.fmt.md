[bevy](../../index.html)::[log](../index.html)::[tracing\_subscriber](index.html)

# Function fmt 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#324)

```rust
pub fn fmt() -> SubscriberBuilder
```

Returns a new [`SubscriberBuilder`](fmt/struct.SubscriberBuilder.html "struct bevy::log::tracing_subscriber::fmt::SubscriberBuilder") for configuring a [formatting subscriber](struct.FmtSubscriber.html "struct bevy::log::tracing_subscriber::FmtSubscriber").

This is essentially shorthand for \[`SubscriberBuilder::default()]`.

## Examples

Using [`init`](fmt/struct.SubscriberBuilder.html#method.init "method bevy::log::tracing_subscriber::fmt::SubscriberBuilder::init") to set the default subscriber:

```rust
tracing_subscriber::fmt().init();
```

Configuring the output format:

```rust
tracing_subscriber::fmt()
    // Configure formatting settings.
    .with_target(false)
    .with_timer(tracing_subscriber::fmt::time::uptime())
    .with_level(true)
    // Set the subscriber as the default.
    .init();
```

[`try_init`](fmt/struct.SubscriberBuilder.html#method.try_init "method bevy::log::tracing_subscriber::fmt::SubscriberBuilder::try_init") returns an error if the default subscriber could not be set:

```rust
use std::error::Error;

fn init_subscriber() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        // Configure the subscriber to emit logs in JSON format.
        .json()
        // Configure the subscriber to flatten event fields in the output JSON objects.
        .flatten_event(true)
        // Set the subscriber as the default, returning an error if this fails.
        .try_init()?;

    Ok(())
}
```

Rather than setting the subscriber as the default, [`finish`](fmt/struct.SubscriberBuilder.html#method.finish "method bevy::log::tracing_subscriber::fmt::SubscriberBuilder::finish") _returns_ the constructed subscriber, which may then be passed to other functions:

```rust
let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .compact()
    .finish();

tracing::subscriber::with_default(subscriber, || {
    // the subscriber will only be set as the default
    // inside this closure...
})
```
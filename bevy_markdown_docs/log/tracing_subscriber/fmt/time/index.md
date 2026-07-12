[bevy](../../../../index.html)::[log](../../../index.html)::[tracing\_subscriber](../../index.html)::[fmt](../index.html)

# Module time 

[Source](https://docs.rs/tracing-subscriber/0.3.23/x86_64-unknown-linux-gnu/src/tracing_subscriber/fmt/mod.rs.html#202)

Available on **crate features `fmt` and `std`** only.

Formatters for event timestamps.

## Structs

[SystemTime](struct.SystemTime.html "struct bevy::log::tracing_subscriber::fmt::time::SystemTime")

Retrieve and print the current wall-clock time.

[Uptime](struct.Uptime.html "struct bevy::log::tracing_subscriber::fmt::time::Uptime")

Retrieve and print the relative elapsed wall-clock time since an epoch.

## Traits

[FormatTime](trait.FormatTime.html "trait bevy::log::tracing_subscriber::fmt::time::FormatTime")

A type that can measure and format the current time.

## Functions

[time](fn.time.html "fn bevy::log::tracing_subscriber::fmt::time::time")

Returns a new `SystemTime` timestamp provider.

[uptime](fn.uptime.html "fn bevy::log::tracing_subscriber::fmt::time::uptime")

Returns a new `Uptime` timestamp provider.
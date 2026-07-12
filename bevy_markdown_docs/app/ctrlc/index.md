[bevy](../../index.html)::[app](../index.html)

# Crate ctrlc 

[Source](https://docs.rs/ctrlc/3.5.2/x86_64-unknown-linux-gnu/src/ctrlc/lib.rs.html#10-148)

Cross platform handling of Ctrl-C signals.

[set\_handler()](fn.set_handler.html) allows setting a handler closure which is executed on `Ctrl+C`. On Unix, this corresponds to a `SIGINT` signal. On windows, `Ctrl+C` corresponds to [`CTRL_C_EVENT`](https://msdn.microsoft.com/en-us/library/windows/desktop/ms683242.aspx) or [`CTRL_BREAK_EVENT`](https://msdn.microsoft.com/en-us/library/windows/desktop/ms683242.aspx).

Setting a handler will start a new dedicated signal handling thread where we execute the handler each time we receive a `Ctrl+C` signal. There can only be one handler, you would typically set one at the start of your program.

## Example

```rust
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {}
    println!("Got it! Exiting...");
}
```

## Handling SIGTERM and SIGHUP

Handling of `SIGTERM` and `SIGHUP` can be enabled with `termination` feature. If this is enabled, the handler specified by `set_handler()` will be executed for `SIGINT`, `SIGTERM` and `SIGHUP`.

## Enums

[Error](enum.Error.html "enum bevy::app::ctrlc::Error")

Ctrl-C error.

[SignalType](enum.SignalType.html "enum bevy::app::ctrlc::SignalType")

A cross-platform way to represent Ctrl-C or program termination signal. Other signals/events are supported via `Other`\-variant.

## Functions

[set\_handler](fn.set_handler.html "fn bevy::app::ctrlc::set_handler")

Register signal handler for Ctrl-C.

[try\_set\_handler](fn.try_set_handler.html "fn bevy::app::ctrlc::try_set_handler")

The same as ctrlc::set\_handler but errors if a handler already exists for the signal(s).

## Type Aliases

[Signal](type.Signal.html "type bevy::app::ctrlc::Signal")

Platform specific signal type
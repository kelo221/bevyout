[bevy](../../index.html)::[app](../index.html)::[ctrlc](index.html)

# Function set\_handler 

[Source](https://docs.rs/ctrlc/3.5.2/x86_64-unknown-linux-gnu/src/ctrlc/lib.rs.html#93-95)

```rust
pub fn set_handler<F>(user_handler: F) -> Result<(), Error>where
    F: FnMut() + 'static + Send,
```

Register signal handler for Ctrl-C.

Starts a new dedicated signal handling thread. Should only be called once, typically at the start of your program.

## Example

```rust
ctrlc::set_handler(|| println!("Hello world!")).expect("Error setting Ctrl-C handler");
```

## Warning

On Unix, the handler registration for `SIGINT`, (`SIGTERM` and `SIGHUP` if termination feature is enabled) or `SA_SIGINFO` posix signal handlers will be overwritten. On Windows, multiple handler routines are allowed, but they are called on a last-registered, first-called basis until the signal is handled.

ctrlc::try\_set\_handler will error (on Unix) if another signal handler exists for the same signal(s) that ctrlc is trying to attach the handler to.

On Unix, signal dispositions and signal handlers are inherited by child processes created via `fork(2)` on, but not by child processes created via `execve(2)`. Signal handlers are not inherited on Windows.

## Errors

Will return an error if a system error occurred while setting the handler.

## Panics

Any panic in the handler will not be caught and will cause the signal handler thread to stop.
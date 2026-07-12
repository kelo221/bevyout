[bevy](../../index.html)::[app](../index.html)::[ctrlc](index.html)

# Function try\_set\_handler 

[Source](https://docs.rs/ctrlc/3.5.2/x86_64-unknown-linux-gnu/src/ctrlc/lib.rs.html#105-107)

```rust
pub fn try_set_handler<F>(user_handler: F) -> Result<(), Error>where
    F: FnMut() + 'static + Send,
```

The same as ctrlc::set\_handler but errors if a handler already exists for the signal(s).

## Errors

Will return an error if another handler exists or if a system error occurred while setting the handler.
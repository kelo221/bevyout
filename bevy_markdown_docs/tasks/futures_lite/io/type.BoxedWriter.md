[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Type Alias BoxedWriter 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#3028)

```rust
pub type BoxedWriter = Pin<Box<dyn AsyncWrite + Send>>;
```

Available on **crate features `alloc` and `std`** only.

Type alias for `Pin<Box<dyn AsyncWrite + Send + 'static>>`.

## Examples

```rust
use futures_lite::io::AsyncWriteExt;

let writer = Vec::<u8>::new().boxed_writer();
```

## Aliased Type

```rust
pub struct BoxedWriter { /* private fields */ }
```
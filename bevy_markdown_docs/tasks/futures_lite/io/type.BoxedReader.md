[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Type Alias BoxedReader 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#3016)

```rust
pub type BoxedReader = Pin<Box<dyn AsyncRead + Send>>;
```

Available on **crate features `alloc` and `std`** only.

Type alias for `Pin<Box<dyn AsyncRead + Send + 'static>>`.

## Examples

```rust
use futures_lite::io::AsyncReadExt;

let reader = [1, 2, 3].boxed_reader();
```

## Aliased Type

```rust
pub struct BoxedReader { /* private fields */ }
```
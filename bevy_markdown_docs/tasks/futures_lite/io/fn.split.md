[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Function split 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#3042-3044)

```rust
pub fn split<T>(stream: T) -> (ReadHalf<T>, WriteHalf<T>)where
    T: AsyncRead + AsyncWrite + Unpin,
```

Available on **crate feature `std`** only.

Splits a stream into [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") and [`AsyncWrite`](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") halves.

## Examples

```rust
use futures_lite::io::{self, Cursor};

let stream = Cursor::new(vec![]);
let (mut reader, mut writer) = io::split(stream);
```
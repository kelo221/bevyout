[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Function repeat 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1490)

```rust
pub fn repeat(byte: u8) -> Repeat
```

Available on **crate feature `std`** only.

Creates an infinite reader that reads the same byte repeatedly.

## Examples

```rust
use futures_lite::io::{self, AsyncReadExt};

let mut reader = io::repeat(b'a');

let mut contents = vec![0; 5];
reader.read_exact(&mut contents).await?;
assert_eq!(contents, b"aaaaa");
```
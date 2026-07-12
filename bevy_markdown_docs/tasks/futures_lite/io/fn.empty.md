[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Function empty 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1443)

```rust
pub fn empty() -> Empty
```

Available on **crate feature `std`** only.

Creates an empty reader.

## Examples

```rust
use futures_lite::io::{self, AsyncReadExt};

let mut reader = io::empty();

let mut contents = Vec::new();
reader.read_to_end(&mut contents).await?;
assert!(contents.is_empty());
```
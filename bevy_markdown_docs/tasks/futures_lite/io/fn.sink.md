[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Function sink 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1522)

```rust
pub fn sink() -> Sink
```

Available on **crate feature `std`** only.

Creates a writer that consumes and drops all data.

## Examples

```rust
use futures_lite::io::{self, AsyncWriteExt};

let mut writer = io::sink();
writer.write_all(b"hello").await?;
```
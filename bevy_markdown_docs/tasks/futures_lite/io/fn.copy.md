[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Function copy 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#67-70)

```rust
pub async fn copy<R, W>(reader: R, writer: W) -> Result<u64, Error>where
    R: AsyncRead,
    W: AsyncWrite,
```

Available on **crate feature `std`** only.

Copies the entire contents of a reader into a writer.

This function will read data from `reader` and write it into `writer` in a streaming fashion until `reader` returns EOF.

On success, returns the total number of bytes copied.

## Examples

```rust
use futures_lite::io::{self, BufReader, BufWriter};

let input: &[u8] = b"hello";
let reader = BufReader::new(input);

let mut output = Vec::new();
let writer = BufWriter::new(&mut output);

io::copy(reader, writer).await?;
```
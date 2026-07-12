[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Trait AsyncBufReadExt 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1550)

```rust
pub trait AsyncBufReadExt: AsyncBufRead {
    // Provided methods
    fn fill_buf(&mut self) -> FillBuf<'_, Self> ⓘ
       where Self: Unpin { ... }
    fn consume(&mut self, amt: usize)
       where Self: Unpin { ... }
    fn read_until<'a>(
        &'a mut self,
        byte: u8,
        buf: &'a mut Vec<u8>,
    ) -> ReadUntilFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_line<'a>(
        &'a mut self,
        buf: &'a mut String,
    ) -> ReadLineFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn lines(self) -> Lines<Self>
       where Self: Sized { ... }
    fn split(self, byte: u8) -> Split<Self>
       where Self: Sized { ... }
}
```

Available on **crate feature `std`** only.

Extension trait for [`AsyncBufRead`](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead").

## Provided Methods

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1572-1574)

#### fn [fill\_buf](#method.fill_buf)(&mut self) -> [FillBuf](struct.FillBuf.html "struct bevy::tasks::futures_lite::io::FillBuf")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Returns the contents of the internal buffer, filling it with more data if empty.

If the stream has reached EOF, an empty buffer will be returned.

##### Examples

```rust
use futures_lite::io::{AsyncBufReadExt, BufReader};
use std::pin::Pin;

let input: &[u8] = b"hello world";
let mut reader = BufReader::with_capacity(5, input);

assert_eq!(reader.fill_buf().await?, b"hello");
reader.consume(2);
assert_eq!(reader.fill_buf().await?, b"llo");
reader.consume(3);
assert_eq!(reader.fill_buf().await?, b" worl");
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1602-1604)

#### fn [consume](#method.consume)(&mut self, amt: [usize](https://doc.rust-lang.org/nightly/std/primitive.usize.html))

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Consumes `amt` buffered bytes.

This method does not perform any I/O, it simply consumes some amount of bytes from the internal buffer.

The `amt` must be <= the number of bytes in the buffer returned by [`fill_buf()`](../trait.AsyncBufReadExt.html#method.fill_buf "method bevy::tasks::futures_lite::AsyncBufReadExt::fill_buf").

##### Examples

```rust
use futures_lite::io::{AsyncBufReadExt, BufReader};
use std::pin::Pin;

let input: &[u8] = b"hello";
let mut reader = BufReader::with_capacity(4, input);

assert_eq!(reader.fill_buf().await?, b"hell");
reader.consume(2);
assert_eq!(reader.fill_buf().await?, b"ll");
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1629-1631)

#### fn [read\_until](#method.read_until)<'a>( &'a mut self, byte: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html), buf: &'a mut [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, ) -> [ReadUntilFuture](struct.ReadUntilFuture.html "struct bevy::tasks::futures_lite::io::ReadUntilFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Reads all bytes and appends them into `buf` until the delimiter `byte` or EOF is found.

This method will read bytes from the underlying stream until the delimiter or EOF is found. All bytes up to and including the delimiter (if found) will be appended to `buf`.

If successful, returns the total number of bytes read.

##### Examples

```rust
use futures_lite::io::{AsyncBufReadExt, BufReader};

let input: &[u8] = b"hello";
let mut reader = BufReader::new(input);

let mut buf = Vec::new();
let n = reader.read_until(b'\n', &mut buf).await?;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1662-1664)

#### fn [read\_line](#method.read_line)<'a>(&'a mut self, buf: &'a mut [String](../../../prelude/struct.String.html "struct bevy::prelude::String")) -> [ReadLineFuture](struct.ReadLineFuture.html "struct bevy::tasks::futures_lite::io::ReadLineFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Reads all bytes and appends them into `buf` until a newline (the 0xA byte) or EOF is found.

This method will read bytes from the underlying stream until the newline delimiter (the 0xA byte) or EOF is found. All bytes up to, and including, the newline delimiter (if found) will be appended to `buf`.

If successful, returns the total number of bytes read.

##### Examples

```rust
use futures_lite::io::{AsyncBufReadExt, BufReader};

let input: &[u8] = b"hello";
let mut reader = BufReader::new(input);

let mut line = String::new();
let n = reader.read_line(&mut line).await?;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1697-1699)

#### fn [lines](#method.lines)(self) -> [Lines](struct.Lines.html "struct bevy::tasks::futures_lite::io::Lines")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a stream over the lines of this byte stream.

The stream returned from this method yields items of type [`io::Result`](type.Result.html "type bevy::tasks::futures_lite::io::Result")`<`[`String`](../../../prelude/struct.String.html "struct bevy::prelude::String")`>`. Each string returned will _not_ have a newline byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end.

##### Examples

```rust
use futures_lite::io::{AsyncBufReadExt, BufReader};
use futures_lite::stream::StreamExt;

let input: &[u8] = b"hello\nworld\n";
let mut reader = BufReader::new(input);
let mut lines = reader.lines();

while let Some(line) = lines.next().await {
    println!("{}", line?);
}
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1730-1732)

#### fn [split](#method.split)(self, byte: [u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)) -> [Split](struct.Split.html "struct bevy::tasks::futures_lite::io::Split")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Returns a stream over the contents of this reader split on the specified `byte`.

The stream returned from this method yields items of type [`io::Result`](type.Result.html "type bevy::tasks::futures_lite::io::Result")`<`[`Vec<u8>`](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")`>`. Each vector returned will _not_ have the delimiter byte at the end.

##### Examples

```rust
use futures_lite::io::{AsyncBufReadExt, Cursor};
use futures_lite::stream::StreamExt;

let cursor = Cursor::new(b"lorem-ipsum-dolor");
let items: Vec<Vec<u8>> = cursor.split(b'-').try_collect().await?;

assert_eq!(items[0], b"lorem");
assert_eq!(items[1], b"ipsum");
assert_eq!(items[2], b"dolor");
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1743)

### impl<R> [AsyncBufReadExt](../trait.AsyncBufReadExt.html "trait bevy::tasks::futures_lite::AsyncBufReadExt") for R

where R: [AsyncBufRead](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

{"FillBuf<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.FillBuf.html\\" title=\\"struct bevy::tasks::futures\_lite::io::FillBuf\\">FillBuf</a>&lt;'a, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;'a, R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.FillBuf.html\\" title=\\"struct bevy::tasks::futures\_lite::io::FillBuf\\">FillBuf</a>&lt;'a, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncBufRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncBufRead\\">AsyncBufRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;&amp;'a \[<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\\">u8</a>\], <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","ReadLineFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ReadLineFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadLineFuture\\">ReadLineFuture</a>&lt;'\_, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ReadLineFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadLineFuture\\">ReadLineFuture</a>&lt;'\_, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncBufRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncBufRead\\">AsyncBufRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","ReadUntilFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ReadUntilFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadUntilFuture\\">ReadUntilFuture</a>&lt;'\_, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ReadUntilFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadUntilFuture\\">ReadUntilFuture</a>&lt;'\_, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncBufRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncBufRead\\">AsyncBufRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>"}
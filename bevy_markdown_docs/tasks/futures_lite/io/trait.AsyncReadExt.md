[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[io](index.html)

# Trait AsyncReadExt 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1965)

```rust
pub trait AsyncReadExt: AsyncRead {
    // Provided methods
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_vectored<'a>(
        &'a mut self,
        bufs: &'a mut [IoSliceMut<'a>],
    ) -> ReadVectoredFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_to_end<'a>(
        &'a mut self,
        buf: &'a mut Vec<u8>,
    ) -> ReadToEndFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_to_string<'a>(
        &'a mut self,
        buf: &'a mut String,
    ) -> ReadToStringFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn read_exact<'a>(
        &'a mut self,
        buf: &'a mut [u8],
    ) -> ReadExactFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn take(self, limit: u64) -> Take<Self>
       where Self: Sized { ... }
    fn bytes(self) -> Bytes<Self>
       where Self: Sized { ... }
    fn chain<R>(self, next: R) -> Chain<Self, R>
       where R: AsyncRead,
             Self: Sized { ... }
    fn boxed_reader<'a>(self) -> Pin<Box<dyn AsyncRead + Send + 'a>>
       where Self: Sized + Send + 'a { ... }
}
```

Available on **crate feature `std`** only.

Extension trait for [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead").

## Provided Methods

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#1993-1995)

#### fn [read](#method.read)<'a>(&'a mut self, buf: &'a mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [ReadFuture](struct.ReadFuture.html "struct bevy::tasks::futures_lite::io::ReadFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Reads some bytes from the byte stream.

On success, returns the total number of bytes read.

If the return value is `Ok(n)`, then it must be guaranteed that `0 <= n <= buf.len()`. A nonzero `n` value indicates that the buffer has been filled with `n` bytes of data. If `n` is `0`, then it can indicate one of two scenarios:

1.  This reader has reached its “end of file” and will likely no longer be able to produce bytes. Note that this does not mean that the reader will always no longer be able to produce bytes.
2.  The buffer specified was 0 bytes in length.

##### Examples

```rust
use futures_lite::io::{AsyncReadExt, BufReader};

let input: &[u8] = b"hello";
let mut reader = BufReader::new(input);

let mut buf = vec![0; 1024];
let n = reader.read(&mut buf).await?;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2005-2010)

#### fn [read\_vectored](#method.read_vectored)<'a>( &'a mut self, bufs: &'a mut \[[IoSliceMut](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSliceMut.html "struct core::io::io_slice::IoSliceMut")<'a>\], ) -> [ReadVectoredFuture](struct.ReadVectoredFuture.html "struct bevy::tasks::futures_lite::io::ReadVectoredFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Like [`read()`](../../../asset/trait.AsyncReadExt.html#method.read "method bevy::asset::AsyncReadExt::read"), except it reads into a slice of buffers.

Data is copied to fill each buffer in order, with the final buffer possibly being only partially filled. This method must behave same as a single call to [`read()`](../../../asset/trait.AsyncReadExt.html#method.read "method bevy::asset::AsyncReadExt::read") with the buffers concatenated would.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2033-2035)

#### fn [read\_to\_end](#method.read_to_end)<'a>( &'a mut self, buf: &'a mut [Vec](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec")<[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\>, ) -> [ReadToEndFuture](struct.ReadToEndFuture.html "struct bevy::tasks::futures_lite::io::ReadToEndFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Reads the entire contents and appends them to a [`Vec`](../../../prelude/struct.Vec.html "struct bevy::prelude::Vec").

On success, returns the total number of bytes read.

##### Examples

```rust
use futures_lite::io::{AsyncReadExt, Cursor};

let mut reader = Cursor::new(vec![1, 2, 3]);
let mut contents = Vec::new();

let n = reader.read_to_end(&mut contents).await?;
assert_eq!(n, 3);
assert_eq!(contents, [1, 2, 3]);
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2063-2065)

#### fn [read\_to\_string](#method.read_to_string)<'a>( &'a mut self, buf: &'a mut [String](../../../prelude/struct.String.html "struct bevy::prelude::String"), ) -> [ReadToStringFuture](struct.ReadToStringFuture.html "struct bevy::tasks::futures_lite::io::ReadToStringFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Reads the entire contents and appends them to a [`String`](../../../prelude/struct.String.html "struct bevy::prelude::String").

On success, returns the total number of bytes read.

##### Examples

```rust
use futures_lite::io::{AsyncReadExt, Cursor};

let mut reader = Cursor::new(&b"hello");
let mut contents = String::new();

let n = reader.read_to_string(&mut contents).await?;
assert_eq!(n, 5);
assert_eq!(contents, "hello");
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2090-2092)

#### fn [read\_exact](#method.read_exact)<'a>(&'a mut self, buf: &'a mut \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [ReadExactFuture](struct.ReadExactFuture.html "struct bevy::tasks::futures_lite::io::ReadExactFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Reads the exact number of bytes required to fill `buf`.

##### Examples

```rust
use futures_lite::io::{AsyncReadExt, Cursor};

let mut reader = Cursor::new(&b"hello");
let mut contents = vec![0; 3];

reader.read_exact(&mut contents).await?;
assert_eq!(contents, b"hel");
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2116-2118)

#### fn [take](#method.take)(self, limit: [u64](https://doc.rust-lang.org/nightly/std/primitive.u64.html)) -> [Take](struct.Take.html "struct bevy::tasks::futures_lite::io::Take")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an adapter which will read at most `limit` bytes from it.

This method returns a new instance of [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") which will read at most `limit` bytes, after which it will always return `Ok(0)` indicating EOF.

##### Examples

```rust
use futures_lite::io::{AsyncReadExt, Cursor};

let mut reader = Cursor::new(&b"hello");
let mut contents = String::new();

let n = reader.take(3).read_to_string(&mut contents).await?;
assert_eq!(n, 3);
assert_eq!(contents, "hel");
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2140-2142)

#### fn [bytes](#method.bytes)(self) -> [Bytes](struct.Bytes.html "struct bevy::tasks::futures_lite::io::Bytes")<Self>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Converts this [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") into a [`Stream`](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") of bytes.

The returned type implements [`Stream`](../trait.Stream.html "trait bevy::tasks::futures_lite::Stream") where `Item` is `io::Result<u8>`.

```rust
use futures_lite::io::{AsyncReadExt, Cursor};
use futures_lite::stream::StreamExt;

let reader = Cursor::new(&b"hello");
let mut bytes = reader.bytes();

while let Some(byte) = bytes.next().await {
    println!("byte: {}", byte?);
}
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2167-2169)

#### fn [chain](#method.chain)<R>(self, next: R) -> [Chain](struct.Chain.html "struct bevy::tasks::futures_lite::io::Chain")<Self, R>

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead"), Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

Creates an adapter which will chain this stream with another.

The returned [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") instance will first read all bytes from this reader until EOF is found, and then continue with `next`.

##### Examples

```rust
use futures_lite::io::{AsyncReadExt, Cursor};

let r1 = Cursor::new(&b"hello");
let r2 = Cursor::new(&b"world");
let mut reader = r1.chain(r2);

let mut contents = String::new();
reader.read_to_string(&mut contents).await?;
assert_eq!(contents, "helloworld");
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2188-2190)

#### fn [boxed\_reader](#method.boxed_reader)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Boxes the reader and changes its type to `dyn AsyncRead + Send + 'a`.

##### Examples

```rust
use futures_lite::io::AsyncReadExt;

let reader = [1, 2, 3].boxed_reader();
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2196)

### impl<R> [AsyncReadExt](../../../asset/trait.AsyncReadExt.html "trait bevy::asset::AsyncReadExt") for R

where R: [AsyncRead](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

{"ReadExactFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ReadExactFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadExactFuture\\">ReadExactFuture</a>&lt;'\_, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ReadExactFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadExactFuture\\">ReadExactFuture</a>&lt;'\_, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncRead\\">AsyncRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","ReadFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ReadFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadFuture\\">ReadFuture</a>&lt;'\_, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ReadFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadFuture\\">ReadFuture</a>&lt;'\_, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncRead\\">AsyncRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","ReadToEndFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ReadToEndFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadToEndFuture\\">ReadToEndFuture</a>&lt;'\_, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ReadToEndFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadToEndFuture\\">ReadToEndFuture</a>&lt;'\_, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncRead\\">AsyncRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","ReadToStringFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ReadToStringFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadToStringFuture\\">ReadToStringFuture</a>&lt;'\_, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ReadToStringFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadToStringFuture\\">ReadToStringFuture</a>&lt;'\_, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncRead\\">AsyncRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","ReadVectoredFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.ReadVectoredFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadVectoredFuture\\">ReadVectoredFuture</a>&lt;'\_, R&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;R&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.ReadVectoredFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::ReadVectoredFuture\\">ReadVectoredFuture</a>&lt;'\_, R&gt;<div class=\\"where\\">where\\n R: <a class=\\"trait\\" href=\\"../trait.AsyncRead.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncRead\\">AsyncRead</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>"}
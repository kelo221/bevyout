[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)

# Module io 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/lib.rs.html#62)

Available on **crate feature `std`** only.

Tools and combinators for I/O.

## Examples

```rust
use futures_lite::io::{self, AsyncReadExt};

let input: &[u8] = b"hello";
let mut reader = io::BufReader::new(input);

let mut contents = String::new();
reader.read_to_string(&mut contents).await?;
```

## Structs

[AssertAsync](struct.AssertAsync.html "struct bevy::tasks::futures_lite::io::AssertAsync")

Asserts that a type implementing [`std::io`](https://doc.rust-lang.org/nightly/std/io/index.html "mod std::io") traits can be used as an async type.

[AsyncAsSync](struct.AsyncAsSync.html "struct bevy::tasks::futures_lite::io::AsyncAsSync")

A wrapper around a type that implements `AsyncRead` or `AsyncWrite` that converts `Pending` polls to `WouldBlock` errors.

[BlockOn](struct.BlockOn.html "struct bevy::tasks::futures_lite::io::BlockOn")

Blocks on all async I/O operations and implements [`std::io`](https://doc.rust-lang.org/nightly/std/io/index.html "mod std::io") traits.

[BufReader](struct.BufReader.html "struct bevy::tasks::futures_lite::io::BufReader")

Adds buffering to a reader.

[BufWriter](struct.BufWriter.html "struct bevy::tasks::futures_lite::io::BufWriter")

Adds buffering to a writer.

[Bytes](struct.Bytes.html "struct bevy::tasks::futures_lite::io::Bytes")

Reader for the [`AsyncReadExt::bytes()`](../../../asset/trait.AsyncReadExt.html#method.bytes "method bevy::asset::AsyncReadExt::bytes") method.

[Chain](struct.Chain.html "struct bevy::tasks::futures_lite::io::Chain")

Reader for the [`AsyncReadExt::chain()`](../../../asset/trait.AsyncReadExt.html#method.chain "method bevy::asset::AsyncReadExt::chain") method.

[CloseFuture](struct.CloseFuture.html "struct bevy::tasks::futures_lite::io::CloseFuture")

Future for the [`AsyncWriteExt::close()`](../../../asset/trait.AsyncWriteExt.html#method.close "method bevy::asset::AsyncWriteExt::close") method.

[Cursor](struct.Cursor.html "struct bevy::tasks::futures_lite::io::Cursor")

Gives an in-memory buffer a cursor for reading and writing.

[Empty](struct.Empty.html "struct bevy::tasks::futures_lite::io::Empty")

Reader for the [`empty()`](fn.empty.html "fn bevy::tasks::futures_lite::io::empty") function.

[Error](struct.Error.html "struct bevy::tasks::futures_lite::io::Error")

The error type for I/O operations of the [`Read`](https://doc.rust-lang.org/nightly/std/io/trait.Read.html "trait std::io::Read"), [`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html "trait std::io::Write"), [`Seek`](https://doc.rust-lang.org/nightly/std/io/trait.Seek.html "trait std::io::Seek"), and associated traits.

[FillBuf](struct.FillBuf.html "struct bevy::tasks::futures_lite::io::FillBuf")

Future for the [`AsyncBufReadExt::fill_buf()`](../trait.AsyncBufReadExt.html#method.fill_buf "method bevy::tasks::futures_lite::AsyncBufReadExt::fill_buf") method.

[FlushFuture](struct.FlushFuture.html "struct bevy::tasks::futures_lite::io::FlushFuture")

Future for the [`AsyncWriteExt::flush()`](../../../asset/trait.AsyncWriteExt.html#method.flush "method bevy::asset::AsyncWriteExt::flush") method.

[Lines](struct.Lines.html "struct bevy::tasks::futures_lite::io::Lines")

Stream for the [`AsyncBufReadExt::lines()`](../trait.AsyncBufReadExt.html#method.lines "method bevy::tasks::futures_lite::AsyncBufReadExt::lines") method.

[ReadExactFuture](struct.ReadExactFuture.html "struct bevy::tasks::futures_lite::io::ReadExactFuture")

Future for the [`AsyncReadExt::read_exact()`](../../../asset/trait.AsyncReadExt.html#method.read_exact "method bevy::asset::AsyncReadExt::read_exact") method.

[ReadFuture](struct.ReadFuture.html "struct bevy::tasks::futures_lite::io::ReadFuture")

Future for the [`AsyncReadExt::read()`](../../../asset/trait.AsyncReadExt.html#method.read "method bevy::asset::AsyncReadExt::read") method.

[ReadHalf](struct.ReadHalf.html "struct bevy::tasks::futures_lite::io::ReadHalf")

The read half returned by [`split()`](fn.split.html "fn bevy::tasks::futures_lite::io::split").

[ReadLineFuture](struct.ReadLineFuture.html "struct bevy::tasks::futures_lite::io::ReadLineFuture")

Future for the [`AsyncBufReadExt::read_line()`](../trait.AsyncBufReadExt.html#method.read_line "method bevy::tasks::futures_lite::AsyncBufReadExt::read_line") method.

[ReadToEndFuture](struct.ReadToEndFuture.html "struct bevy::tasks::futures_lite::io::ReadToEndFuture")

Future for the [`AsyncReadExt::read_to_end()`](../../../asset/trait.AsyncReadExt.html#method.read_to_end "method bevy::asset::AsyncReadExt::read_to_end") method.

[ReadToStringFuture](struct.ReadToStringFuture.html "struct bevy::tasks::futures_lite::io::ReadToStringFuture")

Future for the [`AsyncReadExt::read_to_string()`](../../../asset/trait.AsyncReadExt.html#method.read_to_string "method bevy::asset::AsyncReadExt::read_to_string") method.

[ReadUntilFuture](struct.ReadUntilFuture.html "struct bevy::tasks::futures_lite::io::ReadUntilFuture")

Future for the [`AsyncBufReadExt::read_until()`](../trait.AsyncBufReadExt.html#method.read_until "method bevy::tasks::futures_lite::AsyncBufReadExt::read_until") method.

[ReadVectoredFuture](struct.ReadVectoredFuture.html "struct bevy::tasks::futures_lite::io::ReadVectoredFuture")

Future for the [`AsyncReadExt::read_vectored()`](../../../asset/trait.AsyncReadExt.html#method.read_vectored "method bevy::asset::AsyncReadExt::read_vectored") method.

[Repeat](struct.Repeat.html "struct bevy::tasks::futures_lite::io::Repeat")

Reader for the [`repeat()`](fn.repeat.html "fn bevy::tasks::futures_lite::io::repeat") function.

[SeekFuture](struct.SeekFuture.html "struct bevy::tasks::futures_lite::io::SeekFuture")

Future for the [`AsyncSeekExt::seek()`](../../../asset/trait.AsyncSeekExt.html#method.seek "method bevy::asset::AsyncSeekExt::seek") method.

[Sink](struct.Sink.html "struct bevy::tasks::futures_lite::io::Sink")

Writer for the [`sink()`](fn.sink.html "fn bevy::tasks::futures_lite::io::sink") function.

[Split](struct.Split.html "struct bevy::tasks::futures_lite::io::Split")

Stream for the [`AsyncBufReadExt::split()`](../trait.AsyncBufReadExt.html#method.split "method bevy::tasks::futures_lite::AsyncBufReadExt::split") method.

[Take](struct.Take.html "struct bevy::tasks::futures_lite::io::Take")

Reader for the [`AsyncReadExt::take()`](../../../asset/trait.AsyncReadExt.html#method.take "method bevy::asset::AsyncReadExt::take") method.

[WriteAllFuture](struct.WriteAllFuture.html "struct bevy::tasks::futures_lite::io::WriteAllFuture")

Future for the [`AsyncWriteExt::write_all()`](../../../asset/trait.AsyncWriteExt.html#method.write_all "method bevy::asset::AsyncWriteExt::write_all") method.

[WriteFuture](struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")

Future for the [`AsyncWriteExt::write()`](../../../asset/trait.AsyncWriteExt.html#method.write "method bevy::asset::AsyncWriteExt::write") method.

[WriteHalf](struct.WriteHalf.html "struct bevy::tasks::futures_lite::io::WriteHalf")

The write half returned by [`split()`](fn.split.html "fn bevy::tasks::futures_lite::io::split").

[WriteVectoredFuture](struct.WriteVectoredFuture.html "struct bevy::tasks::futures_lite::io::WriteVectoredFuture")

Future for the [`AsyncWriteExt::write_vectored()`](../../../asset/trait.AsyncWriteExt.html#method.write_vectored "method bevy::asset::AsyncWriteExt::write_vectored") method.

## Enums

[ErrorKind](enum.ErrorKind.html "enum bevy::tasks::futures_lite::io::ErrorKind")

A list specifying general categories of I/O error.

[SeekFrom](enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom")

Enumeration of possible methods to seek within an I/O object.

## Traits

[AsyncBufRead](trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::io::AsyncBufRead")

Read bytes asynchronously.

[AsyncBufReadExt](trait.AsyncBufReadExt.html "trait bevy::tasks::futures_lite::io::AsyncBufReadExt")

Extension trait for [`AsyncBufRead`](../trait.AsyncBufRead.html "trait bevy::tasks::futures_lite::AsyncBufRead").

[AsyncRead](trait.AsyncRead.html "trait bevy::tasks::futures_lite::io::AsyncRead")

Read bytes asynchronously.

[AsyncReadExt](trait.AsyncReadExt.html "trait bevy::tasks::futures_lite::io::AsyncReadExt")

Extension trait for [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead").

[AsyncSeek](trait.AsyncSeek.html "trait bevy::tasks::futures_lite::io::AsyncSeek")

Seek bytes asynchronously.

[AsyncSeekExt](trait.AsyncSeekExt.html "trait bevy::tasks::futures_lite::io::AsyncSeekExt")

Extension trait for [`AsyncSeek`](../trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek").

[AsyncWrite](trait.AsyncWrite.html "trait bevy::tasks::futures_lite::io::AsyncWrite")

Write bytes asynchronously.

[AsyncWriteExt](trait.AsyncWriteExt.html "trait bevy::tasks::futures_lite::io::AsyncWriteExt")

Extension trait for [`AsyncWrite`](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite").

## Functions

[copy](fn.copy.html "fn bevy::tasks::futures_lite::io::copy")

Copies the entire contents of a reader into a writer.

[empty](fn.empty.html "fn bevy::tasks::futures_lite::io::empty")

Creates an empty reader.

[repeat](fn.repeat.html "fn bevy::tasks::futures_lite::io::repeat")

Creates an infinite reader that reads the same byte repeatedly.

[sink](fn.sink.html "fn bevy::tasks::futures_lite::io::sink")

Creates a writer that consumes and drops all data.

[split](fn.split.html "fn bevy::tasks::futures_lite::io::split")

Splits a stream into [`AsyncRead`](../trait.AsyncRead.html "trait bevy::tasks::futures_lite::AsyncRead") and [`AsyncWrite`](../trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") halves.

## Type Aliases

[BoxedReader](type.BoxedReader.html "type bevy::tasks::futures_lite::io::BoxedReader")`alloc`

Type alias for `Pin<Box<dyn AsyncRead + Send + 'static>>`.

[BoxedWriter](type.BoxedWriter.html "type bevy::tasks::futures_lite::io::BoxedWriter")`alloc`

Type alias for `Pin<Box<dyn AsyncWrite + Send + 'static>>`.

[Result](type.Result.html "type bevy::tasks::futures_lite::io::Result")

A specialized [`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html "enum core::result::Result") type for I/O operations.
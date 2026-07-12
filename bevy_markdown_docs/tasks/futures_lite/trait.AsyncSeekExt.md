[bevy](../../index.html)::[tasks](../index.html)::[futures\_lite](index.html)

# Trait AsyncSeekExt 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2723)

```rust
pub trait AsyncSeekExt: AsyncSeek {
    // Provided method
    fn seek(&mut self, pos: SeekFrom) -> SeekFuture<'_, Self> ⓘ
       where Self: Unpin { ... }
}
```

Extension trait for [`AsyncSeek`](trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek").

## Provided Methods

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2745-2747)

#### fn [seek](#method.seek)(&mut self, pos: [SeekFrom](io/enum.SeekFrom.html "enum bevy::tasks::futures_lite::io::SeekFrom")) -> [SeekFuture](io/struct.SeekFuture.html "struct bevy::tasks::futures_lite::io::SeekFuture")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Seeks to a new position in a byte stream.

Returns the new position in the byte stream.

A seek beyond the end of stream is allowed, but behavior is defined by the implementation.

##### Examples

```rust
use futures_lite::io::{AsyncSeekExt, Cursor, SeekFrom};

let mut cursor = Cursor::new("hello");

// Move the cursor to the end.
cursor.seek(SeekFrom::End(0)).await?;

// Check the current position.
assert_eq!(cursor.seek(SeekFrom::Current(0)).await?, 5);
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2753)

### impl<S> [AsyncSeekExt](../../asset/trait.AsyncSeekExt.html "trait bevy::asset::AsyncSeekExt") for S

where S: [AsyncSeek](trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

{"SeekFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"io/struct.SeekFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::SeekFuture\\">SeekFuture</a>&lt;'\_, S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"io/struct.SeekFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::SeekFuture\\">SeekFuture</a>&lt;'\_, S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"trait.AsyncSeek.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncSeek\\">AsyncSeek</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\\">u64</a>, <a class=\\"struct\\" href=\\"io/struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>"}
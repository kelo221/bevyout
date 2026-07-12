[bevy](../../index.html)::[tasks](../index.html)::[futures\_lite](index.html)

# Trait AsyncWriteExt 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2775)

```rust
pub trait AsyncWriteExt: AsyncWrite {
    // Provided methods
    fn write<'a>(&'a mut self, buf: &'a [u8]) -> WriteFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn write_vectored<'a>(
        &'a mut self,
        bufs: &'a [IoSlice<'a>],
    ) -> WriteVectoredFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> WriteAllFuture<'a, Self> ⓘ
       where Self: Unpin { ... }
    fn flush(&mut self) -> FlushFuture<'_, Self> ⓘ
       where Self: Unpin { ... }
    fn close(&mut self) -> CloseFuture<'_, Self> ⓘ
       where Self: Unpin { ... }
    fn boxed_writer<'a>(self) -> Pin<Box<dyn AsyncWrite + Send + 'a>>
       where Self: Sized + Send + 'a { ... }
}
```

Extension trait for [`AsyncWrite`](trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite").

## Provided Methods

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2797-2799)

#### fn [write](#method.write)<'a>(&'a mut self, buf: &'a \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [WriteFuture](io/struct.WriteFuture.html "struct bevy::tasks::futures_lite::io::WriteFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Writes some bytes into the byte stream.

Returns the number of bytes written from the start of the buffer.

If the return value is `Ok(n)` then it must be guaranteed that `0 <= n <= buf.len()`. A return value of `0` typically means that the underlying object is no longer able to accept bytes and will likely not be able to in the future as well, or that the provided buffer is empty.

##### Examples

```rust
use futures_lite::io::{AsyncWriteExt, BufWriter};

let mut output = Vec::new();
let mut writer = BufWriter::new(&mut output);

let n = writer.write(b"hello").await?;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2809-2811)

#### fn [write\_vectored](#method.write_vectored)<'a>( &'a mut self, bufs: &'a \[[IoSlice](https://doc.rust-lang.org/nightly/core/io/io_slice/struct.IoSlice.html "struct core::io::io_slice::IoSlice")<'a>\], ) -> [WriteVectoredFuture](io/struct.WriteVectoredFuture.html "struct bevy::tasks::futures_lite::io::WriteVectoredFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Like [`write()`](../../asset/trait.AsyncWriteExt.html#method.write "method bevy::asset::AsyncWriteExt::write"), except that it writes a slice of buffers.

Data is copied from each buffer in order, with the final buffer possibly being only partially consumed. This method must behave same as a call to [`write()`](../../asset/trait.AsyncWriteExt.html#method.write "method bevy::asset::AsyncWriteExt::write") with the buffers concatenated would.

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2834-2836)

#### fn [write\_all](#method.write_all)<'a>(&'a mut self, buf: &'a \[[u8](https://doc.rust-lang.org/nightly/std/primitive.u8.html)\]) -> [WriteAllFuture](io/struct.WriteAllFuture.html "struct bevy::tasks::futures_lite::io::WriteAllFuture")<'a, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Writes an entire buffer into the byte stream.

This method will keep calling [`write()`](../../asset/trait.AsyncWriteExt.html#method.write "method bevy::asset::AsyncWriteExt::write") until there is no more data to be written or an error occurs. It will not return before the entire buffer is successfully written or an error occurs.

##### Examples

```rust
use futures_lite::io::{AsyncWriteExt, BufWriter};

let mut output = Vec::new();
let mut writer = BufWriter::new(&mut output);

let n = writer.write_all(b"hello").await?;
```

##### [Examples found in repository](#scraped-examples)[?](../../../scrape-examples-help.html)

examples/asset/processing/asset\_processing.rs ([line 223](../../../src/asset_processing/asset_processing.rs.html#223))

```rust
216    async fn save(
217        &self,
218        writer: &mut Writer,
219        asset: SavedAsset<'_, '_, Self::Asset>,
220        _settings: &Self::Settings,
221        _asset_path: AssetPath<'_>,
222    ) -> Result<TextSettings, Self::Error> {
223        writer.write_all(asset.text.as_bytes()).await?;
224        Ok(TextSettings::default())
225    }
```

Hide additional examples

examples/asset/asset\_saving\_with\_subassets.rs ([line 191](../../../src/asset_saving_with_subassets/asset_saving_with_subassets.rs.html#191))

```rust
169    async fn save(
170        &self,
171        writer: &mut Writer,
172        asset: SavedAsset<'_, '_, Self::Asset>,
173        _settings: &Self::Settings,
174        _asset_path: AssetPath<'_>,
175    ) -> Result<(), Self::Error> {
176        let boxes = asset
177            .boxes
178            .iter()
179            .map(|handle| {
180                asset
181                    .get_labeled_by_id::<OneBox>(handle)
182                    .unwrap()
183                    .get()
184                    .clone()
185            })
186            .collect();
187
188        // Note: serializing to string isn't ideal since we can't do a streaming write, but this is
189        // fine for an example.
190        let serialized = ron::to_string(&SerializableManyBoxes { boxes })?;
191        writer.write_all(serialized.as_bytes()).await?;
192
193        Ok(())
194    }
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2856-2858)

#### fn [flush](#method.flush)(&mut self) -> [FlushFuture](io/struct.FlushFuture.html "struct bevy::tasks::futures_lite::io::FlushFuture")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Flushes the stream to ensure that all buffered contents reach their destination.

##### Examples

```rust
use futures_lite::io::{AsyncWriteExt, BufWriter};

let mut output = Vec::new();
let mut writer = BufWriter::new(&mut output);

writer.write_all(b"hello").await?;
writer.flush().await?;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2877-2879)

#### fn [close](#method.close)(&mut self) -> [CloseFuture](io/struct.CloseFuture.html "struct bevy::tasks::futures_lite::io::CloseFuture")<'\_, Self> [ⓘ](#)

where Self: [Unpin](https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html "trait core::marker::Unpin"),

Closes the writer.

##### Examples

```rust
use futures_lite::io::{AsyncWriteExt, BufWriter};

let mut output = Vec::new();
let mut writer = BufWriter::new(&mut output);

writer.close().await?;
```

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2894-2896)

#### fn [boxed\_writer](#method.boxed_writer)<'a>(self) -> [Pin](https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html "struct core::pin::Pin")<[Box](../../prelude/struct.Box.html "struct bevy::prelude::Box")<dyn [AsyncWrite](trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a>>

where Self: [Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized") + [Send](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html "trait core::marker::Send") + 'a,

Available on **crate feature `alloc`** only.

Boxes the writer and changes its type to `dyn AsyncWrite + Send + 'a`.

##### Examples

```rust
use futures_lite::io::AsyncWriteExt;

let writer = Vec::<u8>::new().boxed_writer();
```

## Dyn Compatibility

This trait is **not** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/io.rs.html#2902)

### impl<W> [AsyncWriteExt](../../asset/trait.AsyncWriteExt.html "trait bevy::asset::AsyncWriteExt") for W

where W: [AsyncWrite](trait.AsyncWrite.html "trait bevy::tasks::futures_lite::AsyncWrite") + ?[Sized](https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html "trait core::marker::Sized"),

{"CloseFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"io/struct.CloseFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::CloseFuture\\">CloseFuture</a>&lt;'\_, W&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"io/struct.CloseFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::CloseFuture\\">CloseFuture</a>&lt;'\_, W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"trait.AsyncWrite.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncWrite\\">AsyncWrite</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, <a class=\\"struct\\" href=\\"io/struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","FlushFuture<'\_, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"io/struct.FlushFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::FlushFuture\\">FlushFuture</a>&lt;'\_, W&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"io/struct.FlushFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::FlushFuture\\">FlushFuture</a>&lt;'\_, W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"trait.AsyncWrite.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncWrite\\">AsyncWrite</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, <a class=\\"struct\\" href=\\"io/struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","WriteAllFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"io/struct.WriteAllFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::WriteAllFuture\\">WriteAllFuture</a>&lt;'\_, W&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"io/struct.WriteAllFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::WriteAllFuture\\">WriteAllFuture</a>&lt;'\_, W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"trait.AsyncWrite.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncWrite\\">AsyncWrite</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>, <a class=\\"struct\\" href=\\"io/struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","WriteFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"io/struct.WriteFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::WriteFuture\\">WriteFuture</a>&lt;'\_, W&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"io/struct.WriteFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::WriteFuture\\">WriteFuture</a>&lt;'\_, W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"trait.AsyncWrite.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncWrite\\">AsyncWrite</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"io/struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>","WriteVectoredFuture<'a, Self>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"io/struct.WriteVectoredFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::WriteVectoredFuture\\">WriteVectoredFuture</a>&lt;'\_, W&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;W&gt; <a class=\\"trait\\" href=\\"trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"io/struct.WriteVectoredFuture.html\\" title=\\"struct bevy::tasks::futures\_lite::io::WriteVectoredFuture\\">WriteVectoredFuture</a>&lt;'\_, W&gt;<div class=\\"where\\">where\\n W: <a class=\\"trait\\" href=\\"trait.AsyncWrite.html\\" title=\\"trait bevy::tasks::futures\_lite::AsyncWrite\\">AsyncWrite</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a> + ?<a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html\\" title=\\"trait core::marker::Sized\\">Sized</a>,</div></div><div class=\\"where\\"> type <a href=\\"trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\\">usize</a>, <a class=\\"struct\\" href=\\"io/struct.Error.html\\" title=\\"struct bevy::tasks::futures\_lite::io::Error\\">Error</a>&gt;;</div>"}
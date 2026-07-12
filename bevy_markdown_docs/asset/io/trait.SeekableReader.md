[bevy](../../index.html)::[asset](../index.html)::[io](index.html)

# Trait SeekableReader 

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#144)

```rust
pub trait SeekableReader: Reader + AsyncSeek { }
```

A [`Reader`](trait.Reader.html "trait bevy::asset::io::Reader") that also has [`AsyncSeek`](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek") functionality. See [`Reader::seekable`](trait.Reader.html#tymethod.seekable "method bevy::asset::io::Reader::seekable") for details.

## Dyn Compatibility

This trait **is** [dyn compatible](https://doc.rust-lang.org/nightly/reference/items/traits.html#dyn-compatibility).

_In older versions of Rust, dyn compatibility was called "object safety"._

## Implementors

[Source](https://docs.rs/bevy_asset/0.19.0/x86_64-unknown-linux-gnu/src/bevy_asset/io/mod.rs.html#146)

### impl<T> [SeekableReader](trait.SeekableReader.html "trait bevy::asset::io::SeekableReader") for T

where T: [Reader](trait.Reader.html "trait bevy::asset::io::Reader") + [AsyncSeek](../../tasks/futures_lite/trait.AsyncSeek.html "trait bevy::tasks::futures_lite::AsyncSeek"),
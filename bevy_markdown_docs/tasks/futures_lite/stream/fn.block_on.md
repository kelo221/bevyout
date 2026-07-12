[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[stream](index.html)

# Function block\_on 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/stream.rs.html#53)

```rust
pub fn block_on<S>(stream: S) -> BlockOn<S> ⓘwhere
    S: Stream + Unpin,
```

Available on **crate feature `std`** only.

Converts a stream into a blocking iterator.

## Examples

```rust
use futures_lite::{pin, stream};

let stream = stream::once(7);
pin!(stream);

let mut iter = stream::block_on(stream);
assert_eq!(iter.next(), Some(7));
assert_eq!(iter.next(), None);
```

{"BlockOn<S>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.BlockOn.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::BlockOn\\">BlockOn</a>&lt;S&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;S&gt; <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html\\" title=\\"trait core::iter::traits::iterator::Iterator\\">Iterator</a> for <a class=\\"struct\\" href=\\"struct.BlockOn.html\\" title=\\"struct bevy::tasks::futures\_lite::stream::BlockOn\\">BlockOn</a>&lt;S&gt;<div class=\\"where\\">where\\n S: <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a> + <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\\" title=\\"trait core::marker::Unpin\\">Unpin</a>,</div></div><div class=\\"where\\"> type <a href=\\"https://doc.rust-lang.org/nightly/core/iter/traits/iterator/trait.Iterator.html#associatedtype.Item\\" class=\\"associatedtype\\">Item</a> = &lt;S as <a class=\\"trait\\" href=\\"../trait.Stream.html\\" title=\\"trait bevy::tasks::futures\_lite::Stream\\">Stream</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Stream.html#associatedtype.Item\\" title=\\"type bevy::tasks::futures\_lite::Stream::Item\\">Item</a>;</div>"}
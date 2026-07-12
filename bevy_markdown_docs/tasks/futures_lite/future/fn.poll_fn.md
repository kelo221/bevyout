[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function poll\_fn 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#168-170)

```rust
pub fn poll_fn<T, F>(f: F) -> PollFn<F> ⓘwhere
    F: FnMut(&mut Context<'_>) -> Poll<T>,
```

Creates a future from a function returning [`Poll`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html "enum core::task::poll::Poll").

## Examples

```rust
use futures_lite::future;
use std::task::{Context, Poll};

fn f(_: &mut Context<'_>) -> Poll<i32> {
    Poll::Ready(7)
}

assert_eq!(future::poll_fn(f).await, 7);
```

{"PollFn<F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.PollFn.html\\" title=\\"struct bevy::tasks::futures\_lite::future::PollFn\\">PollFn</a>&lt;F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.PollFn.html\\" title=\\"struct bevy::tasks::futures\_lite::future::PollFn\\">PollFn</a>&lt;F&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"https://doc.rust-lang.org/nightly/core/ops/function/trait.FnMut.html\\" title=\\"trait core::ops::function::FnMut\\">FnMut</a>(&amp;mut <a class=\\"struct\\" href=\\"https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html\\" title=\\"struct core::task::wake::Context\\">Context</a>&lt;'\_&gt;) -&gt; <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html\\" title=\\"enum core::task::poll::Poll\\">Poll</a>&lt;T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}
[bevy](../index.html)::[tasks](index.html)

# Function poll\_once 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#116-118)

```rust
pub fn poll_once<T, F>(f: F) -> PollOnce<F> ⓘwhere
    F: Future<Output = T>,
```

Polls a future just once and returns an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html "enum core::option::Option") with the result.

## Examples

```rust
use futures_lite::future;

assert_eq!(future::poll_once(future::pending::<()>()).await, None);
assert_eq!(future::poll_once(future::ready(42)).await, Some(42));
```

{"PollOnce<F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"futures\_lite/future/struct.PollOnce.html\\" title=\\"struct bevy::tasks::futures\_lite::future::PollOnce\\">PollOnce</a>&lt;F&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F&gt; <a class=\\"trait\\" href=\\"futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"futures\_lite/future/struct.PollOnce.html\\" title=\\"struct bevy::tasks::futures\_lite::future::PollOnce\\">PollOnce</a>&lt;F&gt;<div class=\\"where\\">where\\n F: <a class=\\"trait\\" href=\\"futures\_lite/trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"futures\_lite/trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\\" title=\\"enum core::option::Option\\">Option</a>&lt;T&gt;;</div>"}
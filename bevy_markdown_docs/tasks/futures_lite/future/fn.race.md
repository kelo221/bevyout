[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function race 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#543-546)

```rust
pub fn race<T, F1, F2>(future1: F1, future2: F2) -> Race<F1, F2> ⓘwhere
    F1: Future<Output = T>,
    F2: Future<Output = T>,
```

Available on **crate features `race` and `std`** only.

Returns the result of the future that completes first, with no preference if both are ready.

Each time [`Race`](struct.Race.html "struct bevy::tasks::futures_lite::future::Race") is polled, the two inner futures are polled in random order. Therefore, no future takes precedence over the other if both can complete at the same time.

If you have preference for one of the futures, use the [`or()`](fn.or.html "fn bevy::tasks::futures_lite::future::or") function or the [`FutureExt::or()`](../trait.FutureExt.html#method.or "method bevy::tasks::futures_lite::FutureExt::or") method.

## Examples

```rust
use futures_lite::future::{self, pending, ready};

assert_eq!(future::race(ready(1), pending()).await, 1);
assert_eq!(future::race(pending(), ready(2)).await, 2);

// One of the two futures is randomly chosen as the winner.
let res = future::race(ready(1), ready(2)).await;
```

{"Race<F1, F2>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}
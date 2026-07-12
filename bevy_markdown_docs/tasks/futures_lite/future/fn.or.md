[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function or 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#421-424)

```rust
pub fn or<T, F1, F2>(future1: F1, future2: F2) -> Or<F1, F2> ⓘwhere
    F1: Future<Output = T>,
    F2: Future<Output = T>,
```

Returns the result of the future that completes first, preferring `future1` if both are ready.

If you need to treat the two futures fairly without a preference for either, use the [`race()`](fn.race.html "fn bevy::tasks::futures_lite::future::race") function or the [`FutureExt::race()`](../trait.FutureExt.html#method.race "method bevy::tasks::futures_lite::FutureExt::race") method.

## Examples

```rust
use futures_lite::future::{self, pending, ready};

assert_eq!(future::or(ready(1), pending()).await, 1);
assert_eq!(future::or(pending(), ready(2)).await, 2);

// The first future wins.
assert_eq!(future::or(ready(1), ready(2)).await, 1);
```

{"Or<F1, F2>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Or.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Or\\">Or</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Or.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Or\\">Or</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}
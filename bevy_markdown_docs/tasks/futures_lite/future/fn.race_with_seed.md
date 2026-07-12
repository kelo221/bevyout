[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function race\_with\_seed 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#580-583)

```rust
pub fn race_with_seed<T, F1, F2>(
    future1: F1,
    future2: F2,
    seed: u64,
) -> Race<F1, F2> ⓘwhere
    F1: Future<Output = T>,
    F2: Future<Output = T>,
```

Available on **crate feature `race`** only.

Race two futures but with a predefined random seed.

This function is identical to [`race`](fn.race.html "fn bevy::tasks::futures_lite::future::race"), but instead of using a random seed from a thread-local RNG, it allows the user to provide a seed. It is useful for when you already have a source of randomness available, or if you want to use a fixed seed.

See documentation of the [`race`](fn.race.html "fn bevy::tasks::futures_lite::future::race") function for features and caveats.

## Examples

```rust
use futures_lite::future::{self, pending, ready};

// A fixed seed is used, so the result is deterministic.
const SEED: u64 = 0x42;

assert_eq!(future::race_with_seed(ready(1), pending(), SEED).await, 1);
assert_eq!(future::race_with_seed(pending(), ready(2), SEED).await, 2);

// One of the two futures is randomly chosen as the winner.
let res = future::race_with_seed(ready(1), ready(2), SEED).await;
```

{"Race<F1, F2>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Race.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Race\\">Race</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = T&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}
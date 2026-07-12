[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function try\_zip 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#338-341)

```rust
pub fn try_zip<T1, T2, E, F1, F2>(
    future1: F1,
    future2: F2,
) -> TryZip<F1, T1, F2, T2> ⓘwhere
    F1: Future<Output = Result<T1, E>>,
    F2: Future<Output = Result<T2, E>>,
```

Joins two fallible futures, waiting for both to complete or one of them to error.

## Examples

```rust
use futures_lite::future;

let a = async { Ok::<i32, i32>(1) };
let b = async { Err::<i32, i32>(2) };

assert_eq!(future::try_zip(a, b).await, Err(2));
```

{"TryZip<F1, T1, F2, T2>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.TryZip.html\\" title=\\"struct bevy::tasks::futures\_lite::future::TryZip\\">TryZip</a>&lt;F1, T1, F2, T2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T1, T2, E, F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.TryZip.html\\" title=\\"struct bevy::tasks::futures\_lite::future::TryZip\\">TryZip</a>&lt;F1, T1, F2, T2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T1, E&gt;&gt;,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&lt;Output = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;T2, E&gt;&gt;,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"enum\\" href=\\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\\" title=\\"enum core::result::Result\\">Result</a>&lt;<a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\\">(T1, T2)</a>, E&gt;;</div>"}
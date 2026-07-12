[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function zip 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#253-256)

```rust
pub fn zip<F1, F2>(future1: F1, future2: F2) -> Zip<F1, F2> ⓘwhere
    F1: Future,
    F2: Future,
```

Joins two futures, waiting for both to complete.

## Examples

```rust
use futures_lite::future;

let a = async { 1 };
let b = async { 2 };

assert_eq!(future::zip(a, b).await, (1, 2));
```

{"Zip<F1, F2>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Zip.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Zip\\">Zip</a>&lt;F1, F2&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;F1, F2&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Zip.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Zip\\">Zip</a>&lt;F1, F2&gt;<div class=\\"where\\">where\\n F1: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,\\n F2: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = (&lt;F1 as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>, &lt;F2 as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>);</div>"}
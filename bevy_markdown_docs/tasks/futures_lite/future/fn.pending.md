[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function pending 

1.48.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/pending.rs.html#33)

```rust
pub fn pending<T>() -> Pending<T> ⓘ
```

Creates a future which never resolves, representing a computation that never finishes.

## Examples

```rust
use std::future;

let future = future::pending();
let () = future.await;
unreachable!();
```

{"Pending<T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Pending.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Pending\\">Pending</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Pending.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Pending\\">Pending</a>&lt;T&gt;</div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}
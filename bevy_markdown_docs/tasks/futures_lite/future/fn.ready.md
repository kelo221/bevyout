[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function ready 

1.48.0 · [Source](https://doc.rust-lang.org/nightly/src/core/future/ready.rs.html#67)

```rust
pub fn ready<T>(t: T) -> Ready<T> ⓘ
```

Creates a future that is immediately ready with a value.

Futures created through this function are functionally similar to those created through `async {}`. The main difference is that futures created through this function are named and implement `Unpin`.

## Examples

```rust
use std::future;

let a = future::ready(1);
assert_eq!(a.await, 1);
```

{"Ready<T>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Ready.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Ready\\">Ready</a>&lt;T&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;T&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Ready.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Ready\\">Ready</a>&lt;T&gt;</div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = T;</div>"}
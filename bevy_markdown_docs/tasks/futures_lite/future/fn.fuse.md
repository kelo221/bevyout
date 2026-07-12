[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function fuse 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#477-479)

```rust
pub fn fuse<F>(future: F) -> Fuse<F> ⓘwhere
    F: Future,
```

Fuse a future such that `poll` will never again be called once it has completed. This method can be used to turn any `Future` into a `FusedFuture`.

Normally, once a future has returned `Poll::Ready` from `poll`, any further calls could exhibit bad behavior such as blocking forever, panicking, never returning, etc. If it is known that `poll` may be called too often then this method can be used to ensure that it has defined semantics.

If a `fuse`d future is `poll`ed after having returned `Poll::Ready` previously, it will return `Poll::Pending`, from `poll` again (and will continue to do so for all future calls to `poll`).

This combinator will drop the underlying future as soon as it has been completed to ensure resources are reclaimed as soon as possible.

{"Fuse<F>":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.Fuse.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Fuse\\">Fuse</a>&lt;Fut&gt;</code></h3><pre><code><div class=\\"where\\">impl&lt;Fut&gt; <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.Fuse.html\\" title=\\"struct bevy::tasks::futures\_lite::future::Fuse\\">Fuse</a>&lt;Fut&gt;<div class=\\"where\\">where\\n Fut: <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>,</div></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = &lt;Fut as <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a>&gt;::<a class=\\"associatedtype\\" href=\\"../trait.Future.html#associatedtype.Output\\" title=\\"type bevy::tasks::futures\_lite::Future::Output\\">Output</a>;</div>"}
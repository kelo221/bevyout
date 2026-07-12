[bevy](../../index.html)::[tasks](../index.html)::[futures\_lite](index.html)

# Macro ready 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/lib.rs.html#107)

```rust
macro_rules! ready {
    ($e:expr $(,)?) => { ... };
}
```

Unwraps `Poll<T>` or returns [`Pending`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Pending "variant core::task::poll::Poll::Pending").

## Examples

```rust
use futures_lite::{future, prelude::*, ready};
use std::pin::Pin;
use std::task::{Context, Poll};

fn do_poll(cx: &mut Context<'_>) -> Poll<()> {
    let mut fut = future::ready(42);
    let fut = Pin::new(&mut fut);

    let num = ready!(fut.poll(cx));
    // ... use num

    Poll::Ready(())
}
```

The `ready!` call expands to:

```rust
let num = match fut.poll(cx) {
    Poll::Ready(t) => t,
    Poll::Pending => return Poll::Pending,
};
```
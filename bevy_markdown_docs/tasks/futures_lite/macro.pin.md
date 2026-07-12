[bevy](../../index.html)::[tasks](../index.html)::[futures\_lite](index.html)

# Macro pin 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/lib.rs.html#137)

```rust
macro_rules! pin {
    ($($x:ident),* $(,)?) => { ... };
}
```

Pins a variable of type `T` on the stack and rebinds it as `Pin<&mut T>`.

```rust
use futures_lite::{future, pin};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::time::Instant;

// Inspects each invocation of `Future::poll()`.
async fn inspect<T: Debug>(f: impl Future<Output = T>) -> T {
    pin!(f);
    future::poll_fn(|cx| dbg!(f.as_mut().poll(cx))).await
}

let f = async { 1 + 2 };
inspect(f).await;
```
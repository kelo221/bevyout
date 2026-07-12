[bevy](../../../index.html)::[tasks](../../index.html)::[futures\_lite](../index.html)::[future](index.html)

# Function yield\_now 

[Source](https://docs.rs/futures-lite/2.6.1/x86_64-unknown-linux-gnu/src/futures_lite/future.rs.html#216)

```rust
pub fn yield_now() -> YieldNow ⓘ
```

Wakes the current task and returns [`Poll::Pending`](https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html#variant.Pending "variant core::task::poll::Poll::Pending") once.

This function is useful when we want to cooperatively give time to the task scheduler. It is generally a good idea to yield inside loops because that way we make sure long-running tasks don’t prevent other tasks from running.

## Examples

```rust
use futures_lite::future;

future::yield_now().await;
```

{"YieldNow":"<h3>Notable traits for <code><a class=\\"struct\\" href=\\"struct.YieldNow.html\\" title=\\"struct bevy::tasks::futures\_lite::future::YieldNow\\">YieldNow</a></code></h3><pre><code><div class=\\"where\\">impl <a class=\\"trait\\" href=\\"../trait.Future.html\\" title=\\"trait bevy::tasks::futures\_lite::Future\\">Future</a> for <a class=\\"struct\\" href=\\"struct.YieldNow.html\\" title=\\"struct bevy::tasks::futures\_lite::future::YieldNow\\">YieldNow</a></div><div class=\\"where\\"> type <a href=\\"../trait.Future.html#associatedtype.Output\\" class=\\"associatedtype\\">Output</a> = <a class=\\"primitive\\" href=\\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\\">()</a>;</div>"}
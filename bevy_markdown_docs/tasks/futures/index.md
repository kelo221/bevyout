[bevy](../../index.html)::[tasks](../index.html)

# Module futures 

[Source](https://docs.rs/bevy_tasks/0.19.0/x86_64-unknown-linux-gnu/src/bevy_tasks/lib.rs.html#67)

Utilities for working with [`Future`](../futures_lite/trait.Future.html "trait bevy::tasks::futures_lite::Future")s.

## Functions

[check\_ready](fn.check_ready.html "fn bevy::tasks::futures::check_ready")

Polls a future once, and returns the output if ready or returns `None` if it wasn’t ready yet.

[now\_or\_never](fn.now_or_never.html "fn bevy::tasks::futures::now_or_never")

Consumes a future, polls it once, and immediately returns the output or returns `None` if it wasn’t ready yet.
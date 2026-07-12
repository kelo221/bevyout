[bevy](../../index.html)::[ecs](../index.html)::[error](index.html)

# Function bevy\_error\_panic\_hook 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/error/bevy_error.rs.html#445-447)

```rust
pub fn bevy_error_panic_hook(
    current_hook: impl Fn(&PanicHookInfo<'_>),
) -> impl Fn(&PanicHookInfo<'_>)
```

Available on **crate feature `backtrace`** only.

When called, this will skip the currently configured panic hook when a [`BevyError`](../../prelude/struct.BevyError.html "struct bevy::prelude::BevyError") backtrace has already been printed.
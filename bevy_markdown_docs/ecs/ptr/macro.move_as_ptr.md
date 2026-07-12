[bevy](../../index.html)::[ecs](../index.html)::[ptr](index.html)

# Macro move\_as\_ptr 

[Source](https://docs.rs/bevy_ptr/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ptr/lib.rs.html#1274)

```rust
macro_rules! move_as_ptr {
    ($value: ident) => { ... };
}
```

Safely converts a owned value into a [`MovingPtr`](struct.MovingPtr.html "struct bevy::ecs::ptr::MovingPtr") while minimizing the number of stack copies.

This cannot be used as expression and must be used as a statement. Internally this macro works via variable shadowing.
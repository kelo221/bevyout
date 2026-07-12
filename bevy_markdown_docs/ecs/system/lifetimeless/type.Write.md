[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[lifetimeless](index.html)

# Type Alias Write 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2113)

```rust
pub type Write<T> = &'static mut T;
```

A shorthand for writing `&'static mut T`.
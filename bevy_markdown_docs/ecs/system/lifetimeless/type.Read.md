[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[lifetimeless](index.html)

# Type Alias Read 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2111)

```rust
pub type Read<T> = &'static T;
```

A shorthand for writing `&'static T`.
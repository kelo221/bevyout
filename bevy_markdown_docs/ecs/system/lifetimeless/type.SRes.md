[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[lifetimeless](index.html)

# Type Alias SRes 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2115)

```rust
pub type SRes<T> = Res<'static, T>;
```

A [`Res`](../../../prelude/struct.Res.html "struct bevy::prelude::Res") with `'static` lifetimes.

## Aliased Type

```rust
pub struct SRes<T> { /* private fields */ }
```
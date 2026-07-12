[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[lifetimeless](index.html)

# Type Alias SResMut 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2117)

```rust
pub type SResMut<T> = ResMut<'static, T>;
```

A [`ResMut`](../../../prelude/struct.ResMut.html "struct bevy::prelude::ResMut") with `'static` lifetimes.

## Aliased Type

```rust
pub struct SResMut<T> { /* private fields */ }
```
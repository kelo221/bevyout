[bevy](../../../index.html)::[ecs](../../index.html)::[system](../index.html)::[lifetimeless](index.html)

# Type Alias SQuery 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/system/system_param.rs.html#2109)

```rust
pub type SQuery<D, F = ()> = Query<'static, 'static, D, F>;
```

A [`Query`](../../../prelude/struct.Query.html "struct bevy::prelude::Query") with `'static` lifetimes.

## Aliased Type

```rust
pub struct SQuery<D, F = ()> { /* private fields */ }
```
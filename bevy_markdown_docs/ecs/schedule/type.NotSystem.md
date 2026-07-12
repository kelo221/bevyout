[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias NotSystem 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/condition.rs.html#1364)

```rust
pub type NotSystem<S> = AdapterSystem<NotMarker, S>;
```

Invokes [`Not`](https://doc.rust-lang.org/nightly/core/ops/bit/trait.Not.html "trait core::ops::bit::Not") with the output of another system.

See [`common_conditions::not`](../../prelude/fn.not.html "fn bevy::prelude::not") for examples.

## Aliased Type

```rust
pub struct NotSystem<S> { /* private fields */ }
```
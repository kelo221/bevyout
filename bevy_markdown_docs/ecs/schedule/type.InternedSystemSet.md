[bevy](../../index.html)::[ecs](../index.html)::[schedule](index.html)

# Type Alias InternedSystemSet 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/schedule/set.rs.html#179)

```rust
pub type InternedSystemSet = Interned<dyn SystemSet>;
```

A shorthand for `Interned<dyn SystemSet>`.

## Aliased Type

```rust
pub struct InternedSystemSet(pub &'static dyn SystemSet);
```

## Tuple Fields

`0: &'static dyn [SystemSet](../../prelude/trait.SystemSet.html "trait bevy::prelude::SystemSet")`
[bevy](../../../index.html)::[platform](../../index.html)::[collections](../index.html)::[hash\_map](index.html)

# Type Alias Entry 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_map.rs.html#36)

```rust
pub type Entry<'a, K, V, S = FixedHasher> = Entry<'a, K, V, S>;
```

Shortcut for [`Entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/map/enum.Entry.html "enum hashbrown::map::Entry") with [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider.

## Aliased Type

```rust
pub enum Entry<'a, K, V, S = FixedHasher> {
    Occupied(OccupiedEntry<'a, K, V, S>),
    Vacant(VacantEntry<'a, K, V, S>),
}
```

## Variants

### Occupied([OccupiedEntry](struct.OccupiedEntry.html "struct bevy::platform::collections::hash_map::OccupiedEntry")<'a, K, V, S>)

An occupied entry.

#### Examples

```rust
use hashbrown::hash_map::{Entry, HashMap};
let mut map: HashMap<_, _> = [("a", 100), ("b", 200)].into();

match map.entry("a") {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(_) => { }
}
```

### Vacant([VacantEntry](struct.VacantEntry.html "struct bevy::platform::collections::hash_map::VacantEntry")<'a, K, V, S>)

A vacant entry.

#### Examples

```rust
use hashbrown::hash_map::{Entry, HashMap};
let mut map: HashMap<&str, i32> = HashMap::new();

match map.entry("a") {
    Entry::Occupied(_) => unreachable!(),
    Entry::Vacant(_) => { }
}
```
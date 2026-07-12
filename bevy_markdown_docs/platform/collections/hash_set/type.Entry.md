[bevy](../../../index.html)::[platform](../../index.html)::[collections](../index.html)::[hash\_set](index.html)

# Type Alias Entry 

[Source](https://docs.rs/bevy_platform/0.19.0/x86_64-unknown-linux-gnu/src/bevy_platform/collections/hash_set.rs.html#30)

```rust
pub type Entry<'a, T, S = FixedHasher> = Entry<'a, T, S>;
```

Shortcut for [`Entry`](https://docs.rs/hashbrown/0.15.5/x86_64-unknown-linux-gnu/hashbrown/set/enum.Entry.html "enum hashbrown::set::Entry") with [`FixedHasher`](../../hash/struct.FixedHasher.html "struct bevy::platform::hash::FixedHasher") as the default hashing provider.

## Aliased Type

```rust
pub enum Entry<'a, T, S = FixedHasher> {
    Occupied(OccupiedEntry<'a, T, S>),
    Vacant(VacantEntry<'a, T, S>),
}
```

## Variants

### Occupied([OccupiedEntry](struct.OccupiedEntry.html "struct bevy::platform::collections::hash_set::OccupiedEntry")<'a, T, S>)

An occupied entry.

#### Examples

```rust
use hashbrown::hash_set::{Entry, HashSet};
let mut set: HashSet<_> = ["a", "b"].into();

match set.entry("a") {
    Entry::Vacant(_) => unreachable!(),
    Entry::Occupied(_) => { }
}
```

### Vacant([VacantEntry](struct.VacantEntry.html "struct bevy::platform::collections::hash_set::VacantEntry")<'a, T, S>)

A vacant entry.

#### Examples

```rust
use hashbrown::hash_set::{Entry, HashSet};
let mut set: HashSet<&str> = HashSet::new();

match set.entry("a") {
    Entry::Occupied(_) => unreachable!(),
    Entry::Vacant(_) => { }
}
```
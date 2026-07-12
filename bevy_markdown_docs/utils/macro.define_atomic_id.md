[bevy](../index.html)::[utils](index.html)

# Macro define\_atomic\_id 

[Source](https://docs.rs/bevy_utils/0.19.0/x86_64-unknown-linux-gnu/src/bevy_utils/atomic_id.rs.html#6)

```rust
macro_rules! define_atomic_id {
    ($atomic_id_type:ident) => { ... };
}
```

Defines a 32-bit id type which guarantees global uniqueness via atomics on a static global.

Note that this means the id space is process-wide, as such it may potentially be exhausted by a combination of long-running processes and multiple bevy `World`s, at which point we panic.
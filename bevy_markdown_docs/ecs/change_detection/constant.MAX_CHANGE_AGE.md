[bevy](../../index.html)::[ecs](../index.html)::[change\_detection](index.html)

# Constant MAX\_CHANGE\_AGE 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/mod.rs.html#26)

```rust
pub const MAX_CHANGE_AGE: u32 = _; // 3_258_167_296u32
```

The maximum change tick difference that won’t overflow before the next `check_tick` scan.

Changes stop being detected once they become this old.
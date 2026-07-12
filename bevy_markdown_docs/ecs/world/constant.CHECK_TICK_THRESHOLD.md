[bevy](../../index.html)::[ecs](../index.html)::[world](index.html)

# Constant CHECK\_TICK\_THRESHOLD 

[Source](https://docs.rs/bevy_ecs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_ecs/change_detection/mod.rs.html#21)

```rust
pub const CHECK_TICK_THRESHOLD: u32 = 518_400_000; // 518_400_000u32
```

The (arbitrarily chosen) minimum number of world tick increments between `check_tick` scans.

Change ticks can only be scanned when systems aren’t running. Thus, if the threshold is `N`, the maximum is `2 * N - 1` (i.e. the world ticks `N - 1` times, then `N` times).

If no change is older than `u32::MAX - (2 * N - 1)` following a scan, none of their ages can overflow and cause false positives.
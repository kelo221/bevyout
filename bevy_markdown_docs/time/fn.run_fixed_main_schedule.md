[bevy](../index.html)::[time](index.html)

# Function run\_fixed\_main\_schedule 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/fixed.rs.html#243)

```rust
pub fn run_fixed_main_schedule(world: &mut World)
```

Runs [`FixedMain`](../app/struct.FixedMain.html "struct bevy::app::FixedMain") zero or more times based on delta of [`Time<Virtual>`](../prelude/struct.Virtual.html "struct bevy::prelude::Virtual") and [`Time::overstep`](../prelude/struct.Time.html#method.overstep "method bevy::prelude::Time::overstep"). You can order your systems relative to this by using [`RunFixedMainLoopSystems`](../prelude/enum.RunFixedMainLoopSystems.html "enum bevy::prelude::RunFixedMainLoopSystems").
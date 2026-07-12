[bevy](../../index.html)::[time](../index.html)

# Module common\_conditions 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#16)

Common run conditions

## Functions

[on\_real\_timer](fn.on_real_timer.html "fn bevy::time::common_conditions::on_real_timer")

Run condition that is active on a regular time interval, using [`Time<Real>`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks are not scaled.

[on\_timer](fn.on_timer.html "fn bevy::time::common_conditions::on_timer")

Run condition that is active on a regular time interval, using [`Time`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks at the rate of [`Time::relative_speed`](../../prelude/struct.Time.html#method.relative_speed "method bevy::prelude::Time::relative_speed").

[once\_after\_delay](fn.once_after_delay.html "fn bevy::time::common_conditions::once_after_delay")

Run condition that is active _once_ after the specified delay, using [`Time`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks at the rate of [`Time::relative_speed`](../../prelude/struct.Time.html#method.relative_speed "method bevy::prelude::Time::relative_speed").

[once\_after\_real\_delay](fn.once_after_real_delay.html "fn bevy::time::common_conditions::once_after_real_delay")

Run condition that is active _once_ after the specified delay, using [`Time<Real>`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks are not scaled.

[paused](fn.paused.html "fn bevy::time::common_conditions::paused")

Run condition that is active when the [`Time<Virtual>`](../../prelude/struct.Time.html "struct bevy::prelude::Time") clock is paused. Use [`bevy_ecs::schedule::common_conditions::not`](../../prelude/fn.not.html "fn bevy::prelude::not") to make it active when it’s not paused.

[repeating\_after\_delay](fn.repeating_after_delay.html "fn bevy::time::common_conditions::repeating_after_delay")

Run condition that is active _indefinitely_ after the specified delay, using [`Time`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks at the rate of [`Time::relative_speed`](../../prelude/struct.Time.html#method.relative_speed "method bevy::prelude::Time::relative_speed").

[repeating\_after\_real\_delay](fn.repeating_after_real_delay.html "fn bevy::time::common_conditions::repeating_after_real_delay")

Run condition that is active _indefinitely_ after the specified delay, using [`Time<Real>`](../../prelude/struct.Time.html "struct bevy::prelude::Time") to advance the timer. The timer ticks are not scaled.
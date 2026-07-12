[bevy](../../index.html)::[time](../index.html)

# Module prelude 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#36)

The time prelude.

This includes the most common types in this crate, re-exported for your convenience.

## Structs

[Fixed](struct.Fixed.html "struct bevy::time::prelude::Fixed")

The fixed timestep game clock following virtual time.

[Real](struct.Real.html "struct bevy::time::prelude::Real")

Real time clock representing elapsed wall clock time.

[Time](struct.Time.html "struct bevy::time::prelude::Time")

A generic clock resource that tracks how much it has advanced since its previous update and since its creation.

[Timer](struct.Timer.html "struct bevy::time::prelude::Timer")

Tracks elapsed time. Enters the finished state once `duration` is reached.

[Virtual](struct.Virtual.html "struct bevy::time::prelude::Virtual")

The virtual game clock representing game time.

## Enums

[TimerMode](enum.TimerMode.html "enum bevy::time::prelude::TimerMode")

Specifies [`Timer`](../../prelude/struct.Timer.html "struct bevy::prelude::Timer") behavior.

## Traits

[DelayedCommandsExt](trait.DelayedCommandsExt.html "trait bevy::time::prelude::DelayedCommandsExt")

Extension trait for [`Commands`](../../prelude/struct.Commands.html "struct bevy::prelude::Commands") that provides delayed command functionality.
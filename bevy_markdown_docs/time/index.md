[bevy](../index.html)

# Crate time 

[Source](https://docs.rs/bevy_time/0.19.0/x86_64-unknown-linux-gnu/src/bevy_time/lib.rs.html#1-415)

## Bevy Time

[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/bevyengine/bevy#license) [![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy_time) [![Downloads](https://img.shields.io/crates/d/bevy_time.svg)](https://crates.io/crates/bevy_time) [![Docs](https://docs.rs/bevy_time/badge.svg)](https://docs.rs/bevy_time/latest/bevy_time/) [![Discord](https://img.shields.io/discord/691052431525675048.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/bevy)

The built-in timekeeping plugin for the Bevy game engine.

## Modules

[common\_conditions](common_conditions/index.html "mod bevy::time::common_conditions")

Common run conditions

[prelude](prelude/index.html "mod bevy::time::prelude")

The time prelude.

## Structs

[DelayedCommandQueue](struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")

A component with a [`CommandQueue`](../ecs/world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue") to be submitted later.

[DelayedCommands](struct.DelayedCommands.html "struct bevy::time::DelayedCommands")

A wrapper over [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands") that stores [`CommandQueue`](../ecs/world/struct.CommandQueue.html "struct bevy::ecs::world::CommandQueue")s to be applied with given delays.

[Fixed](struct.Fixed.html "struct bevy::time::Fixed")

The fixed timestep game clock following virtual time.

[Real](struct.Real.html "struct bevy::time::Real")

Real time clock representing elapsed wall clock time.

[Stopwatch](struct.Stopwatch.html "struct bevy::time::Stopwatch")

A Stopwatch is a struct that tracks elapsed time when started.

[Time](struct.Time.html "struct bevy::time::Time")

A generic clock resource that tracks how much it has advanced since its previous update and since its creation.

[TimePlugin](struct.TimePlugin.html "struct bevy::time::TimePlugin")

Adds time functionality to Apps.

[TimeReceiver](struct.TimeReceiver.html "struct bevy::time::TimeReceiver")`std`

Channel resource used to receive time from the render world.

[TimeSender](struct.TimeSender.html "struct bevy::time::TimeSender")`std`

Channel resource used to send time from the render world.

[TimeSystems](struct.TimeSystems.html "struct bevy::time::TimeSystems")

Updates the elapsed time. Any system that interacts with [`Time`](../prelude/struct.Time.html "struct bevy::prelude::Time") component should run after this.

[Timer](struct.Timer.html "struct bevy::time::Timer")

Tracks elapsed time. Enters the finished state once `duration` is reached.

[Virtual](struct.Virtual.html "struct bevy::time::Virtual")

The virtual game clock representing game time.

## Enums

[TimeUpdateStrategy](enum.TimeUpdateStrategy.html "enum bevy::time::TimeUpdateStrategy")

Configuration resource used to determine how the time system should run.

[TimerMode](enum.TimerMode.html "enum bevy::time::TimerMode")

Specifies [`Timer`](../prelude/struct.Timer.html "struct bevy::prelude::Timer") behavior.

[TrySendError](enum.TrySendError.html "enum bevy::time::TrySendError")

An error returned from the [`try_send`](https://docs.rs/crossbeam-channel/0.5.15/x86_64-unknown-linux-gnu/crossbeam_channel/channel/struct.Sender.html#method.try_send "method crossbeam_channel::channel::Sender::try_send") method.

## Traits

[DelayedCommandsExt](trait.DelayedCommandsExt.html "trait bevy::time::DelayedCommandsExt")

Extension trait for [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands") that provides delayed command functionality.

## Functions

[check\_delayed\_command\_queues](fn.check_delayed_command_queues.html "fn bevy::time::check_delayed_command_queues")

The system used to check [`DelayedCommandQueue`](struct.DelayedCommandQueue.html "struct bevy::time::DelayedCommandQueue")s, which are usually spawned by [`DelayedCommands`](struct.DelayedCommands.html "struct bevy::time::DelayedCommands"). When the elapsed time exceeds a queue’s `submit_at` time, the contained `queue` is appended to the system’s [`Commands`](../prelude/struct.Commands.html "struct bevy::prelude::Commands").

[create\_time\_channels](fn.create_time_channels.html "fn bevy::time::create_time_channels")`std`

Creates channels used for sending time between the render world and the main world.

[run\_fixed\_main\_schedule](fn.run_fixed_main_schedule.html "fn bevy::time::run_fixed_main_schedule")

Runs [`FixedMain`](../app/struct.FixedMain.html "struct bevy::app::FixedMain") zero or more times based on delta of [`Time<Virtual>`](../prelude/struct.Virtual.html "struct bevy::prelude::Virtual") and [`Time::overstep`](../prelude/struct.Time.html#method.overstep "method bevy::prelude::Time::overstep"). You can order your systems relative to this by using [`RunFixedMainLoopSystems`](../prelude/enum.RunFixedMainLoopSystems.html "enum bevy::prelude::RunFixedMainLoopSystems").

[time\_system](fn.time_system.html "fn bevy::time::time_system")

The system used to update the [`Time`](../prelude/struct.Time.html "struct bevy::prelude::Time") used by app logic. If there is a render world the time is sent from there to this system through channels. Otherwise the time is updated in this system.

[update\_virtual\_time](fn.update_virtual_time.html "fn bevy::time::update_virtual_time")

Advances [`Time<Virtual>`](../prelude/struct.Time.html "struct bevy::prelude::Time") and [`Time`](../prelude/struct.Time.html "struct bevy::prelude::Time") based on the elapsed [`Time<Real>`](../prelude/struct.Time.html "struct bevy::prelude::Time").
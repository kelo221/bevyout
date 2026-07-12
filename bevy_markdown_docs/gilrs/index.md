[bevy](../index.html)

# Crate gilrs 

[Source](https://docs.rs/bevy_gilrs/0.19.0/x86_64-unknown-linux-gnu/src/bevy_gilrs/lib.rs.html#1-135)

Systems and type definitions for gamepad handling in Bevy.

This crate is built on top of [GilRs](https://docs.rs/gilrs/0.11.2/x86_64-unknown-linux-gnu/gilrs/index.html "mod gilrs"), a library that handles abstracting over platform-specific gamepad APIs.

## Structs

[GilrsPlugin](struct.GilrsPlugin.html "struct bevy::gilrs::GilrsPlugin")

Plugin that provides gamepad handling to an [`App`](../prelude/struct.App.html "struct bevy::prelude::App").

[RumbleSystems](struct.RumbleSystems.html "struct bevy::gilrs::RumbleSystems")

Updates the running gamepad rumble effects.
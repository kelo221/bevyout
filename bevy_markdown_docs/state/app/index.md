[bevy](../../index.html)::[state](../index.html)

# Module app 

[Source](https://docs.rs/bevy_state/0.19.0/x86_64-unknown-linux-gnu/src/bevy_state/lib.rs.html#55)

Available on **crate feature `bevy_app`** only.

Provides [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") and [`SubApp`](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp") with state installation methods

## Structs

[StatesPlugin](struct.StatesPlugin.html "struct bevy::state::app::StatesPlugin")

Registers the [`StateTransition`](../../prelude/struct.StateTransition.html "struct bevy::prelude::StateTransition") schedule in the [`MainScheduleOrder`](../../app/struct.MainScheduleOrder.html "struct bevy::app::MainScheduleOrder") to enable state processing.

## Traits

[AppExtStates](trait.AppExtStates.html "trait bevy::state::app::AppExtStates")

State installation methods for [`App`](../../prelude/struct.App.html "struct bevy::prelude::App") and [`SubApp`](../../prelude/struct.SubApp.html "struct bevy::prelude::SubApp").
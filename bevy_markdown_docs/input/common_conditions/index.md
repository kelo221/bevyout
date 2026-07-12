[bevy](../../index.html)::[input](../index.html)

# Module common\_conditions 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#23)

Common run conditions

## Functions

[input\_just\_pressed](fn.input_just_pressed.html "fn bevy::input::common_conditions::input_just_pressed")

Run condition that is active if [`ButtonInput::just_pressed`](../../prelude/struct.ButtonInput.html#method.just_pressed "method bevy::prelude::ButtonInput::just_pressed") is true for the given input.

[input\_just\_released](fn.input_just_released.html "fn bevy::input::common_conditions::input_just_released")

Run condition that is active if [`ButtonInput::just_released`](../../prelude/struct.ButtonInput.html#method.just_released "method bevy::prelude::ButtonInput::just_released") is true for the given input.

[input\_pressed](fn.input_pressed.html "fn bevy::input::common_conditions::input_pressed")

Run condition that is active if [`ButtonInput::pressed`](../../prelude/struct.ButtonInput.html#method.pressed "method bevy::prelude::ButtonInput::pressed") is true for the given input.

[input\_toggle\_active](fn.input_toggle_active.html "fn bevy::input::common_conditions::input_toggle_active")

Stateful run condition that can be toggled via an input press using [`ButtonInput::just_pressed`](../../prelude/struct.ButtonInput.html#method.just_pressed "method bevy::prelude::ButtonInput::just_pressed").
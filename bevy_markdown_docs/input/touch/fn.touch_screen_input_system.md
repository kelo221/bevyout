[bevy](../../index.html)::[input](../index.html)::[touch](index.html)

# Function touch\_screen\_input\_system 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/touch.rs.html#435-438)

```rust
pub fn touch_screen_input_system(
    touch_state: ResMut<'_, Touches>,
    touch_input_reader: MessageReader<'_, '_, TouchInput>,
)
```

Available on **crate features `mouse` or `touch`** only.

Updates the [`Touches`](../../prelude/struct.Touches.html "struct bevy::prelude::Touches") resource with the latest [`TouchInput`](../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput") events.

This is not clearing the `pressed` collection, because it could incorrectly mark a touch input as not pressed even though it is pressed. This could happen if the touch input is not moving for a single frame and would therefore be marked as not pressed, because this function is called on every single frame no matter if there was an event or not.

### Differences

The main difference between the [`TouchInput`](../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput") event and the [`Touches`](../../prelude/struct.Touches.html "struct bevy::prelude::Touches") resource is that the latter has convenient functions like [`Touches::just_pressed`](../../prelude/struct.Touches.html#method.just_pressed "method bevy::prelude::Touches::just_pressed") and [`Touches::just_released`](../../prelude/struct.Touches.html#method.just_released "method bevy::prelude::Touches::just_released").
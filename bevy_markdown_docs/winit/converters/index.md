[bevy](../../index.html)::[winit](../index.html)

# Module converters 

[Source](https://docs.rs/bevy_winit/0.19.0/x86_64-unknown-linux-gnu/src/bevy_winit/lib.rs.html#45)

Helpers for mapping between winit and bevy types

## Functions

[convert\_element\_state](fn.convert_element_state.html "fn bevy::winit::converters::convert_element_state")

Converts a [`winit::event::ElementState`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/enum.ElementState.html "enum winit::event::ElementState") to a Bevy [`ButtonState`](../../input/enum.ButtonState.html "enum bevy::input::ButtonState")

[convert\_enabled\_buttons](fn.convert_enabled_buttons.html "fn bevy::winit::converters::convert_enabled_buttons")

Converts a Bevy [`EnabledButtons`](../../window/struct.EnabledButtons.html "struct bevy::window::EnabledButtons") to a [`winit::window::WindowButtons`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/window/struct.WindowButtons.html "struct winit::window::WindowButtons")

[convert\_keyboard\_input](fn.convert_keyboard_input.html "fn bevy::winit::converters::convert_keyboard_input")

Converts a [`winit::event::KeyEvent`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/struct.KeyEvent.html "struct winit::event::KeyEvent") and a window [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to a Bevy [`KeyboardInput`](../../input/keyboard/struct.KeyboardInput.html "struct bevy::input::keyboard::KeyboardInput")

[convert\_logical\_key](fn.convert_logical_key.html "fn bevy::winit::converters::convert_logical_key")

Converts a [`winit::keyboard::Key`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.Key.html "enum winit::keyboard::Key") to a Bevy [`bevy_input::keyboard::Key`](../../input/keyboard/enum.Key.html "enum bevy::input::keyboard::Key")

[convert\_mouse\_button](fn.convert_mouse_button.html "fn bevy::winit::converters::convert_mouse_button")

Converts a [`winit::event::MouseButton`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/enum.MouseButton.html "enum winit::event::MouseButton") to a Bevy [`MouseButton`](../../prelude/enum.MouseButton.html "enum bevy::prelude::MouseButton")

[convert\_native\_key](fn.convert_native_key.html "fn bevy::winit::converters::convert_native_key")

Converts a [`winit::keyboard::NativeKey`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.NativeKey.html "enum winit::keyboard::NativeKey") to a Bevy [`NativeKey`](../../input/keyboard/enum.NativeKey.html "enum bevy::input::keyboard::NativeKey")

[convert\_physical\_key\_code](fn.convert_physical_key_code.html "fn bevy::winit::converters::convert_physical_key_code")

Converts a [`winit::keyboard::PhysicalKey`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.PhysicalKey.html "enum winit::keyboard::PhysicalKey") to a Bevy [`KeyCode`](../../prelude/enum.KeyCode.html "enum bevy::prelude::KeyCode")

[convert\_physical\_native\_key\_code](fn.convert_physical_native_key_code.html "fn bevy::winit::converters::convert_physical_native_key_code")

Converts a [`winit::keyboard::NativeKeyCode`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/keyboard/enum.NativeKeyCode.html "enum winit::keyboard::NativeKeyCode") to a Bevy [`NativeKeyCode`](../../input/keyboard/enum.NativeKeyCode.html "enum bevy::input::keyboard::NativeKeyCode")

[convert\_resize\_direction](fn.convert_resize_direction.html "fn bevy::winit::converters::convert_resize_direction")

Converts a Bevy [`CompassOctant`](../../math/enum.CompassOctant.html "enum bevy::math::CompassOctant") to a [`winit::window::ResizeDirection`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/window/enum.ResizeDirection.html "enum winit::window::ResizeDirection")

[convert\_system\_cursor\_icon](fn.convert_system_cursor_icon.html "fn bevy::winit::converters::convert_system_cursor_icon")

Converts a Bevy [`SystemCursorIcon`](../../window/enum.SystemCursorIcon.html "enum bevy::window::SystemCursorIcon") to a [`winit::window::CursorIcon`](https://docs.rs/cursor-icon/1.2.0/x86_64-unknown-linux-gnu/cursor_icon/enum.CursorIcon.html "enum cursor_icon::CursorIcon").

[convert\_touch\_input](fn.convert_touch_input.html "fn bevy::winit::converters::convert_touch_input")

Converts a [`winit::event::Touch`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/struct.Touch.html "struct winit::event::Touch"), [`winit::dpi::LogicalPosition<f64>`](https://docs.rs/dpi/0.1.2/x86_64-unknown-linux-gnu/dpi/struct.LogicalPosition.html "struct dpi::LogicalPosition") and window [`Entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity") to a Bevy [`TouchInput`](../../prelude/struct.TouchInput.html "struct bevy::prelude::TouchInput")

[convert\_touch\_phase](fn.convert_touch_phase.html "fn bevy::winit::converters::convert_touch_phase")

Converts a [`winit::event::TouchPhase`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/event/enum.TouchPhase.html "enum winit::event::TouchPhase") to a Bevy [`TouchPhase`](../../input/touch/enum.TouchPhase.html "enum bevy::input::touch::TouchPhase").

[convert\_window\_level](fn.convert_window_level.html "fn bevy::winit::converters::convert_window_level")

Converts a Bevy [`WindowLevel`](../../window/enum.WindowLevel.html "enum bevy::window::WindowLevel") to a [`winit::window::WindowLevel`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/window/enum.WindowLevel.html "enum winit::window::WindowLevel")

[convert\_window\_theme](fn.convert_window_theme.html "fn bevy::winit::converters::convert_window_theme")

Converts a Bevy [`WindowTheme`](../../window/enum.WindowTheme.html "enum bevy::window::WindowTheme") to a [`winit::window::Theme`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/window/enum.Theme.html "enum winit::window::Theme")

[convert\_winit\_theme](fn.convert_winit_theme.html "fn bevy::winit::converters::convert_winit_theme")

Converts a [`winit::window::Theme`](https://docs.rs/winit/0.30.13/x86_64-unknown-linux-gnu/winit/window/enum.Theme.html "enum winit::window::Theme") to a Bevy [`WindowTheme`](../../window/enum.WindowTheme.html "enum bevy::window::WindowTheme")
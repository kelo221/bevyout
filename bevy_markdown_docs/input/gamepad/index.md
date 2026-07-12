[bevy](../../index.html)::[input](../index.html)

# Module gamepad 

[Source](https://docs.rs/bevy_input/0.19.0/x86_64-unknown-linux-gnu/src/bevy_input/lib.rs.html#26)

Available on **crate feature `gamepad`** only.

The gamepad input functionality.

## Structs

[AxisSettings](struct.AxisSettings.html "struct bevy::input::gamepad::AxisSettings")

Settings for a [`GamepadAxis`](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis").

[ButtonAxisSettings](struct.ButtonAxisSettings.html "struct bevy::input::gamepad::ButtonAxisSettings")

Settings for a [`GamepadButton`](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton").

[ButtonSettings](struct.ButtonSettings.html "struct bevy::input::gamepad::ButtonSettings")

Manages settings for gamepad buttons.

[Gamepad](struct.Gamepad.html "struct bevy::input::gamepad::Gamepad")

Stores a connected gamepad’s metadata such as the name and its [`GamepadButton`](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton") and [`GamepadAxis`](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis").

[GamepadAxisChangedEvent](struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent")

[`GamepadAxis`](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis") event triggered by an analog state change.

[GamepadButtonChangedEvent](struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent")

[`GamepadButton`](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton") event triggered by an analog state change.

[GamepadButtonStateChangedEvent](struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent")

[`GamepadButton`](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton") event triggered by a digital state change.

[GamepadConnectionEvent](struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")

A [`Gamepad`](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad") connection event. Created when a connection to a gamepad is established and when a gamepad is disconnected.

[GamepadRumbleIntensity](struct.GamepadRumbleIntensity.html "struct bevy::input::gamepad::GamepadRumbleIntensity")

The intensity at which a gamepad’s force-feedback motors may rumble.

[GamepadSettings](struct.GamepadSettings.html "struct bevy::input::gamepad::GamepadSettings")

Gamepad settings component.

[RawGamepadAxisChangedEvent](struct.RawGamepadAxisChangedEvent.html "struct bevy::input::gamepad::RawGamepadAxisChangedEvent")

[`GamepadAxis`](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis") changed event unfiltered by [`GamepadSettings`](../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings").

[RawGamepadButtonChangedEvent](struct.RawGamepadButtonChangedEvent.html "struct bevy::input::gamepad::RawGamepadButtonChangedEvent")

[`GamepadButton`](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton") changed event unfiltered by [`GamepadSettings`](../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings").

## Enums

[AxisSettingsError](enum.AxisSettingsError.html "enum bevy::input::gamepad::AxisSettingsError")

Errors that occur when setting axis settings for gamepad input.

[ButtonSettingsError](enum.ButtonSettingsError.html "enum bevy::input::gamepad::ButtonSettingsError")

Errors that occur when setting button settings for gamepad input.

[GamepadAxis](enum.GamepadAxis.html "enum bevy::input::gamepad::GamepadAxis")

Represents gamepad input types that are mapped in the range \[-1.0, 1.0\].

[GamepadButton](enum.GamepadButton.html "enum bevy::input::gamepad::GamepadButton")

Represents gamepad input types that are mapped in the range \[0.0, 1.0\].

[GamepadConnection](enum.GamepadConnection.html "enum bevy::input::gamepad::GamepadConnection")

The connection status of a gamepad.

[GamepadEvent](enum.GamepadEvent.html "enum bevy::input::gamepad::GamepadEvent")

A gamepad event.

[GamepadInput](enum.GamepadInput.html "enum bevy::input::gamepad::GamepadInput")

Encapsulation over [`GamepadAxis`](../../prelude/enum.GamepadAxis.html "enum bevy::prelude::GamepadAxis") and [`GamepadButton`](../../prelude/enum.GamepadButton.html "enum bevy::prelude::GamepadButton").

[GamepadRumbleRequest](enum.GamepadRumbleRequest.html "enum bevy::input::gamepad::GamepadRumbleRequest")

An event that controls force-feedback rumbling of a [`Gamepad`](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad") [`entity`](../../prelude/struct.Entity.html "struct bevy::prelude::Entity").

[RawGamepadEvent](enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent")

A raw gamepad event.

## Functions

[gamepad\_connection\_system](fn.gamepad_connection_system.html "fn bevy::input::gamepad::gamepad_connection_system")

Handles [`GamepadConnectionEvent`](struct.GamepadConnectionEvent.html "struct bevy::input::gamepad::GamepadConnectionEvent")s events.

[gamepad\_event\_processing\_system](fn.gamepad_event_processing_system.html "fn bevy::input::gamepad::gamepad_event_processing_system")

Consumes [`RawGamepadEvent`](enum.RawGamepadEvent.html "enum bevy::input::gamepad::RawGamepadEvent") events, filters them using their [`GamepadSettings`](../../prelude/struct.GamepadSettings.html "struct bevy::prelude::GamepadSettings") and if successful, updates the [`Gamepad`](../../prelude/struct.Gamepad.html "struct bevy::prelude::Gamepad") and sends [`GamepadAxisChangedEvent`](struct.GamepadAxisChangedEvent.html "struct bevy::input::gamepad::GamepadAxisChangedEvent"), [`GamepadButtonStateChangedEvent`](struct.GamepadButtonStateChangedEvent.html "struct bevy::input::gamepad::GamepadButtonStateChangedEvent"), [`GamepadButtonChangedEvent`](struct.GamepadButtonChangedEvent.html "struct bevy::input::gamepad::GamepadButtonChangedEvent") events.